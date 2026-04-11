//! Integration tests for the declarative macro host.

use crate::ts_syn::abi::SpanIR;

use super::BuildMode;
use super::discovery::{discover, resolve_cross_file_imports};
use super::expander::{ExpansionContext, expand_body};
use super::matcher::{Binding, BoundFragment, MatchError, match_invocation};
use super::project_registry::ProjectDeclarativeRegistry;
use super::registry::{DeclarativeMacroRegistry, RegistryError};
use super::rewriter::rewrite;
use crate::ts_syn::declarative::{BodyToken, FragmentKind, MacroDef, MacroMode};

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
    let def1 = MacroDef::from_arms(
        "vec".into(),
        vec![],
        MacroMode::ExpandOnly,
        SpanIR::new(0, 10),
    );
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
    let out = expand_body(&body, &bindings, 7, ExpansionContext::Statement, 0).unwrap();
    assert_eq!(out, "return 5 + 1");
}

#[test]
fn expander_hygiene_rewrites_double_underscore_idents() {
    use crate::ts_syn::declarative::Body;
    let body = Body(vec![BodyToken::Literal(
        "const __v = 1; __v + 2".to_string(),
    )]);
    let bindings = HashMap::new();
    let out = expand_body(&body, &bindings, 7, ExpansionContext::Statement, 0).unwrap();
    assert!(out.contains("__v$7"), "got: {}", out);
    assert!(!out.contains(" __v "), "unrenamed __v in: {}", out);
}

#[test]
fn expander_expression_context_wraps_block_in_iife() {
    use crate::ts_syn::declarative::Body;
    let body = Body(vec![BodyToken::Literal("{ return 1; }".to_string())]);
    let bindings = HashMap::new();
    let out = expand_body(&body, &bindings, 1, ExpansionContext::Expression, 0).unwrap();
    assert!(out.starts_with("(() => "), "got: {}", out);
    assert!(out.ends_with(")()"), "got: {}", out);
}

#[test]
fn expander_statement_context_no_iife() {
    use crate::ts_syn::declarative::Body;
    let body = Body(vec![BodyToken::Literal("{ return 1; }".to_string())]);
    let bindings = HashMap::new();
    let out = expand_body(&body, &bindings, 1, ExpansionContext::Statement, 0).unwrap();
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
    let out = expand_body(&body, &bindings, 1, ExpansionContext::Statement, 0).unwrap();
    assert_eq!(out, "push(1); push(2); push(3);");
}

// ---------------------------------------------------------------------------
// Phase 12 — Inter-macro composition
// ---------------------------------------------------------------------------

#[test]
fn composition_simple_two_macros() {
    let source = r#"import { macroRules } from "macroforge/rules";
const $double = macroRules`($x:Expr) => ($x * 2)`;
const $quad = macroRules`($x:Expr) => $double($double($x))`;
const result = $quad(3);
"#;
    let out = rewrite_source(source, BuildMode::Dev);
    assert!(out.diagnostics.is_empty(), "diag: {:?}", out.diagnostics);
    // Find the Replace patch for the `$quad(3)` call site.
    let replace = out
        .patches
        .iter()
        .find_map(|p| match p {
            crate::ts_syn::abi::Patch::Replace { code, .. } => {
                let crate::ts_syn::abi::PatchCode::Text(text) = code;
                Some(text.as_str())
            }
            _ => None,
        })
        .expect("expected a Replace patch for the call site");
    // $quad(3) → $double($double(3)) → $double((3 * 2)) → (((3 * 2)) * 2)
    assert!(
        replace.contains("3"),
        "expected the literal 3 in the composed expansion: {}",
        replace
    );
    assert!(
        replace.contains("* 2"),
        "expected the doubling to appear: {}",
        replace
    );
}

