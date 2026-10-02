//! Per-file storage for parsed declarative macros.
//!
//! PR 11 added lexical-scope awareness: the registry can now hold
//! multiple entries with the same name as long as their enclosing
//! scope spans don't overlap. Lookups take an optional byte
//! position so the rewriter can resolve `$name` to the *innermost*
//! declaration that contains the call site, giving macros the
//! same lexical-shadowing behaviour JavaScript variables get.

use std::sync::Arc;

use crate::ts_syn::abi::SpanIR;
use crate::ts_syn::declarative::MacroDef;

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
/// strictly nested: a shadowing nested declaration takes
/// precedence over an outer one at positions inside the inner
/// scope, and the outer one is restored once we leave the nested
/// scope.
///
/// [`Self::lookup`] serves callers without a position: it returns
/// the entry with the **largest** scope span, the most globally
/// visible candidate.
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
    /// spans are either strictly nested (one inside the other,
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
    /// span is **largest**, i.e. the most globally-visible
    /// candidate. Used by callers that don't have a position to
    /// query against.
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
    /// used by the rewriter's `try_rewrite_call`: it resolves
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
    /// count once: a file with `const $foo` at the top level
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
}

/// Returns `true` iff `pos` is inside `span` (inclusive start,
/// exclusive end, matching the half-open convention used throughout
/// the patch IR). A zero-width span `[x, x)` never contains anything.
fn span_contains(span: SpanIR, pos: u32) -> bool {
    pos >= span.start && pos < span.end
}

/// Returns `true` iff two spans are either strictly nested (one
/// contains the other with proper inclusion) or completely disjoint.
/// Returns `false` when they overlap without containment: the
/// "half-overlap" case that registration rejects as a duplicate.
///
/// Identical spans count as "nested" (trivially: each contains
/// the other) and are therefore considered colliding: a
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
