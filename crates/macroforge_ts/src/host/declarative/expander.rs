//! Expand a matched arm's body into source text, with hygienic renaming.
//!
//! The expander walks a [`Body`] token-by-token, substituting bound
//! fragments and unrolling repetitions. Identifiers inside the body that
//! start with `__` (double underscore) are treated as macro-introduced
//! and get a unique per-expansion suffix (`__v` → `__v$7`) so they
//! don't collide with call-site identifiers.

use std::collections::{HashMap, HashSet};

use crate::ts_syn::declarative::{Body, BodyToken};

use super::matcher::Binding;

/// Whether the expansion slot is expression or statement position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpansionContext {
    /// Expression position — a block body needs an IIFE wrap.
    Expression,
    /// Statement position — block bodies are spliced in directly.
    Statement,
}

/// Errors that can occur during body expansion.
#[derive(Debug, Clone)]
pub enum ExpandError {
    /// A `$name` substitution referenced an unbound name.
    UnboundName(String),
    /// A single-binding name appeared inside a repetition, or a
    /// sequence-binding name appeared outside a repetition.
    WrongBindingShape(String),
    /// Two sequence bindings inside the same repetition have different lengths.
    InconsistentSequenceLength(usize, usize),
    /// A repetition mentioned no sequence bindings — we can't know how
    /// many times to iterate.
    UnanchoredRepetition,
}

impl std::fmt::Display for ExpandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpandError::UnboundName(name) => write!(f, "unbound macro metavariable `${}`", name),
            ExpandError::WrongBindingShape(name) => write!(
                f,
                "metavariable `${}` has the wrong binding shape (single vs sequence)",
                name
            ),
            ExpandError::InconsistentSequenceLength(a, b) => write!(
                f,
                "sequence bindings in the same repetition have different lengths ({} vs {})",
                a, b
            ),
            ExpandError::UnanchoredRepetition => write!(
                f,
                "repetition in body mentions no sequence-bound metavariable; cannot infer length"
            ),
        }
    }
}

impl std::error::Error for ExpandError {}

/// Expand `body` into a source string, given captured fragment bindings.
pub fn expand_body(
    body: &Body,
    bindings: &HashMap<String, Binding>,
    expansion_id: u32,
    context: ExpansionContext,
) -> Result<String, ExpandError> {
    let mut out = String::new();
    render_tokens(&body.0, bindings, expansion_id, &mut out)?;
    let rewritten = rewrite_hygiene(out, expansion_id);
    Ok(maybe_wrap_iife(rewritten, context))
}

fn render_tokens(
    tokens: &[BodyToken],
    bindings: &HashMap<String, Binding>,
    expansion_id: u32,
    out: &mut String,
) -> Result<(), ExpandError> {
    for token in tokens {
        match token {
            BodyToken::Literal(s) => out.push_str(s),
            BodyToken::Substitution(name) => {
                let binding = bindings
                    .get(name)
                    .ok_or_else(|| ExpandError::UnboundName(name.clone()))?;
                match binding {
                    Binding::Single(frag) => out.push_str(&frag.source),
                    Binding::Sequence(_) => {
                        return Err(ExpandError::WrongBindingShape(name.clone()));
                    }
                }
            }
            BodyToken::Repetition {
                body,
                separator,
                kind: _,
            } => {
                expand_repetition(body, separator.as_deref(), bindings, expansion_id, out)?;
            }
        }
    }
    Ok(())
}

