//! Walk an OXC program and collect every JSDoc `@cfg / @deprecated / @mustUse
//! / @nonExhaustive` annotation paired with the declaration it sits on.
//!
//! The walk is intentionally close to `crate::host::buildtime::discovery` so
//! span bookkeeping stays consistent across pre-passes: spans come out as
//! [`SpanIR`] (1-based) because that's what the patch applicator consumes.

use oxc::ast::ast::{Declaration, Program, Statement};
use oxc::span::GetSpan;

use crate::ts_syn::abi::SpanIR;

/// Which attribute macro fired.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeKind {
    /// `@cfg({ key: value, ... })` — predicate; strips the decl on mismatch.
    Cfg,
    /// `@deprecated('message', { since: '...' })` — emits JSDoc + runtime warn.
    Deprecated,
    /// `@mustUse` or `@mustUse('reason')` — flags calls whose return is discarded.
    MustUse,
    /// `@nonExhaustive` — intersects the target type with a brand sentinel.
    NonExhaustive,
}

/// Which kind of declaration the annotation sits on. Used by later passes to
/// decide whether a given annotation is even applicable (e.g. `@nonExhaustive`
/// only makes sense on type aliases).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclKind {
    /// `function name(...) { ... }`
    Function,
    /// `const name = ...`
    Const,
    /// `type name = ...`
    TypeAlias,
    /// `class name { ... }`
    Class,
    /// Any other declaration type we still want to track for consistency;
    /// later passes typically ignore it.
    Other,
}

/// Whether the declaration was exported.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Private,
    Export,
}

/// A single discovered attribute annotation.
#[derive(Debug, Clone)]
pub struct AttributeAnnotation {
    pub kind: AttributeKind,
    pub decl_kind: DeclKind,
    pub visibility: Visibility,
    /// The name of the declaration (e.g. `render`, `UserKind`). Empty for
    /// anonymous declarations (rare at the top level; preserved as `""`).
    pub name: String,
    /// Raw parenthesised argument text for the annotation, without the outer
    /// parens. `None` when the annotation is written bare (`@mustUse`).
    pub args_raw: Option<String>,
    /// Span of the owning declaration, in SpanIR coords. Used by `@cfg` to
    /// Patch::Delete and as the de-duplication key in [`mod.rs`](super::mod).
    pub decl_span: SpanIR,
    /// Span of the leading JSDoc block (the one carrying the annotation),
    /// extended to include indentation on its line.
    pub jsdoc_span: SpanIR,
    /// Span of the type-alias RHS, when the annotation sits on a type alias.
    /// `@nonExhaustive` uses this to patch just the RHS.
    pub type_alias_rhs_span: Option<SpanIR>,
}

impl AttributeAnnotation {
    /// Returns a stable identifier for the declaration this annotation
    /// targets. Callers use this to deduplicate multiple annotations that
    /// share a decl (e.g. `@mustUse @deprecated`).
    pub fn owner_span(&self) -> (u32, u32) {
        (self.decl_span.start, self.decl_span.end)
    }
}

/// Walk the program's top-level statements and emit one
/// [`AttributeAnnotation`] per matching JSDoc tag.
pub fn discover(program: &Program<'_>, source: &str) -> Vec<AttributeAnnotation> {
    // Cheap reject: if none of the tag heads appear in the source, skip the walk.
    if !source.contains('@') {
        return Vec::new();
    }
    let has_any = ["@cfg", "@deprecated", "@mustUse", "@nonExhaustive"]
        .iter()
        .any(|tag| source.contains(tag));
    if !has_any {
        return Vec::new();
    }

    let mut out = Vec::new();
    for stmt in &program.body {
        collect_from_statement(stmt, source, &mut out);
    }
    out
}

