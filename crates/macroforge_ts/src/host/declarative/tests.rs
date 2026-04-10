//! Integration tests for the declarative macro host.

use crate::ts_syn::abi::SpanIR;

use super::discovery::discover;
use super::expander::{ExpansionContext, expand_body};
use super::matcher::{Binding, BoundFragment, MatchError, match_invocation};
use super::registry::{DeclarativeMacroRegistry, RegistryError};
use super::rewriter::rewrite;
use crate::ts_syn::declarative::{BodyToken, FragmentKind, MacroDef};

use oxc::allocator::Allocator;
use oxc::parser::Parser;
use oxc::span::SourceType;
use std::collections::HashMap;

fn parse_program<'a>(allocator: &'a Allocator, source: &'a str) -> oxc::parser::ParserReturn<'a> {
    Parser::new(allocator, source, SourceType::ts()).parse()
}

#[test]
fn discovers_simple_macro() {
    let source = r#"import { macroRules } from "macroforge/rules";
const $vec = macroRules`
  () => []
`;
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    assert!(
        parsed.errors.is_empty(),
        "parse errors: {:?}",
        parsed.errors
    );
    let defs = discover(&parsed.program, source).expect("discover");
    assert_eq!(defs.len(), 1);
    assert_eq!(defs[0].def.name, "vec");
    assert_eq!(defs[0].def.arms.len(), 1);
}

#[test]
fn skips_files_without_rules_import() {
    let source = r#"const $vec = macroRules`() => []`;"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let defs = discover(&parsed.program, source).expect("discover");
    assert!(defs.is_empty());
}

#[test]
fn registry_rejects_duplicates() {
    let mut registry = DeclarativeMacroRegistry::new();
    let def1 = MacroDef {
        name: "vec".into(),
        arms: vec![],
        mode: crate::ts_syn::declarative::MacroMode::ExpandOnly,
        span: SpanIR::new(0, 10),
    };
    let def2 = def1.clone();
    registry.register(def1).unwrap();
    assert!(matches!(
        registry.register(def2),
        Err(RegistryError::DuplicateName(_))
    ));
}

#[test]
fn matcher_binds_single_fragment() {
    let source = r#"import { macroRules } from "macroforge/rules";
const $id = macroRules`
  ($x:Expr) => $x
`;
$id(1 + 2);
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let discovered = discover(&parsed.program, source).unwrap();
    assert_eq!(discovered.len(), 1);
    let def = &discovered[0].def;

    // Find the call expression `$id(1 + 2)` in the program.
    let call = find_first_call(&parsed.program).expect("call");
    let result = match_invocation(def, &call.arguments, source).expect("match");
    assert_eq!(result.arm_index, 0);
    assert!(result.bindings.contains_key("x"));
    match result.bindings.get("x").unwrap() {
        Binding::Single(frag) => assert_eq!(frag.source, "1 + 2"),
        _ => panic!("expected Single binding"),
    }
}

#[test]
fn matcher_repetition_collects_sequence() {
    let source = r#"import { macroRules } from "macroforge/rules";
const $vec = macroRules`
  ($($x:Expr),+) => [$($x),+]
`;
$vec(1, 2, 3);
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let discovered = discover(&parsed.program, source).unwrap();
    let def = &discovered[0].def;
    let call = find_first_call(&parsed.program).expect("call");
    let result = match_invocation(def, &call.arguments, source).expect("match");
    match result.bindings.get("x").unwrap() {
        Binding::Sequence(frags) => {
            assert_eq!(frags.len(), 3);
            assert_eq!(frags[0].source, "1");
            assert_eq!(frags[1].source, "2");
            assert_eq!(frags[2].source, "3");
        }
        _ => panic!("expected Sequence"),
    }
}

#[test]
fn matcher_no_arm_matches_returns_error() {
    let source = r#"import { macroRules } from "macroforge/rules";
const $only = macroRules`
  ($x:Expr) => $x
`;
$only(1, 2);
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let discovered = discover(&parsed.program, source).unwrap();
    let def = &discovered[0].def;
    let call = find_first_call(&parsed.program).expect("call");
    let err = match_invocation(def, &call.arguments, source).unwrap_err();
    assert!(matches!(err, MatchError::NoArmMatched { .. }));
}

