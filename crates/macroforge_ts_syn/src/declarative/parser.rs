//! Hand-written recursive-descent parser for declarative macro template bodies.
//!
//! The template body is its own mini-language, not TypeScript. It consists of:
//!
//! - Multiple **arms** separated by blank lines (or adjacent lines each starting
//!   with `(` at the outermost level).
//! - Each arm is `pattern "=>" body`.
//! - A pattern is `"(" elements? ")"`.
//! - An element is a literal, a fragment (`$ident:Kind`), or a repetition
//!   (`$( inner )<sep?><repkind>`).
//! - A body is a sequence of literal chunks, `$ident` substitutions, and
//!   `$( inner )<sep?><repkind>` repetitions.
//!
//! Spans attached to parsed nodes are **relative to the original source file**
//! — the caller passes the template body's origin span so we can map byte
//! offsets correctly.

use crate::abi::SpanIR;

use super::errors::DeclarativeError;
use super::types::{
    Body, BodyToken, FragmentKind, MacroArm, MacroDef, MacroMode, Pattern, PatternElement,
    RepetitionKind,
};

/// Metavariable name reserved by the cluster-aware runtime-name
/// template feature (Phase E of the production-hardening plan).
///
/// When the body parser sees `$__cluster__` it always emits a
/// [`BodyToken::Substitution`], even if the identifier is followed by
/// `(` — normally that combination is a macro-call token, but
/// `__cluster__` is special so users can write the natural form
/// `__helper_$__cluster__($args)` in a `call_arms` body without it
/// being mis-parsed as a call to a macro named `__cluster__`. The
/// double-underscore suffix matches the conventional "system reserved"
/// naming convention and is vanishingly unlikely to collide with a
/// user-defined metavariable name or a user-declared macro called
/// `$cluster` (which IS now allowed — the earlier total reservation
/// of the short `cluster` name was dropped in PR 12 of the
/// production-hardening plan).
const RESERVED_CLUSTER_NAME: &str = "__cluster__";

/// Parse a macro template body into a [`MacroDef`].
///
/// `template_body` is the static text between the `` macro` `` and the
/// closing backtick (without the backticks themselves). `template_span`
/// is that text's span in the original source file — byte offsets inside
/// `template_body` are remapped by adding `template_span.start`.
///
/// The returned [`MacroDef`] has `name` left empty; the host sets it during
/// discovery since the parser can't see the surrounding const binding.
pub fn parse_macro_def(
    template_body: &str,
    template_span: SpanIR,
) -> Result<MacroDef, DeclarativeError> {
    let arms = split_arms(template_body, template_span)?;
    let mut parsed_arms = Vec::with_capacity(arms.len());
    for arm in arms {
        parsed_arms.push(parse_arm(arm.text, arm.span)?);
    }

    if parsed_arms.is_empty() {
        return Err(DeclarativeError::new(
            template_span,
            "macro definition has no arms",
        ));
    }

    Ok(MacroDef::from_arms(
        String::new(),
        parsed_arms,
        MacroMode::ExpandOnly,
        template_span,
    ))
}

// ---------------------------------------------------------------------------
// Arm splitting
// ---------------------------------------------------------------------------

struct RawArm<'a> {
    text: &'a str,
    span: SpanIR,
}

/// Split a template body into raw arm slices.
///
/// An arm starts at the first non-whitespace character of a line whose first
/// non-whitespace character is `(`, provided we're at depth 0 (no enclosing
/// braces or parens from a previous arm's body). The first arm additionally
/// starts at the first `(` in the template body regardless of leading newlines.
fn split_arms<'a>(
    template_body: &'a str,
    template_span: SpanIR,
) -> Result<Vec<RawArm<'a>>, DeclarativeError> {
    let bytes = template_body.as_bytes();
    let len = bytes.len();
    let mut arms = Vec::new();

    // Find arm start positions: byte offsets inside template_body at which a
    // new arm's `(` lives.
    let mut starts: Vec<usize> = Vec::new();

    // Depth counters, tracking nesting inside arm bodies so that a `(` inside
    // a body doesn't get mistaken for the start of a new arm.
    let mut paren: i32 = 0;
    let mut brace: i32 = 0;
    let mut bracket: i32 = 0;

    // `at_line_start` is true when the next non-whitespace character is the
    // first one of a logical line.
    let mut at_line_start = true;
    let mut i = 0;
    while i < len {
        let c = bytes[i];
        match c {
            b'\n' => {
                at_line_start = true;
                i += 1;
                continue;
            }
            b' ' | b'\t' | b'\r' => {
                i += 1;
                continue;
            }
            b'(' => {
                if at_line_start && paren == 0 && brace == 0 && bracket == 0 {
                    starts.push(i);
                }
                paren += 1;
                at_line_start = false;
            }
            b')' => {
                paren -= 1;
                at_line_start = false;
            }
            b'{' => {
                brace += 1;
                at_line_start = false;
            }
            b'}' => {
                brace -= 1;
                at_line_start = false;
            }
            b'[' => {
                bracket += 1;
                at_line_start = false;
            }
            b']' => {
                bracket -= 1;
                at_line_start = false;
            }
            _ => {
                at_line_start = false;
            }
        }
        i += 1;
    }

    if paren != 0 || brace != 0 || bracket != 0 {
        return Err(DeclarativeError::new(
            template_span,
            "unbalanced delimiters in macro template",
        ));
    }

    if starts.is_empty() {
        return Err(DeclarativeError::new(
            template_span,
            "macro template contains no arms (expected at least one `(...)=> ...`)",
        ));
    }

    // Build slices from each start to the next start (or end of template).
    for (idx, &start) in starts.iter().enumerate() {
        let end = starts.get(idx + 1).copied().unwrap_or(len);
        let slice = &template_body[start..end];
        let trimmed = slice.trim_end();
        let trimmed_end = start + trimmed.len();
        arms.push(RawArm {
            text: trimmed,
            span: SpanIR::new(
                template_span.start + start as u32,
                template_span.start + trimmed_end as u32,
            ),
        });
    }

    Ok(arms)
}

