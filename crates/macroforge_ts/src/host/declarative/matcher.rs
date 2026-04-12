//! Match a parsed macro arm against a call's argument list.
//!
//! Given a [`MacroDef`] and the OXC `Argument`s from a `CallExpression`,
//! walk the arms in source order and return the first one whose pattern
//! is satisfied by the arguments. Binds fragments to the verbatim source
//! slices of the matched arguments.
//!
//! Phase 13 added a sibling path: [`match_type_invocation_against_arms`]
//! takes a slice of `TSType` nodes (from a `TSTypeReference`'s type
//! parameters) and runs the same pattern-matching loop, but only accepts
//! the `Type` fragment kind. The bindings it produces are shape-identical
//! to the value-position path, so the expander doesn't need to know
//! which walker invoked it.

use std::collections::HashMap;

use oxc::allocator::Vec as OxcVec;
use oxc::ast::ast::{Argument, Expression, TSType};
use oxc::span::GetSpan;

use crate::ts_syn::abi::SpanIR;
use crate::ts_syn::declarative::{FragmentKind, MacroDef, Pattern, PatternElement, RepetitionKind};

/// Result of a successful arm match.
#[derive(Debug, Clone)]
pub struct MatchResult {
    /// Which arm (by index) matched.
    pub arm_index: usize,
    /// Bindings produced by the match.
    pub bindings: HashMap<String, Binding>,
}

/// A binding from a pattern variable to one or more captured fragments.
#[derive(Debug, Clone)]
pub enum Binding {
    Single(BoundFragment),
    Sequence(Vec<BoundFragment>),
}

/// A captured call argument with its source slice and position.
#[derive(Debug, Clone)]
pub struct BoundFragment {
    pub kind: FragmentKind,
    /// Verbatim source text of the captured argument.
    pub source: String,
    pub span: SpanIR,
}

#[derive(Debug, Clone)]
pub enum MatchError {
    /// No arm matched the argument list.
    NoArmMatched {
        /// Short human-readable summary of each tried pattern.
        tried: Vec<String>,
    },
    /// An unsupported fragment kind was used in call-argument position.
    ///
    /// The `reason` field carries a concrete explanation of why the
    /// kind is impossible in call-argument position — e.g. "`Stmt`
    /// is a declaration or control-flow node; JavaScript doesn't
    /// allow statements as call arguments". PR 9 added this so
    /// users see a grammatical explanation rather than a bare
    /// "not supported" message.
    UnsupportedFragmentKind {
        kind: FragmentKind,
        reason: &'static str,
    },
    /// Two sequence bindings in the same body would need different lengths.
    InconsistentSequenceLength,
}

impl std::fmt::Display for MatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchError::NoArmMatched { tried } => {
                write!(f, "no arm matched; tried {} pattern(s)", tried.len())?;
                if !tried.is_empty() {
                    write!(f, ": {}", tried.join(" | "))?;
                }
                // PR 9 note: after PR 9 the matcher does bounded
                // backtracking, so patterns like `$( $x:Expr ),* $last:Expr`
                // CAN match. If we still end up here with a
                // repetition-containing arm, the match genuinely
                // failed for other reasons. We keep the hint so
                // users understand repetition+tail patterns are
                // expected to work — if they don't, it's likely a
                // legitimate mismatch on the tail fragment kinds.
                if tried.iter().any(|p| p.contains("$(...)")) {
                    write!(
                        f,
                        " (note: repetitions `$(...)*` / `$(...)+` / `$(...)?` are matched with bounded backtracking — the matcher tries decreasing counts until the tail elements fit; if you expected a match here, double-check that the fragment kinds after the repetition are compatible with the remaining call arguments)"
                    )?;
                }
                Ok(())
            }
            MatchError::UnsupportedFragmentKind { kind, reason } => {
                write!(
                    f,
                    "fragment kind `{:?}` cannot appear in call-argument position: {}",
                    kind, reason
                )
            }
            MatchError::InconsistentSequenceLength => {
                write!(
                    f,
                    "repeated metavariables bound inside the same repetition have mismatched lengths"
                )
            }
        }
    }
}