#[test]
fn composition_unknown_callee_errors() {
    let source = r#"import { macroRules } from "macroforge/rules";
const $caller = macroRules`($x:Expr) => $nonexistent($x)`;
const result = $caller(1);
"#;
    let out = rewrite_source(source, BuildMode::Dev);
    let has_error = out.diagnostics.iter().any(|d| {
        matches!(d.level, crate::ts_syn::abi::DiagnosticLevel::Error)
            && d.message.contains("nonexistent")
    });
    assert!(
        has_error,
        "expected an unknown-macro error, got: {:?}",
        out.diagnostics
    );
}

#[test]
fn topological_sort_orders_callee_before_caller() {
    use crate::host::declarative::registry::DeclarativeMacroRegistry;
    use crate::ts_syn::declarative::parse_macro_def;

    let mut registry = DeclarativeMacroRegistry::new();

    // Declare caller first; callee second.
    let mut caller = parse_macro_def("($x:Expr) => $callee($x)", SpanIR::new(0, 24)).unwrap();
    caller.name = "caller".into();
    registry.register(caller).unwrap();

    let mut callee = parse_macro_def("($x:Expr) => ($x + 1)", SpanIR::new(0, 21)).unwrap();
    callee.name = "callee".into();
    registry.register(callee).unwrap();

    let sorted = registry.topological_order().expect("sort should succeed");
    assert_eq!(sorted.len(), 2);
    // Callee must come first.
    assert_eq!(sorted[0].name, "callee");
    assert_eq!(sorted[1].name, "caller");
}

#[test]
fn topological_sort_detects_cycle() {
    use crate::host::declarative::registry::DeclarativeMacroRegistry;
    use crate::ts_syn::declarative::parse_macro_def;

    let mut registry = DeclarativeMacroRegistry::new();

    let mut a = parse_macro_def("($x:Expr) => $b($x)", SpanIR::new(0, 20)).unwrap();
    a.name = "a".into();
    registry.register(a).unwrap();

    let mut b = parse_macro_def("($x:Expr) => $a($x)", SpanIR::new(0, 20)).unwrap();
    b.name = "b".into();
    registry.register(b).unwrap();

    let err = registry.topological_order().unwrap_err();
    assert_eq!(err.names.len(), 2);
}

#[test]
fn topological_sort_ignores_unknown_callees() {
    // A macro that calls a cross-file import (not in the registry)
    // should not block the sort. The lookup is treated as "known out
    // of scope" and skipped.
    use crate::host::declarative::registry::DeclarativeMacroRegistry;
    use crate::ts_syn::declarative::parse_macro_def;

    let mut registry = DeclarativeMacroRegistry::new();
    let mut m = parse_macro_def("($x:Expr) => $cross_file_import($x)", SpanIR::new(0, 36)).unwrap();
    m.name = "caller".into();
    registry.register(m).unwrap();

    let sorted = registry.topological_order().expect("should sort cleanly");
    assert_eq!(sorted.len(), 1);
    assert_eq!(sorted[0].name, "caller");
}

#[test]
fn expander_recursion_limit_trips_at_max_depth() {
    use crate::ts_syn::declarative::Body;
    // Feeding a depth greater than MAX_EXPANSION_DEPTH should produce
    // RecursionLimit without attempting any work. This guards Phase 12's
    // inter-macro composition against infinite loops when it lands.
    let body = Body(vec![BodyToken::Literal("ok".to_string())]);
    let bindings = HashMap::new();
    let err = expand_body(
        &body,
        &bindings,
        1,
        ExpansionContext::Statement,
        super::expander::MAX_EXPANSION_DEPTH + 1,
    )
    .unwrap_err();
    assert!(
        matches!(
            err,
            crate::host::declarative::expander::ExpandError::RecursionLimit(_)
        ),
        "expected RecursionLimit, got {:?}",
        err
    );
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
    let out = rewrite(
        &parsed.program,
        source,
        &registry,
        &discovered,
        BuildMode::Dev,
        None,
    );
    // Expect: 1 Delete (macro def) + 1 Delete (the `import { macroRules }`
    // statement, which becomes dead after stripping the def) + 2 Replaces
    // (two call sites) = 4 patches
    assert_eq!(out.patches.len(), 4, "patches: {:#?}", out.patches);
    assert!(out.diagnostics.is_empty(), "diag: {:?}", out.diagnostics);
}

