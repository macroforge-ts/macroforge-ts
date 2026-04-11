//! Phase 13 — type-position macro walker.
//!
//! Walks a parsed OXC program looking for `$name<...>` uses inside type
//! annotations. When a `TSTypeReference` resolves to a declarative
//! macro with `kind: "type"`, the walker matches the type parameter
//! list against the macro's arms, expands the matched arm's body, and
//! emits a `Patch::Replace` over the full type reference span.
//!
//! Value-position macros use a completely separate walker
//! (`rewriter.rs`). The two walks share the registry, the expander,
//! and the patch output vector, but walk different AST node types and
//! use sibling matcher functions that know how to bind type fragments.

use std::collections::HashSet;

use oxc::ast::ast::{
    Declaration, ExportDefaultDeclarationKind, Program, Statement, TSSignature, TSTupleElement,
    TSType, TSTypeName,
};

use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, Patch, PatchCode, SpanIR};
use crate::ts_syn::declarative::MacroKind;

use super::expander::{ExpansionContext, expand_body_with_registry};
use super::matcher::{MatchError, match_type_invocation_against_arms};
use super::registry::DeclarativeMacroRegistry;
use super::rewriter::RewriteOutput;

/// Entry point. Walks every top-level statement that can contain a TS
/// type and rewrites any type-position macro invocations found inside.
pub fn walk_type_positions(
    program: &Program<'_>,
    source: &str,
    registry: &DeclarativeMacroRegistry,
    out: &mut RewriteOutput,
) {
    // Skip the walk entirely if the registry has no type-position
    // macros. This is a pure savings — value-only files never parse
    // their type annotations in this pass.
    let has_type_macros = registry
        .iter()
        .any(|(_, def)| def.kind == MacroKind::Type);
    if !has_type_macros {
        return;
    }

    let mut ctx = TypeWalkCtx {
        registry,
        source,
        output: out,
        counter: 0,
        // Track rewritten TSTypeReference spans so a given reference is
        // never patched twice (e.g. if the same node shows up in two
        // reachable positions).
        rewritten: HashSet::new(),
    };
    for stmt in &program.body {
        walk_stmt(stmt, &mut ctx);
    }
}

struct TypeWalkCtx<'a> {
    registry: &'a DeclarativeMacroRegistry,
    source: &'a str,
    output: &'a mut RewriteOutput,
    counter: u32,
    rewritten: HashSet<(u32, u32)>,
}

impl TypeWalkCtx<'_> {
    fn next_id(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }
}

fn walk_stmt(stmt: &Statement<'_>, ctx: &mut TypeWalkCtx<'_>) {
    match stmt {
        Statement::TSTypeAliasDeclaration(alias) => {
            walk_type(&alias.type_annotation, ctx);
        }
        Statement::TSInterfaceDeclaration(iface) => {
            for member in &iface.body.body {
                walk_signature(member, ctx);
            }
        }
        Statement::VariableDeclaration(var) => {
            for decl in &var.declarations {
                if let Some(ann) = &decl.type_annotation {
                    walk_type(&ann.type_annotation, ctx);
                }
            }
        }
        Statement::FunctionDeclaration(func) => {
            // Walk parameter types.
            for param in &func.params.items {
                if let Some(ann) = &param.type_annotation {
                    walk_type(&ann.type_annotation, ctx);
                }
            }
            // Return type.
            if let Some(ret) = &func.return_type {
                walk_type(&ret.type_annotation, ctx);
            }
            // Body statements may contain nested types (local type
            // aliases, variable declarations, etc.).
            if let Some(body) = &func.body {
                for s in &body.statements {
                    walk_stmt(s, ctx);
                }
            }
        }
        Statement::ClassDeclaration(class) => {
            for member in &class.body.body {
                walk_class_member(member, ctx);
            }
        }
        Statement::BlockStatement(block) => {
            for s in &block.body {
                walk_stmt(s, ctx);
            }
        }
        Statement::ExportNamedDeclaration(export) => {
            if let Some(decl) = &export.declaration {
                walk_declaration(decl, ctx);
            }
        }
        Statement::ExportDefaultDeclaration(export) => match &export.declaration {
            ExportDefaultDeclarationKind::FunctionDeclaration(func) => {
                for param in &func.params.items {
                    if let Some(ann) = &param.type_annotation {
                        walk_type(&ann.type_annotation, ctx);
                    }
                }
                if let Some(ret) = &func.return_type {
                    walk_type(&ret.type_annotation, ctx);
                }
            }
            ExportDefaultDeclarationKind::ClassDeclaration(class) => {
                for member in &class.body.body {
                    walk_class_member(member, ctx);
                }
            }
            ExportDefaultDeclarationKind::TSInterfaceDeclaration(iface) => {
                for member in &iface.body.body {
                    walk_signature(member, ctx);
                }
            }
            _ => {}
        },
        _ => {}
    }
}

