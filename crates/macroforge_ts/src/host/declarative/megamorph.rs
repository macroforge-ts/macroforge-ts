//! Megamorphism analyzer for `Auto`-mode declarative macros.
//!
//! When a macro is declared with `mode: "auto"`, the build pipeline needs
//! to decide at prod time whether its shared runtime helper will stay fast
//! under V8's inline caches. V8's ICs start monomorphic, stay fast up to
//! ~4 distinct shapes per call site, then fall back to megamorphic
//! lookups that are much slower.
//!
//! The analyzer walks every collected call site of every `Auto` macro,
//! extracts a coarse "shape" from each argument (class name, literal
//! kind, or opaque), counts distinct shapes per macro, and returns a
//! [`Recommendation`] the rewriter consults in Phase 9c:
//!
//! - **Share** (≤ threshold distinct shapes): emit one shared runtime
//!   helper and have every call site call it. Hot and cold paths look
//!   the same to V8 because all call sites hit the same function with
//!   the same argument shapes.
//! - **Cluster** (> threshold): partition the shapes into sub-groups
//!   that share structural similarity, emit one helper per cluster, and
//!   dispatch each call site to its cluster's helper.
//! - **ForceExpand**: the cluster analysis degenerated (every shape is
//!   unique). Inline expansion at every call site — no shared state to
//!   go megamorphic.
//!
//! The shape extractor is deliberately heuristic. When the project-wide
//! type registry is available, the extractor also attaches a sorted
//! field-name fingerprint to each `Named` shape so that structural
//! clustering (Phase 14) can group types by shape similarity instead of
//! by name prefix alone. Without the registry, the fingerprint is
//! `None` and the clusterer falls back to the first-letter heuristic.

use std::collections::HashMap;

use crate::ts_syn::abi::SpanIR;
use crate::ts_syn::abi::ir::type_registry::{TypeDefinitionIR, TypeRegistry};
use crate::ts_syn::declarative::MacroMode;

use super::registry::DeclarativeMacroRegistry;

/// A single call site of an `Auto`-mode macro, recorded by the rewriter
/// during Phase 9c's first pass.
#[derive(Debug, Clone)]
pub struct ResolvedCallSite {
    /// The macro being called, sans leading `$`.
    pub macro_name: String,
    /// Span of the call expression in the source (1-based patch
    /// convention).
    pub call_span: SpanIR,
    /// Coarse shape of every argument at the call site, in
    /// left-to-right order. A zero-arg call produces an empty vec;
    /// single-arg calls produce a one-element vec (matching what the
    /// pre-PR-7 `arg_shape: TypeShape` field represented).
    ///
    /// The analyzer treats two call sites as "same polymorphism
    /// class" iff their `arg_shapes` vectors are element-wise equal
    /// — so `(User, Order)` and `(User, Product)` count as two
    /// distinct classes even though the first argument matches.
    /// This closes the gap where pre-PR-7 `Auto`-mode multi-arg
    /// macros under-reported their polymorphism by ignoring every
    /// argument past the first.
    pub arg_shapes: Vec<TypeShape>,
}

/// A coarse classification of a call argument's shape.
///
/// `Named` captures concrete class/interface references (the author's
/// type flowing in). When the project-wide type registry is available
/// to the analyzer, `fields` holds a sorted list of the type's field
/// names — used by [`cluster_shapes`] for structural Jaccard
/// clustering. `Literal` captures primitive literal calls. `Opaque` is
/// the fallback — anonymous object literals, computed expressions,
/// function-call results, anything the heuristic can't pin down.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeShape {
    /// A known type identifier: `User`, `Admin`, etc.
    Named {
        /// The surface identifier used at the call site.
        name: String,
        /// Sorted list of field names from the type registry, if the
        /// type is known. `None` means the registry didn't have the
        /// type (or wasn't passed to the analyzer) — clustering falls
        /// back to first-letter grouping in that case.
        fields: Option<Vec<String>>,
    },
    /// A primitive literal — the string names the JS runtime type:
    /// `"string"`, `"number"`, `"boolean"`, `"null"`, etc.
    Literal(String),
    /// Anything that isn't a bare identifier or literal — the
    /// heuristic can't narrow it any further.
    Opaque,
}

impl TypeShape {
    /// Convenience constructor for a `Named` shape without a field
    /// fingerprint (i.e. when the type registry is not available).
    pub fn named(name: impl Into<String>) -> Self {
        TypeShape::Named {
            name: name.into(),
            fields: None,
        }
    }
}

/// Per-macro polymorphism summary returned by [`analyze`].
#[derive(Debug, Clone)]
pub struct MacroPolymorphism {
    /// How many distinct [`TypeShape`] values flowed into this macro
    /// across all its call sites.
    pub distinct_shapes: usize,
    /// The recommendation the rewriter should follow for this macro.
    pub recommendation: Recommendation,
}

/// What to do with an `Auto`-mode macro based on its polymorphism.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Recommendation {
    /// Emit a single shared runtime helper and replace every call site
    /// with a call to it.
    Share,
    /// Shape count is above the threshold. Partition into sub-clusters,
    /// emit one helper per cluster, dispatch calls to their cluster.
    Cluster(Vec<TypeCluster>),
    /// Shape count is so high and the shapes so diverse that sharing
    /// would go megamorphic in every cluster too. Fall back to inline
    /// expansion at every call site.
    ForceExpand,
}

/// A cluster of call-site shape tuples that together stay under the
/// megamorphism threshold. All call sites whose argument shapes (as a
/// tuple) belong to this cluster dispatch to the same helper function.
///
/// Each entry in [`Self::shapes`] is a `Vec<TypeShape>` — one shape
/// per positional argument at a call site. Single-arg `Auto` macros
/// put one-element tuples in their clusters; multi-arg macros put
/// multi-element tuples. Two call sites belong to the same cluster
/// iff their full shape tuples are element-wise equal after any
/// structural / prefix coalescing the analyzer applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeCluster {
    /// The call-site shape tuples that belong to this cluster.
    pub shapes: Vec<Vec<TypeShape>>,
    /// Stable identifier derived from the shapes — used to suffix the
    /// generated helper name, e.g. `$serialize__a_cluster` for a
    /// cluster of shapes starting with `a`.
    pub id: String,
}