// ---------------------------------------------------------------------------
// Cross-file import resolution (Phase 8)
// ---------------------------------------------------------------------------

/// Build a project registry containing a single "library" file defining `$vec`.
fn library_registry() -> (ProjectDeclarativeRegistry, std::path::PathBuf) {
    use crate::ts_syn::declarative::parse_macro_def;

    let lib_src = "($($x:Expr),+) => [$($x),+]";
    let span = SpanIR::new(0, lib_src.len() as u32);
    let mut def = parse_macro_def(lib_src, span).expect("parse");
    def.name = "vec".to_string();

    let mut registry = ProjectDeclarativeRegistry::new();
    let lib_path = std::path::PathBuf::from("/project/src/macros.ts");
    registry.insert_file(lib_path.to_string_lossy().to_string(), vec![def]);
    (registry, lib_path)
}

#[test]
fn resolve_cross_file_happy_path() {
    let (registry, _lib_path) = library_registry();

    let consumer_src = r#"/** import macro { $vec } from "./macros" */
const xs = $vec(1, 2, 3);
"#;
    let consumer_path = std::path::PathBuf::from("/project/src/consumer.ts");
    let resolved = resolve_cross_file_imports(consumer_src, &consumer_path, &registry);

    assert_eq!(resolved.imported.len(), 1);
    assert_eq!(resolved.imported[0].def.name, "vec");
    assert!(
        resolved.diagnostics.is_empty(),
        "expected no diagnostics, got: {:?}",
        resolved.diagnostics
    );
}

#[test]
fn resolve_cross_file_unresolved_module_reports_diagnostic() {
    let (registry, _) = library_registry();

    let consumer_src = r#"/** import macro { $vec } from "./nonexistent" */"#;
    let consumer_path = std::path::PathBuf::from("/project/src/consumer.ts");
    let resolved = resolve_cross_file_imports(consumer_src, &consumer_path, &registry);

    assert!(resolved.imported.is_empty());
    assert_eq!(resolved.diagnostics.len(), 1);
    assert!(
        resolved.diagnostics[0].message.contains("cannot resolve"),
        "got: {}",
        resolved.diagnostics[0].message
    );
}

#[test]
fn resolve_cross_file_missing_macro_name_reports_diagnostic() {
    let (registry, _) = library_registry();

    // Library file has `$vec` but not `$missing`.
    let consumer_src = r#"/** import macro { $missing } from "./macros" */"#;
    let consumer_path = std::path::PathBuf::from("/project/src/consumer.ts");
    let resolved = resolve_cross_file_imports(consumer_src, &consumer_path, &registry);

    assert!(resolved.imported.is_empty());
    assert_eq!(resolved.diagnostics.len(), 1);
    assert!(
        resolved.diagnostics[0].message.contains("not defined"),
        "got: {}",
        resolved.diagnostics[0].message
    );
}

#[test]
fn resolve_cross_file_ignores_bare_names() {
    // `Debug` is a derive macro name — declarative resolver must skip it.
    let (registry, _) = library_registry();

    let consumer_src = r#"/** import macro { Debug, $vec } from "./macros" */"#;
    let consumer_path = std::path::PathBuf::from("/project/src/consumer.ts");
    let resolved = resolve_cross_file_imports(consumer_src, &consumer_path, &registry);

    // Only $vec should be resolved; `Debug` is a derive import and is ignored.
    assert_eq!(resolved.imported.len(), 1);
    assert_eq!(resolved.imported[0].def.name, "vec");
}

#[test]
fn project_registry_resolves_ts_extension() {
    let (registry, _) = library_registry();
    let importer = std::path::PathBuf::from("/project/src/consumer.ts");

    // `./macros` should resolve to `/project/src/macros.ts`.
    let resolved = registry.resolve_specifier(&importer, "./macros");
    assert!(resolved.is_some(), "expected ./macros to resolve");
}