fn collect_from_statement(stmt: &Statement<'_>, source: &str, out: &mut Vec<AttributeAnnotation>) {
    let (visibility, decl_kind, name, type_alias_rhs_span) = match stmt {
        Statement::VariableDeclaration(d) => {
            let name = d
                .declarations
                .first()
                .and_then(|decl| decl.id.get_identifier_name())
                .map(|s| s.to_string())
                .unwrap_or_default();
            (Visibility::Private, DeclKind::Const, name, None)
        }
        Statement::FunctionDeclaration(d) => {
            let name =
                d.id.as_ref()
                    .map(|id| id.name.to_string())
                    .unwrap_or_default();
            (Visibility::Private, DeclKind::Function, name, None)
        }
        Statement::TSTypeAliasDeclaration(d) => {
            let name = d.id.name.to_string();
            let rhs = d.type_annotation.span();
            (
                Visibility::Private,
                DeclKind::TypeAlias,
                name,
                Some(SpanIR {
                    start: rhs.start + 1,
                    end: rhs.end + 1,
                }),
            )
        }
        Statement::ClassDeclaration(d) => {
            let name =
                d.id.as_ref()
                    .map(|id| id.name.to_string())
                    .unwrap_or_default();
            (Visibility::Private, DeclKind::Class, name, None)
        }
        Statement::ExportNamedDeclaration(export) => match &export.declaration {
            Some(Declaration::VariableDeclaration(d)) => {
                let name = d
                    .declarations
                    .first()
                    .and_then(|decl| decl.id.get_identifier_name())
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                (Visibility::Export, DeclKind::Const, name, None)
            }
            Some(Declaration::FunctionDeclaration(d)) => {
                let name =
                    d.id.as_ref()
                        .map(|id| id.name.to_string())
                        .unwrap_or_default();
                (Visibility::Export, DeclKind::Function, name, None)
            }
            Some(Declaration::TSTypeAliasDeclaration(d)) => {
                let name = d.id.name.to_string();
                let rhs = d.type_annotation.span();
                (
                    Visibility::Export,
                    DeclKind::TypeAlias,
                    name,
                    Some(SpanIR {
                        start: rhs.start + 1,
                        end: rhs.end + 1,
                    }),
                )
            }
            Some(Declaration::ClassDeclaration(d)) => {
                let name =
                    d.id.as_ref()
                        .map(|id| id.name.to_string())
                        .unwrap_or_default();
                (Visibility::Export, DeclKind::Class, name, None)
            }
            _ => return,
        },
        _ => return,
    };

    let stmt_span = stmt_byte_span(stmt);
    let Some((jsdoc_span, tags)) = find_leading_jsdoc_with_tags(source, stmt_span.start) else {
        return;
    };

    for (kind, args_raw) in tags {
        out.push(AttributeAnnotation {
            kind,
            decl_kind,
            visibility,
            name: name.clone(),
            args_raw,
            decl_span: stmt_span,
            jsdoc_span,
            type_alias_rhs_span,
        });
    }
}

fn stmt_byte_span(stmt: &Statement<'_>) -> SpanIR {
    let span = match stmt {
        Statement::VariableDeclaration(d) => d.span,
        Statement::FunctionDeclaration(d) => d.span,
        Statement::TSTypeAliasDeclaration(d) => d.span,
        Statement::ClassDeclaration(d) => d.span,
        Statement::ExportNamedDeclaration(d) => d.span,
        _ => oxc::span::Span::default(),
    };
    SpanIR {
        start: span.start + 1,
        end: span.end + 1,
    }
}

/// One JSDoc tag that `find_leading_jsdoc_with_tags` extracted from a comment:
/// the kind of annotation plus the raw parenthesised-args text, if any.
type JsDocTag = (AttributeKind, Option<String>);