// ---------------------------------------------------------------------------
// Arm parsing
// ---------------------------------------------------------------------------

fn parse_arm(text: &str, span: SpanIR) -> Result<MacroArm, DeclarativeError> {
    // The arm must start with `(`.
    let bytes = text.as_bytes();
    if bytes.is_empty() || bytes[0] != b'(' {
        return Err(DeclarativeError::new(span, "arm does not start with `(`"));
    }

    // Find the matching `)` that closes the top-level pattern parentheses.
    let pattern_end = match_balanced(bytes, 0)?;
    let pattern_src = &text[1..pattern_end];
    let pattern_span = SpanIR::new(span.start + 1, span.start + pattern_end as u32);

    // After the `)`, expect `=>`.
    let mut i = pattern_end + 1;
    i = skip_ws(bytes, i);
    if i + 1 >= bytes.len() || bytes[i] != b'=' || bytes[i + 1] != b'>' {
        return Err(DeclarativeError::new(
            SpanIR::new(span.start + i as u32, span.end),
            "expected `=>` after pattern",
        ));
    }
    i += 2;
    i = skip_ws(bytes, i);

    let body_src = &text[i..];
    let body_span = SpanIR::new(span.start + i as u32, span.end);

    let pattern = parse_pattern(pattern_src, pattern_span)?;
    let body = parse_body(body_src, body_span)?;

    Ok(MacroArm {
        pattern,
        body,
        span,
    })
}

/// Find the byte offset of the `)` that closes the `(` at `open_idx`.
fn match_balanced(bytes: &[u8], open_idx: usize) -> Result<usize, DeclarativeError> {
    debug_assert_eq!(bytes[open_idx], b'(');
    let mut depth = 0i32;
    let mut i = open_idx;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    Err(DeclarativeError::new(
        SpanIR::new(open_idx as u32, bytes.len() as u32),
        "unbalanced `(` in pattern",
    ))
}

fn skip_ws(bytes: &[u8], mut i: usize) -> usize {
    while i < bytes.len() && matches!(bytes[i], b' ' | b'\t' | b'\r' | b'\n') {
        i += 1;
    }
    i
}

// ---------------------------------------------------------------------------
// Pattern parsing
// ---------------------------------------------------------------------------

fn parse_pattern(src: &str, span: SpanIR) -> Result<Pattern, DeclarativeError> {
    let trimmed = src.trim();
    if trimmed.is_empty() {
        return Ok(Pattern::Empty);
    }
    let mut parser = PatternParser::new(src, span);
    let elements = parser.parse_elements(None)?;
    Ok(Pattern::Sequence(elements))
}

struct PatternParser<'a> {
    src: &'a [u8],
    // Absolute span base: adding this to a byte index inside `src` yields the
    // span position in the original source file.
    base: u32,
    pos: usize,
}

impl<'a> PatternParser<'a> {
    fn new(src: &'a str, span: SpanIR) -> Self {
        Self {
            src: src.as_bytes(),
            base: span.start,
            pos: 0,
        }
    }

