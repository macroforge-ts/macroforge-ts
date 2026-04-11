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
    /// Coarse shape of the argument(s) — what V8 would see as the
    /// "object shape" of the flow into the shared runtime.
    pub arg_shape: TypeShape,
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

/// A cluster of type shapes that together stay under the megamorphism
/// threshold. All call sites with shapes in this cluster dispatch to
/// the same helper function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeCluster {
    /// The shapes that belong to this cluster.
    pub shapes: Vec<TypeShape>,
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
        let mut shape_set: Vec<TypeShape> = Vec::new();
        for site in &sites {
            if !shape_set.contains(&site.arg_shape) {
                shape_set.push(site.arg_shape.clone());
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
            // distinct *structural fingerprints* than the threshold. Two
            // shapes with identical fingerprints look like the same
            // hidden class to V8, so they don't add to the cluster's
            // polymorphism budget. Named shapes without a fingerprint
            // fall back to counting by name (each distinct name = one
            // distinct fingerprint).
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

/// Partition a set of shapes into clusters that each stay at or below
/// the threshold.
///
/// Strategy:
///
/// 1. **Structural grouping** — for `Named` shapes carrying a
///    `fields` fingerprint, run a greedy Jaccard-similarity pass:
///    each shape joins the first existing cluster whose members all
///    have a Jaccard similarity ≥ [`JACCARD_THRESHOLD`] with it,
///    otherwise it seeds a new cluster. This groups `User` and
///    `AdminUser` (same fields) together but keeps `User` and
///    `OrderItem` (disjoint fields) apart, even when the first-letter
///    heuristic would have conflated them.
///
/// 2. **Name-prefix fallback** — `Named` shapes without a fingerprint
///    (e.g. the registry lacked the type, or we were called without
///    a registry) fall back to the MVP behavior: group by the
///    first letter of the name.
///
/// 3. **Literal / opaque pass-through** — `Literal` goes into a
///    `"lit"` cluster and `Opaque` goes into an `"opaque"` cluster,
///    same as before.
///
/// Clusters that are still above the per-macro threshold after this
/// pass are not split further — the caller detects that case and
/// falls back to `ForceExpand`.
fn cluster_shapes(shapes: &[TypeShape], _threshold: usize) -> Vec<TypeCluster> {
    // Keep a stable traversal order so the output is deterministic.
    let mut structural: Vec<Vec<TypeShape>> = Vec::new();
    let mut prefix_buckets: HashMap<String, Vec<TypeShape>> = HashMap::new();
    let mut literal_bucket: Vec<TypeShape> = Vec::new();
    let mut opaque_bucket: Vec<TypeShape> = Vec::new();

    for shape in shapes {
        match shape {
            TypeShape::Named {
                fields: Some(fields),
                ..
            } if !fields.is_empty() => {
                // Try to join an existing structural cluster: a shape
                // may join a cluster iff its Jaccard similarity with
                // *every* member of that cluster is ≥ threshold.
                // Otherwise seed a new cluster. `name` is ignored in
                // the structural pass — field similarity is the signal
                // we trust.
                let mut joined = false;
                for cluster in structural.iter_mut() {
                    let fits = cluster.iter().all(|existing| {
                        if let TypeShape::Named {
                            fields: Some(ef), ..
                        } = existing
                        {
                            jaccard(fields, ef) >= JACCARD_THRESHOLD
                        } else {
                            false
                        }
                    });
                    if fits {
                        cluster.push(shape.clone());
                        joined = true;
                        break;
                    }
                }
                if !joined {
                    structural.push(vec![shape.clone()]);
                }
            }
            TypeShape::Named { name, .. } => {
                // Fingerprint missing (or empty). Fall back to first-
                // letter grouping so we don't regress vs. the MVP.
                let key = name
                    .chars()
                    .next()
                    .map(|c| c.to_ascii_lowercase().to_string())
                    .unwrap_or_else(|| "_".to_string());
                prefix_buckets.entry(key).or_default().push(shape.clone());
            }
            TypeShape::Literal(_) => literal_bucket.push(shape.clone()),
            TypeShape::Opaque => opaque_bucket.push(shape.clone()),
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

/// Count how many distinct "V8 hidden class equivalent" buckets a set
/// of shapes spans. This is the denominator in the megamorphism check:
///
/// - Two `Named` shapes with identical `fields` fingerprints count as
///   one bucket (same declared shape → V8 would likely collapse them).
/// - Two `Named` shapes without a fingerprint count as distinct buckets
///   per *name* (the MVP heuristic).
/// - `Literal` shapes collapse by their JS runtime-type tag.
/// - All `Opaque` shapes collapse to a single bucket.
fn count_distinct_fingerprints(shapes: &[TypeShape]) -> usize {
    use std::collections::HashSet;
    let mut seen: HashSet<String> = HashSet::new();
    for shape in shapes {
        let key = match shape {
            TypeShape::Named {
                fields: Some(fs), ..
            } if !fs.is_empty() => format!("s:{}", fs.join(",")),
            TypeShape::Named { name, .. } => format!("n:{}", name),
            TypeShape::Literal(kind) => format!("l:{}", kind),
            TypeShape::Opaque => "o".to_string(),
        };
        seen.insert(key);
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

/// Derive a stable cluster id from its members' names. We use the
/// sorted, `_`-joined list of names — deterministic across runs and
/// distinct from the prefix-bucket keys (which are single letters).
fn structural_cluster_id(group: &[TypeShape]) -> String {
    let mut names: Vec<&str> = group
        .iter()
        .filter_map(|s| match s {
            TypeShape::Named { name, .. } => Some(name.as_str()),
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
/// When `type_registry` is `Some`, the `Named` result is augmented
/// with a sorted field-name fingerprint from the registry (if the
/// type is known). The fingerprint is used by [`cluster_shapes`] for
/// structural clustering (Phase 14).
pub fn extract_type_shape(
    arg: &oxc::ast::ast::Argument<'_>,
    type_registry: Option<&TypeRegistry>,
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
fn named_with_fingerprint(name: &str, type_registry: Option<&TypeRegistry>) -> TypeShape {
    let fields = type_registry.and_then(|reg| reg.get(name)).map(|entry| {
        let mut fields = match &entry.definition {
            TypeDefinitionIR::Class(class) => class.fields.iter().map(|f| f.name.clone()).collect(),
            TypeDefinitionIR::Interface(iface) => {
                iface.fields.iter().map(|f| f.name.clone()).collect()
            }
            // Enums and type aliases don't have a "field set" in the
            // same sense, but we could fingerprint them on variants /
            // structural members. For now, leave them as non-fingerprinted
            // (None returned as Some(empty) isn't useful — caller treats
            // empty field lists as "no fingerprint" via the match arm).
            TypeDefinitionIR::Enum(_) | TypeDefinitionIR::TypeAlias(_) => Vec::<String>::new(),
        };
        fields.sort_unstable();
        fields.dedup();
        fields
    });

    TypeShape::Named {
        name: name.to_string(),
        fields,
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
        ResolvedCallSite {
            macro_name: macro_name.to_string(),
            call_span: SpanIR::new(0, 0),
            arg_shape: shape,
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

    #[test]
    fn cluster_shapes_groups_identical_fingerprints() {
        // Two types with identical fields → single structural cluster.
        let shapes = vec![
            named_with_fields("User", &["id", "name", "email"]),
            named_with_fields("Admin", &["id", "name", "email"]),
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
            named_with_fields("User", &["id", "name", "email", "phone", "address"]),
            named_with_fields("Contact", &["id", "name", "email", "phone", "company"]),
        ];
        let clusters = cluster_shapes(&shapes, 4);
        assert_eq!(clusters.len(), 1);
    }

    #[test]
    fn cluster_shapes_splits_low_overlap() {
        // 1/5 overlap → Jaccard = 0.2 < 0.6 → separate clusters.
        let shapes = vec![
            named_with_fields("User", &["id", "name", "email", "phone", "address"]),
            named_with_fields("Order", &["id", "total", "status", "items", "customer"]),
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
            TypeShape::named("Alice"),
            TypeShape::named("Admin"),
            TypeShape::named("Bob"),
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
            named_with_fields("User", &["id", "name"]),
            named_with_fields("Person", &["id", "name"]),
            TypeShape::named("Order"),
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
}