/// Return the JSDoc comment immediately preceding `decl_start` (1-based) plus
/// every attribute tag it contains. The JSDoc span is extended backward to
/// capture leading indentation so strip-patches don't leave orphan spaces.
fn find_leading_jsdoc_with_tags(source: &str, decl_start: u32) -> Option<(SpanIR, Vec<JsDocTag>)> {
    let start_0 = (decl_start as usize).saturating_sub(1);
    if start_0 == 0 || start_0 > source.len() {
        return None;
    }
    let search_area = &source[..start_0];
    let comment_start = search_area.rfind("/**")?;
    let rest = &search_area[comment_start..];
    let end_rel = rest.find("*/")?;
    let comment_close_abs = comment_start + end_rel + 2;
    if !source[comment_close_abs..start_0]
        .chars()
        .all(char::is_whitespace)
    {
        return None;
    }

    // Walk back over horizontal whitespace to grab indentation.
    let mut new_start = comment_start;
    let bytes = source.as_bytes();
    while new_start > 0 {
        let b = bytes[new_start - 1];
        if b == b' ' || b == b'\t' {
            new_start -= 1;
        } else {
            break;
        }
    }

    let body = &rest[3..end_rel];
    let tags = extract_attribute_tags(body);
    if tags.is_empty() {
        return None;
    }

    Some((
        SpanIR {
            start: (new_start + 1) as u32,
            end: (comment_close_abs + 1) as u32,
        },
        tags,
    ))
}

/// Parse the JSDoc body and return every `@cfg(...) / @deprecated(...) /
/// @mustUse[(...)] / @nonExhaustive` tag it contains. JSDoc bodies can span
/// multiple lines with leading `*`; we strip those first.
fn extract_attribute_tags(body: &str) -> Vec<JsDocTag> {
    // Strip leading `*` / whitespace on each line, then join back together.
    let normalized: String = body
        .lines()
        .map(|line| line.trim().trim_start_matches('*').trim_start().to_string())
        .collect::<Vec<_>>()
        .join(" ");

    let mut out = Vec::new();
    let mut i = 0;
    let bytes = normalized.as_bytes();
    while i < bytes.len() {
        if bytes[i] != b'@' {
            i += 1;
            continue;
        }
        let tag_start = i + 1;
        let mut tag_end = tag_start;
        while tag_end < bytes.len()
            && (bytes[tag_end].is_ascii_alphanumeric() || bytes[tag_end] == b'_')
        {
            tag_end += 1;
        }
        let tag_name = &normalized[tag_start..tag_end];
        let kind = match tag_name {
            "cfg" => Some(AttributeKind::Cfg),
            "deprecated" => Some(AttributeKind::Deprecated),
            "mustUse" => Some(AttributeKind::MustUse),
            "nonExhaustive" => Some(AttributeKind::NonExhaustive),
            _ => None,
        };

        if let Some(kind) = kind {
            let (args, consumed) = parse_parenthesised(&normalized[tag_end..]);
            out.push((kind, args));
            i = tag_end + consumed;
        } else {
            i = tag_end;
        }
    }
    out
}

/// Read an optional `( ... )` starting at the beginning of `rest`, respecting
/// balanced parens and double/single-quoted strings. Returns the contents
/// without outer parens plus the number of bytes consumed.
fn parse_parenthesised(rest: &str) -> (Option<String>, usize) {
    let bytes = rest.as_bytes();
    // Skip whitespace.
    let mut i = 0;
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
        i += 1;
    }
    if i >= bytes.len() || bytes[i] != b'(' {
        return (None, 0);
    }
    let open = i;
    i += 1;
    let content_start = i;
    let mut depth = 1i32;
    let mut in_string: Option<u8> = None;
    while i < bytes.len() {
        let b = bytes[i];
        if let Some(quote) = in_string {
            if b == b'\\' {
                i += 2;
                continue;
            }
            if b == quote {
                in_string = None;
            }
        } else {
            match b {
                b'"' | b'\'' | b'`' => in_string = Some(b),
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        let content = &rest[content_start..i];
                        let consumed = i + 1 - open;
                        return (Some(content.to_string()), consumed);
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    // Unbalanced parens — treat as no args.
    (None, 0)
}