/// The full analyzer output. Keyed by macro name (sans `$`).
#[derive(Debug, Default, Clone)]
pub struct MegamorphReport {
    pub per_macro: HashMap<String, MacroPolymorphism>,
}

impl MegamorphReport {
    /// Look up the recommendation for a given macro. Returns `None` if
    /// the macro had no call sites (so no polymorphism summary exists).
    pub fn lookup(&self, macro_name: &str) -> Option<&MacroPolymorphism> {
        self.per_macro.get(macro_name)
    }
}

/// Run the megamorphism analysis over the collected call sites.
///
/// Only considers macros registered with [`MacroMode::Auto`]; `ShareOnly`
/// and `ShareAnyway` bypass the analyzer by design (the user opted in
/// explicitly). `ExpandOnly` macros don't care about shape count.
///
/// `threshold` is the maximum number of distinct shapes allowed to share
/// a single helper — typically 4 (V8's IC cap).
pub fn analyze(
    registry: &DeclarativeMacroRegistry,
    call_sites: &[ResolvedCallSite],
    threshold: u8,
) -> MegamorphReport {
    // Group call sites by macro name, filtering to Auto-mode macros.
    let mut per_macro: HashMap<String, Vec<&ResolvedCallSite>> = HashMap::new();
    for site in call_sites {
        if let Some(def) = registry.lookup(&site.macro_name)
            && def.mode == MacroMode::Auto
        {
            per_macro
                .entry(site.macro_name.clone())
                .or_default()
                .push(site);
        }
    }

    let mut report = MegamorphReport::default();
    for (name, sites) in per_macro {
        // Dedupe on the full shape tuple — two call sites with
        // identical `arg_shapes` are the same polymorphism class.
        let mut shape_set: Vec<Vec<TypeShape>> = Vec::new();
        for site in &sites {
            if !shape_set.contains(&site.arg_shapes) {
                shape_set.push(site.arg_shapes.clone());
            }
        }
        let distinct_shapes = shape_set.len();

        let per_macro_threshold = registry
            .lookup(&name)
            .map(|d| d.megamorphism_threshold as usize)
            .unwrap_or(threshold as usize);

        let recommendation = if distinct_shapes <= per_macro_threshold {
            Recommendation::Share
        } else {
            let clusters = cluster_shapes(&shape_set, per_macro_threshold);
            // A cluster is "still megamorphic" only if it contains more
            // distinct *structural fingerprints* than the threshold.
            // Two tuples with identical per-position fingerprints look
            // like the same hidden class to V8, so they don't add to
            // the cluster's polymorphism budget. Tuples without
            // fingerprints count each distinct full-tuple identity as
            // one slot.
            if clusters
                .iter()
                .any(|c| count_distinct_fingerprints(&c.shapes) > per_macro_threshold)
            {
                // The heuristic can't partition these shapes usefully.
                // Fall back to inlining.
                Recommendation::ForceExpand
            } else {
                Recommendation::Cluster(clusters)
            }
        };

        report.per_macro.insert(
            name,
            MacroPolymorphism {
                distinct_shapes,
                recommendation,
            },
        );
    }
    report
}

/// Minimum Jaccard similarity for two `Named` shapes with field
/// fingerprints to land in the same cluster. Chosen empirically:
/// - 1.00: identical field sets (clearly the "same shape").
/// - 0.80: one type has a small extension like `PendingUser` ⊇ `User`.
/// - 0.60: genuine structural overlap even when names diverge — the
///   lower bound we accept for "these should share a helper".
/// - <0.60: the types are structurally different enough that V8 would
///   still see them as distinct shapes; splitting helps cache locality.
const JACCARD_THRESHOLD: f64 = 0.60;