#[test]
fn expander_single_substitution() {
    use crate::ts_syn::declarative::Body;
    let body = Body(vec![
        BodyToken::Literal("return ".to_string()),
        BodyToken::Substitution("x".to_string()),
        BodyToken::Literal(" + 1".to_string()),
    ]);
    let mut bindings = HashMap::new();
    bindings.insert(
        "x".to_string(),
        Binding::Single(BoundFragment {
            kind: FragmentKind::Expr,
            source: "5".to_string(),
            span: SpanIR::new(0, 0),
        }),
    );
    let out = expand_body(&body, &bindings, 7, ExpansionContext::Statement).unwrap();
    assert_eq!(out, "return 5 + 1");
}

#[test]
fn expander_hygiene_rewrites_double_underscore_idents() {
    use crate::ts_syn::declarative::Body;
    let body = Body(vec![BodyToken::Literal(
        "const __v = 1; __v + 2".to_string(),
    )]);
    let bindings = HashMap::new();
    let out = expand_body(&body, &bindings, 7, ExpansionContext::Statement).unwrap();
    assert!(out.contains("__v$7"), "got: {}", out);
    assert!(!out.contains(" __v "), "unrenamed __v in: {}", out);
}

#[test]
fn expander_expression_context_wraps_block_in_iife() {
    use crate::ts_syn::declarative::Body;
    let body = Body(vec![BodyToken::Literal("{ return 1; }".to_string())]);
    let bindings = HashMap::new();
    let out = expand_body(&body, &bindings, 1, ExpansionContext::Expression).unwrap();
    assert!(out.starts_with("(() => "), "got: {}", out);
    assert!(out.ends_with(")()"), "got: {}", out);
}

#[test]
fn expander_statement_context_no_iife() {
    use crate::ts_syn::declarative::Body;
    let body = Body(vec![BodyToken::Literal("{ return 1; }".to_string())]);
    let bindings = HashMap::new();
    let out = expand_body(&body, &bindings, 1, ExpansionContext::Statement).unwrap();
    assert_eq!(out, "{ return 1; }");
}

#[test]
fn expander_repetition_unrolls_sequence() {
    use crate::ts_syn::declarative::{Body, RepetitionKind};
    let body = Body(vec![BodyToken::Repetition {
        body: vec![
            BodyToken::Literal("push(".to_string()),
            BodyToken::Substitution("x".to_string()),
            BodyToken::Literal(");".to_string()),
        ],
        separator: Some(" ".to_string()),
        kind: RepetitionKind::OneOrMore,
    }]);
    let mut bindings = HashMap::new();
    bindings.insert(
        "x".to_string(),
        Binding::Sequence(vec![
            BoundFragment {
                kind: FragmentKind::Expr,
                source: "1".to_string(),
                span: SpanIR::new(0, 0),
            },
            BoundFragment {
                kind: FragmentKind::Expr,
                source: "2".to_string(),
                span: SpanIR::new(0, 0),
            },
            BoundFragment {
                kind: FragmentKind::Expr,
                source: "3".to_string(),
                span: SpanIR::new(0, 0),
            },
        ]),
    );
    let out = expand_body(&body, &bindings, 1, ExpansionContext::Statement).unwrap();
    assert_eq!(out, "push(1); push(2); push(3);");
}

#[test]
fn rewriter_end_to_end_vec_basic() {
    let source = r#"import { macroRules } from "macroforge/rules";
const $vec = macroRules`
  () => []
  ($($x:Expr),+) => [$($x),+]
`;
const xs = $vec(1, 2, 3);
const ys = $vec();
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    assert!(parsed.errors.is_empty());
    let discovered = discover(&parsed.program, source).unwrap();
    let mut registry = DeclarativeMacroRegistry::new();
    for dm in &discovered {
        registry.register(dm.def.clone()).unwrap();
    }
    let out = rewrite(&parsed.program, source, &registry, &discovered);
    // Expect: 1 Delete (macro def) + 2 Replaces (two call sites) = 3 patches
    assert_eq!(out.patches.len(), 3, "patches: {:#?}", out.patches);
    assert!(out.diagnostics.is_empty(), "diag: {:?}", out.diagnostics);
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn find_first_call<'a>(
    program: &'a oxc::ast::ast::Program<'a>,
) -> Option<&'a oxc::ast::ast::CallExpression<'a>> {
    use oxc::ast::ast::{Expression, Statement};
    for stmt in &program.body {
        if let Statement::ExpressionStatement(es) = stmt
            && let Expression::CallExpression(call) = &es.expression
        {
            return Some(call);
        }
    }
    None
}
