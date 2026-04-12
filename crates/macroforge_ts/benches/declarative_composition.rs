//! Criterion microbenchmarks for the declarative macro hot paths.
//!
//! Shipped as PR 16 of the production-hardening plan. The in-suite
//! regression guard (`deep_composition_chain_expands_quickly` in
//! `host/declarative/tests.rs`) catches orders-of-magnitude
//! regressions; this benchmark suite provides finer-grained
//! throughput tracking for future work.
//!
//! Run with:
//!
//! ```sh
//! cargo bench --features oxc -p macroforge_ts declarative_composition
//! ```
//!
//! Benches:
//!
//! - `composition/depth_{4,16,64,192}` — deeply-nested inter-macro
//!   composition, exercising `expand_macro_call`'s OXC re-parse
//!   per nested level. Useful for catching regressions in the
//!   composition loop or in the body parser's performance.
//!   192 is just under `MAX_EXPANSION_DEPTH = 256` so we don't
//!   trip the recursion cap.
//!
//! - `hygiene/rewrite_large_body` — a single hygiene rewrite over
//!   a macro body containing ~50 `__`-prefixed declarations plus
//!   ~200 uses. Exercises the `hygiene` cursor's token scan.
//!
//! - `cluster/analyze_50_shapes` — run the megamorph analyzer over
//!   50 synthesized call sites. Covers `cluster_shapes` and the
//!   pairwise Jaccard path.

#![cfg(feature = "oxc")]

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use macroforge_ts::host::declarative::{BuildMode, DeclarativeMacroRegistry, discover, rewrite};
use oxc::allocator::Allocator;
use oxc::parser::Parser;
use oxc::span::SourceType;

/// Build source text for a composition chain of the given depth:
///
/// ```text
/// const $double = macroRules`($x:Expr) => ($x * 2)`;
/// const $quad = macroRules`($x:Expr) => $double($double( ... $double($x) ... ))`;
/// const out = $quad(1);
/// ```
///
/// The inner chain of `$double(...)` calls is `depth` levels deep.
fn composition_source(depth: usize) -> String {
    let mut body = String::from("$x");
    for _ in 0..depth {
        body = format!("$double({})", body);
    }
    format!(
        "import {{ macroRules }} from \"macroforge/rules\";\n\
         const $double = macroRules`($x:Expr) => ($x * 2)`;\n\
         const $q = macroRules`($x:Expr) => {}`;\n\
         const out = $q(1);\n",
        body
    )
}

fn run_rewrite(source: &str) {
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, source, SourceType::ts()).parse();
    assert!(
        parsed.errors.is_empty(),
        "parse errors in bench fixture: {:?}",
        parsed.errors
    );
    let discovered = discover(&parsed.program, source).expect("discover");
    let mut registry = DeclarativeMacroRegistry::new();
    for dm in &discovered {
        registry
            .register_scoped(dm.def.clone(), dm.scope_span)
            .unwrap();
    }
    let out = rewrite(
        &parsed.program,
        source,
        &registry,
        &discovered,
        BuildMode::dev(),
        None,
    );
    black_box(out);
}

fn bench_composition(c: &mut Criterion) {
    let mut group = c.benchmark_group("composition");
    for depth in [4usize, 16, 64, 192] {
        let source = composition_source(depth);
        group.bench_with_input(BenchmarkId::from_parameter(depth), &source, |b, src| {
            b.iter(|| run_rewrite(src));
        });
    }
    group.finish();
}

/// Build a macro body that declares ~50 `__`-prefixed locals and
/// uses each one a few times, stressing the hygiene rewriter's
/// identifier scan.
fn hygiene_source() -> String {
    let mut body = String::from("{\n");
    for i in 0..50 {
        body.push_str(&format!("  const __v{} = {};\n", i, i));
    }
    for i in 0..50 {
        body.push_str(&format!(
            "  const __r{} = __v{} + __v{} + __v{};\n",
            i,
            i,
            (i + 1) % 50,
            (i + 2) % 50
        ));
    }
    body.push_str("  __v0\n}");
    format!(
        "import {{ macroRules }} from \"macroforge/rules\";\n\
         const $big = macroRules`() => {}`;\n\
         const out = $big();\n",
        body
    )
}

fn bench_hygiene(c: &mut Criterion) {
    let source = hygiene_source();
    c.bench_function("hygiene/rewrite_large_body", |b| {
        b.iter(|| run_rewrite(&source));
    });
}

/// Build a source with a 50-shape Auto macro so `cluster_shapes`
/// does real work. Uses first-letter bucketing (no type registry).
fn cluster_source() -> String {
    let mut calls = String::new();
    // 10 names each starting with 5 different letters → 50 total
    // shapes, clustered into 5 first-letter buckets of 10.
    for prefix in ['A', 'B', 'C', 'D', 'E'] {
        for i in 0..10 {
            calls.push_str(&format!(
                "const v_{}_{} = $serialize({}{});\n",
                prefix, i, prefix, i
            ));
        }
    }
    format!(
        "import {{ macroRules }} from \"macroforge/rules\";\n\
         const $serialize = macroRules({{\n\
           mode: \"auto\",\n\
           expand: macroRules`($x:Expr) => __inline($x)`,\n\
           runtime: \"function __serialize(v) {{ return v; }}\",\n\
           runtimeName: \"__serialize_$__cluster__\",\n\
           call: macroRules`($x:Expr) => __serialize_$__cluster__($x)`,\n\
           megamorphismThreshold: 10,\n\
         }});\n\
         {}",
        calls
    )
}

fn bench_cluster(c: &mut Criterion) {
    let source = cluster_source();
    c.bench_function("cluster/analyze_50_shapes", |b| {
        b.iter(|| {
            // Use prod build mode so the megamorph analyzer actually
            // runs — in dev it's short-circuited for Auto macros.
            let allocator = Allocator::default();
            let parsed = Parser::new(&allocator, &source, SourceType::ts()).parse();
            assert!(
                parsed.errors.is_empty(),
                "parse errors: {:?}",
                parsed.errors
            );
            let discovered = discover(&parsed.program, &source).expect("discover");
            let mut registry = DeclarativeMacroRegistry::new();
            for dm in &discovered {
                registry
                    .register_scoped(dm.def.clone(), dm.scope_span)
                    .unwrap();
            }
            let out = rewrite(
                &parsed.program,
                &source,
                &registry,
                &discovered,
                BuildMode::prod(),
                None,
            );
            black_box(out);
        });
    });
}

criterion_group!(benches, bench_composition, bench_hygiene, bench_cluster);
criterion_main!(benches);