/// Partition a set of call-site shape tuples into clusters that each
/// stay at or below the threshold.
///
/// Strategy:
///
/// 1. **Arity bucketing** — tuples with different argument counts
///    never cluster together. `(User)` and `(User, Order)` go into
///    separate buckets up front so the per-position logic below
///    always sees matching arities.
///
/// 2. **First-argument classification within an arity** — within a
///    same-arity bucket, we classify each tuple by its first
///    element: structural (has fingerprint fields), name-prefix
///    (no fingerprint, falls back to the first letter of the
///    surface name), literal, or opaque.
///
/// 3. **Structural Jaccard merge for fingerprinted firsts** — a
///    tuple whose first element has a structural fingerprint may
///    join an existing structural cluster iff (a) all members of
///    the cluster have a first-element Jaccard similarity
///    ≥ [`JACCARD_THRESHOLD`] with this tuple's first element,
///    AND (b) the per-position discriminants of the REMAINING
///    positions all match element-wise. This keeps `(User, Order)`
///    and `(AdminUser, Order)` together (first args are
///    structurally similar, second args match exactly) but keeps
///    `(User, Order)` and `(User, Product)` apart (first args
///    match but second args don't).
///
/// 4. **Name-prefix fallback** — tuples whose first element has no
///    fingerprint bucket by the first letter of the name, matching
///    the MVP behavior for single-arg calls while still honoring
///    the arity split for multi-arg.
///
/// 5. **Literal / opaque pass-through** — `Literal` and `Opaque`
///    firsts go into dedicated `"lit"` and `"opaque"` clusters per
///    arity.
///
/// Clusters that are still above the per-macro threshold after this
/// pass are not split further — the caller detects that case and
/// falls back to `ForceExpand`.
fn cluster_shapes(shapes: &[Vec<TypeShape>], _threshold: usize) -> Vec<TypeCluster> {
    // Keep a stable traversal order so the output is deterministic.
    let mut structural: Vec<Vec<Vec<TypeShape>>> = Vec::new();
    let mut prefix_buckets: HashMap<String, Vec<Vec<TypeShape>>> = HashMap::new();
    let mut literal_bucket: Vec<Vec<TypeShape>> = Vec::new();
    let mut opaque_bucket: Vec<Vec<TypeShape>> = Vec::new();
    let mut empty_bucket: Vec<Vec<TypeShape>> = Vec::new();

    for tuple in shapes {
        let Some(first) = tuple.first() else {
            // Zero-argument calls — all cluster together trivially.
            empty_bucket.push(tuple.clone());
            continue;
        };
        match first {
            TypeShape::Named {
                fields: Some(fields),
                ..
            } if !fields.is_empty() => {
                // PR 10: full pairwise-averaged Jaccard across every
                // position of the tuple. The tuple may join an
                // existing structural cluster iff every member's
                // tuple has a mean pairwise Jaccard ≥ threshold with
                // this one. This is strictly more permissive than
                // the PR 7 "exact tail equality" check — tuples
                // like `(User, OrderA)` and `(User, OrderB)` where
                // `OrderA` and `OrderB` have a high field overlap
                // now cluster together, where previously they'd
                // split into separate clusters on the tail mismatch.
                let _ = fields; // silence unused-binding; `first` is used via tuple.
                let mut joined = false;
                for cluster in structural.iter_mut() {
                    let fits = cluster.iter().all(|existing| {
                        mean_pairwise_jaccard(tuple, existing) >= JACCARD_THRESHOLD
                    });
                    if fits {
                        cluster.push(tuple.clone());
                        joined = true;
                        break;
                    }
                }
                if !joined {
                    structural.push(vec![tuple.clone()]);
                }
            }
            TypeShape::Named { name, .. } => {
                // Fingerprint missing. Bucket by the first letter of
                // the first-arg's name, with the full arity embedded
                // in the key so arity mismatches split.
                let key = name
                    .chars()
                    .next()
                    .map(|c| c.to_ascii_lowercase().to_string())
                    .unwrap_or_else(|| "_".to_string());
                // Prepend arity so `(Alice,)` and `(Alice, Order)`
                // never end up in the same bucket.
                let keyed = if tuple.len() == 1 {
                    key
                } else {
                    format!("{}{}", key, tuple.len())
                };
                prefix_buckets.entry(keyed).or_default().push(tuple.clone());
            }
            TypeShape::Literal(_) => literal_bucket.push(tuple.clone()),
            TypeShape::Opaque => opaque_bucket.push(tuple.clone()),
        }
    }

    let mut clusters: Vec<TypeCluster> = Vec::new();

    // Emit structural clusters first, with a stable id derived from
    // the sorted member names.
    for group in structural {
        let id = structural_cluster_id(&group);
        clusters.push(TypeCluster { id, shapes: group });
    }

    // Then the prefix fallback buckets, sorted by id.
    let mut prefix_clusters: Vec<TypeCluster> = prefix_buckets
        .into_iter()
        .map(|(id, shapes)| TypeCluster { id, shapes })
        .collect();
    prefix_clusters.sort_by(|a, b| a.id.cmp(&b.id));
    clusters.extend(prefix_clusters);

    if !empty_bucket.is_empty() {
        clusters.push(TypeCluster {
            id: "empty".to_string(),
            shapes: empty_bucket,
        });
    }
    if !literal_bucket.is_empty() {
        clusters.push(TypeCluster {
            id: "lit".to_string(),
            shapes: literal_bucket,
        });
    }
    if !opaque_bucket.is_empty() {
        clusters.push(TypeCluster {
            id: "opaque".to_string(),
            shapes: opaque_bucket,
        });
    }

    clusters
}

/// Count how many distinct "V8 hidden class equivalent" tuples a set
/// of call-site shape tuples spans. Two tuples collapse to one bucket
/// iff their elements are pairwise fingerprint-equivalent:
///
/// - Two `Named` elements with identical `fields` fingerprints are
///   equivalent (same declared shape).
/// - Two `Named` elements without a fingerprint are equivalent iff
///   their names match (the MVP heuristic).
/// - `Literal` elements collapse by their JS runtime-type tag.
/// - `Opaque` elements all collapse to a single tag.
///
/// Arity mismatches always count as distinct buckets.
fn count_distinct_fingerprints(shapes: &[Vec<TypeShape>]) -> usize {
    use std::collections::HashSet;
    let mut seen: HashSet<String> = HashSet::new();
    for tuple in shapes {
        // Build a tuple-level key by concatenating per-element keys
        // with a `|` separator. Arity-different tuples get distinct
        // keys because the `|` separators differ.
        let key = tuple
            .iter()
            .map(|s| match s {
                TypeShape::Named {
                    fields: Some(fs), ..
                } if !fs.is_empty() => format!("s:{}", fs.join(",")),
                TypeShape::Named { name, .. } => format!("n:{}", name),
                TypeShape::Literal(kind) => format!("l:{}", kind),
                TypeShape::Opaque => "o".to_string(),
            })
            .collect::<Vec<_>>()
            .join("|");
        seen.insert(format!("arity{}:{}", tuple.len(), key));
    }
    seen.len()
}

/// Compute the Jaccard similarity `|A ∩ B| / |A ∪ B|` of two sorted
/// field-name lists. Both inputs are assumed pre-sorted; dedup is
/// performed on the fly so repeated field names (rare but possible
/// with mixins) don't skew the ratio. Empty inputs return `0.0`.
fn jaccard(a: &[String], b: &[String]) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 0.0;
    }
    let mut i = 0;
    let mut j = 0;
    let mut inter: usize = 0;
    let mut uni: usize = 0;
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Equal => {
                inter += 1;
                uni += 1;
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Less => {
                uni += 1;
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                uni += 1;
                j += 1;
            }
        }
    }
    uni += a.len() - i;
    uni += b.len() - j;
    if uni == 0 {
        0.0
    } else {
        inter as f64 / uni as f64
    }
}