fn walk_declaration(decl: &Declaration<'_>, ctx: &mut TypeWalkCtx<'_>) {
    match decl {
        Declaration::VariableDeclaration(var) => {
            for d in &var.declarations {
                if let Some(ann) = &d.type_annotation {
                    walk_type(&ann.type_annotation, ctx);
                }
            }
        }
        Declaration::TSTypeAliasDeclaration(alias) => {
            walk_type(&alias.type_annotation, ctx);
        }
        Declaration::TSInterfaceDeclaration(iface) => {
            for member in &iface.body.body {
                walk_signature(member, ctx);
            }
        }
        Declaration::FunctionDeclaration(func) => {
            for param in &func.params.items {
                if let Some(ann) = &param.type_annotation {
                    walk_type(&ann.type_annotation, ctx);
                }
            }
            if let Some(ret) = &func.return_type {
                walk_type(&ret.type_annotation, ctx);
            }
        }
        Declaration::ClassDeclaration(class) => {
            for member in &class.body.body {
                walk_class_member(member, ctx);
            }
        }
        _ => {}
    }
}

fn walk_signature(sig: &TSSignature<'_>, ctx: &mut TypeWalkCtx<'_>) {
    match sig {
        TSSignature::TSPropertySignature(p) => {
            if let Some(ann) = &p.type_annotation {
                walk_type(&ann.type_annotation, ctx);
            }
        }
        TSSignature::TSMethodSignature(m) => {
            for param in &m.params.items {
                if let Some(ann) = &param.type_annotation {
                    walk_type(&ann.type_annotation, ctx);
                }
            }
            if let Some(ret) = &m.return_type {
                walk_type(&ret.type_annotation, ctx);
            }
        }
        TSSignature::TSIndexSignature(idx) => {
            walk_type(&idx.type_annotation.type_annotation, ctx);
        }
        _ => {}
    }
}

fn walk_class_member(member: &oxc::ast::ast::ClassElement<'_>, ctx: &mut TypeWalkCtx<'_>) {
    use oxc::ast::ast::ClassElement;
    match member {
        ClassElement::PropertyDefinition(p) => {
            if let Some(ann) = &p.type_annotation {
                walk_type(&ann.type_annotation, ctx);
            }
        }
        ClassElement::MethodDefinition(m) => {
            for param in &m.value.params.items {
                if let Some(ann) = &param.type_annotation {
                    walk_type(&ann.type_annotation, ctx);
                }
            }
            if let Some(ret) = &m.value.return_type {
                walk_type(&ret.type_annotation, ctx);
            }
        }
        _ => {}
    }
}

/// Recursively walk a type, attempting to rewrite any `TSTypeReference`
/// whose name resolves to a type-position macro. For compound types
/// (union, intersection, array, etc.) we descend into the children.
fn walk_type(ty: &TSType<'_>, ctx: &mut TypeWalkCtx<'_>) {
    match ty {
        TSType::TSTypeReference(tr) => {
            // First try to rewrite this reference as a macro call. If
            // it doesn't resolve to a macro, fall through and walk
            // into its type parameters — a macro might still appear
            // inside them.
            if try_rewrite_type_ref(tr, ctx) {
                return;
            }
            if let Some(params) = &tr.type_arguments {
                for p in &params.params {
                    walk_type(p, ctx);
                }
            }
        }
        TSType::TSUnionType(u) => {
            for t in &u.types {
                walk_type(t, ctx);
            }
        }
        TSType::TSIntersectionType(i) => {
            for t in &i.types {
                walk_type(t, ctx);
            }
        }
        TSType::TSParenthesizedType(p) => walk_type(&p.type_annotation, ctx),
        TSType::TSArrayType(a) => walk_type(&a.element_type, ctx),
        TSType::TSTupleType(tup) => {
            for el in &tup.element_types {
                walk_tuple_element(el, ctx);
            }
        }
        TSType::TSConditionalType(c) => {
            walk_type(&c.check_type, ctx);
            walk_type(&c.extends_type, ctx);
            walk_type(&c.true_type, ctx);
            walk_type(&c.false_type, ctx);
        }
        TSType::TSTypeOperatorType(op) => walk_type(&op.type_annotation, ctx),
        TSType::TSIndexedAccessType(idx) => {
            walk_type(&idx.object_type, ctx);
            walk_type(&idx.index_type, ctx);
        }
        // Leaf / not-worth-descending cases.
        _ => {}
    }
}

