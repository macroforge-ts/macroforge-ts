//! Expand a matched arm's body into source text, with hygienic renaming.
//!
//! The expander walks a [`Body`] token-by-token, substituting bound
//! fragments and unrolling repetitions. Identifiers inside the body that
//! start with `__` (double underscore) are treated as macro-introduced
//! and get a unique per-expansion suffix (`__v` → `__v$7`) so they
//! don't collide with call-site identifiers.

use std::collections::{HashMap, HashSet};

use crate::ts_syn::declarative::{Body, BodyToken};

use super::matcher::{Binding, match_invocation_against_arms};
use super::registry::DeclarativeMacroRegistry;

/// Whether the expansion slot is expression, statement, or type position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpansionContext {
    /// Expression position — a block body needs an IIFE wrap.
    Expression,
    /// Statement position — block bodies are spliced in directly.
    Statement,
    /// Type position — used by the Phase 13 type-position walker. The
    /// body is spliced as a type, so no IIFE wrapping and no JS-level
    /// block handling apply.
    Type,
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
    /// Expansion recursed past the depth limit. Fires when a macro
    /// indirectly calls itself or when composition nests too deeply.
    /// The `u32` is the limit that was exceeded.
    RecursionLimit(u32),
    /// A `$name(...)` macro call referenced a macro that isn't in the
    /// registry. Either a typo or a name the user hasn't defined yet
    /// (registry hasn't been populated).
    UnknownMacroCall(String),
    /// A `$name(...)` macro call's argument list failed to re-parse as
    /// OXC source. Usually means the caller's body expansion produced
    /// invalid JS.
    MalformedMacroCallArgs { callee: String, reason: String },
    /// A nested macro call didn't match any arm of the callee.
    NestedMatchFailure { callee: String, tried: Vec<String> },
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
            ExpandError::RecursionLimit(limit) => write!(
                f,
                "macro expansion exceeded the recursion limit of {} levels — did a macro call itself?",
                limit
            ),
            ExpandError::UnknownMacroCall(name) => write!(
                f,
                "macro body calls unknown macro `${}` — not registered or out of scope",
                name
            ),
            ExpandError::MalformedMacroCallArgs { callee, reason } => write!(
                f,
                "macro body calls `${}` but its argument list failed to parse: {}",
                callee, reason
            ),
            ExpandError::NestedMatchFailure { callee, tried } => write!(
                f,
                "nested call to `${}` did not match any arm (tried: {})",
                callee,
                tried.join(" | ")
            ),
        }
    }
}

impl std::error::Error for ExpandError {}

/// Maximum macro expansion depth. Deeper recursion than this is
/// treated as a runaway expansion and returns [`ExpandError::RecursionLimit`]
/// instead of blowing the stack. 256 is generous enough that no
/// realistic hand-written macro composition will hit it.
pub const MAX_EXPANSION_DEPTH: u32 = 256;

/// Expand `body` into a source string, given captured fragment bindings.
///
/// `depth` is the current recursion depth; top-level callers pass `0`.
/// Phase 12's inter-macro composition bumps it on each nested expansion.
/// Exceeding [`MAX_EXPANSION_DEPTH`] returns [`ExpandError::RecursionLimit`]
/// instead of growing the stack.
///
/// When the body contains no `BodyToken::MacroCall` tokens, the
/// `registry` argument is unused — you can pass `None` to skip the
/// inter-macro composition path. Phase 12 call sites that want
/// composition supply the registry.
pub fn expand_body(
    body: &Body,
    bindings: &HashMap<String, Binding>,
    expansion_id: u32,
    context: ExpansionContext,
    depth: u32,
) -> Result<String, ExpandError> {
    expand_body_with_registry(body, bindings, expansion_id, context, depth, None)
}

/// Same as [`expand_body`] but takes an optional registry reference
/// used to resolve `BodyToken::MacroCall` tokens to other macros.
pub fn expand_body_with_registry(
    body: &Body,
    bindings: &HashMap<String, Binding>,
    expansion_id: u32,
    context: ExpansionContext,
    depth: u32,
    registry: Option<&DeclarativeMacroRegistry>,
) -> Result<String, ExpandError> {
    if depth > MAX_EXPANSION_DEPTH {
        return Err(ExpandError::RecursionLimit(MAX_EXPANSION_DEPTH));
    }
    let mut out = String::new();
    render_tokens(&body.0, bindings, expansion_id, depth, registry, &mut out)?;
    let rewritten = rewrite_hygiene(out, expansion_id);
    Ok(maybe_wrap_iife(rewritten, context))
}