/// Arithmetic mean of per-position Jaccard similarity across two
/// call-site shape tuples. Used by [`cluster_shapes`] to decide
/// whether two tuples belong in the same structural cluster.
///
/// Scoring rules:
///
/// - **Arity mismatch** → `0.0` immediately. Tuples with different
///   argument counts never cluster together (they'd dispatch to
///   helpers with different arities).
/// - **Both positions `Named` with fingerprints** → field-level
///   [`jaccard`] on the sorted field lists.
/// - **Both positions `Named` without fingerprints** → `1.0` iff
///   names match exactly, else `0.0`. The MVP name-prefix heuristic
///   is deliberately NOT used here — at this level we want precise
///   equality for unfingerprinted types, and first-letter bucketing
///   is handled as a separate fallback by the outer [`cluster_shapes`]
///   loop for tuples whose first element is unfingerprinted.
/// - **Both `Literal` with matching tags** → `1.0` (same runtime
///   type; V8 sees them as one hidden class).
/// - **Both `Opaque`** → `1.0` (we treat all opaque values as
///   equivalent for clustering purposes).
/// - **Discriminant mismatch** (e.g. `Named` vs `Literal`) →
///   `0.0`. Different kinds of values can't collapse to the same
///   helper.
///
/// Returns a value in the closed interval `[0.0, 1.0]`.
fn mean_pairwise_jaccard(a: &[TypeShape], b: &[TypeShape]) -> f64 {
    if a.len() != b.len() {
        return 0.0;
    }
    if a.is_empty() {
        // Both are zero-arity tuples. Empty tuples are trivially
        // "identical" — return 1.0 so they cluster together.
        return 1.0;
    }
    let mut sum = 0.0;
    for (ai, bi) in a.iter().zip(b.iter()) {
        sum += position_jaccard(ai, bi);
    }
    sum / a.len() as f64
}

fn position_jaccard(a: &TypeShape, b: &TypeShape) -> f64 {
    match (a, b) {
        (
            TypeShape::Named {
                name: na,
                fields: Some(fa),
            },
            TypeShape::Named {
                name: nb,
                fields: Some(fb),
            },
        ) if !fa.is_empty() && !fb.is_empty() => {
            // Both sides have structural fingerprints — compute
            // field-level Jaccard. We ignore names here intentionally:
            // structurally-equivalent types should cluster regardless
            // of their surface names.
            let _ = (na, nb);
            jaccard(fa, fb)
        }
        (TypeShape::Named { name: na, .. }, TypeShape::Named { name: nb, .. }) if na == nb => {
            // At least one side lacks a fingerprint — fall back to
            // exact name equality. (First-letter bucketing happens
            // at the cluster_shapes level, not per position.)
            1.0
        }
        (TypeShape::Named { .. }, TypeShape::Named { .. }) => 0.0,
        (TypeShape::Literal(la), TypeShape::Literal(lb)) if la == lb => 1.0,
        (TypeShape::Literal(_), TypeShape::Literal(_)) => 0.0,
        (TypeShape::Opaque, TypeShape::Opaque) => 1.0,
        // Discriminant mismatch — different kinds of values.
        _ => 0.0,
    }
}

/// Derive a stable cluster id from its members' tuple shapes. We use
/// the sorted, `_`-joined list of the FIRST-argument names across
/// all tuples in the group — deterministic across runs and distinct
/// from the prefix-bucket keys (which are single letters or
/// letter+arity). Falls back to `"struct"` when no member has a
/// named first argument (e.g. cluster of literal-first tuples).
fn structural_cluster_id(group: &[Vec<TypeShape>]) -> String {
    let mut names: Vec<&str> = group
        .iter()
        .filter_map(|tuple| match tuple.first() {
            Some(TypeShape::Named { name, .. }) => Some(name.as_str()),
            _ => None,
        })
        .collect();
    names.sort_unstable();
    names.dedup();
    if names.is_empty() {
        "struct".to_string()
    } else {
        format!("struct_{}", names.join("_"))
    }
}

/// Extract a coarse `TypeShape` from an OXC argument expression.
///
/// The heuristic uses the argument's surface syntax only — there's no
/// type-checker access here, so a bare `user` identifier turns into
/// `Named { name: "user" }` even though at the JS type level it might
/// be `User | Admin`. That's a conscious trade-off: the analyzer
/// catches the common case of "user calls `$serialize(thisClassInstance)`
/// consistently", and opaque fallbacks prevent the heuristic from
/// over-committing when it's unsure.
///
/// `Named` results are augmented with a sorted field-name fingerprint
/// from `type_registry` (if the type is known). An empty registry
/// produces no fingerprint, falling back to the name-prefix heuristic
/// in [`cluster_shapes`] (Phase 14).
pub fn extract_type_shape(
    arg: &oxc::ast::ast::Argument<'_>,
    type_registry: &TypeRegistry,
) -> TypeShape {
    use oxc::ast::ast::Expression;
    let Some(expr) = arg.as_expression() else {
        return TypeShape::Opaque;
    };
    match expr {
        Expression::Identifier(ident) => {
            // Capitalize-first-letter is a conventional signal for "this
            // is a type/class name"; lowercase identifiers are locals or
            // function params and don't bound the type tightly enough to
            // trust the name alone, so treat them as opaque for now.
            let name = ident.name.as_str();
            if name.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
                named_with_fingerprint(name, type_registry)
            } else {
                TypeShape::Opaque
            }
        }
        Expression::NewExpression(new_expr) => {
            // `new User(...)` → shape `User`.
            if let Expression::Identifier(ident) = &new_expr.callee {
                named_with_fingerprint(ident.name.as_str(), type_registry)
            } else {
                TypeShape::Opaque
            }
        }
        Expression::StringLiteral(_) => TypeShape::Literal("string".into()),
        Expression::NumericLiteral(_) => TypeShape::Literal("number".into()),
        Expression::BooleanLiteral(_) => TypeShape::Literal("boolean".into()),
        Expression::NullLiteral(_) => TypeShape::Literal("null".into()),
        Expression::BigIntLiteral(_) => TypeShape::Literal("bigint".into()),
        Expression::TemplateLiteral(_) => TypeShape::Literal("string".into()),
        _ => TypeShape::Opaque,
    }
}

/// Build a `TypeShape::Named` for `name`, looking up the field
/// fingerprint in the registry when available.
///
/// Fingerprint extraction (PR 10):
///
/// - **Class / Interface** — sorted list of field names, same as PR 7.
/// - **Enum** — sorted list of variant names. Two enums with
///   identical variant sets fingerprint-equal, so they cluster
///   together even under different names.
/// - **Type alias** — if the alias body is structural (an object
///   literal `type T = { a: number; b: string }`), the member names
///   become the fingerprint. If the alias body is a simple type
///   reference (`type T = SomeClass`), the fingerprint is pulled
///   from the target with one level of indirection — we don't
///   follow chains of aliases to avoid unbounded recursion.
/// - **Anything else** (union aliases, tuple aliases, intersection
///   aliases) — no fingerprint, falls back to the name-prefix
///   heuristic in `cluster_shapes`.
fn named_with_fingerprint(name: &str, type_registry: &TypeRegistry) -> TypeShape {
    let fields = type_registry
        .get(name)
        .and_then(|entry| extract_fingerprint_fields(&entry.definition, type_registry));

    TypeShape::Named {
        name: name.to_string(),
        fields,
    }
}