/// Walk a tuple element. `TSTupleElement` uses `inherit_variants!` in
/// OXC so it contains every `TSType` variant directly plus two
/// wrappers (`TSOptionalType`, `TSRestType`). For the wrappers we
/// unwrap and recurse; for everything else (the inherited `TSType`
/// variants) we give up — traversing them would require re-dispatch
/// through a `@inherit TSType` matcher and the MVP doesn't need tuple
/// elements containing type-macro calls.
fn walk_tuple_element(el: &TSTupleElement<'_>, ctx: &mut TypeWalkCtx<'_>) {
    match el {
        TSTupleElement::TSOptionalType(o) => walk_type(&o.type_annotation, ctx),
        TSTupleElement::TSRestType(r) => walk_type(&r.type_annotation, ctx),
        _ => {}
    }
}

/// Attempt to rewrite a `TSTypeReference` node as a type-position
/// macro invocation. Returns `true` if the reference was rewritten
/// (the caller should then skip its children).
fn try_rewrite_type_ref(
    tr: &oxc::ast::ast::TSTypeReference<'_>,
    ctx: &mut TypeWalkCtx<'_>,
) -> bool {
    // Resolve `type_name` to a bare `$identifier`. Qualified names
    // (`a.b.c`) can't declare macros in MVP.
    let TSTypeName::IdentifierReference(ident) = &tr.type_name else {
        return false;
    };
    let Some(macro_name) = ident.name.as_str().strip_prefix('$') else {
        return false;
    };
    let Some(def) = ctx.registry.lookup(macro_name) else {
        return false;
    };
    if def.kind != MacroKind::Type {
        // The macro exists but it's value-position only. Emit a hard
        // error so the user knows they're using it wrong.
        ctx.output.diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Error,
            message: format!(
                "macro `${}` is value-only; cannot use it in type position",
                macro_name
            ),
            span: Some(SpanIR::new(tr.span.start + 1, tr.span.end + 1)),
            notes: vec![],
            help: None,
        });
        return false;
    }

    // Dedupe: if we've already rewritten this exact span, skip.
    let key = (tr.span.start, tr.span.end);
    if !ctx.rewritten.insert(key) {
        return false;
    }

    // Extract the type parameters. `$Foo<A, B>` → the OXC Vec of A, B;
    // `$Foo` with no params uses the empty-pattern fast path since we
    // can't easily construct an OXC Vec outside its allocator.
    let result = match tr.type_arguments.as_ref() {
        Some(tp) => match_type_invocation_against_arms(&def.arms, &tp.params, ctx.source),
        None => match_type_invocation_empty(&def.arms),
    };

    match result {
        Ok((arm_index, bindings)) => {
            let arm = &def.arms[arm_index];
            // Type-position expansions use the dedicated `Type`
            // context so the expander doesn't apply the JS-level
            // IIFE wrap that expressions need for block bodies.
            let expansion_id = ctx.next_id();
            match expand_body_with_registry(
                &arm.body,
                &bindings,
                expansion_id,
                ExpansionContext::Type,
                0,
                Some(ctx.registry),
            ) {
                Ok(expanded) => {
                    ctx.output.patches.push(Patch::Replace {
                        span: SpanIR::new(tr.span.start + 1, tr.span.end + 1),
                        code: PatchCode::Text(expanded),
                        source_macro: Some(format!("${}", macro_name)),
                    });
                    true
                }
                Err(e) => {
                    ctx.output.diagnostics.push(Diagnostic {
                        level: DiagnosticLevel::Error,
                        message: format!(
                            "error expanding type-position macro `${}`: {}",
                            macro_name, e
                        ),
                        span: Some(SpanIR::new(tr.span.start + 1, tr.span.end + 1)),
                        notes: vec![],
                        help: None,
                    });
                    false
                }
            }
        }
        Err(MatchError::NoArmMatched { tried }) => {
            ctx.output.diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!(
                    "no arm of type-position macro `${}` matched its invocation; tried {} arm(s): {}",
                    macro_name,
                    tried.len(),
                    tried.join(" | ")
                ),
                span: Some(SpanIR::new(tr.span.start + 1, tr.span.end + 1)),
                notes: vec![],
                help: None,
            });
            false
        }
        Err(err) => {
            ctx.output.diagnostics.push(Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!(
                    "type-position macro `${}` match failed: {}",
                    macro_name, err
                ),
                span: Some(SpanIR::new(tr.span.start + 1, tr.span.end + 1)),
                notes: vec![],
                help: None,
            });
            false
        }
    }
}

/// Handle the zero-type-parameters case without constructing a fake
/// OXC `Vec`. Only arms whose pattern is `Empty` can match.
fn match_type_invocation_empty(
    arms: &[crate::ts_syn::declarative::MacroArm],
) -> Result<(usize, std::collections::HashMap<String, super::matcher::Binding>), MatchError> {
    use crate::ts_syn::declarative::Pattern;
    for (arm_index, arm) in arms.iter().enumerate() {
        if matches!(arm.pattern, Pattern::Empty) {
            return Ok((arm_index, std::collections::HashMap::new()));
        }
    }
    Err(MatchError::NoArmMatched {
        tried: vec!["()".to_string()],
    })
}
