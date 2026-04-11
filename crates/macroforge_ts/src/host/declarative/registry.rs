//! Per-file storage for parsed declarative macros.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

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

/// In-file registry of parsed declarative macros, keyed by `$name`.
///
/// A fresh registry is built per-file by the discovery pass. Cross-file
/// macro imports are deferred; the current MVP scope is definition and
/// invocation inside the same file.
#[derive(Debug, Default, Clone)]
pub struct DeclarativeMacroRegistry {
    by_name: HashMap<String, Arc<MacroDef>>,
}

impl DeclarativeMacroRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a parsed macro. Returns an error if the name is already taken.
    pub fn register(&mut self, def: MacroDef) -> Result<(), RegistryError> {
        if self.by_name.contains_key(&def.name) {
            return Err(RegistryError::DuplicateName(def.name));
        }
        let name = def.name.clone();
        self.by_name.insert(name, Arc::new(def));
        Ok(())
    }

    /// Look up a macro by name (the name excludes the leading `$`).
    pub fn lookup(&self, name: &str) -> Option<&Arc<MacroDef>> {
        self.by_name.get(name)
    }

    /// `true` iff no macros have been registered.
    pub fn is_empty(&self) -> bool {
        self.by_name.is_empty()
    }

    /// Number of registered macros.
    pub fn len(&self) -> usize {
        self.by_name.len()
    }

    /// Iterate over all registered macros.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Arc<MacroDef>)> {
        self.by_name.iter()
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
        // Build the adjacency list: caller → set of callees.
        let mut edges: HashMap<String, HashSet<String>> = HashMap::new();
        let mut reverse: HashMap<String, HashSet<String>> = HashMap::new();
        for (name, def) in &self.by_name {
            let callees = collect_macro_call_names(&def.arms);
            edges
                .entry(name.clone())
                .or_default()
                .extend(callees.iter().cloned());
            for callee in &callees {
                // Ignore references to unknown macros — cross-file
                // imports and bare placeholders shouldn't block the
                // sort.
                if self.by_name.contains_key(callee) {
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
        for name in self.by_name.keys() {
            // Count outgoing edges that resolve within the registry
            // (callees not in the registry are cross-file imports
            // and don't contribute to the sort order here).
            let out = edges
                .get(name)
                .map(|s| s.iter().filter(|c| self.by_name.contains_key(*c)).count())
                .unwrap_or(0);
            in_degree.insert(name.clone(), out);
        }

        let mut queue: VecDeque<String> = in_degree
            .iter()
            .filter(|&(_, d)| *d == 0)
            .map(|(n, _)| n.clone())
            .collect();
        let mut sorted: Vec<Arc<MacroDef>> = Vec::with_capacity(self.by_name.len());
        let mut visited: HashSet<String> = HashSet::new();

        while let Some(name) = queue.pop_front() {
            if !visited.insert(name.clone()) {
                continue;
            }
            if let Some(def) = self.by_name.get(&name) {
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

        if sorted.len() != self.by_name.len() {
            // Whatever didn't get visited is in a cycle.
            let cycle_names: Vec<String> = self
                .by_name
                .keys()
                .filter(|n| !visited.contains(*n))
                .cloned()
                .collect();
            return Err(CycleError { names: cycle_names });
        }
        Ok(sorted)
    }
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