    fn span_at(&self, start: usize, end: usize) -> SpanIR {
        SpanIR::new(self.base + start as u32, self.base + end as u32)
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            if matches!(c, b' ' | b'\t' | b'\r' | b'\n') {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    /// Parse a sequence of elements until `stop` (if any) or end of input.
    fn parse_elements(
        &mut self,
        stop: Option<u8>,
    ) -> Result<Vec<PatternElement>, DeclarativeError> {
        let mut elements = Vec::new();
        loop {
            self.skip_ws();
            match self.peek() {
                None => break,
                Some(c) if Some(c) == stop => break,
                Some(b'$') => {
                    // Fragment or repetition.
                    self.pos += 1; // consume `$`
                    match self.peek() {
                        Some(b'(') => {
                            self.pos += 1; // consume `(`
                            let inner = self.parse_elements(Some(b')'))?;
                            match self.peek() {
                                Some(b')') => self.pos += 1,
                                _ => {
                                    return Err(DeclarativeError::new(
                                        self.span_at(self.pos, self.pos),
                                        "expected `)` closing repetition pattern",
                                    ));
                                }
                            }
                            // After `)`, an optional separator, then a repkind.
                            let (separator, kind) = self.parse_rep_suffix()?;
                            elements.push(PatternElement::Repetition {
                                pattern: Box::new(Pattern::Sequence(inner)),
                                separator,
                                kind,
                            });
                        }
                        Some(c) if is_ident_start(c) => {
                            let name = self.consume_ident();
                            self.skip_ws();
                            if self.peek() != Some(b':') {
                                return Err(DeclarativeError::new(
                                    self.span_at(self.pos, self.pos),
                                    format!("expected `:<Kind>` after fragment name `${}`", name),
                                ));
                            }
                            self.pos += 1; // consume `:`
                            self.skip_ws();
                            let kind_start = self.pos;
                            let kind_name = self.consume_ident();
                            if kind_name.is_empty() {
                                return Err(DeclarativeError::new(
                                    self.span_at(kind_start, kind_start),
                                    format!(
                                        "expected fragment kind after `${}:` (known: {})",
                                        name,
                                        FragmentKind::known_names(),
                                    ),
                                ));
                            }
                            let kind = FragmentKind::from_name(&kind_name).ok_or_else(|| {
                                DeclarativeError::new(
                                    self.span_at(kind_start, self.pos),
                                    format!(
                                        "unknown fragment kind `{}` (known: {})",
                                        kind_name,
                                        FragmentKind::known_names(),
                                    ),
                                )
                            })?;
                            elements.push(PatternElement::Fragment { name, kind });
                        }
                        Some(other) => {
                            return Err(DeclarativeError::new(
                                self.span_at(self.pos, self.pos + 1),
                                format!(
                                    "unexpected character `{}` after `$` (expected identifier or `(`)",
                                    other as char
                                ),
                            ));
                        }
                        None => {
                            return Err(DeclarativeError::new(
                                self.span_at(self.pos, self.pos),
                                "unexpected end of pattern after `$`",
                            ));
                        }
                    }
                }
                Some(b',') | Some(b';') => {
                    // Separator — emit as literal element so matchers can
                    // consume it between positional fragments.
                    let c = self.src[self.pos];
                    self.pos += 1;
                    elements.push(PatternElement::Literal((c as char).to_string()));
                }
                Some(_) => {
                    // A literal run (e.g., a bare comma or semicolon handled above;
                    // anything else we treat as an error for MVP).
                    let start = self.pos;
                    let c = self.src[self.pos];
                    self.pos += 1;
                    return Err(DeclarativeError::new(
                        self.span_at(start, self.pos),
                        format!(
                            "unexpected character `{}` in pattern (expected `$name:Kind`, `$( ... )`, `,` or `;`)",
                            c as char
                        ),
                    ));
                }
            }
        }
        Ok(elements)
    }

    /// Parse the suffix after `$(...)`: an optional separator followed by a
    /// repetition kind marker (`*`, `+`, `?`). Common forms:
    ///
    /// - `$(...),+`  — comma-separated, one-or-more
    /// - `$(...),*`  — comma-separated, zero-or-more
    /// - `$(...);+`  — semicolon-separated, one-or-more
    /// - `$(...)?`   — zero-or-one (no separator)
    /// - `$(...)+`   — no separator, one-or-more
    /// - `$(...)*`   — no separator, zero-or-more
    fn parse_rep_suffix(&mut self) -> Result<(Option<String>, RepetitionKind), DeclarativeError> {
        self.skip_ws();
        let separator = match self.peek() {
            Some(c @ (b',' | b';')) => {
                self.pos += 1;
                Some((c as char).to_string())
            }
            _ => None,
        };
        self.skip_ws();
        let kind = match self.peek() {
            Some(b'*') => {
                self.pos += 1;
                RepetitionKind::ZeroOrMore
            }
            Some(b'+') => {
                self.pos += 1;
                RepetitionKind::OneOrMore
            }
            Some(b'?') => {
                self.pos += 1;
                RepetitionKind::ZeroOrOne
            }
            _ => {
                return Err(DeclarativeError::new(
                    self.span_at(self.pos, self.pos),
                    "expected `*`, `+`, or `?` after repetition",
                ));
            }
        };
        Ok((separator, kind))
    }

    fn consume_ident(&mut self) -> String {
        let start = self.pos;
        if let Some(c) = self.peek() {
            if !is_ident_start(c) {
                return String::new();
            }
        } else {
            return String::new();
        }
        while let Some(c) = self.peek() {
            if is_ident_continue(c) {
                self.pos += 1;
            } else {
                break;
            }
        }
        std::str::from_utf8(&self.src[start..self.pos])
            .expect("ident is ASCII")
            .to_string()
    }
}

fn is_ident_start(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}

fn is_ident_continue(c: u8) -> bool {
    is_ident_start(c) || c.is_ascii_digit()
}

// ---------------------------------------------------------------------------
// Body parsing
// ---------------------------------------------------------------------------

fn parse_body(src: &str, span: SpanIR) -> Result<Body, DeclarativeError> {
    let mut parser = BodyParser::new(src, span);
    let tokens = parser.parse_until(None)?;
    Ok(Body(tokens))
}

/// Which kind of string literal, if any, the parser is currently inside.
///
/// The body parser is lexically shallow — it only understands three things
/// it needs to avoid eating: single-quoted strings, double-quoted strings,
/// and template literals. Inside `Single`, `Double`, or the text portion
/// of `Template`, a bare `$` is always literal text, never the start of a
/// substitution or macro escape. Template-expression slots (`${...}`) are
/// a return-trip: we push the enclosing `Template` onto the stack and
/// parse the expression in `Code` mode, so `${$x}` still sees `$x` as a
/// substitution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringCtx {
    /// Normal macro-body code. `$ident` is a substitution/call, `$$` is
    /// an escape.
    Code,
    /// Inside `'...'`. `$` is literal.
    Single,
    /// Inside `"..."`. `$` is literal.
    Double,
    /// Inside `` `...` `` template literal, in the string-text portion
    /// (not currently inside a `${}` expression slot).
    Template,
}

/// Incremental string-context tracker. The three body-parsing loops
/// (`parse_until`, `parse_call_args`, `parse_repetition_inner`) each
/// instantiate one of these and call [`StringState::advance`] on every
/// byte as the cursor walks forward. Between advances, callers consult
/// [`StringState::in_string`] to decide whether to treat `$` as special
/// at the current position.
///
/// The state machine handles:
///
/// - `\\` escapes inside single/double/template strings (next byte is
///   literal regardless of what it is).
/// - Entry to and exit from each string kind.
/// - Template-expression slots: when a `$` immediately followed by `{`
///   appears in `Template` context, the state pushes `Template` onto
///   `stack`, switches to `Code`, and resets `brace_depth`. The first
///   `{` seen in the new `Code` region is special-cased (it's the
///   opener of the `${...}` slot, not a nested block), so a
///   `pending_open_brace` flag absorbs it. Subsequent `{` / `}` inside
///   the slot increment and decrement `brace_depth` normally; when
///   `brace_depth` is 0 and a `}` arrives, the stack is popped and we
///   return to `Template`.
#[derive(Debug, Clone)]
struct StringState {
    ctx: StringCtx,
    /// Next byte is an escape target (already saw `\` inside a string).
    escape_next: bool,
    /// Stack of saved contexts — one entry per nested template-expression
    /// slot. Each entry is the context to return to (always `Template`
    /// in practice, since only template literals have expression slots).
    stack: Vec<StringCtx>,
    /// `{`/`}` nesting depth within the current template-expression slot.
    /// Only meaningful when `stack` is non-empty (i.e., we're inside one).
    brace_depth: u32,
    /// The next `{` seen in `Code` context is the opener of a
    /// `${...}` slot — absorb it without touching `brace_depth`.
    pending_open_brace: bool,
}

impl StringState {
    fn new() -> Self {
        Self {
            ctx: StringCtx::Code,
            escape_next: false,
            stack: Vec::new(),
            brace_depth: 0,
            pending_open_brace: false,
        }
    }

