//! Find `/** @buildtime */`-annotated declarations in an OXC program.
//!
//! A `@buildtime` declaration is a top-level `const` (Tier 1) or `function`
//! (Tier 2) whose directly preceding doc comment contains `@buildtime`.
//! The pre-pass runs each one in the sandbox and replaces the declaration
//! with the serialized result.
//!
//! This module performs the AST walk and text extraction. Running the
//! sandbox and building patches happens in [`super::prepass`].

use std::borrow::Cow;

use oxc::ast::ast::{Declaration, Expression, Program, Statement, VariableDeclarationKind};

use crate::ts_syn::abi::SpanIR;

/// Which tier the declaration belongs to — decides how the sandbox
/// result is spliced back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildtimeKind {
    /// `const NAME = <expr>;` — evaluate `<expr>`, serialize the result,
    /// emit `const NAME = <literal>;`.
    Tier1Const,
    /// `function NAME() { <body> }` — evaluate the body, and:
    /// * if the return value is a string, splice it verbatim;
    /// * otherwise, serialize the value and emit `const NAME = <literal>;`.
    Tier2Function,
    /// `type NAME = <expr>;` — evaluate `<expr>` (which must return a
    /// string of TypeScript type syntax), splice the returned text as
    /// the RHS. Emit `type NAME = <returned>;`.
    ///
    /// If the evaluated expression returns a non-string, it's an error
    /// (TS types can't be expressed as arbitrary runtime values).
    Tier3Type,
}

/// Visibility of the original declaration. Preserved on the rewritten
/// output so `export const` → `export const` rather than losing the
/// export keyword.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Private,
    Export,
}

/// A single `@buildtime` declaration discovered during the walk.
#[derive(Debug, Clone)]
pub struct BuildtimeDecl {
    pub kind: BuildtimeKind,
    pub visibility: Visibility,
    /// Identifier the declaration bound. For `const X = ...`, this is
    /// `"X"`; for `function f() { ... }`, `"f"`.
    pub name: String,
    /// Span of the entire statement that should be replaced by the
    /// serialized result (including the leading `export` keyword when
    /// present). 0-based byte offsets into the original source.
    pub decl_span: SpanIR,
    /// Source text to pass to the sandbox. For Tier 1 this is `return <expr>;`,
    /// for Tier 2 it's the function body with its outer braces stripped.
    pub body_source: String,
}

/// Walk the program's top-level statements and return every
/// `@buildtime`-annotated declaration.
///
/// Returns an empty vector when the source contains no `@buildtime`
/// marker — callers should fast-path this by checking the source text
/// first; the walk is relatively cheap but allocation-free is cheaper.
pub fn discover(program: &Program<'_>, source: &str) -> Vec<BuildtimeDecl> {
    let mut out = Vec::new();
    for stmt in &program.body {
        if let Some(decl) = try_extract_decl(stmt, source) {
            out.push(decl);
        }
    }
    out
}

