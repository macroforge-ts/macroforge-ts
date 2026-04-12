//! Per-file storage for parsed declarative macros.
//!
//! PR 11 added lexical-scope awareness: the registry can now hold
//! multiple entries with the same name as long as their enclosing
//! scope spans don't overlap. Lookups take an optional byte
//! position so the rewriter can resolve `$name` to the *innermost*
//! declaration that contains the call site — giving macros the
//! same lexical-shadowing behaviour JavaScript variables get.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

use crate::ts_syn::abi::SpanIR;
use crate::ts_syn::declarative::{BodyToken, MacroDef};

/// Error returned when registration fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    /// Two macros in the same file share the same `$name`.
    DuplicateName(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::DuplicateName(name) => {
                write!(
                    f,
                    "declarative macro `${}` is defined more than once in this file",
                    name
                )
            }
        }
    }
}

impl std::error::Error for RegistryError {}

/// Error returned when inter-macro composition contains a cycle.
///
/// The `names` list is the cycle participants in the order they were
/// discovered during sort — it's not necessarily a minimal cycle, but
/// it's enough for a useful diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleError {
    pub names: Vec<String>,
}

impl std::fmt::Display for CycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "declarative macros form a cycle: {}",
            self.names
                .iter()
                .map(|n| format!("${}", n))
                .collect::<Vec<_>>()
                .join(" → ")
        )
    }
}

impl std::error::Error for CycleError {}

/// In-file registry of parsed declarative macros.
///
/// Built per-file by the discovery pass. Cross-file macro imports
/// are handled separately via `/** import macro */` comments and
/// are registered here as if they were top-level declarations of
/// the importing file.
///
/// PR 11 made the registry scope-aware: each registered macro
/// carries an enclosing `scope_span`, and [`Self::lookup_at`]
/// resolves a name at a specific byte position to the **innermost**
/// declaration whose scope contains that position. Multiple macros
/// can share a name as long as their scope spans are disjoint or
/// strictly nested — a shadowing nested declaration takes
/// precedence over an outer one at positions inside the inner
/// scope, and the outer one is restored once we leave the nested
/// scope.
///
/// The legacy [`Self::lookup`] method still exists for callers
/// that don't have a position handy (e.g. the topological-sort
/// build). It returns the entry with the **largest** scope span,
/// which is the most globally-visible candidate and matches the
/// pre-PR-11 semantic for flat top-level registrations.
#[derive(Debug, Default, Clone)]
pub struct DeclarativeMacroRegistry {
    entries: Vec<ScopedEntry>,
}

#[derive(Debug, Clone)]
struct ScopedEntry {
    def: Arc<MacroDef>,
    scope_span: SpanIR,
}