    /// Returns `true` iff the position **before** the next `advance` call
    /// is inside a string literal text region where `$` must be treated
    /// as a literal character. Template-expression slots return `false`
    /// because they're code, not string text.
    fn in_string(&self) -> bool {
        matches!(
            self.ctx,
            StringCtx::Single | StringCtx::Double | StringCtx::Template
        )
    }

    /// Walk the state forward by one byte. `b` is the byte at the
    /// current cursor position; `next` is the byte immediately after
    /// (used to detect `${` in template context — the `$` needs to know
    /// whether a `{` follows before it decides to open an expression
    /// slot).
    fn advance(&mut self, b: u8, next: Option<u8>) {
        if self.escape_next {
            self.escape_next = false;
            return;
        }
        match self.ctx {
            StringCtx::Code => match b {
                b'\'' => self.ctx = StringCtx::Single,
                b'"' => self.ctx = StringCtx::Double,
                b'`' => self.ctx = StringCtx::Template,
                b'{' if !self.stack.is_empty() => {
                    if self.pending_open_brace {
                        self.pending_open_brace = false;
                    } else {
                        self.brace_depth += 1;
                    }
                }
                b'}' if !self.stack.is_empty() => {
                    if self.brace_depth == 0 {
                        // Closing `}` of the surrounding `${...}` slot —
                        // pop back to the enclosing template literal.
                        self.ctx = self.stack.pop().unwrap_or(StringCtx::Code);
                    } else {
                        self.brace_depth -= 1;
                    }
                }
                _ => {}
            },
            StringCtx::Single => match b {
                b'\\' => self.escape_next = true,
                b'\'' => self.ctx = StringCtx::Code,
                _ => {}
            },
            StringCtx::Double => match b {
                b'\\' => self.escape_next = true,
                b'"' => self.ctx = StringCtx::Code,
                _ => {}
            },
            StringCtx::Template => match b {
                b'\\' => self.escape_next = true,
                b'`' => self.ctx = StringCtx::Code,
                b'$' if next == Some(b'{') => {
                    // Enter an expression slot. The next byte will be
                    // the `{`, which `pending_open_brace` causes us to
                    // absorb without a depth bump.
                    self.stack.push(StringCtx::Template);
                    self.ctx = StringCtx::Code;
                    self.brace_depth = 0;
                    self.pending_open_brace = true;
                }
                _ => {}
            },
        }
    }
}

struct BodyParser<'a> {
    src: &'a [u8],
    base: u32,
    pos: usize,
}

impl<'a> BodyParser<'a> {
    fn new(src: &'a str, span: SpanIR) -> Self {
        Self {
            src: src.as_bytes(),
            base: span.start,
            pos: 0,
        }
    }