fn expand_repetition(
    inner: &[BodyToken],
    separator: Option<&str>,
    outer_bindings: &HashMap<String, Binding>,
    expansion_id: u32,
    out: &mut String,
) -> Result<(), ExpandError> {
    // Find sequence bindings referenced anywhere in the inner body.
    let names = collect_substitutions(inner);
    let mut length: Option<usize> = None;
    let mut sequence_names: Vec<&String> = Vec::new();
    for name in &names {
        if let Some(Binding::Sequence(frags)) = outer_bindings.get(*name) {
            match length {
                None => length = Some(frags.len()),
                Some(prev) if prev != frags.len() => {
                    return Err(ExpandError::InconsistentSequenceLength(prev, frags.len()));
                }
                _ => {}
            }
            sequence_names.push(*name);
        }
    }
    let Some(length) = length else {
        return Err(ExpandError::UnanchoredRepetition);
    };

    for i in 0..length {
        if i > 0
            && let Some(sep) = separator
        {
            out.push_str(sep);
        }
        // Build an inner-scope binding map: sequence bindings become Single
        // for the i-th element; other bindings pass through unchanged.
        let mut scope: HashMap<String, Binding> = HashMap::new();
        for (name, binding) in outer_bindings {
            match binding {
                Binding::Single(_) => {
                    scope.insert(name.clone(), binding.clone());
                }
                Binding::Sequence(frags) => {
                    if sequence_names.contains(&name) {
                        scope.insert(name.clone(), Binding::Single(frags[i].clone()));
                    }
                    // Sequence bindings not referenced in this repetition
                    // stay out of scope — they belong to outer repetitions.
                }
            }
        }
        render_tokens(inner, &scope, expansion_id, out)?;
    }
    Ok(())
}

fn collect_substitutions(tokens: &[BodyToken]) -> Vec<&String> {
    let mut names = Vec::new();
    for token in tokens {
        match token {
            BodyToken::Substitution(name) => names.push(name),
            BodyToken::Repetition { body, .. } => {
                names.extend(collect_substitutions(body));
            }
            BodyToken::Literal(_) => {}
        }
    }
    names
}

/// Rewrite identifiers starting with `__` in the expanded text so they
/// get a unique per-expansion suffix.
///
/// This is a simple, approximate form of hygiene: it catches the common
/// pattern of macro-internal temporaries (e.g., `const __v = ...; __v.push`)
/// while leaving everything else alone. It can be fooled by `__`-prefixed
/// identifiers that appear inside string literals or comments — that's an
/// accepted MVP limitation and is called out in the execution plan.
fn rewrite_hygiene(source: String, expansion_id: u32) -> String {
    // First pass: collect all `__`-prefixed identifiers that appear as
    // bare tokens. Then rewrite each occurrence, appending `$<id>`.
    let mut renamed: HashSet<&str> = HashSet::new();
    let bytes = source.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if is_ident_start(c) {
            let start = i;
            while i < bytes.len() && is_ident_continue(bytes[i]) {
                i += 1;
            }
            let ident = &source[start..i];
            if ident.starts_with("__") && !ident.contains('$') {
                renamed.insert(ident);
            }
        } else {
            i += 1;
        }
    }
    if renamed.is_empty() {
        return source;
    }

    // Second pass: build a new string, replacing each occurrence.
    let suffix = format!("${}", expansion_id);
    let mut out = String::with_capacity(source.len() + renamed.len() * suffix.len());
    let bytes = source.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if is_ident_start(c) {
            let start = i;
            while i < bytes.len() && is_ident_continue(bytes[i]) {
                i += 1;
            }
            let ident = &source[start..i];
            if renamed.contains(ident) {
                out.push_str(ident);
                out.push_str(&suffix);
            } else {
                out.push_str(ident);
            }
        } else {
            out.push(c as char);
            i += 1;
        }
    }
    out
}

fn maybe_wrap_iife(source: String, context: ExpansionContext) -> String {
    match context {
        ExpansionContext::Statement => source,
        ExpansionContext::Expression => {
            let trimmed = source.trim();
            if trimmed.starts_with('{') && trimmed.ends_with('}') {
                format!("(() => {})()", trimmed)
            } else {
                source
            }
        }
    }
}

fn is_ident_start(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_' || c == b'$'
}

fn is_ident_continue(c: u8) -> bool {
    is_ident_start(c) || c.is_ascii_digit()
}