impl DeclarativeMacroRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a parsed macro with no scope restriction (i.e. at
    /// the file's global scope). Fails if another globally-scoped
    /// entry with the same name already exists. Retained as a
    /// convenience for tests and cross-file import registration.
    pub fn register(&mut self, def: MacroDef) -> Result<(), RegistryError> {
        // The "global" scope span: any position is inside it. We
        // use `(1, u32::MAX)` so `span_contains` always returns
        // true for a valid 1-based position.
        self.register_scoped(def, SpanIR::new(1, u32::MAX))
    }

    /// Register a macro with a specific enclosing scope span. Two
    /// entries with the same name may coexist iff their scope
    /// spans are either strictly nested (one inside the other —
    /// legal lexical shadowing) or completely disjoint. Overlap
    /// without containment is rejected as a duplicate.
    ///
    /// The rule matches JavaScript's lexical-variable semantics:
    /// `{ const x; { const x; } }` is legal (inner shadows outer),
    /// `const x; const x;` in the same scope is not.
    pub fn register_scoped(
        &mut self,
        def: MacroDef,
        scope_span: SpanIR,
    ) -> Result<(), RegistryError> {
        for existing in &self.entries {
            if existing.def.name == def.name
                && !spans_are_disjoint_or_nested(existing.scope_span, scope_span)
            {
                return Err(RegistryError::DuplicateName(def.name));
            }
        }
        self.entries.push(ScopedEntry {
            def: Arc::new(def),
            scope_span,
        });
        Ok(())
    }

    /// Look up a macro by name, returning the entry whose scope
    /// span is **largest** — i.e. the most globally-visible
    /// candidate. Used by callers that don't have a position to
    /// query against (the topological-sort build, cross-file
    /// analysis, tests).
    pub fn lookup(&self, name: &str) -> Option<&Arc<MacroDef>> {
        self.entries
            .iter()
            .filter(|e| e.def.name == name)
            .max_by_key(|e| (e.scope_span.end as u64).saturating_sub(e.scope_span.start as u64))
            .map(|e| &e.def)
    }

    /// Look up a macro by name at a specific 1-based byte position.
    /// Returns the **innermost** (smallest scope span) entry whose
    /// scope contains `pos`. This is the scoped-lookup entry point
    /// used by the rewriter's `try_rewrite_call` — it resolves
    /// call-site references against the lexically-nearest macro
    /// declaration, matching JavaScript's variable-shadowing
    /// semantic.
    pub fn lookup_at(&self, name: &str, pos: u32) -> Option<&Arc<MacroDef>> {
        self.entries
            .iter()
            .filter(|e| e.def.name == name && span_contains(e.scope_span, pos))
            .min_by_key(|e| (e.scope_span.end as u64).saturating_sub(e.scope_span.start as u64))
            .map(|e| &e.def)
    }

    /// `true` iff no macros have been registered.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Number of registered entries. Shadowed declarations each
    /// count once — a file with `const $foo` at the top level
    /// plus `const $foo` shadowed inside a function body reports
    /// `len() == 2`.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Iterate over every registered macro as `(&name, &def)`
    /// tuples. With shadowing enabled, the same name may appear
    /// multiple times. Callers that only care about uniqueness
    /// should dedupe themselves.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Arc<MacroDef>)> {
        self.entries.iter().map(|e| (&e.def.name, &e.def))
    }

    /// Return the registered macros in topological order — callees
    /// before callers — so that when the rewriter walks macros in
    /// order, any inter-macro reference resolves to a macro that's
    /// already been processed.
    ///
    /// Returns `Err(CycleError)` if any macro (transitively) calls
    /// itself. Mutual recursion between declarative macros is an
    /// infinite expansion and must be rejected at registration time
    /// rather than at expansion time.
    ///
    /// Macros without `MacroCall` tokens in their bodies trivially
    /// sort first.
    pub fn topological_order(&self) -> Result<Vec<Arc<MacroDef>>, CycleError> {
        // Topological sort works on UNIQUE NAMES — the graph node
        // is "the name `$foo`", not a specific scoped entry. When
        // the registry holds multiple entries for the same name
        // (PR 11 shadowing), each name node contributes one
        // MacroDef to the output — we pick the one with the
        // largest scope (most globally visible) so composition
        // resolves consistently.
        //
        // Shadowing + macro-body composition is a niche
        // intersection that's deliberately not supported: a macro
        // body that calls `$foo` will resolve to the outermost
        // `$foo` regardless of lexical nesting at the definition
        // site. Callers that need shadowed composition should
        // resolve the callee themselves via `lookup_at`.
        let mut by_name: HashMap<String, Arc<MacroDef>> = HashMap::new();
        for entry in &self.entries {
            by_name
                .entry(entry.def.name.clone())
                .and_modify(|existing| {
                    // Prefer the most globally-scoped entry for
                    // name→def resolution.
                    let existing_size = existing.span.end.saturating_sub(existing.span.start);
                    let candidate_size =
                        entry.scope_span.end.saturating_sub(entry.scope_span.start);
                    if candidate_size > existing_size {
                        *existing = entry.def.clone();
                    }
                })
                .or_insert_with(|| entry.def.clone());
        }

        // Build the adjacency list: caller → set of callees.
        let mut edges: HashMap<String, HashSet<String>> = HashMap::new();
        let mut reverse: HashMap<String, HashSet<String>> = HashMap::new();
        for (name, def) in &by_name {
            let callees = collect_macro_call_names(&def.arms);
            edges
                .entry(name.clone())
                .or_default()
                .extend(callees.iter().cloned());
            for callee in &callees {
                // Ignore references to unknown macros — cross-file
                // imports and bare placeholders shouldn't block the
                // sort.
                if by_name.contains_key(callee) {
                    reverse
                        .entry(callee.clone())
                        .or_default()
                        .insert(name.clone());
                }
            }
        }

        // Kahn's algorithm: start with nodes that have no outgoing
        // edges (pure leaf macros), process them, strip their incoming
        // edges, and repeat. When we run out of processable nodes
        // before visiting everything, whatever's left is a cycle.
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        for name in by_name.keys() {
            // Count outgoing edges that resolve within the registry
            // (callees not in the registry are cross-file imports
            // and don't contribute to the sort order here).
            let out = edges
                .get(name)
                .map(|s| s.iter().filter(|c| by_name.contains_key(*c)).count())
                .unwrap_or(0);
            in_degree.insert(name.clone(), out);
        }

        let mut queue: VecDeque<String> = in_degree
            .iter()
            .filter(|&(_, d)| *d == 0)
            .map(|(n, _)| n.clone())
            .collect();
        let mut sorted: Vec<Arc<MacroDef>> = Vec::with_capacity(by_name.len());
        let mut visited: HashSet<String> = HashSet::new();

        while let Some(name) = queue.pop_front() {
            if !visited.insert(name.clone()) {
                continue;
            }
            if let Some(def) = by_name.get(&name) {
                sorted.push(def.clone());
            }
            // For each macro that depends on `name` (i.e., lists
            // `name` as a callee), decrement its in_degree. When it
            // hits zero, enqueue it.
            if let Some(dependents) = reverse.get(&name) {
                for dep in dependents {
                    if let Some(d) = in_degree.get_mut(dep) {
                        *d = d.saturating_sub(1);
                        if *d == 0 {
                            queue.push_back(dep.clone());
                        }
                    }
                }
            }
        }

        if sorted.len() != by_name.len() {
            // Whatever didn't get visited is in a cycle.
            let cycle_names: Vec<String> = by_name
                .keys()
                .filter(|n| !visited.contains(*n))
                .cloned()
                .collect();
            return Err(CycleError { names: cycle_names });
        }
        Ok(sorted)
    }
}