fn render_tokens(
    tokens: &[BodyToken],
    bindings: &HashMap<String, Binding>,
    expansion_id: u32,
    depth: u32,
    registry: Option<&DeclarativeMacroRegistry>,
    out: &mut String,
) -> Result<(), ExpandError> {
    if depth > MAX_EXPANSION_DEPTH {
        return Err(ExpandError::RecursionLimit(MAX_EXPANSION_DEPTH));
    }
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
            BodyToken::MacroCall {
                name: callee_name,
                args,
            } => {
                expand_macro_call(
                    callee_name,
                    args,
                    bindings,
                    expansion_id,
                    depth,
                    registry,
                    out,
                )?;
            }
            BodyToken::Repetition {
                body,
                separator,
                kind: _,
            } => {
                expand_repetition(
                    body,
                    separator.as_deref(),
                    bindings,
                    expansion_id,
                    depth,
                    registry,
                    out,
                )?;
            }
        }
    }
    Ok(())
}

/// Dispatch a `$callee(...)` macro call inside a macro body.
///
/// 1. Render the `args` tokens as a source string (substituting any
///    bindings from the outer scope).
/// 2. Parse the rendered text as an OXC call expression.
/// 3. Match the parsed arguments against the callee macro's arms.
/// 4. Recursively expand the matched arm's body with `depth + 1`.
fn expand_macro_call(
    callee_name: &str,
    args: &[BodyToken],
    bindings: &HashMap<String, Binding>,
    expansion_id: u32,
    depth: u32,
    registry: Option<&DeclarativeMacroRegistry>,
    out: &mut String,
) -> Result<(), ExpandError> {
    // Resolve the callee. If no registry was provided (the
    // `expand_body` overload without a registry), we can't dispatch —
    // emit a clear error rather than blindly pasting literals.
    let Some(registry) = registry else {
        return Err(ExpandError::UnknownMacroCall(callee_name.to_string()));
    };
    let Some(callee_def) = registry.lookup(callee_name).cloned() else {
        return Err(ExpandError::UnknownMacroCall(callee_name.to_string()));
    };

    // Render args as a source string. We recurse through render_tokens
    // so nested MacroCalls expand too (composition depth bumps each
    // level).
    let mut rendered_args = String::new();
    render_tokens(
        args,
        bindings,
        expansion_id,
        depth,
        Some(registry),
        &mut rendered_args,
    )?;

    // Wrap in a dummy call so OXC can give us a real `CallExpression`
    // with a proper `Argument` list.
    let wrapper_source = format!("__m4cr0f0rg3_dummy__({});", rendered_args.trim());

    use oxc::allocator::Allocator;
    use oxc::ast::ast::{Expression, Statement};
    use oxc::parser::Parser;
    use oxc::span::SourceType;

    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, &wrapper_source, SourceType::ts()).parse();
    if !parsed.errors.is_empty() {
        return Err(ExpandError::MalformedMacroCallArgs {
            callee: callee_name.to_string(),
            reason: parsed
                .errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("; "),
        });
    }

    // Find the call expression.
    let call = parsed.program.body.iter().find_map(|stmt| {
        if let Statement::ExpressionStatement(es) = stmt
            && let Expression::CallExpression(call) = &es.expression
        {
            Some(call)
        } else {
            None
        }
    });
    let Some(call) = call else {
        return Err(ExpandError::MalformedMacroCallArgs {
            callee: callee_name.to_string(),
            reason: "wrapper did not produce a call expression".to_string(),
        });
    };

    // Match call args against the callee's arms.
    let (arm_index, callee_bindings) = match_invocation_against_arms(
        &callee_def.arms,
        &call.arguments,
        &wrapper_source,
    )
    .map_err(|e| match e {
        super::matcher::MatchError::NoArmMatched { tried } => ExpandError::NestedMatchFailure {
            callee: callee_name.to_string(),
            tried,
        },
        other => ExpandError::MalformedMacroCallArgs {
            callee: callee_name.to_string(),
            reason: other.to_string(),
        },
    })?;

    // Recursively expand the callee's body with its own bindings and
    // depth + 1. Statement context here — the caller is already
    // splicing text into a larger surrounding context, and wrapping in
    // an IIFE per composition step would clutter the output.
    //
    let nested = expand_body_with_registry(
        &callee_def.arms[arm_index].body,
        &callee_bindings,
        // Use a fresh expansion id for the nested scope so hygiene
        // renames don't collide with the caller's.
        expansion_id.wrapping_add(depth + 1),
        ExpansionContext::Statement,
        depth + 1,
        Some(registry),
    )?;
    out.push_str(&nested);
    Ok(())
}