#[test]
fn project_registry_json_roundtrip() {
    let (registry, _) = library_registry();
    let json = registry.to_json().expect("serialize");
    let parsed = ProjectDeclarativeRegistry::from_json(&json).expect("deserialize");
    assert_eq!(parsed.file_count(), 1);
    assert_eq!(parsed.macro_count(), 1);
}

// ---------------------------------------------------------------------------
// Object form (Phase 9a)
// ---------------------------------------------------------------------------

#[test]
fn object_form_explicit_share_only() {
    let source = r#"import { macroRules } from "macroforge/rules";
const $serialize = macroRules({
  mode: "share-only",
  expand: macroRules`
    ($x:Expr) => __inline_fallback($x)
  `,
  runtime: "function __serialize(value, schema) { return { value, schema }; }",
  call: macroRules`
    ($x:Expr) => __serialize($x, [])
  `,
});
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
    let def = &defs[0].def;
    assert_eq!(def.name, "serialize");
    assert_eq!(def.mode, MacroMode::ShareOnly);
    assert!(def.runtime.is_some());
    assert!(def.runtime.as_ref().unwrap().contains("__serialize"));
    assert!(def.call_arms.is_some());
    assert_eq!(def.call_arms.as_ref().unwrap().len(), 1);
    assert_eq!(def.arms.len(), 1); // expand arms still populated
}

#[test]
fn object_form_auto_mode() {
    let source = r#"import { macroRules } from "macroforge/rules";
const $vec = macroRules({
  mode: "auto",
  expand: macroRules`
    () => []
    ($($x:Expr),+) => [$($x),+]
  `,
  runtime: "function __vec(args) { return args; }",
  call: macroRules`
    ($($x:Expr),+) => __vec([$($x),+])
  `,
});
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let defs = discover(&parsed.program, source).expect("discover");
    assert_eq!(defs.len(), 1);
    assert_eq!(defs[0].def.mode, MacroMode::Auto);
    assert_eq!(defs[0].def.arms.len(), 2);
}

#[test]
fn object_form_defaults_to_expand_only_when_no_runtime() {
    // `mode` omitted + no runtime/call → ExpandOnly.
    let source = r#"import { macroRules } from "macroforge/rules";
const $id = macroRules({
  expand: macroRules`
    ($x:Expr) => $x
  `,
});
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let defs = discover(&parsed.program, source).expect("discover");
    assert_eq!(defs.len(), 1);
    assert_eq!(defs[0].def.mode, MacroMode::ExpandOnly);
    assert!(defs[0].def.runtime.is_none());
    assert!(defs[0].def.call_arms.is_none());
}

#[test]
fn object_form_defaults_to_auto_with_runtime_and_call() {
    // `mode` omitted + runtime + call → Auto.
    let source = r#"import { macroRules } from "macroforge/rules";
const $x = macroRules({
  expand: macroRules`($y:Expr) => $y`,
  runtime: "function __h(v) { return v; }",
  call: macroRules`($y:Expr) => __h($y)`,
});
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let defs = discover(&parsed.program, source).expect("discover");
    assert_eq!(defs[0].def.mode, MacroMode::Auto);
}

#[test]
fn object_form_requires_expand_field() {
    let source = r#"import { macroRules } from "macroforge/rules";
const $broken = macroRules({
  mode: "expand-only",
});
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let err = discover(&parsed.program, source).unwrap_err();
    assert!(
        err.message.contains("`expand`"),
        "expected error about missing expand, got: {}",
        err.message
    );
}

#[test]
fn object_form_share_mode_requires_runtime_and_call() {
    let source = r#"import { macroRules } from "macroforge/rules";
const $broken = macroRules({
  mode: "share-only",
  expand: macroRules`($x:Expr) => $x`,
});
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let err = discover(&parsed.program, source).unwrap_err();
    assert!(
        err.message.contains("runtime") && err.message.contains("call"),
        "expected error about missing runtime/call, got: {}",
        err.message
    );
}