    fn span_at(&self, start: usize, end: usize) -> SpanIR {
        SpanIR::new(self.base + start as u32, self.base + end as u32)
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn peek_n(&self, n: usize) -> Option<u8> {
        self.src.get(self.pos + n).copied()
    }

    /// Parse body tokens until we hit the byte `stop` (if any) at depth 0
    /// relative to the repetition nesting. Literal parens/braces inside the
    /// body don't cause early termination — only the matching `)` of a
    /// repetition does, and we rely on a separate depth counter for that.
    fn parse_until(&mut self, _stop: Option<u8>) -> Result<Vec<BodyToken>, DeclarativeError> {
        let mut tokens: Vec<BodyToken> = Vec::new();
        let mut literal_start = self.pos;
        let mut str_state = StringState::new();

        while self.pos < self.src.len() {
            let c = self.src[self.pos];
            // Inside a string literal, `$` is always literal text — skip
            // special handling and let the cursor advance naturally.
            if !str_state.in_string() && c == b'$' {
                // Could be `$ident` substitution, `$(...)rep` repetition, or `$$` escape.
                match self.peek_n(1) {
                    Some(b'$') => {
                        // `$$` escape — emit literal `$`, skip both.
                        // Flush literal up to here, then push `$` as its own literal.
                        self.flush_literal(&mut tokens, literal_start, self.pos);
                        tokens.push(BodyToken::Literal("$".to_string()));
                        // Tick the state machine past both consumed bytes
                        // so the context reflects two forward steps.
                        str_state.advance(c, self.peek_n(1));
                        str_state.advance(b'$', self.peek_n(2));
                        self.pos += 2;
                        literal_start = self.pos;
                        continue;
                    }
                    Some(b'(') => {
                        self.flush_literal(&mut tokens, literal_start, self.pos);
                        self.pos += 2; // consume `$(`
                        let inner = self.parse_repetition_inner()?;
                        match self.peek() {
                            Some(b')') => self.pos += 1,
                            _ => {
                                return Err(DeclarativeError::new(
                                    self.span_at(self.pos, self.pos),
                                    "expected `)` closing repetition in body",
                                ));
                            }
                        }
                        let (separator, kind) = self.parse_rep_suffix()?;
                        tokens.push(BodyToken::Repetition {
                            body: inner,
                            separator,
                            kind,
                        });
                        // The repetition sub-parser maintains its own
                        // string state; we treat the whole `$(...)+`
                        // as one opaque step and reset ours to Code.
                        str_state = StringState::new();
                        literal_start = self.pos;
                        continue;
                    }
                    Some(c) if is_ident_start(c) => {
                        self.flush_literal(&mut tokens, literal_start, self.pos);
                        self.pos += 1; // consume `$`
                        let name = self.consume_ident();
                        // Phase 12: if the next non-whitespace char on the
                        // SAME LINE is `(`, this is a macro call, not a
                        // substitution. Newlines break the association so
                        // `$foo\n(stmt)` stays a substitution followed by a
                        // parenthesized expression — the same rule Rust's
                        // `macro_rules!` uses.
                        //
                        // Phase E exception: `$__cluster__` is a reserved
                        // substitution name used by the cluster-aware
                        // runtime-name template feature. It must always
                        // parse as a substitution so `__h_$__cluster__($y)`
                        // renders correctly as `__h_<id>($y)` instead of
                        // being misread as a call to a macro named
                        // `__cluster__`.
                        //
                        // Type-position composition: `$ident<args>` with
                        // **strict adjacency** (no whitespace between
                        // ident and `<`) is parsed as a macro call too,
                        // so type macros can naturally compose:
                        // `$Result<T, E> => $Box<T> | E`. The strict-
                        // adjacency rule prevents value-position bodies
                        // from misreading `$x < $y` (with whitespace) as
                        // a call.
                        let is_value_call = name != RESERVED_CLUSTER_NAME
                            && self.peek_after_horizontal_ws() == Some(b'(');
                        let is_type_call = name != RESERVED_CLUSTER_NAME
                            && !is_value_call
                            && self.peek() == Some(b'<')
                            && self.angle_args_balance_ok();
                        if is_value_call {
                            self.skip_horizontal_ws();
                            self.pos += 1; // consume `(`
                            let args = self.parse_call_args()?;
                            match self.peek() {
                                Some(b')') => self.pos += 1,
                                _ => {
                                    return Err(DeclarativeError::new(
                                        self.span_at(self.pos, self.pos),
                                        "expected `)` closing macro call in body",
                                    ));
                                }
                            }
                            tokens.push(BodyToken::MacroCall { name, args });
                        } else if is_type_call {
                            self.pos += 1; // consume `<`
                            let args = self.parse_type_call_args()?;
                            match self.peek() {
                                Some(b'>') => self.pos += 1,
                                _ => {
                                    return Err(DeclarativeError::new(
                                        self.span_at(self.pos, self.pos),
                                        "expected `>` closing type-position macro call in body",
                                    ));
                                }
                            }
                            tokens.push(BodyToken::MacroCall { name, args });
                        } else {
                            tokens.push(BodyToken::Substitution(name));
                        }
                        // The sub-parse may have walked through strings
                        // itself; reset our tracker to Code for whatever
                        // follows.
                        str_state = StringState::new();
                        literal_start = self.pos;
                        continue;
                    }
                    _ => {
                        // Bare `$` in body — treat as literal.
                    }
                }
            }
            // Default fall-through: advance the cursor and the string
            // state machine by one byte.
            str_state.advance(c, self.peek_n(1));
            self.pos += 1;
        }

        self.flush_literal(&mut tokens, literal_start, self.pos);
        Ok(tokens)
    }

    /// Peek past **horizontal** ASCII whitespace (spaces and tabs only) and
    /// return the next byte. Used by `$ident` → macro-call disambiguation
    /// so that a newline between the identifier and a following `(` breaks
    /// the association — `$foo\n(stmt)` parses as substitution + paren
    /// expression, not as `$foo(stmt)`.
    fn peek_after_horizontal_ws(&self) -> Option<u8> {
        let mut j = self.pos;
        while j < self.src.len() && matches!(self.src[j], b' ' | b'\t') {
            j += 1;
        }
        self.src.get(j).copied()
    }

    /// Skip only spaces and tabs — not newlines. Paired with
    /// [`Self::peek_after_horizontal_ws`] for the newline-sensitive
    /// `$ident ( args )` disambiguation.
    fn skip_horizontal_ws(&mut self) {
        while self.pos < self.src.len() && matches!(self.src[self.pos], b' ' | b'\t') {
            self.pos += 1;
        }
    }

    /// Look ahead from `self.pos` (which must point at `<`) to verify
    /// that the angle-bracket span balances cleanly — i.e. there's a
    /// matching `>` at the same depth, no unbalanced parens / braces
    /// inside, and the contents look like a comma-separated argument
    /// list rather than a chained comparison expression.
    ///
    /// Used by the body parser to disambiguate `$ident<args>` (a
    /// type-position macro call) from `$x < $y > z` (a chain of
    /// comparison operators that should fall through to substitution
    /// + literal). The lookahead is read-only and does not advance
    /// `self.pos`.
    ///
    /// Conservative: returns `false` whenever the lookahead is
    /// ambiguous, so the parser falls back to the substitution path
    /// and the user can always disambiguate by inserting whitespace
    /// (`$x < $y` is never a macro call because it has space before
    /// the `<`).
    fn angle_args_balance_ok(&self) -> bool {
        if self.peek() != Some(b'<') {
            return false;
        }
        let mut depth: i32 = 0;
        let mut paren: i32 = 0;
        let mut brace: i32 = 0;
        let mut bracket: i32 = 0;
        let mut j = self.pos;
        let mut in_string: Option<u8> = None;
        let mut escape_next = false;
        while j < self.src.len() {
            let c = self.src[j];
            if let Some(quote) = in_string {
                if escape_next {
                    escape_next = false;
                } else if c == b'\\' {
                    escape_next = true;
                } else if c == quote {
                    in_string = None;
                }
                j += 1;
                continue;
            }
            match c {
                b'\'' | b'"' | b'`' => {
                    in_string = Some(c);
                }
                b'(' => paren += 1,
                b')' => paren -= 1,
                b'{' => brace += 1,
                b'}' => brace -= 1,
                b'[' => bracket += 1,
                b']' => bracket -= 1,
                b'<' => depth += 1,
                b'>' => {
                    depth -= 1;
                    if depth == 0 {
                        // Balanced — make sure paren/brace/bracket are
                        // also balanced inside the angle span.
                        return paren == 0 && brace == 0 && bracket == 0;
                    }
                }
                // A semicolon or end-of-line inside the angles
                // strongly suggests we're not in a type argument
                // list. Bail out conservatively.
                b';' | b'\n' => return false,
                _ => {}
            }
            j += 1;
        }
        false
    }

    /// Parse the contents of a type-position macro call's argument
    /// list — the body tokens between `<` and the matching `>`.
    /// Tracks `<`/`>` depth so nested generic args (e.g.
    /// `$Outer<$Inner<$x>>`) parse correctly. Body substitutions
    /// (`$ident`) and nested macro calls (`$other(...)` or
    /// `$other<...>`) are recognized inside the type-arg list the
    /// same way they are inside parenthesized arg lists.
    fn parse_type_call_args(&mut self) -> Result<Vec<BodyToken>, DeclarativeError> {
        let mut tokens: Vec<BodyToken> = Vec::new();
        let mut literal_start = self.pos;
        let mut depth: i32 = 0;

        while self.pos < self.src.len() {
            let c = self.src[self.pos];
            if c == b'>' && depth == 0 {
                self.flush_literal(&mut tokens, literal_start, self.pos);
                return Ok(tokens);
            }
            if c == b'<' {
                depth += 1;
                self.pos += 1;
                continue;
            }
            if c == b'>' {
                depth -= 1;
                self.pos += 1;
                continue;
            }
            if c == b'$' {
                match self.peek_n(1) {
                    Some(b'$') => {
                        self.flush_literal(&mut tokens, literal_start, self.pos);
                        tokens.push(BodyToken::Literal("$".to_string()));
                        self.pos += 2;
                        literal_start = self.pos;
                        continue;
                    }
                    Some(c) if is_ident_start(c) => {
                        self.flush_literal(&mut tokens, literal_start, self.pos);
                        self.pos += 1; // consume `$`
                        let name = self.consume_ident();
                        // Inside a type-arg list we ALSO recognize
                        // nested type-call shapes for the natural
                        // form `$Outer<$Inner<$x>>`. Strict
                        // adjacency on `<` to avoid false positives.
                        let nested_value_call = name != RESERVED_CLUSTER_NAME
                            && self.peek_after_horizontal_ws() == Some(b'(');
                        let nested_type_call = name != RESERVED_CLUSTER_NAME
                            && !nested_value_call
                            && self.peek() == Some(b'<')
                            && self.angle_args_balance_ok();
                        if nested_value_call {
                            self.skip_horizontal_ws();
                            self.pos += 1;
                            let nested_args = self.parse_call_args()?;
                            match self.peek() {
                                Some(b')') => self.pos += 1,
                                _ => {
                                    return Err(DeclarativeError::new(
                                        self.span_at(self.pos, self.pos),
                                        "expected `)` closing nested macro call in type arg list",
                                    ));
                                }
                            }
                            tokens.push(BodyToken::MacroCall {
                                name,
                                args: nested_args,
                            });
                        } else if nested_type_call {
                            self.pos += 1; // consume `<`
                            let nested_args = self.parse_type_call_args()?;
                            match self.peek() {
                                Some(b'>') => self.pos += 1,
                                _ => {
                                    return Err(DeclarativeError::new(
                                        self.span_at(self.pos, self.pos),
                                        "expected `>` closing nested type-macro call in type arg list",
                                    ));
                                }
                            }
                            tokens.push(BodyToken::MacroCall {
                                name,
                                args: nested_args,
                            });
                        } else {
                            tokens.push(BodyToken::Substitution(name));
                        }
                        literal_start = self.pos;
                        continue;
                    }
                    _ => {}
                }
            }
            self.pos += 1;
        }

        Err(DeclarativeError::new(
            self.span_at(self.pos, self.pos),
            "unterminated type-position macro call in body (no matching `>`)",
        ))
    }

    /// Parse the contents of a macro call's argument list — the body
    /// tokens between `(` and the matching `)`. Nested `()` are tracked
    /// so e.g. `$outer(foo($x))` is parsed as one `MacroCall` containing
    /// a literal `foo(` + substitution + literal `)`.
    fn parse_call_args(&mut self) -> Result<Vec<BodyToken>, DeclarativeError> {
        let mut tokens: Vec<BodyToken> = Vec::new();
        let mut literal_start = self.pos;
        let mut paren_depth: i32 = 0;
        let mut str_state = StringState::new();

        while self.pos < self.src.len() {
            let c = self.src[self.pos];
            // `)` and `(` inside string literals don't affect call
            // argument balancing — they're just text.
            if !str_state.in_string() {
                if c == b')' && paren_depth == 0 {
                    self.flush_literal(&mut tokens, literal_start, self.pos);
                    return Ok(tokens);
                }
                if c == b'(' {
                    paren_depth += 1;
                    str_state.advance(c, self.peek_n(1));
                    self.pos += 1;
                    continue;
                }
                if c == b')' {
                    paren_depth -= 1;
                    str_state.advance(c, self.peek_n(1));
                    self.pos += 1;
                    continue;
                }
                if c == b'$' {
                    match self.peek_n(1) {
                        Some(b'$') => {
                            self.flush_literal(&mut tokens, literal_start, self.pos);
                            tokens.push(BodyToken::Literal("$".to_string()));
                            str_state.advance(c, self.peek_n(1));
                            str_state.advance(b'$', self.peek_n(2));
                            self.pos += 2;
                            literal_start = self.pos;
                            continue;
                        }
                        Some(c) if is_ident_start(c) => {
                            self.flush_literal(&mut tokens, literal_start, self.pos);
                            self.pos += 1; // consume `$`
                            let name = self.consume_ident();
                            // Nested macro call inside arg list. Newlines
                            // between the identifier and `(` break the
                            // association, mirroring `parse_until`.
                            // `$cluster` is reserved — see the
                            // [`RESERVED_CLUSTER_NAME`] constant.
                            if name != RESERVED_CLUSTER_NAME
                                && self.peek_after_horizontal_ws() == Some(b'(')
                            {
                                self.skip_horizontal_ws();
                                self.pos += 1;
                                let nested_args = self.parse_call_args()?;
                                match self.peek() {
                                    Some(b')') => self.pos += 1,
                                    _ => {
                                        return Err(DeclarativeError::new(
                                            self.span_at(self.pos, self.pos),
                                            "expected `)` closing nested macro call in arg list",
                                        ));
                                    }
                                }
                                tokens.push(BodyToken::MacroCall {
                                    name,
                                    args: nested_args,
                                });
                            } else {
                                tokens.push(BodyToken::Substitution(name));
                            }
                            str_state = StringState::new();
                            literal_start = self.pos;
                            continue;
                        }
                        _ => {}
                    }
                }
            }
            str_state.advance(c, self.peek_n(1));
            self.pos += 1;
        }

        Err(DeclarativeError::new(
            self.span_at(self.pos, self.pos),
            "unterminated macro call in body (no matching `)`)",
        ))
    }

    fn parse_repetition_inner(&mut self) -> Result<Vec<BodyToken>, DeclarativeError> {
        // Track `(` depth so nested `()` inside the repetition body don't
        // cause early termination. The opening `$(` consumed one paren's
        // worth of "depth context" already; we stop at the matching `)`.
        let mut tokens: Vec<BodyToken> = Vec::new();
        let mut literal_start = self.pos;
        let mut paren_depth: i32 = 0;
        let mut str_state = StringState::new();

        while self.pos < self.src.len() {
            let c = self.src[self.pos];
            if !str_state.in_string() {
                if c == b')' && paren_depth == 0 {
                    self.flush_literal(&mut tokens, literal_start, self.pos);
                    return Ok(tokens);
                }
                if c == b'(' {
                    paren_depth += 1;
                    str_state.advance(c, self.peek_n(1));
                    self.pos += 1;
                    continue;
                }
                if c == b')' {
                    paren_depth -= 1;
                    str_state.advance(c, self.peek_n(1));
                    self.pos += 1;
                    continue;
                }
                if c == b'$' {
                    match self.peek_n(1) {
                        Some(b'$') => {
                            self.flush_literal(&mut tokens, literal_start, self.pos);
                            tokens.push(BodyToken::Literal("$".to_string()));
                            str_state.advance(c, self.peek_n(1));
                            str_state.advance(b'$', self.peek_n(2));
                            self.pos += 2;
                            literal_start = self.pos;
                            continue;
                        }
                        Some(b'(') => {
                            self.flush_literal(&mut tokens, literal_start, self.pos);
                            self.pos += 2;
                            let inner = self.parse_repetition_inner()?;
                            match self.peek() {
                                Some(b')') => self.pos += 1,
                                _ => {
                                    return Err(DeclarativeError::new(
                                        self.span_at(self.pos, self.pos),
                                        "expected `)` closing nested repetition in body",
                                    ));
                                }
                            }
                            let (separator, kind) = self.parse_rep_suffix()?;
                            tokens.push(BodyToken::Repetition {
                                body: inner,
                                separator,
                                kind,
                            });
                            str_state = StringState::new();
                            literal_start = self.pos;
                            continue;
                        }
                        Some(c) if is_ident_start(c) => {
                            self.flush_literal(&mut tokens, literal_start, self.pos);
                            self.pos += 1;
                            let name = self.consume_ident();
                            // Phase 12: macro call inside a repetition
                            // body — `$( $double($x); )+`. Newline-
                            // sensitive disambiguation matches the
                            // top-level rule. `$cluster` is reserved —
                            // see [`RESERVED_CLUSTER_NAME`].
                            if name != RESERVED_CLUSTER_NAME
                                && self.peek_after_horizontal_ws() == Some(b'(')
                            {
                                self.skip_horizontal_ws();
                                self.pos += 1;
                                // Note: we re-use parse_call_args here
                                // even though we're inside a repetition
                                // body. The call args are balanced on
                                // their own `()`; the repetition's outer
                                // `)` is handled by the paren_depth
                                // counter above.
                                let call_args = self.parse_call_args()?;
                                match self.peek() {
                                    Some(b')') => self.pos += 1,
                                    _ => {
                                        return Err(DeclarativeError::new(
                                            self.span_at(self.pos, self.pos),
                                            "expected `)` closing macro call in repetition body",
                                        ));
                                    }
                                }
                                tokens.push(BodyToken::MacroCall {
                                    name,
                                    args: call_args,
                                });
                            } else {
                                tokens.push(BodyToken::Substitution(name));
                            }
                            str_state = StringState::new();
                            literal_start = self.pos;
                            continue;
                        }
                        _ => {}
                    }
                }
            }
            str_state.advance(c, self.peek_n(1));
            self.pos += 1;
        }

        Err(DeclarativeError::new(
            self.span_at(self.pos, self.pos),
            "unterminated repetition body (no matching `)`)",
        ))
    }

    fn parse_rep_suffix(&mut self) -> Result<(Option<String>, RepetitionKind), DeclarativeError> {
        let separator = match self.peek() {
            Some(c @ (b',' | b';')) => {
                // Lookahead: only treat as separator if followed by a rep
                // kind marker (with optional whitespace). Otherwise it's part
                // of the subsequent literal.
                let mut probe = self.pos + 1;
                while probe < self.src.len()
                    && matches!(self.src[probe], b' ' | b'\t' | b'\r' | b'\n')
                {
                    probe += 1;
                }
                if probe < self.src.len() && matches!(self.src[probe], b'*' | b'+' | b'?') {
                    self.pos += 1;
                    Some((c as char).to_string())
                } else {
                    None
                }
            }
            _ => None,
        };
        let kind = match self.peek() {
            Some(b'*') => {
                self.pos += 1;
                RepetitionKind::ZeroOrMore
            }
            Some(b'+') => {
                self.pos += 1;
                RepetitionKind::OneOrMore
            }
            Some(b'?') => {
                self.pos += 1;
                RepetitionKind::ZeroOrOne
            }
            _ => {
                return Err(DeclarativeError::new(
                    self.span_at(self.pos, self.pos),
                    "expected `*`, `+`, or `?` after repetition in body",
                ));
            }
        };
        Ok((separator, kind))
    }

    fn flush_literal(&self, tokens: &mut Vec<BodyToken>, start: usize, end: usize) {
        if end > start {
            let text = std::str::from_utf8(&self.src[start..end])
                .expect("body source is valid UTF-8")
                .to_string();
            tokens.push(BodyToken::Literal(text));
        }
    }

    fn consume_ident(&mut self) -> String {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if is_ident_continue(c) {
                self.pos += 1;
            } else {
                break;
            }
        }
        std::str::from_utf8(&self.src[start..self.pos])
            .expect("ident is ASCII")
            .to_string()
    }
}