fn expand_repetition(
    inner: &[BodyToken],
    separator: Option<&str>,
    outer_bindings: &HashMap<String, Binding>,
    expansion_id: u32,
    depth: u32,
    registry: Option<&DeclarativeMacroRegistry>,
    out: &mut String,
) -> Result<(), ExpandError> {
    if depth > MAX_EXPANSION_DEPTH {
        return Err(ExpandError::RecursionLimit(MAX_EXPANSION_DEPTH));
    }
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
        // Repetitions are "horizontal" — they don't add a level of
        // recursion conceptually — so we pass depth through unchanged.
        // Only macro-to-macro composition (Phase 12) bumps depth.
        render_tokens(inner, &scope, expansion_id, depth, registry, out)?;
    }
    Ok(())
}

fn collect_substitutions(tokens: &[BodyToken]) -> Vec<&String> {
    let mut names = Vec::new();
    for token in tokens {
        match token {
            BodyToken::Substitution(name) => names.push(name),
            BodyToken::MacroCall { args, .. } => {
                // Repetition length is determined by substitutions in
                // the arg list — e.g. `$($double($x)),+` should iterate
                // over `$x`, not over `$double` (which is a callee name,
                // not a binding).
                names.extend(collect_substitutions(args));
            }
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
/// Only identifiers that are *declared* within the macro body get
/// renamed — i.e., names on the left of `const __x`, `let __x`,
/// or `var __x`. Pure references to externally-declared `__`-prefixed
/// names (such as a shared runtime helper emitted by a share-mode
/// macro) are left untouched, because renaming them would break the
/// link between the call site and the helper.
///
/// This is a simple, approximate form of hygiene: it catches the common
/// pattern of macro-internal temporaries (e.g., `const __v = ...; __v.push`)
/// while leaving everything else alone. It can be fooled by `__`-prefixed
/// identifiers that appear inside string literals or comments — that's
/// an accepted MVP limitation and is called out in the execution plan.
fn rewrite_hygiene(source: String, expansion_id: u32) -> String {
    // First pass: collect only `__`-prefixed identifiers that are the
    // BINDING NAME of a `const` / `let` / `var` declaration. Scan for
    // the keywords and look at the identifier immediately following.
    let declared = collect_declared_underscore_names(&source);
    if declared.is_empty() {
        return source;
    }

    // Second pass: build a new string, replacing only occurrences of
    // the declared names with the suffixed form.
    let suffix = format!("${}", expansion_id);
    let mut out = String::with_capacity(source.len() + declared.len() * suffix.len());
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
            if declared.contains(ident) {
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

/// Scan `source` for `const __x`, `let __x`, and `var __x` declarations
/// and collect the `__`-prefixed binding names. Only exact whole-word
/// matches of the keywords count, so `const_something` doesn't trigger.
fn collect_declared_underscore_names(source: &str) -> HashSet<&str> {
    let mut out: HashSet<&str> = HashSet::new();
    let bytes = source.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        // Only look at keywords at token boundaries.
        let at_token_boundary = i == 0 || !is_ident_continue(bytes[i - 1]);
        if at_token_boundary {
            let tail = &bytes[i..];
            let consumed = if tail.starts_with(b"const ") || tail.starts_with(b"const\t") {
                Some(5)
            } else if tail.starts_with(b"let ")
                || tail.starts_with(b"let\t")
                || tail.starts_with(b"var ")
                || tail.starts_with(b"var\t")
            {
                Some(3)
            } else {
                None
            };
            if let Some(kw_len) = consumed {
                let mut j = i + kw_len;
                // Skip whitespace between keyword and identifier.
                while j < len && matches!(bytes[j], b' ' | b'\t') {
                    j += 1;
                }
                // Read the identifier if one follows.
                if j < len && is_ident_start(bytes[j]) {
                    let start = j;
                    while j < len && is_ident_continue(bytes[j]) {
                        j += 1;
                    }
                    let ident = &source[start..j];
                    if ident.starts_with("__") && !ident.contains('$') {
                        out.insert(ident);
                    }
                }
                i = j;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn maybe_wrap_iife(source: String, context: ExpansionContext) -> String {
    match context {
        ExpansionContext::Statement | ExpansionContext::Type => source,
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