fn try_extract_decl(stmt: &Statement<'_>, source: &str) -> Option<BuildtimeDecl> {
    let (visibility, inner_kind) = match stmt {
        Statement::VariableDeclaration(var_decl) => {
            (Visibility::Private, StatementKind::Var(var_decl))
        }
        Statement::FunctionDeclaration(func) => (Visibility::Private, StatementKind::Func(func)),
        Statement::TSTypeAliasDeclaration(ty) => {
            (Visibility::Private, StatementKind::TypeAlias(ty))
        }
        Statement::ExportNamedDeclaration(export) => match &export.declaration {
            Some(Declaration::VariableDeclaration(var_decl)) => {
                (Visibility::Export, StatementKind::Var(var_decl))
            }
            Some(Declaration::FunctionDeclaration(func)) => {
                (Visibility::Export, StatementKind::Func(func))
            }
            Some(Declaration::TSTypeAliasDeclaration(ty)) => {
                (Visibility::Export, StatementKind::TypeAlias(ty))
            }
            _ => return None,
        },
        _ => return None,
    };

    let stmt_span = stmt_byte_span(stmt);
    // `has_buildtime_annotation` walks bytes of `source` directly, so
    // it needs a 0-based offset (SpanIR is 1-based — subtract).
    if !has_buildtime_annotation(source, stmt_span.start.saturating_sub(1)) {
        return None;
    }
    // Extend the span backward to swallow the `/** @buildtime */`
    // JSDoc itself. Without this step the comment survives the
    // rewrite and downstream passes (attribute macro dispatch,
    // declarative scanner) interpret `@buildtime` as a macro name.
    let stmt_span = with_jsdoc_lead(source, stmt_span);

    match inner_kind {
        StatementKind::Var(var_decl) => {
            // `const NAME = EXPR;` is the only shape we accept for Tier 1 —
            // destructuring, multiple bindings, and missing initializers
            // all mean the user's code can't be faithfully lowered to a
            // single literal.
            if var_decl.kind != VariableDeclarationKind::Const {
                return None;
            }
            if var_decl.declarations.len() != 1 {
                return None;
            }
            let declarator = &var_decl.declarations[0];
            let name = declarator.id.get_identifier_name()?.to_string();
            let init = declarator.init.as_ref()?;
            // Unwrap TS-only wrappers (`expr as T`, `expr satisfies T`,
            // `expr!`, `<T>expr`) so the JS-only Boa parser sees only
            // the runtime expression. Without this step, a user writing
            // `const X = foo() as Bar` would feed `as Bar` to Boa and
            // get a syntax error.
            let runtime_expr = unwrap_ts_expression(init);
            let expr_span = expression_span(runtime_expr);
            let expr_text = &source[expr_span.0..expr_span.1];
            let body_source = format!("return ({});", expr_text);
            Some(BuildtimeDecl {
                kind: BuildtimeKind::Tier1Const,
                visibility,
                name,
                decl_span: stmt_span,
                body_source,
            })
        }
        StatementKind::Func(func) => {
            let name = func.id.as_ref()?.name.to_string();
            let body = func.body.as_ref()?;
            // The function body span covers the braces. Strip them so
            // the body becomes a sequence of statements we can wrap in
            // our own IIFE.
            let (body_start, body_end) = (body.span.start as usize, body.span.end as usize);
            if body_end <= body_start + 1 {
                return None;
            }
            let inner = &source[body_start + 1..body_end - 1];
            Some(BuildtimeDecl {
                kind: BuildtimeKind::Tier2Function,
                visibility,
                name,
                decl_span: stmt_span,
                body_source: inner.to_string(),
            })
        }
        StatementKind::TypeAlias(ty) => {
            use oxc::span::GetSpan;
            let name = ty.id.name.to_string();
            // Tier 3 semantics: the RHS is a TS type. But TS types
            // aren't valid JS expressions, so we can't just `return (T);`
            // like Tier 1 does.
            //
            // The clean design uses Tier 2 structure instead: we
            // interpret the RHS as a *type-shaped string literal* whose
            // content gets spliced verbatim as the new type's RHS. The
            // user writes:
            //
            //     /** @buildtime */
            //     type UserId = "string";
            //
            // The RHS is `"string"` (a TS string-literal type whose
            // contents happen to be a valid type expression). We
            // evaluate the string as a JS expression — same string —
            // and splice it. The sandbox's SandboxValue::String becomes
            // the spliced type text.
            //
            // For computed types, the user writes a template literal:
            //
            //     /** @buildtime */
            //     type Sig = `() => ${buildtime.fs.readText("./return.d.ts")}`;
            //
            // which evaluates the template at compile time and splices
            // the resulting string as the type.
            let ann_span = ty.type_annotation.span();
            let (ann_start, ann_end) = (ann_span.start as usize, ann_span.end as usize);
            let inner = &source[ann_start..ann_end];
            Some(BuildtimeDecl {
                kind: BuildtimeKind::Tier3Type,
                visibility,
                name,
                decl_span: stmt_span,
                body_source: format!("return ({});", inner),
            })
        }
    }
}

enum StatementKind<'a, 'b> {
    Var(&'b oxc::ast::ast::VariableDeclaration<'a>),
    Func(&'b oxc::ast::ast::Function<'a>),
    TypeAlias(&'b oxc::ast::ast::TSTypeAliasDeclaration<'a>),
}

fn stmt_byte_span(stmt: &Statement<'_>) -> SpanIR {
    let span = match stmt {
        Statement::VariableDeclaration(d) => d.span,
        Statement::FunctionDeclaration(d) => d.span,
        Statement::TSTypeAliasDeclaration(d) => d.span,
        Statement::ExportNamedDeclaration(d) => d.span,
        _ => oxc::span::Span::default(),
    };
    // SpanIR is 1-based (the patch applicator subtracts 1 before
    // indexing) — OXC is 0-based, so convert.
    SpanIR {
        start: span.start + 1,
        end: span.end + 1,
    }
}