impl std::error::Error for MatchError {}

/// Match the call's arguments against the macro's arms and return the first
/// successful match (or [`MatchError::NoArmMatched`] if none fit).
///
/// Thin wrapper around [`match_invocation_against_arms`] that uses
/// `def.arms` — the dev-form / expand-mode arms.
pub fn match_invocation<'a>(
    def: &MacroDef,
    call_args: &'a OxcVec<'a, Argument<'a>>,
    source: &str,
) -> Result<MatchResult, MatchError> {
    match_invocation_against_arms(&def.arms, call_args, source).map(|(arm_index, bindings)| {
        MatchResult {
            arm_index,
            bindings,
        }
    })
}

/// Match the call's arguments against an arbitrary slice of arms, returning
/// the matched arm index and its bindings.
///
/// Used by the rewriter to dispatch between `def.arms` (expand mode) and
/// `def.call_arms` (share mode) without duplicating the matching loop.
pub fn match_invocation_against_arms<'a>(
    arms: &[crate::ts_syn::declarative::MacroArm],
    call_args: &'a OxcVec<'a, Argument<'a>>,
    source: &str,
) -> Result<(usize, HashMap<String, Binding>), MatchError> {
    let mut tried = Vec::with_capacity(arms.len());
    for (arm_index, arm) in arms.iter().enumerate() {
        let mut bindings: HashMap<String, Binding> = HashMap::new();
        let mut cursor = 0usize;
        let matched = match_pattern(&arm.pattern, call_args, &mut cursor, &mut bindings, source)?;
        if matched && cursor == call_args.len() {
            return Ok((arm_index, bindings));
        }
        tried.push(describe_pattern(&arm.pattern));
    }
    Err(MatchError::NoArmMatched { tried })
}

fn match_pattern<'a>(
    pattern: &Pattern,
    args: &'a OxcVec<'a, Argument<'a>>,
    cursor: &mut usize,
    bindings: &mut HashMap<String, Binding>,
    source: &str,
) -> Result<bool, MatchError> {
    match pattern {
        Pattern::Empty => Ok(*cursor == 0 && args.is_empty()),
        Pattern::Sequence(elements) => match_elements(elements, args, cursor, bindings, source),
    }
}

fn match_elements<'a>(
    elements: &[PatternElement],
    args: &'a OxcVec<'a, Argument<'a>>,
    cursor: &mut usize,
    bindings: &mut HashMap<String, Binding>,
    source: &str,
) -> Result<bool, MatchError> {
    for (idx, elem) in elements.iter().enumerate() {
        match elem {
            PatternElement::Literal(lit) => {
                // Literal separators like `,` are implicitly consumed between
                // other pattern elements (OXC already split the arg list on
                // commas). Treat them as no-ops.
                let _ = lit;
            }
            PatternElement::Fragment { name, kind } => {
                if *cursor >= args.len() {
                    return Ok(false);
                }
                let Some(fragment) = bind_fragment(&args[*cursor], *kind, source)? else {
                    return Ok(false);
                };
                bindings.insert(name.clone(), Binding::Single(fragment));
                *cursor += 1;
            }
            PatternElement::Repetition {
                pattern,
                separator: _,
                kind,
            } => {
                // PR 9: bounded backtracking. We delegate to a helper
                // that (a) runs the greedy pass capturing a snapshot
                // after each iteration, then (b) tries to match the
                // tail elements (everything after this Repetition)
                // against each snapshot from greediest down to the
                // minimum required by `kind`, returning on the first
                // count that leads to a complete match.
                //
                // Returning from here unconditionally is correct
                // because [`match_elements_with_repetition_backtrack`]
                // handles the tail itself — control does not come
                // back to this loop.
                let tail = &elements[idx + 1..];
                return match_elements_with_repetition_backtrack(
                    pattern, *kind, tail, args, cursor, bindings, source,
                );
            }
        }
    }
    Ok(true)
}