/// Returns `true` iff `pos` is inside `span` (inclusive start,
/// exclusive end — matching the half-open convention used throughout
/// the patch IR). A zero-width span `[x, x)` never contains anything.
fn span_contains(span: SpanIR, pos: u32) -> bool {
    pos >= span.start && pos < span.end
}

/// Returns `true` iff two spans are either strictly nested (one
/// contains the other with proper inclusion) or completely disjoint.
/// Returns `false` when they overlap without containment — the
/// "half-overlap" case that registration rejects as a duplicate.
///
/// Identical spans count as "nested" (trivially: each contains
/// the other) and are therefore considered colliding — a
/// duplicate in the same scope.
fn spans_are_disjoint_or_nested(a: SpanIR, b: SpanIR) -> bool {
    // Identical spans: same scope → collision.
    if a.start == b.start && a.end == b.end {
        return false;
    }
    // Strict containment either way → legal shadowing.
    if a.start <= b.start && b.end <= a.end {
        return true;
    }
    if b.start <= a.start && a.end <= b.end {
        return true;
    }
    // Completely disjoint → legal (unrelated scopes).
    if a.end <= b.start || b.end <= a.start {
        return true;
    }
    // Partial overlap.
    false
}

/// Walk a list of arms and collect every `MacroCall` callee name,
/// recursing through nested `MacroCall` and `Repetition` tokens.
fn collect_macro_call_names(arms: &[crate::ts_syn::declarative::MacroArm]) -> HashSet<String> {
    let mut out = HashSet::new();
    for arm in arms {
        collect_from_tokens(&arm.body.0, &mut out);
    }
    out
}

fn collect_from_tokens(tokens: &[BodyToken], out: &mut HashSet<String>) {
    for t in tokens {
        match t {
            BodyToken::MacroCall { name, args } => {
                out.insert(name.clone());
                collect_from_tokens(args, out);
            }
            BodyToken::Repetition { body, .. } => collect_from_tokens(body, out),
            BodyToken::Literal(_) | BodyToken::Substitution(_) => {}
        }
    }
}
