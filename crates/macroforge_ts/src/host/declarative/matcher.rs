//! Match a parsed macro arm against a call's argument list.
//!
//! Given a [`MacroDef`] and the OXC `Argument`s from a `CallExpression`,
//! walk the arms in source order and return the first one whose pattern
//! is satisfied by the arguments. Binds fragments to the verbatim source
//! slices of the matched arguments.

use std::collections::HashMap;

use oxc::allocator::Vec as OxcVec;
use oxc::ast::ast::{Argument, Expression};
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
    UnsupportedFragmentKind(FragmentKind),
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
                Ok(())
            }
            MatchError::UnsupportedFragmentKind(k) => {
                write!(
                    f,
                    "fragment kind `{:?}` is not supported in call-argument position",
                    k
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
pub fn match_invocation<'a>(
    def: &MacroDef,
    call_args: &'a OxcVec<'a, Argument<'a>>,
    source: &str,
) -> Result<MatchResult, MatchError> {
    let mut tried = Vec::with_capacity(def.arms.len());
    for (arm_index, arm) in def.arms.iter().enumerate() {
        let mut bindings: HashMap<String, Binding> = HashMap::new();
        let mut cursor = 0usize;
        let matched = match_pattern(&arm.pattern, call_args, &mut cursor, &mut bindings, source)?;
        if matched && cursor == call_args.len() {
            return Ok(MatchResult {
                arm_index,
                bindings,
            });
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
    let mut i = 0usize;
    while i < elements.len() {
        let elem = &elements[i];
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
                // Greedily match as many inner patterns as possible.
                let mut collected: HashMap<String, Vec<BoundFragment>> = HashMap::new();
                let mut count = 0usize;
                loop {
                    let saved_cursor = *cursor;
                    let mut temp_bindings: HashMap<String, Binding> = HashMap::new();
                    let inner_match =
                        match_pattern(pattern, args, cursor, &mut temp_bindings, source)?;
                    if !inner_match {
                        *cursor = saved_cursor;
                        break;
                    }
                    // Fold the inner bindings into accumulators.
                    for (name, binding) in temp_bindings {
                        let bucket = collected.entry(name).or_default();
                        match binding {
                            Binding::Single(frag) => bucket.push(frag),
                            Binding::Sequence(frags) => bucket.extend(frags),
                        }
                    }
                    count += 1;

                    // Enforce repetition upper bound for `?`.
                    if *kind == RepetitionKind::ZeroOrOne && count >= 1 {
                        break;
                    }

                    // If we didn't advance, abort to avoid an infinite loop.
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

                // Record the collected sequences as Sequence bindings.
                for (name, frags) in collected {
                    bindings.insert(name, Binding::Sequence(frags));
                }
            }
        }
        i += 1;
    }
    Ok(true)
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
        // Not supported in call-arg position in MVP.
        FragmentKind::Stmt
        | FragmentKind::Type
        | FragmentKind::Pat
        | FragmentKind::Item
        | FragmentKind::Decorator => {
            return Err(MatchError::UnsupportedFragmentKind(kind));
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