/// Handle a [`PatternElement::Repetition`] with bounded backtracking.
///
/// Called from [`match_elements`] when a Repetition is encountered.
/// The caller passes the repetition's inner pattern and kind, plus
/// the `tail` of elements that come AFTER the repetition in the
/// outer pattern. This function:
///
/// 1. Runs the greedy pass — keep matching `inner` against `args`
///    until it fails or stops advancing, remembering a snapshot
///    `(cursor, collected_bindings)` after each successful iteration.
/// 2. Iterates candidate counts from the greedy maximum down to the
///    minimum allowed by `kind` (0 for `*`/`?`, 1 for `+`). For each
///    candidate, it restores the snapshot's cursor position,
///    splices the collected bindings into `outer_bindings` as
///    `Binding::Sequence`s, and recursively calls [`match_elements`]
///    on `tail`. If the tail match succeeds AND the cursor ends up
///    at `args.len()` (full consumption), the function returns
///    `Ok(true)` with all bindings committed.
/// 3. If no candidate count produces a complete match, the function
///    returns `Ok(false)` and leaves `cursor` / `bindings` in a
///    neutral state (the caller's next action is to return
///    `Ok(false)` itself or advance to the next arm).
///
/// Cost. Snapshot storage is `O(args.len() × keys_in_inner_pattern)`
/// and the backtracking loop is `O(args.len())` retries, each
/// running `match_elements` on the tail. For realistic macros the
/// tail is a small constant-sized pattern, so the total work per
/// arm is still linear in the number of call arguments.
#[allow(clippy::too_many_arguments)]
fn match_elements_with_repetition_backtrack<'a>(
    inner: &Pattern,
    kind: RepetitionKind,
    tail: &[PatternElement],
    args: &'a OxcVec<'a, Argument<'a>>,
    cursor: &mut usize,
    outer_bindings: &mut HashMap<String, Binding>,
    source: &str,
) -> Result<bool, MatchError> {
    // --- Phase 1: greedy pass collecting per-count snapshots. ---
    //
    // `snapshots[k]` holds the state AFTER `k` successful iterations
    // of the inner pattern. `snapshots[0]` is the pre-repetition
    // state (empty bindings, cursor where we started). Each entry
    // pairs the cursor position with the set of collected inner
    // bindings accumulated so far.
    let mut snapshots: Vec<(usize, HashMap<String, Vec<BoundFragment>>)> = Vec::new();
    snapshots.push((*cursor, HashMap::new()));

    let mut collected: HashMap<String, Vec<BoundFragment>> = HashMap::new();
    let mut count = 0usize;
    loop {
        let saved_cursor = *cursor;
        let mut temp_bindings: HashMap<String, Binding> = HashMap::new();
        let inner_match = match_pattern(inner, args, cursor, &mut temp_bindings, source)?;
        if !inner_match {
            *cursor = saved_cursor;
            break;
        }
        // Fold the inner bindings into the running accumulator.
        for (name, binding) in temp_bindings {
            let bucket = collected.entry(name).or_default();
            match binding {
                Binding::Single(frag) => bucket.push(frag),
                Binding::Sequence(frags) => bucket.extend(frags),
            }
        }
        count += 1;
        // Record snapshot AFTER this successful iteration.
        snapshots.push((*cursor, collected.clone()));

        // `?` has max count 1.
        if kind == RepetitionKind::ZeroOrOne && count >= 1 {
            break;
        }
        // Guard against zero-width inner patterns that would loop
        // forever.
        if *cursor == saved_cursor {
            break;
        }
    }

    // --- Phase 2: try each candidate count from greediest to min. ---
    //
    // The minimum allowed count is determined by `kind`. We also
    // cap at `count` (the greedy maximum) because nothing past that
    // can match anyway.
    let min_count: usize = match kind {
        RepetitionKind::ZeroOrMore | RepetitionKind::ZeroOrOne => 0,
        RepetitionKind::OneOrMore => 1,
    };

    // Remember which keys this Repetition contributes so we can
    // clean them up between tail attempts.
    let rep_keys: Vec<String> = collected.keys().cloned().collect();

    for try_count in (min_count..=count).rev() {
        // ZeroOrOne caps at 1 — don't try counts above it.
        if kind == RepetitionKind::ZeroOrOne && try_count > 1 {
            continue;
        }

        let (snap_cursor, snap_bindings) = &snapshots[try_count];
        *cursor = *snap_cursor;

        // Splice this count's collected bindings into outer_bindings.
        for (name, frags) in snap_bindings {
            outer_bindings.insert(name.clone(), Binding::Sequence(frags.clone()));
        }

        // For repetitions that referenced metavariables not bound
        // on this iteration (can happen when the candidate count is
        // 0 and the metavariable was in the inner pattern), make
        // sure every `rep_keys` entry has AT LEAST a (possibly
        // empty) Sequence binding so body expansion doesn't hit
        // UnboundName.
        for key in &rep_keys {
            outer_bindings
                .entry(key.clone())
                .or_insert_with(|| Binding::Sequence(Vec::new()));
        }

        // Snapshot the outer bindings BEFORE tail matching so we
        // can roll back if the tail fails. This is necessary
        // because `match_elements` mutates `bindings` in-place
        // (e.g. inserts `Fragment` matches) as it goes.
        let bindings_before_tail = outer_bindings.clone();
        let cursor_before_tail = *cursor;

        let tail_matched = match_elements(tail, args, cursor, outer_bindings, source)?;
        if tail_matched && *cursor == args.len() {
            return Ok(true);
        }

        // Tail failed at this count. Roll back and try a smaller
        // count. We also clear out any rep_keys that the snapshot
        // contributed, since a smaller count will rewrite them.
        *outer_bindings = bindings_before_tail;
        *cursor = cursor_before_tail;
        for key in &rep_keys {
            outer_bindings.remove(key);
        }
    }

    // No count produced a full match.
    Ok(false)
}