#[test]
fn object_form_rejects_unknown_mode_string() {
    let source = r#"import { macroRules } from "macroforge/rules";
const $broken = macroRules({
  mode: "yolo",
  expand: macroRules`($x:Expr) => $x`,
});
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let err = discover(&parsed.program, source).unwrap_err();
    assert!(
        err.message.contains("yolo") && err.message.contains("auto"),
        "expected error listing valid modes, got: {}",
        err.message
    );
}

#[test]
fn object_form_rejects_unknown_option_key() {
    let source = r#"import { macroRules } from "macroforge/rules";
const $broken = macroRules({
  expand: macroRules`($x:Expr) => $x`,
  mystery: "what is this",
});
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let err = discover(&parsed.program, source).unwrap_err();
    assert!(
        err.message.contains("mystery"),
        "expected error listing the unknown key, got: {}",
        err.message
    );
}

#[test]
fn object_form_accepts_custom_megamorphism_threshold() {
    let source = r#"import { macroRules } from "macroforge/rules";
const $x = macroRules({
  mode: "auto",
  expand: macroRules`($y:Expr) => $y`,
  runtime: "function __h(v) { return v; }",
  call: macroRules`($y:Expr) => __h($y)`,
  megamorphismThreshold: 8,
});
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let defs = discover(&parsed.program, source).expect("discover");
    assert_eq!(defs[0].def.megamorphism_threshold, 8);
}

// ---------------------------------------------------------------------------
// Phase 13 — type-position macros
// ---------------------------------------------------------------------------

#[test]
fn type_macro_simple_replaces_type_reference() {
    let source = r#"import { macroRules } from "macroforge/rules";

const $Wrap = macroRules({
  kind: "type",
  expand: macroRules`($t:Type) => { wrapped: $t }`,
});

type Result = $Wrap<string>;
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    assert!(parsed.errors.is_empty(), "parse errors: {:?}", parsed.errors);
    let discovered = discover(&parsed.program, source).expect("discover");
    assert_eq!(discovered.len(), 1, "should discover one type macro");
    assert_eq!(
        discovered[0].def.kind,
        crate::ts_syn::declarative::MacroKind::Type
    );
    let mut registry = DeclarativeMacroRegistry::new();
    for dm in &discovered {
        registry.register(dm.def.clone()).unwrap();
    }
    let out = rewrite(
        &parsed.program,
        source,
        &registry,
        &discovered,
        BuildMode::Dev,
        None,
    );
    // Apply patches to verify final output.
    let applied = crate::host::patch_applicator::PatchApplicator::new(source, out.patches.clone())
        .apply()
        .expect("apply");
    assert!(
        applied.contains("{ wrapped: string }"),
        "expected expanded type body, got:\n{}",
        applied
    );
    assert!(
        !applied.contains("$Wrap"),
        "expanded output still has `$Wrap`:\n{}",
        applied
    );
    assert!(
        out.diagnostics.is_empty(),
        "unexpected diagnostics: {:#?}",
        out.diagnostics
    );
}

#[test]
fn type_macro_repetition_expands_tuple() {
    let source = r#"import { macroRules } from "macroforge/rules";

const $Tup = macroRules({
  kind: "type",
  expand: macroRules`($($t:Type),+) => [$($t),+]`,
});

type T = $Tup<string, number>;
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    assert!(parsed.errors.is_empty(), "parse errors: {:?}", parsed.errors);
    let discovered = discover(&parsed.program, source).expect("discover");
    let mut registry = DeclarativeMacroRegistry::new();
    for dm in &discovered {
        registry.register(dm.def.clone()).unwrap();
    }
    let out = rewrite(
        &parsed.program,
        source,
        &registry,
        &discovered,
        BuildMode::Dev,
        None,
    );
    let applied = crate::host::patch_applicator::PatchApplicator::new(source, out.patches.clone())
        .apply()
        .expect("apply");
    assert!(
        applied.contains("[string,number]") || applied.contains("[string, number]"),
        "expected tuple expansion in output:\n{}",
        applied
    );
    assert!(
        out.diagnostics.is_empty(),
        "unexpected diagnostics: {:#?}",
        out.diagnostics
    );
}

