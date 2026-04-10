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

    Ok(MacroDef {
        name: String::new(),
        arms: parsed_arms,
        mode: MacroMode::ExpandOnly,
        span: template_span,
    })
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

        while self.pos < self.src.len() {
            let c = self.src[self.pos];
            if c == b'$' {
                // Could be `$ident` substitution, `$(...)rep` repetition, or `$$` escape.
                match self.peek_n(1) {
                    Some(b'$') => {
                        // `$$` escape — emit literal `$`, skip both.
                        // Flush literal up to here, then push `$` as its own literal.
                        self.flush_literal(&mut tokens, literal_start, self.pos);
                        tokens.push(BodyToken::Literal("$".to_string()));
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
                        literal_start = self.pos;
                        continue;
                    }
                    Some(c) if is_ident_start(c) => {
                        self.flush_literal(&mut tokens, literal_start, self.pos);
                        self.pos += 1; // consume `$`
                        let name = self.consume_ident();
                        tokens.push(BodyToken::Substitution(name));
                        literal_start = self.pos;
                        continue;
                    }
                    _ => {
                        // Bare `$` in body — treat as literal.
                    }
                }
            }
            self.pos += 1;
        }

        self.flush_literal(&mut tokens, literal_start, self.pos);
        Ok(tokens)
    }

    fn parse_repetition_inner(&mut self) -> Result<Vec<BodyToken>, DeclarativeError> {
        // Track `(` depth so nested `()` inside the repetition body don't
        // cause early termination. The opening `$(` consumed one paren's
        // worth of "depth context" already; we stop at the matching `)`.
        let mut tokens: Vec<BodyToken> = Vec::new();
        let mut literal_start = self.pos;
        let mut paren_depth: i32 = 0;

        while self.pos < self.src.len() {
            let c = self.src[self.pos];
            if c == b')' && paren_depth == 0 {
                self.flush_literal(&mut tokens, literal_start, self.pos);
                return Ok(tokens);
            }
            if c == b'(' {
                paren_depth += 1;
                self.pos += 1;
                continue;
            }
            if c == b')' {
                paren_depth -= 1;
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
                        literal_start = self.pos;
                        continue;
                    }
                    Some(c) if is_ident_start(c) => {
                        self.flush_literal(&mut tokens, literal_start, self.pos);
                        self.pos += 1;
                        let name = self.consume_ident();
                        tokens.push(BodyToken::Substitution(name));
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