fn bind_fragment(
    arg: &Argument<'_>,
    kind: FragmentKind,
    source: &str,
) -> Result<Option<BoundFragment>, MatchError> {
    // Spread arguments (`...x`) aren't supported in MVP macro invocations.
    let expr = match arg.as_expression() {
        Some(expr) => expr,
        None => return Ok(None),
    };

    let shape_ok = match kind {
        FragmentKind::Expr | FragmentKind::Tt => true,
        FragmentKind::Ident => matches!(expr, Expression::Identifier(_)),
        FragmentKind::Lit => matches!(
            expr,
            Expression::StringLiteral(_)
                | Expression::NumericLiteral(_)
                | Expression::BooleanLiteral(_)
                | Expression::NullLiteral(_)
                | Expression::BigIntLiteral(_)
                | Expression::RegExpLiteral(_)
                | Expression::TemplateLiteral(_)
        ),
        FragmentKind::Path => matches!(
            expr,
            Expression::Identifier(_) | Expression::StaticMemberExpression(_)
        ),
        FragmentKind::Block => matches!(expr, Expression::ArrowFunctionExpression(_)),
        // The kinds below are semantically impossible in
        // call-argument position — JavaScript's grammar only allows
        // expressions as call arguments, and the rejection reasons
        // below explain the mismatch so users don't mistake it for
        // "not implemented yet".
        FragmentKind::Stmt => {
            return Err(MatchError::UnsupportedFragmentKind {
                kind,
                reason: "`Stmt` matches control-flow or declaration statements; JavaScript's grammar only allows expressions as call arguments. Wrap the statement in an arrow function if you need to pass it in: `$macro(() => { ...stmts })`.",
            });
        }
        FragmentKind::Type => {
            return Err(MatchError::UnsupportedFragmentKind {
                kind,
                reason: "`Type` matches TypeScript type annotations, which only appear in type position (type aliases, parameter annotations, return types). For a value-position macro, use `Expr`; for a type-position macro, declare the macro with `kind: \"type\"`.",
            });
        }
        FragmentKind::Pat => {
            return Err(MatchError::UnsupportedFragmentKind {
                kind,
                reason: "`Pat` matches destructuring patterns, which can only appear in binding position (function parameters, `let`/`const` LHS). If you need to pass a pattern-like shape into a macro, capture the whole object expression with `Expr` instead.",
            });
        }
        FragmentKind::Item => {
            return Err(MatchError::UnsupportedFragmentKind {
                kind,
                reason: "`Item` matches top-level declarations (class, function, interface, enum). Declarations cannot appear in call-argument position in JavaScript. Use `Expr` to capture a function expression or class expression instead.",
            });
        }
        FragmentKind::Decorator => {
            return Err(MatchError::UnsupportedFragmentKind {
                kind,
                reason: "`Decorator` matches `@decorator(...)` annotations, which can only attach to classes, methods, and fields — not appear as call arguments. If you need a decorator-shaped expression as an argument, capture the call itself with `Expr`.",
            });
        }
    };
    if !shape_ok {
        return Ok(None);
    }

    let span = expr.span();
    let start = span.start as usize;
    let end = span.end as usize;
    let source_text = source.get(start..end).unwrap_or("").to_string();
    Ok(Some(BoundFragment {
        kind,
        source: source_text,
        // Store in 1-based SpanIR convention so it aligns with patches.
        span: SpanIR::new(span.start + 1, span.end + 1),
    }))
}