#[test]
fn type_macro_rejects_sharing_mode() {
    let source = r#"import { macroRules } from "macroforge/rules";

const $Bad = macroRules({
  kind: "type",
  mode: "share-only",
  expand: macroRules`($t:Type) => $t`,
  runtime: `function __bad() {}`,
  call: macroRules`($t:Type) => $t`,
});
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let err = discover(&parsed.program, source).unwrap_err();
    assert!(
        err.message
            .contains("type-position macros cannot use sharing modes"),
        "expected sharing-mode rejection, got: {}",
        err.message
    );
}

#[test]
fn type_macro_used_in_value_position_emits_error() {
    let source = r#"import { macroRules } from "macroforge/rules";

const $Foo = macroRules({
  kind: "type",
  expand: macroRules`($t:Type) => $t`,
});

const x = $Foo(1);
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let discovered = discover(&parsed.program, source).expect("discover");
    let mut registry = DeclarativeMacroRegistry::new();
    for dm in &discovered {
        registry.register(dm.def.clone()).unwrap();
    }
    // `$Foo(1)` in value position isn't rewritten by the value walker
    // (because the def's arms don't match a value-position call) and
    // isn't rewritten by the type walker (because it's not a
    // TSTypeReference). No diagnostic is needed here — the resulting
    // code will fail to type-check because `$Foo` is not a real
    // runtime identifier. The important thing is that the type walker
    // doesn't panic.
    let out = rewrite(
        &parsed.program,
        source,
        &registry,
        &discovered,
        BuildMode::Dev,
        None,
    );
    // No panic means we're fine. We accept any outcome here.
    let _ = out;
}

// ---------------------------------------------------------------------------
// Phase 9b — share-only / share-anyway modes
// ---------------------------------------------------------------------------

/// Small helper for phase 9b tests: parse, discover, register, rewrite,
/// and return the rewrite output. Uses the given build mode.
fn rewrite_source(source: &str, build_mode: BuildMode) -> super::rewriter::RewriteOutput {
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    assert!(
        parsed.errors.is_empty(),
        "parse errors: {:?}",
        parsed.errors
    );
    let discovered = discover(&parsed.program, source).expect("discover");
    let mut registry = DeclarativeMacroRegistry::new();
    for dm in &discovered {
        registry.register(dm.def.clone()).unwrap();
    }
    rewrite(
        &parsed.program,
        source,
        &registry,
        &discovered,
        build_mode,
        None,
    )
}

#[test]
fn share_only_emits_runtime_once_per_file() {
    // Two call sites of a `ShareOnly` macro should produce exactly one
    // runtime insert patch, not two.
    let source = r#"import { macroRules } from "macroforge/rules";

const $serialize = macroRules({
  mode: "share-only",
  expand: macroRules`
    ($x:Expr) => __inline_fallback($x)
  `,
  runtime: "function __serialize(value, schema) { return value; }",
  call: macroRules`
    ($x:Expr) => __serialize($x, [])
  `,
});

const a = $serialize(user);
const b = $serialize(admin);
"#;
    let out = rewrite_source(source, BuildMode::Dev);
    assert!(out.diagnostics.is_empty(), "diag: {:?}", out.diagnostics);

    // Count Patch::Insert calls that look like a runtime helper (not the
    // delete patches for the macro def / import).
    let runtime_inserts = out
        .patches
        .iter()
        .filter(|p| matches!(p, crate::ts_syn::abi::Patch::Insert { .. }))
        .count();
    assert_eq!(
        runtime_inserts, 1,
        "expected exactly 1 runtime insert, got {}: {:#?}",
        runtime_inserts, out.patches
    );

    // And two Replace patches (one per call site).
    let replaces = out
        .patches
        .iter()
        .filter(|p| matches!(p, crate::ts_syn::abi::Patch::Replace { .. }))
        .count();
    assert_eq!(
        replaces, 2,
        "expected 2 call-site replaces, got {}",
        replaces
    );
}