/// Core fingerprint extractor that [`named_with_fingerprint`]
/// delegates to. Handles every `TypeDefinitionIR` variant and, for
/// type aliases, performs one level of indirection into the target
/// type via [`extract_fingerprint_fields_direct`] to avoid alias
/// chains causing unbounded recursion.
///
/// Returns `None` when no meaningful structural fingerprint can be
/// derived — the caller leaves `fields: None` on the resulting
/// [`TypeShape::Named`] and the outer clusterer falls back to the
/// name-prefix heuristic.
fn extract_fingerprint_fields(
    def: &TypeDefinitionIR,
    type_registry: &TypeRegistry,
) -> Option<Vec<String>> {
    match def {
        TypeDefinitionIR::Class(class) => {
            let mut fields: Vec<String> = class.fields.iter().map(|f| f.name.clone()).collect();
            fields.sort_unstable();
            fields.dedup();
            if fields.is_empty() {
                None
            } else {
                Some(fields)
            }
        }
        TypeDefinitionIR::Interface(iface) => {
            let mut fields: Vec<String> = iface.fields.iter().map(|f| f.name.clone()).collect();
            fields.sort_unstable();
            fields.dedup();
            if fields.is_empty() {
                None
            } else {
                Some(fields)
            }
        }
        TypeDefinitionIR::Enum(enum_ir) => {
            // Variant names serve as the "field set" — two enums
            // with identical variant sets fingerprint-equal and so
            // cluster together even under different enum names.
            let mut variants: Vec<String> =
                enum_ir.variants.iter().map(|v| v.name.clone()).collect();
            variants.sort_unstable();
            variants.dedup();
            if variants.is_empty() {
                None
            } else {
                Some(variants)
            }
        }
        TypeDefinitionIR::TypeAlias(alias) => {
            // Structural object aliases: pull the field names
            // directly from the object body.
            if let Some(members) = alias.body.as_object() {
                let mut fields: Vec<String> = members.iter().map(|m| m.name.clone()).collect();
                fields.sort_unstable();
                fields.dedup();
                if !fields.is_empty() {
                    return Some(fields);
                }
            }
            // Simple reference aliases: follow one level of
            // indirection. `type T = SomeClass` inherits SomeClass's
            // fingerprint so call sites passing a `T` cluster with
            // calls sites passing a `SomeClass`.
            if let Some(target_name) = alias.body.as_alias() {
                return type_registry
                    .get(target_name)
                    .and_then(|entry| extract_fingerprint_fields_direct(&entry.definition));
            }
            None
        }
    }
}