fn expression_span(expr: &Expression<'_>) -> (usize, usize) {
    use oxc::span::GetSpan;
    let span = expr.span();
    (span.start as usize, span.end as usize)
}

/// Extend the declaration span backward to cover the immediately
/// preceding `/** @buildtime */` JSDoc comment, so the rewrite
/// erases it. Falls back to the original span when no leading
/// comment is found.
///
/// Also walks backward past the comment's start to swallow the
/// indentation on its line — without this step the patch leaves an
/// orphan run of spaces / tabs from the original comment's indent.
/// The newline before that indent is preserved so the surrounding
/// formatting stays intact.
///
/// `span` is in SpanIR's 1-based coordinates (the patch applicator
/// subtracts 1 internally), so we work in 0-based here and convert
/// back at the end.
fn with_jsdoc_lead(source: &str, span: SpanIR) -> SpanIR {
    let start_0 = (span.start as usize).saturating_sub(1);
    if start_0 == 0 || start_0 > source.len() {
        return span;
    }
    let search_area = &source[..start_0];
    let Some(comment_start) = search_area.rfind("/**") else {
        return span;
    };
    let rest = &search_area[comment_start..];
    let Some(end_rel) = rest.find("*/") else {
        return span;
    };
    let comment_close_abs = comment_start + end_rel + 2;
    if !source[comment_close_abs..start_0]
        .chars()
        .all(char::is_whitespace)
    {
        // Comment isn't immediately before the decl — leave the span alone.
        return span;
    }
    // Walk backward from `comment_start` over horizontal whitespace
    // (spaces, tabs) to capture the indent. Stop at the newline.
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
    SpanIR {
        start: (new_start + 1) as u32,
        end: span.end,
    }
}

/// Recursively strip TypeScript-only expression wrappers, returning
/// the underlying JS expression. Keeps unwrapping until we hit a node
/// that's valid JS syntax — `as X`, `satisfies X`, `!`, and `<X>expr`
/// all peel off.
fn unwrap_ts_expression<'a>(expr: &'a Expression<'a>) -> &'a Expression<'a> {
    match expr {
        Expression::TSAsExpression(e) => unwrap_ts_expression(&e.expression),
        Expression::TSSatisfiesExpression(e) => unwrap_ts_expression(&e.expression),
        Expression::TSNonNullExpression(e) => unwrap_ts_expression(&e.expression),
        Expression::TSTypeAssertion(e) => unwrap_ts_expression(&e.expression),
        Expression::TSInstantiationExpression(e) => unwrap_ts_expression(&e.expression),
        other => other,
    }
}

/// True if a `/** @buildtime */` JSDoc comment is *immediately*
/// before `decl_start` — that is, the comment's closing `*/` and the
/// decl's start are separated only by whitespace.
///
/// This guards against false positives where an earlier `@buildtime`
/// comment attached to a different decl bleeds onto a later, unrelated
/// declaration. Mirrors how derive_targets.rs validates JSDoc
/// adjacency.
fn has_buildtime_annotation(source: &str, decl_start: u32) -> bool {
    let start = decl_start.saturating_sub(1) as usize;
    if start == 0 || start > source.len() {
        return false;
    }
    let search_area = &source[..start];
    let Some(comment_start) = search_area.rfind("/**") else {
        return false;
    };
    let rest = &search_area[comment_start..];
    let Some(end_rel) = rest.find("*/") else {
        return false;
    };
    // Adjacency check: between the comment's closing `*/` and the
    // declaration's start, only whitespace is allowed. Anything else
    // (semicolons, other tokens, another decl) means the comment
    // belongs to a *different* declaration and should not bind here.
    let comment_close_abs = comment_start + end_rel + 2;
    let between = &source[comment_close_abs..decl_start as usize];
    if !between.chars().all(char::is_whitespace) {
        return false;
    }
    let comment_body = &rest[3..end_rel];
    let body_text = normalize_jsdoc(comment_body);
    body_text.contains("@buildtime")
}

/// Strip JSDoc `*` line prefixes and collapse whitespace.
fn normalize_jsdoc(body: &str) -> Cow<'_, str> {
    if !body.contains('*') {
        return Cow::Borrowed(body);
    }
    let mut out = String::with_capacity(body.len());
    for line in body.lines() {
        let cleaned = line.trim().trim_start_matches('*').trim();
        out.push_str(cleaned);
        out.push('\n');
    }
    Cow::Owned(out)
}