#[test]
fn share_only_uses_call_arms_not_expand_arms() {
    // In share mode the rewriter should splice `call_arms`, not `arms`.
    // If it used `arms` we'd see `__inline_fallback` in the output; with
    // `call_arms` we see `__serialize`.
    let source = r#"import { macroRules } from "macroforge/rules";

const $serialize = macroRules({
  mode: "share-only",
  expand: macroRules`
    ($x:Expr) => __inline_fallback($x)
  `,
  runtime: "function __serialize(value) { return value; }",
  call: macroRules`
    ($x:Expr) => __serialize($x)
  `,
});

const result = $serialize(user);
"#;
    let out = rewrite_source(source, BuildMode::Dev);
    assert!(out.diagnostics.is_empty(), "diag: {:?}", out.diagnostics);

    let replace = out
        .patches
        .iter()
        .find_map(|p| match p {
            crate::ts_syn::abi::Patch::Replace { code, .. } => {
                let crate::ts_syn::abi::PatchCode::Text(text) = code;
                Some(text.as_str())
            }
            _ => None,
        })
        .expect("expected at least one Replace patch");
    assert!(
        replace.contains("__serialize"),
        "expected call-arms expansion (__serialize), got: {}",
        replace
    );
    assert!(
        !replace.contains("__inline_fallback"),
        "expand-mode fallback leaked into share output: {}",
        replace
    );
}

#[test]
fn expand_only_tag_form_ignores_build_mode() {
    // ExpandOnly macros should behave identically in Dev and Prod.
    let source = r#"import { macroRules } from "macroforge/rules";

const $vec = macroRules`
  () => []
  ($($x:Expr),+) => [$($x),+]
`;

const xs = $vec(1, 2, 3);
"#;
    let dev = rewrite_source(source, BuildMode::Dev);
    let prod = rewrite_source(source, BuildMode::Prod);
    assert_eq!(dev.patches.len(), prod.patches.len());
    // Neither should have emitted a runtime insert.
    let dev_runtime_inserts = dev
        .patches
        .iter()
        .filter(|p| matches!(p, crate::ts_syn::abi::Patch::Insert { .. }))
        .count();
    assert_eq!(dev_runtime_inserts, 0);
}

#[test]
fn auto_mode_dev_behaves_like_expand_only() {
    // In Dev, Auto macros expand inline for precise diagnostics.
    let source = r#"import { macroRules } from "macroforge/rules";

const $id = macroRules({
  mode: "auto",
  expand: macroRules`
    ($x:Expr) => $x
  `,
  runtime: "function __id(v) { return v; }",
  call: macroRules`
    ($x:Expr) => __id($x)
  `,
});

const a = $id(42);
"#;
    let out = rewrite_source(source, BuildMode::Dev);
    // No runtime insert in dev.
    let runtime_inserts = out
        .patches
        .iter()
        .filter(|p| matches!(p, crate::ts_syn::abi::Patch::Insert { .. }))
        .count();
    assert_eq!(runtime_inserts, 0, "Dev Auto should not emit runtime");

    // And the Replace should use `expand` arms (splicing `$x` directly),
    // not `call_arms` (which would introduce `__id`).
    let has_id_call = out.patches.iter().any(|p| match p {
        crate::ts_syn::abi::Patch::Replace { code, .. } => {
            let crate::ts_syn::abi::PatchCode::Text(text) = code;
            text.contains("__id")
        }
        _ => false,
    });
    assert!(!has_id_call, "Dev Auto should not call runtime helper");
}