// ---------------------------------------------------------------------------
// Phase 13 — type-position matcher
// ---------------------------------------------------------------------------
//
// Mirrors the value-position match logic but walks `TSType` nodes
// instead of `Argument`s. The only fragment kind accepted in
// type-position matching is `Type` (and `Tt` as a structural fallback);
// other kinds produce `UnsupportedFragmentKind`.

/// Match a type-position invocation (`$name<T1, T2, ...>`) against a
/// macro's expand-mode arms. Returns the matched arm index and its
/// bindings.
pub fn match_type_invocation_against_arms<'a>(
    arms: &[crate::ts_syn::declarative::MacroArm],
    type_args: &'a OxcVec<'a, TSType<'a>>,
    source: &str,
) -> Result<(usize, HashMap<String, Binding>), MatchError> {
    let mut tried = Vec::with_capacity(arms.len());
    for (arm_index, arm) in arms.iter().enumerate() {
        let mut bindings: HashMap<String, Binding> = HashMap::new();
        let mut cursor = 0usize;
        let matched =
            match_type_pattern(&arm.pattern, type_args, &mut cursor, &mut bindings, source)?;
        if matched && cursor == type_args.len() {
            return Ok((arm_index, bindings));
        }
        tried.push(describe_pattern(&arm.pattern));
    }
    Err(MatchError::NoArmMatched { tried })
}

fn match_type_pattern<'a>(
    pattern: &Pattern,
    args: &'a OxcVec<'a, TSType<'a>>,
    cursor: &mut usize,
    bindings: &mut HashMap<String, Binding>,
    source: &str,
) -> Result<bool, MatchError> {
    match pattern {
        Pattern::Empty => Ok(*cursor == 0 && args.is_empty()),
        Pattern::Sequence(elements) => {
            match_type_elements(elements, args, cursor, bindings, source)
        }
    }
}