/// Non-recursive fingerprint extractor used when following a type
/// alias's target. Matches [`extract_fingerprint_fields`] but stops
/// at the first alias it encounters instead of chasing the chain —
/// this caps work at `O(1)` indirection and sidesteps any pathological
/// `type A = B; type B = A` cycles the user might have written.
fn extract_fingerprint_fields_direct(def: &TypeDefinitionIR) -> Option<Vec<String>> {
    match def {
        TypeDefinitionIR::Class(class) => {
            let mut fields: Vec<String> = class.fields.iter().map(|f| f.name.clone()).collect();
            fields.sort_unstable();
            fields.dedup();
            if fields.is_empty() {
                None
            } else {
                Some(fields)
            }
        }
        TypeDefinitionIR::Interface(iface) => {
            let mut fields: Vec<String> = iface.fields.iter().map(|f| f.name.clone()).collect();
            fields.sort_unstable();
            fields.dedup();
            if fields.is_empty() {
                None
            } else {
                Some(fields)
            }
        }
        TypeDefinitionIR::Enum(enum_ir) => {
            let mut variants: Vec<String> =
                enum_ir.variants.iter().map(|v| v.name.clone()).collect();
            variants.sort_unstable();
            variants.dedup();
            if variants.is_empty() {
                None
            } else {
                Some(variants)
            }
        }
        // Alias chains terminate at one indirection so we return
        // None here rather than recursing further.
        TypeDefinitionIR::TypeAlias(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ts_syn::declarative::{MacroArm, MacroDef};

    fn fake_def(name: &str, mode: MacroMode) -> MacroDef {
        let mut def = MacroDef::from_arms(
            name.to_string(),
            Vec::<MacroArm>::new(),
            mode,
            SpanIR::new(0, 0),
        );
        def.runtime = Some(format!("function __{}(x) {{ return x; }}", name));
        // call_arms can be empty for analyzer tests — the analyzer
        // doesn't care about arm contents, only about mode + name.
        def.call_arms = Some(Vec::new());
        def
    }

    fn site(macro_name: &str, shape: TypeShape) -> ResolvedCallSite {
        // Convenience wrapper for single-arg test call sites. PR 7
        // changed `arg_shape: TypeShape` to `arg_shapes: Vec<TypeShape>`;
        // this helper keeps the existing single-arg tests compact by
        // wrapping the shape in a one-element vec. Tests that need
        // multi-arg call sites construct `ResolvedCallSite` directly.
        ResolvedCallSite {
            macro_name: macro_name.to_string(),
            call_span: SpanIR::new(0, 0),
            arg_shapes: vec![shape],
        }
    }

    fn multi_arg_site(macro_name: &str, shapes: Vec<TypeShape>) -> ResolvedCallSite {
        ResolvedCallSite {
            macro_name: macro_name.to_string(),
            call_span: SpanIR::new(0, 0),
            arg_shapes: shapes,
        }
    }

    /// Build a `Named` shape with a sorted field fingerprint.
    fn named_with_fields(name: &str, fields: &[&str]) -> TypeShape {
        let mut fields: Vec<String> = fields.iter().map(|s| s.to_string()).collect();
        fields.sort();
        TypeShape::Named {
            name: name.to_string(),
            fields: Some(fields),
        }
    }

    #[test]
    fn analyze_monomorphic_share() {
        let mut reg = DeclarativeMacroRegistry::new();
        reg.register(fake_def("serialize", MacroMode::Auto))
            .unwrap();
        let sites = vec![
            site("serialize", TypeShape::named("User")),
            site("serialize", TypeShape::named("User")),
            site("serialize", TypeShape::named("User")),
        ];
        let report = analyze(&reg, &sites, 4);
        let info = report.lookup("serialize").unwrap();
        assert_eq!(info.distinct_shapes, 1);
        assert_eq!(info.recommendation, Recommendation::Share);
    }

    #[test]
    fn analyze_at_threshold_still_share() {
        let mut reg = DeclarativeMacroRegistry::new();
        reg.register(fake_def("serialize", MacroMode::Auto))
            .unwrap();
        let sites = vec![
            site("serialize", TypeShape::named("User")),
            site("serialize", TypeShape::named("Admin")),
            site("serialize", TypeShape::named("Guest")),
            site("serialize", TypeShape::named("Bot")),
        ];
        let report = analyze(&reg, &sites, 4);
        let info = report.lookup("serialize").unwrap();
        assert_eq!(info.distinct_shapes, 4);
        assert_eq!(info.recommendation, Recommendation::Share);
    }

    #[test]
    fn analyze_above_threshold_clusters_by_first_letter_fallback() {
        // Shapes without a fingerprint fall back to the first-letter
        // heuristic — this test intentionally uses `TypeShape::named`
        // (no fields) to exercise that path.
        let mut reg = DeclarativeMacroRegistry::new();
        reg.register(fake_def("serialize", MacroMode::Auto))
            .unwrap();
        let sites = vec![
            site("serialize", TypeShape::named("User")),
            site("serialize", TypeShape::named("Admin")),
            site("serialize", TypeShape::named("Alice")),
            site("serialize", TypeShape::named("Bob")),
            site("serialize", TypeShape::named("Guest")),
        ];
        let report = analyze(&reg, &sites, 4);
        let info = report.lookup("serialize").unwrap();
        assert_eq!(info.distinct_shapes, 5);
        let Recommendation::Cluster(clusters) = &info.recommendation else {
            panic!("expected Cluster, got {:?}", info.recommendation);
        };
        // Clusters: `a` (Admin, Alice), `b` (Bob), `g` (Guest), `u` (User)
        assert_eq!(clusters.len(), 4);
        let a = clusters.iter().find(|c| c.id == "a").unwrap();
        assert_eq!(a.shapes.len(), 2);
    }

    #[test]
    fn analyze_force_expand_when_cluster_still_megamorphic() {
        // All shapes start with the same letter → one cluster with 6
        // members → still above threshold → ForceExpand.
        let mut reg = DeclarativeMacroRegistry::new();
        reg.register(fake_def("serialize", MacroMode::Auto))
            .unwrap();
        let sites = vec![
            site("serialize", TypeShape::named("User1")),
            site("serialize", TypeShape::named("User2")),
            site("serialize", TypeShape::named("User3")),
            site("serialize", TypeShape::named("User4")),
            site("serialize", TypeShape::named("User5")),
            site("serialize", TypeShape::named("User6")),
        ];
        let report = analyze(&reg, &sites, 4);
        let info = report.lookup("serialize").unwrap();
        assert_eq!(info.recommendation, Recommendation::ForceExpand);
    }

    #[test]
    fn analyze_respects_per_macro_threshold() {
        // Override the threshold for this macro to 2 via MacroDef field.
        let mut reg = DeclarativeMacroRegistry::new();
        let mut def = fake_def("serialize", MacroMode::Auto);
        def.megamorphism_threshold = 2;
        reg.register(def).unwrap();

        let sites = vec![
            site("serialize", TypeShape::named("User")),
            site("serialize", TypeShape::named("Admin")),
            site("serialize", TypeShape::named("Guest")),
        ];
        let report = analyze(&reg, &sites, 4);
        // 3 shapes > per-macro threshold of 2 → Cluster.
        let info = report.lookup("serialize").unwrap();
        assert!(matches!(info.recommendation, Recommendation::Cluster(_)));
    }

    #[test]
    fn analyze_ignores_non_auto_macros() {
        let mut reg = DeclarativeMacroRegistry::new();
        reg.register(fake_def("expand_only", MacroMode::ExpandOnly))
            .unwrap();
        reg.register(fake_def("share_only", MacroMode::ShareOnly))
            .unwrap();
        reg.register(fake_def("auto", MacroMode::Auto)).unwrap();

        let sites = vec![
            site("expand_only", TypeShape::named("X")),
            site("share_only", TypeShape::named("X")),
            site("auto", TypeShape::named("X")),
        ];
        let report = analyze(&reg, &sites, 4);
        // Only the auto-mode macro shows up in the report.
        assert_eq!(report.per_macro.len(), 1);
        assert!(report.lookup("auto").is_some());
    }

    // -----------------------------------------------------------------
    // Phase 14: structural / Jaccard clustering tests
    // -----------------------------------------------------------------

    #[test]
    fn jaccard_identical_sets() {
        let a = vec!["id".into(), "name".into()];
        let b = vec!["id".into(), "name".into()];
        assert!((jaccard(&a, &b) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn jaccard_disjoint_sets() {
        let a = vec!["id".into(), "name".into()];
        let b = vec!["price".into(), "qty".into()];
        assert!(jaccard(&a, &b) < 1e-9);
    }

    #[test]
    fn jaccard_partial_overlap() {
        // a = {id, name, email}, b = {id, name, phone}
        // intersection = {id, name} = 2, union = {id, name, email, phone} = 4
        // Jaccard = 0.5
        let a = vec!["email".to_string(), "id".to_string(), "name".to_string()];
        let b = vec!["id".to_string(), "name".to_string(), "phone".to_string()];
        let j = jaccard(&a, &b);
        assert!((j - 0.5).abs() < 1e-9, "expected 0.5, got {}", j);
    }

    /// Wrap a single `TypeShape` into the one-element tuple form
    /// [`cluster_shapes`] now expects.
    fn tuple(shape: TypeShape) -> Vec<TypeShape> {
        vec![shape]
    }

    #[test]
    fn cluster_shapes_groups_identical_fingerprints() {
        // Two types with identical fields → single structural cluster.
        let shapes = vec![
            tuple(named_with_fields("User", &["id", "name", "email"])),
            tuple(named_with_fields("Admin", &["id", "name", "email"])),
        ];
        let clusters = cluster_shapes(&shapes, 4);
        assert_eq!(
            clusters.len(),
            1,
            "identical fields should collapse to one cluster, got: {:?}",
            clusters
        );
        assert_eq!(clusters[0].shapes.len(), 2);
    }

    #[test]
    fn cluster_shapes_groups_high_overlap() {
        // 4/5 overlap → Jaccard = 0.8 ≥ 0.6 → single cluster.
        let shapes = vec![
            tuple(named_with_fields(
                "User",
                &["id", "name", "email", "phone", "address"],
            )),
            tuple(named_with_fields(
                "Contact",
                &["id", "name", "email", "phone", "company"],
            )),
        ];
        let clusters = cluster_shapes(&shapes, 4);
        assert_eq!(clusters.len(), 1);
    }

    #[test]
    fn cluster_shapes_splits_low_overlap() {
        // 1/5 overlap → Jaccard = 0.2 < 0.6 → separate clusters.
        let shapes = vec![
            tuple(named_with_fields(
                "User",
                &["id", "name", "email", "phone", "address"],
            )),
            tuple(named_with_fields(
                "Order",
                &["id", "total", "status", "items", "customer"],
            )),
        ];
        let clusters = cluster_shapes(&shapes, 4);
        assert_eq!(clusters.len(), 2);
    }

    #[test]
    fn cluster_shapes_falls_back_to_prefix_without_fingerprint() {
        // No fingerprint → first-letter bucketing, matching the MVP
        // path. Two `A`-prefixed names collapse; a `B`-prefixed name
        // splits.
        let shapes = vec![
            tuple(TypeShape::named("Alice")),
            tuple(TypeShape::named("Admin")),
            tuple(TypeShape::named("Bob")),
        ];
        let clusters = cluster_shapes(&shapes, 4);
        assert_eq!(clusters.len(), 2);
        let a = clusters.iter().find(|c| c.id == "a").unwrap();
        assert_eq!(a.shapes.len(), 2);
        let b = clusters.iter().find(|c| c.id == "b").unwrap();
        assert_eq!(b.shapes.len(), 1);
    }

    #[test]
    fn cluster_shapes_mixes_structural_and_prefix_paths() {
        // Two shapes with fingerprints that match, plus one without
        // — the fingerprinted pair forms a structural cluster and the
        // non-fingerprinted shape lands in a prefix bucket.
        let shapes = vec![
            tuple(named_with_fields("User", &["id", "name"])),
            tuple(named_with_fields("Person", &["id", "name"])),
            tuple(TypeShape::named("Order")),
        ];
        let clusters = cluster_shapes(&shapes, 4);
        assert_eq!(clusters.len(), 2);
        // One structural cluster (2 members), one prefix "o" cluster (1 member).
        let sizes: Vec<usize> = clusters.iter().map(|c| c.shapes.len()).collect();
        assert!(sizes.contains(&2));
        assert!(sizes.contains(&1));
    }

    #[test]
    fn analyze_structurally_clusters_diverse_names_same_shape() {
        // Five distinctly named types that all share the same field
        // fingerprint. Under the first-letter fallback this would
        // ForceExpand (5 different buckets); under structural clustering
        // they collapse to a single cluster.
        let mut reg = DeclarativeMacroRegistry::new();
        reg.register(fake_def("serialize", MacroMode::Auto))
            .unwrap();
        let sites = vec![
            site("serialize", named_with_fields("Alpha", &["id", "name"])),
            site("serialize", named_with_fields("Bravo", &["id", "name"])),
            site("serialize", named_with_fields("Charlie", &["id", "name"])),
            site("serialize", named_with_fields("Delta", &["id", "name"])),
            site("serialize", named_with_fields("Echo", &["id", "name"])),
        ];
        let report = analyze(&reg, &sites, 4);
        let info = report.lookup("serialize").unwrap();
        let Recommendation::Cluster(clusters) = &info.recommendation else {
            panic!(
                "expected Cluster (structural grouping), got {:?}",
                info.recommendation
            );
        };
        assert_eq!(
            clusters.len(),
            1,
            "identical fingerprints should collapse to one cluster: {:?}",
            clusters
        );
    }

    // -----------------------------------------------------------------
    // PR 7 / fix D: multi-arg shape tuples
    // -----------------------------------------------------------------

    #[test]
    fn analyze_two_arg_monomorphic_shares() {
        // All call sites have the same `(User, Order)` shape tuple —
        // one distinct polymorphism class → Share.
        let mut reg = DeclarativeMacroRegistry::new();
        reg.register(fake_def("serialize", MacroMode::Auto))
            .unwrap();
        let sites = vec![
            multi_arg_site(
                "serialize",
                vec![TypeShape::named("User"), TypeShape::named("Order")],
            ),
            multi_arg_site(
                "serialize",
                vec![TypeShape::named("User"), TypeShape::named("Order")],
            ),
            multi_arg_site(
                "serialize",
                vec![TypeShape::named("User"), TypeShape::named("Order")],
            ),
        ];
        let report = analyze(&reg, &sites, 4);
        let info = report.lookup("serialize").unwrap();
        assert_eq!(info.distinct_shapes, 1);
        assert_eq!(info.recommendation, Recommendation::Share);
    }

    #[test]
    fn analyze_two_arg_divergent_second_position_clusters() {
        // First argument is always `User`, but the second argument
        // varies: `Order`, `Invoice`, `Product`. With threshold=1, 3
        // distinct tuples > 1 → Cluster. The clustering falls back
        // to first-letter bucketing on the first arg (all `u`), so
        // every tuple lands in the same bucket. That bucket has 3
        // distinct fingerprints — above the per-cluster threshold —
        // so the analyzer picks ForceExpand. Verifies that divergent
        // per-position shapes don't get silently collapsed by the
        // first-arg heuristic.
        let mut reg = DeclarativeMacroRegistry::new();
        let mut def = fake_def("serialize", MacroMode::Auto);
        def.megamorphism_threshold = 1;
        reg.register(def).unwrap();
        let sites = vec![
            multi_arg_site(
                "serialize",
                vec![TypeShape::named("User"), TypeShape::named("Order")],
            ),
            multi_arg_site(
                "serialize",
                vec![TypeShape::named("User"), TypeShape::named("Invoice")],
            ),
            multi_arg_site(
                "serialize",
                vec![TypeShape::named("User"), TypeShape::named("Product")],
            ),
        ];
        let report = analyze(&reg, &sites, 4);
        let info = report.lookup("serialize").unwrap();
        assert_eq!(
            info.distinct_shapes, 3,
            "three distinct tuples expected, got {}",
            info.distinct_shapes
        );
        assert_eq!(
            info.recommendation,
            Recommendation::ForceExpand,
            "divergent second-argument shapes should force expand"
        );
    }

    #[test]
    fn analyze_arity_mismatch_does_not_cluster_together() {
        // `(User)` and `(User, Order)` share the same first argument
        // but have different arity — they must never end up in the
        // same cluster, because the helper function's parameter
        // count differs.
        let mut reg = DeclarativeMacroRegistry::new();
        let mut def = fake_def("serialize", MacroMode::Auto);
        def.megamorphism_threshold = 1;
        reg.register(def).unwrap();
        let sites = vec![
            multi_arg_site("serialize", vec![TypeShape::named("User")]),
            multi_arg_site(
                "serialize",
                vec![TypeShape::named("User"), TypeShape::named("Order")],
            ),
        ];
        let report = analyze(&reg, &sites, 4);
        let info = report.lookup("serialize").unwrap();
        assert_eq!(info.distinct_shapes, 2);
        // Either Cluster with two separate clusters or ForceExpand —
        // both are acceptable; the invariant is that the two tuples
        // never land in the same cluster.
        match &info.recommendation {
            Recommendation::Cluster(clusters) => {
                for cluster in clusters {
                    let arities: std::collections::HashSet<usize> =
                        cluster.shapes.iter().map(|t| t.len()).collect();
                    assert_eq!(
                        arities.len(),
                        1,
                        "cluster `{}` mixed arities: {:?}",
                        cluster.id,
                        cluster.shapes
                    );
                }
            }
            Recommendation::ForceExpand => {}
            other => panic!("unexpected recommendation: {:?}", other),
        }
    }

    // -----------------------------------------------------------------
    // PR 10 — mean_pairwise_jaccard + enum/alias fingerprints
    // -----------------------------------------------------------------

    #[test]
    fn mean_pairwise_jaccard_arity_mismatch_is_zero() {
        let a = vec![named_with_fields("User", &["id", "name"])];
        let b = vec![
            named_with_fields("User", &["id", "name"]),
            named_with_fields("Order", &["id", "total"]),
        ];
        assert!((mean_pairwise_jaccard(&a, &b) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn mean_pairwise_jaccard_identical_tuples_are_one() {
        let a = vec![
            named_with_fields("User", &["id", "name"]),
            named_with_fields("Order", &["id", "total"]),
        ];
        let b = a.clone();
        assert!((mean_pairwise_jaccard(&a, &b) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn mean_pairwise_jaccard_partial_overlap_across_positions() {
        // Position 0: `User` (id, name) vs `User` (id, name) → 1.0
        // Position 1: `OrderA` (id, total, status, items, customer)
        //             vs `OrderB` (id, total, status, items, extra)
        //             intersect = 4, union = 6, Jaccard ≈ 0.667
        // Mean ≈ (1.0 + 0.667) / 2 ≈ 0.833
        let a = vec![
            named_with_fields("User", &["id", "name"]),
            named_with_fields("OrderA", &["customer", "id", "items", "status", "total"]),
        ];
        let b = vec![
            named_with_fields("User", &["id", "name"]),
            named_with_fields("OrderB", &["extra", "id", "items", "status", "total"]),
        ];
        let score = mean_pairwise_jaccard(&a, &b);
        // Should be high enough to exceed the 0.6 threshold.
        assert!(
            score >= JACCARD_THRESHOLD,
            "expected ≥ threshold, got {}",
            score
        );
    }

    #[test]
    fn mean_pairwise_jaccard_disjoint_tail_positions_fall_below_threshold() {
        // Position 0 matches exactly (1.0). Position 1 has 0
        // overlap (0.0). Mean = 0.5, below threshold.
        let a = vec![
            named_with_fields("User", &["id", "name"]),
            named_with_fields("OrderA", &["a", "b", "c"]),
        ];
        let b = vec![
            named_with_fields("User", &["id", "name"]),
            named_with_fields("OrderB", &["x", "y", "z"]),
        ];
        let score = mean_pairwise_jaccard(&a, &b);
        assert!(
            score < JACCARD_THRESHOLD,
            "expected below threshold, got {}",
            score
        );
    }

    #[test]
    fn cluster_shapes_groups_tuples_via_mean_jaccard() {
        // Two 2-arg tuples with high overlap in BOTH positions
        // should cluster together, exercising PR 10's pairwise
        // Jaccard across all positions (vs PR 7's "exact tail
        // equality" which would have split them).
        let shapes = vec![
            vec![
                named_with_fields("User", &["id", "name", "email"]),
                named_with_fields("OrderA", &["id", "total", "status", "customer", "items"]),
            ],
            vec![
                named_with_fields("User", &["id", "name", "email"]),
                named_with_fields("OrderB", &["id", "total", "status", "customer", "notes"]),
            ],
        ];
        let clusters = cluster_shapes(&shapes, 4);
        assert_eq!(
            clusters.len(),
            1,
            "expected pairwise-Jaccard to collapse both tuples into one cluster, got: {:?}",
            clusters
        );
    }

    #[test]
    fn cluster_shapes_splits_tuples_with_divergent_tails() {
        // Two 2-arg tuples: same first arg, completely different
        // second arg. Mean Jaccard = (1.0 + 0.0) / 2 = 0.5, below
        // threshold → separate clusters.
        let shapes = vec![
            vec![
                named_with_fields("User", &["id", "name"]),
                named_with_fields("OrderA", &["a", "b", "c"]),
            ],
            vec![
                named_with_fields("User", &["id", "name"]),
                named_with_fields("OrderB", &["x", "y", "z"]),
            ],
        ];
        let clusters = cluster_shapes(&shapes, 4);
        assert_eq!(
            clusters.len(),
            2,
            "expected divergent tails to split: {:?}",
            clusters
        );
    }
}