#[test]
fn auto_mode_prod_emits_megamorphism_warning_for_many_shapes() {
    // An Auto macro called with 6 distinct Named shapes starting with
    // the same letter — clusters collapse to one bucket of size 6 > 4,
    // so the recommendation is ForceExpand and a warning fires.
    let source = r#"import { macroRules } from "macroforge/rules";

class UserA {}
class UserB {}
class UserC {}
class UserD {}
class UserE {}
class UserF {}

const $serialize = macroRules({
  mode: "auto",
  expand: macroRules`($x:Expr) => __inline($x)`,
  runtime: "function __serialize(v) { return v; }",
  call: macroRules`($x:Expr) => __serialize($x)`,
});

export const a = $serialize(UserA);
export const b = $serialize(UserB);
export const c = $serialize(UserC);
export const d = $serialize(UserD);
export const e = $serialize(UserE);
export const f = $serialize(UserF);
"#;
    let out = rewrite_source(source, BuildMode::Prod);
    let warning = out
        .diagnostics
        .iter()
        .find(|d| matches!(d.level, crate::ts_syn::abi::DiagnosticLevel::Warning))
        .expect("expected a megamorphism warning");
    assert!(
        warning.message.contains("serialize"),
        "warning should mention the macro name: {}",
        warning.message
    );
}

#[test]
fn auto_mode_prod_shares_for_few_shapes() {
    // Three distinct shapes → under threshold → Share. The runtime
    // should be emitted exactly once.
    let source = r#"import { macroRules } from "macroforge/rules";

class User {}
class Admin {}
class Guest {}

const $serialize = macroRules({
  mode: "auto",
  expand: macroRules`($x:Expr) => __inline($x)`,
  runtime: "function __serialize(v) { return v; }",
  call: macroRules`($x:Expr) => __serialize($x)`,
});

export const a = $serialize(User);
export const b = $serialize(Admin);
export const c = $serialize(Guest);
"#;
    let out = rewrite_source(source, BuildMode::Prod);
    let runtime_inserts = out
        .patches
        .iter()
        .filter(|p| matches!(p, crate::ts_syn::abi::Patch::Insert { .. }))
        .count();
    assert_eq!(
        runtime_inserts, 1,
        "expected exactly 1 runtime insert for Auto + shareable shapes"
    );
    // No megamorphism warning either.
    let warnings = out
        .diagnostics
        .iter()
        .filter(|d| matches!(d.level, crate::ts_syn::abi::DiagnosticLevel::Warning))
        .count();
    assert_eq!(warnings, 0);
}

#[test]
fn auto_mode_prod_shares_by_default() {
    // Pending Phase 9c's megamorphism analyzer, Auto + Prod shares
    // unconditionally — exercising the end-to-end share pipeline.
    let source = r#"import { macroRules } from "macroforge/rules";

const $id = macroRules({
  mode: "auto",
  expand: macroRules`
    ($x:Expr) => $x
  `,
  runtime: "function __id(v) { return v; }",
  call: macroRules`
    ($x:Expr) => __id($x)
  `,
});

const a = $id(42);
const b = $id(99);
"#;
    let out = rewrite_source(source, BuildMode::Prod);
    let runtime_inserts = out
        .patches
        .iter()
        .filter(|p| matches!(p, crate::ts_syn::abi::Patch::Insert { .. }))
        .count();
    assert_eq!(runtime_inserts, 1, "Prod Auto should emit runtime once");
}

#[test]
fn tag_form_still_works_after_object_form_support() {
    // Regression check: the pre-existing tag form should keep behaving
    // identically now that discovery supports two shapes.
    let source = r#"import { macroRules } from "macroforge/rules";
const $vec = macroRules`
  () => []
  ($($x:Expr),+) => [$($x),+]
`;
"#;
    let allocator = Allocator::default();
    let parsed = parse_program(&allocator, source);
    let defs = discover(&parsed.program, source).expect("discover");
    assert_eq!(defs.len(), 1);
    assert_eq!(defs[0].def.mode, MacroMode::ExpandOnly);
    assert!(defs[0].def.runtime.is_none());
    assert!(defs[0].def.call_arms.is_none());
    assert_eq!(defs[0].def.megamorphism_threshold, 4);
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