fn match_type_elements<'a>(
    elements: &[PatternElement],
    args: &'a OxcVec<'a, TSType<'a>>,
    cursor: &mut usize,
    bindings: &mut HashMap<String, Binding>,
    source: &str,
) -> Result<bool, MatchError> {
    let mut i = 0usize;
    while i < elements.len() {
        let elem = &elements[i];
        match elem {
            PatternElement::Literal(_) => {
                // Literal separators (commas) are implicit between type
                // params — skip them just like the value-position path does.
            }
            PatternElement::Fragment { name, kind } => {
                if *cursor >= args.len() {
                    return Ok(false);
                }
                let Some(fragment) = bind_type_fragment(&args[*cursor], *kind, source)? else {
                    return Ok(false);
                };
                bindings.insert(name.clone(), Binding::Single(fragment));
                *cursor += 1;
            }
            PatternElement::Repetition {
                pattern,
                separator: _,
                kind,
            } => {
                let mut collected: HashMap<String, Vec<BoundFragment>> = HashMap::new();
                let mut count = 0usize;
                loop {
                    let saved_cursor = *cursor;
                    let mut temp_bindings: HashMap<String, Binding> = HashMap::new();
                    let inner_match =
                        match_type_pattern(pattern, args, cursor, &mut temp_bindings, source)?;
                    if !inner_match {
                        *cursor = saved_cursor;
                        break;
                    }
                    for (name, binding) in temp_bindings {
                        let bucket = collected.entry(name).or_default();
                        match binding {
                            Binding::Single(frag) => bucket.push(frag),
                            Binding::Sequence(frags) => bucket.extend(frags),
                        }
                    }
                    count += 1;

                    if *kind == RepetitionKind::ZeroOrOne && count >= 1 {
                        break;
                    }
                    if *cursor == saved_cursor {
                        break;
                    }
                }

                match kind {
                    RepetitionKind::ZeroOrMore => {}
                    RepetitionKind::OneOrMore => {
                        if count == 0 {
                            return Ok(false);
                        }
                    }
                    RepetitionKind::ZeroOrOne => {
                        if count > 1 {
                            return Ok(false);
                        }
                    }
                }

                for (name, frags) in collected {
                    bindings.insert(name, Binding::Sequence(frags));
                }
            }
        }
        i += 1;
    }
    Ok(true)
}

fn bind_type_fragment(
    ty: &TSType<'_>,
    kind: FragmentKind,
    source: &str,
) -> Result<Option<BoundFragment>, MatchError> {
    // Type-position matching only accepts `Type` and `Tt` (the
    // structural fallback) — Rust's `macro_rules!` in type position
    // works the same way: you only bind to type tokens, not to
    // expressions or identifiers.
    match kind {
        FragmentKind::Type | FragmentKind::Tt => {}
        _ => {
            return Err(MatchError::UnsupportedFragmentKind {
                kind,
                reason: "type-position macros only bind `Type` (or `Tt` as the structural fallback) metavariables. Expressions, identifiers, literals, patterns, and statements don't exist in TypeScript type position.",
            });
        }
    }

    let span = ty.span();
    let start = span.start as usize;
    let end = span.end as usize;
    let source_text = source.get(start..end).unwrap_or("").to_string();
    Ok(Some(BoundFragment {
        kind,
        source: source_text,
        span: SpanIR::new(span.start + 1, span.end + 1),
    }))
}

fn describe_pattern(pattern: &Pattern) -> String {
    match pattern {
        Pattern::Empty => "()".to_string(),
        Pattern::Sequence(elems) => {
            let mut out = String::from("(");
            for (i, elem) in elems.iter().enumerate() {
                if i > 0 {
                    out.push(' ');
                }
                match elem {
                    PatternElement::Literal(s) => out.push_str(s),
                    PatternElement::Fragment { name, kind } => {
                        out.push('$');
                        out.push_str(name);
                        out.push(':');
                        out.push_str(match kind {
                            FragmentKind::Expr => "Expr",
                            FragmentKind::Stmt => "Stmt",
                            FragmentKind::Block => "Block",
                            FragmentKind::Ident => "Ident",
                            FragmentKind::Type => "Type",
                            FragmentKind::Pat => "Pat",
                            FragmentKind::Lit => "Lit",
                            FragmentKind::Path => "Path",
                            FragmentKind::Item => "Item",
                            FragmentKind::Decorator => "Decorator",
                            FragmentKind::Tt => "Tt",
                        });
                    }
                    PatternElement::Repetition { kind, .. } => {
                        out.push_str("$(...)");
                        out.push(match kind {
                            RepetitionKind::ZeroOrMore => '*',
                            RepetitionKind::OneOrMore => '+',
                            RepetitionKind::ZeroOrOne => '?',
                        });
                    }
                }
            }
            out.push(')');
            out
        }
    }
}
