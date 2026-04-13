//! Context-aware hygiene rewriter for declarative macro expansions.
//!
//! The MVP hygiene pass in `expander.rs` was a plain byte-level
//! substring scan: it looked for `const __x` / `let __x` / `var __x`
//! anywhere in the expanded source and renamed matching identifier
//! uses. That was cheap but wrong — it corrupted string literals that
//! happened to mention `__names`, missed declarations split across
//! whitespace runs, and had no awareness of comments, regex literals,
//! or Unicode identifiers.
//!
//! This module replaces that with a small JS/TS-aware lexical cursor.
//! It is **not** a parser; it only tokenizes enough to distinguish
//! "code" from "not code" so that `$`-substitution hygiene decisions
//! are never made based on bytes that live inside a string literal,
//! a comment, a regex, or a template-literal text portion.
//!
//! Two public entry points:
//!
//! - [`collect_declared_underscore_names`] — pass 1. Walks the source
//!   in one forward sweep, lexically aware, and returns the set of
//!   `__`-prefixed identifiers that appear as the *binding name* of a
//!   `const` / `let` / `var` declaration in code context.
//! - [`rewrite_identifiers`] — pass 2. Walks the source again with
//!   the same lexical awareness and replaces every occurrence of each
//!   name in `declared` with `name + suffix`, but only in code
//!   context. Text inside strings, comments, or regex literals is
//!   never touched.
//!
//! Both passes share a [`LexCursor`] state machine so their notion of
//! "code context" is guaranteed to match.
//!
//! ### What this module does NOT try to handle
//!
//! - Source maps or byte offsets into the original, pre-expansion
//!   macro body. Hygiene runs on already-rendered expansion text;
//!   spans are irrelevant here.
//! - JSX. Macro bodies are not expected to contain JSX — hygiene runs
//!   inside already-rendered bodies, not over the program source.
//!
//! Identifier classification follows the ECMAScript rules via the
//! `unicode_ident` crate (transitively present via OXC). That lets
//! `__café` be recognized as a single identifier rather than being
//! cut off at the first non-ASCII byte, which the old byte scanner
//! got wrong.

use std::collections::HashSet;

/// Collect every `__`-prefixed identifier that appears as the binding
/// name of a `const` / `let` / `var` declaration in the code portions
/// of `source`. Skips identifiers that appear inside string literals,
/// comments, regex literals, or template-literal string text.
pub(super) fn collect_declared_underscore_names(source: &str) -> HashSet<String> {
    let mut out: HashSet<String> = HashSet::new();
    let mut cursor = LexCursor::new(source);
    while let Some(event) = cursor.next_event() {
        if let Event::Keyword(KeywordKind::Const | KeywordKind::Let | KeywordKind::Var) = event {
            // The next identifier we see in code context — after
            // any amount of horizontal or vertical whitespace and
            // comments — is the binding name. We look it up via a
            // lightweight lookahead that shares the same state
            // machine, so a `const\n  __x` declaration works the
            // same as `const __x`. Destructuring patterns
            // (`const { __x } = ...`) are out of scope — the next
            // token there is `{`, not an ident, so we skip them.
            if let Some(next) = cursor.peek_next_code_ident()
                && next.starts_with("__")
            {
                out.insert(next.to_string());
            }
        }
    }
    out
}

/// Produce a copy of `source` in which every occurrence of a name in
/// `declared` (within code context) has `suffix` appended. Non-code
/// regions (strings, comments, regex literals, template-literal text)
/// are passed through verbatim.
///
/// `suffix` is the expansion-id suffix like `"$7"`; the final rewritten
/// name is `original + suffix` (so `__v` becomes `__v$7`).
pub(super) fn rewrite_identifiers(
    source: &str,
    declared: &HashSet<String>,
    suffix: &str,
) -> String {
    if declared.is_empty() {
        return source.to_string();
    }

    let mut out = String::with_capacity(source.len() + declared.len() * suffix.len());
    let mut cursor = LexCursor::new(source);
    // We emit output byte-by-byte as the cursor advances, except when
    // we detect a code-position identifier that matches a declared
    // name — in that case we replace the identifier text with
    // `name + suffix` and resume.
    loop {
        let start = cursor.pos();
        let Some(event) = cursor.next_event() else {
            // Flush any remaining bytes past the last event.
            out.push_str(&source[start..]);
            break;
        };
        // Copy every byte between the previous cursor position and
        // where this event started as-is. Each event records its own
        // start boundary in `cursor.event_start`, which is the byte
        // offset of the first character of the event. Anything in
        // between `start` and `cursor.event_start` is "passed-through"
        // context (whitespace, delimiter bytes, etc.).
        out.push_str(&source[start..cursor.event_start]);

        match event {
            Event::Ident { start: is, end: ie } => {
                let name = &source[is..ie];
                if declared.contains(name) {
                    out.push_str(name);
                    out.push_str(suffix);
                } else {
                    out.push_str(name);
                }
            }
            Event::Keyword(_) => {
                // Keywords are just spelled-out idents from a byte
                // perspective — copy them verbatim.
                out.push_str(&source[cursor.event_start..cursor.pos()]);
            }
            Event::StringLike { start: ss, end: se }
            | Event::Comment { start: ss, end: se }
            | Event::Regex { start: ss, end: se } => {
                // Copy the whole non-code run verbatim.
                out.push_str(&source[ss..se]);
            }
            Event::Punct => {
                // Single punctuation byte, already advanced.
                out.push_str(&source[cursor.event_start..cursor.pos()]);
            }
        }
    }

    out
}

// ---------------------------------------------------------------------------
// Lexer state machine
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeywordKind {
    Const,
    Let,
    Var,
    /// Keywords that put the parser in "expression position" — a `/`
    /// immediately following one of these starts a regex literal.
    /// Tracked so slash disambiguation stays correct after e.g.
    /// `return /x/g;`.
    ExprPrefix,
}

/// Events the cursor yields to its caller. Each event carries the
/// byte range it spans in the source so the caller can copy the
/// underlying text verbatim when appropriate.
#[derive(Debug, Clone, Copy)]
enum Event {
    /// A code-context identifier (ECMAScript `IdentifierName`).
    Ident { start: usize, end: usize },
    /// A keyword that the caller cares about for declaration detection
    /// or slash-disambiguation. Non-keyword identifiers surface as
    /// [`Event::Ident`].
    Keyword(KeywordKind),
    /// Any string-shaped literal: `'...'`, `"..."`, `` `...` ``. The
    /// range covers the opening delimiter through the closing one,
    /// including nested template-expression slots and their enclosed
    /// code. The caller copies the whole run verbatim and does not
    /// enter sub-expression scanning.
    StringLike { start: usize, end: usize },
    /// A `// ...` or `/* ... */` comment, delimiters inclusive.
    Comment { start: usize, end: usize },
    /// A `/.../flags` regex literal, delimiters inclusive.
    Regex { start: usize, end: usize },
    /// A single punctuation byte in code context. The byte itself is
    /// already reflected in the cursor's `event_start..pos` range, so
    /// the variant carries no payload — it exists purely so the main
    /// rewrite loop never loses sight of the cursor (every byte in
    /// code context produces either a Keyword / Ident / Punct event
    /// or a StringLike / Comment / Regex run).
    Punct,
}

/// Forward-only lexer used by both hygiene passes. Tracks just enough
/// context to distinguish code from string / comment / regex regions.
struct LexCursor<'a> {
    src: &'a [u8],
    /// Current byte offset in `src`.
    pos: usize,
    /// Byte offset at which the most recently produced event began.
    /// Callers read this after [`Self::next_event`] to know where the
    /// "pass-through" region before the event ended.
    event_start: usize,
    /// Kind of the previous code-context token, used exclusively for
    /// regex-vs-division disambiguation on a `/` byte.
    prev_kind: PrevKind,
    /// Mode stack for template-literal handling. Empty means we're in
    /// normal code mode (the top-level lexical scan). Non-empty means
    /// we're currently inside one or more template literals:
    ///
    /// - [`Mode::TemplateText`] — we're between `${...}` slots in a
    ///   template literal, and the next event will walk forward
    ///   until either `${` (switch to code, push
    ///   `Mode::TemplateExpr`) or a closing backtick (pop the mode).
    /// - [`Mode::TemplateExpr`] — we're inside a template-literal
    ///   `${...}` expression slot. Events look like normal code, but
    ///   we track `{`/`}` depth so the matching `}` pops us back to
    ///   `TemplateText`.
    ///
    /// The stack lets arbitrarily nested template literals work —
    /// `` `outer ${`inner ${x}`}` `` pushes and pops correctly.
    modes: Vec<Mode>,
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    TemplateText,
    TemplateExpr { brace_depth: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PrevKind {
    /// Start of input, or a preceding token that puts the parser in
    /// expression position (operators, `(`, `,`, `[`, `{`, `;`, `:`,
    /// `?`, `return`, `typeof`, etc.). A `/` in this state starts a
    /// regex literal.
    ExprStart,
    /// A preceding token that ends an expression (ident, numeric
    /// literal, `)`, `]`, string literal, `++`, `--`). A `/` in this
    /// state is a division operator.
    ExprEnd,
}

impl<'a> LexCursor<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src: src.as_bytes(),
            pos: 0,
            event_start: 0,
            prev_kind: PrevKind::ExprStart,
            modes: Vec::new(),
        }
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn peek(&self, off: usize) -> Option<u8> {
        self.src.get(self.pos + off).copied()
    }

    /// Advance past ASCII whitespace and comments without producing
    /// events; used by [`Self::peek_next_code_ident`] to look ahead
    /// over the gap between `const` and the following binding name.
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek(0) {
                Some(b' ' | b'\t' | b'\r' | b'\n') => {
                    self.pos += 1;
                }
                Some(b'/') if self.peek(1) == Some(b'/') => {
                    // Line comment — skip to end-of-line.
                    self.pos += 2;
                    while let Some(c) = self.peek(0) {
                        self.pos += 1;
                        if c == b'\n' {
                            break;
                        }
                    }
                }
                Some(b'/') if self.peek(1) == Some(b'*') => {
                    self.pos += 2;
                    while let Some(c) = self.peek(0) {
                        if c == b'*' && self.peek(1) == Some(b'/') {
                            self.pos += 2;
                            break;
                        }
                        self.pos += 1;
                    }
                }
                _ => break,
            }
        }
    }

    /// Look ahead (without consuming the underlying cursor's event
    /// stream) for the next code-context identifier. Returns its text
    /// slice into the original source. Used by the declaration-
    /// collector pass to grab the binding name that follows a
    /// `const`/`let`/`var` keyword.
    fn peek_next_code_ident(&mut self) -> Option<&'a str> {
        // Save the cursor state so we can restore it — this is a
        // lookahead, not an advance, from the caller's perspective.
        let saved_pos = self.pos;
        let saved_event_start = self.event_start;
        let saved_prev_kind = self.prev_kind;
        self.skip_whitespace_and_comments();
        let result = if let Some((_, len)) = self.next_ident_char(true) {
            let start = self.pos;
            self.pos += len;
            while let Some((_, n)) = self.next_ident_char(false) {
                self.pos += n;
            }
            std::str::from_utf8(&self.src[start..self.pos]).ok()
        } else {
            None
        };
        // Restore.
        self.pos = saved_pos;
        self.event_start = saved_event_start;
        self.prev_kind = saved_prev_kind;
        result
    }

    /// Advance to and return the next lexical event.
    fn next_event(&mut self) -> Option<Event> {
        // If we're currently inside the text portion of a template
        // literal, the event sequence is: a `StringLike` for each run
        // of template text, then (on hitting `${`) we push a
        // `TemplateExpr` mode and fall through to normal code scanning
        // for the interior. The matching `}` is handled below in the
        // punctuation dispatch: when brace_depth hits zero we pop the
        // mode and resume template-text scanning.
        if matches!(self.modes.last(), Some(Mode::TemplateText)) {
            return self.next_event_in_template_text();
        }

        while self.pos < self.src.len() {
            let c = self.src[self.pos];
            self.event_start = self.pos;

            // Whitespace — skip entirely; no event, no prev_kind update.
            if matches!(c, b' ' | b'\t' | b'\r' | b'\n') {
                self.pos += 1;
                continue;
            }

            // Comments.
            if c == b'/' && self.peek(1) == Some(b'/') {
                let start = self.pos;
                self.pos += 2;
                while let Some(b) = self.peek(0) {
                    self.pos += 1;
                    if b == b'\n' {
                        break;
                    }
                }
                return Some(Event::Comment {
                    start,
                    end: self.pos,
                });
            }
            if c == b'/' && self.peek(1) == Some(b'*') {
                let start = self.pos;
                self.pos += 2;
                while let Some(b) = self.peek(0) {
                    if b == b'*' && self.peek(1) == Some(b'/') {
                        self.pos += 2;
                        break;
                    }
                    self.pos += 1;
                }
                return Some(Event::Comment {
                    start,
                    end: self.pos,
                });
            }

            // Regex literal — disambiguated from `/` division by the
            // previous token's kind.
            if c == b'/' && self.prev_kind == PrevKind::ExprStart {
                let start = self.pos;
                self.pos += 1; // opening `/`
                let mut in_class = false;
                while let Some(b) = self.peek(0) {
                    match b {
                        b'\\' => {
                            // Escape: consume the backslash and the
                            // next byte (if any) verbatim.
                            self.pos += 1;
                            if self.pos < self.src.len() {
                                self.pos += 1;
                            }
                        }
                        b'[' => {
                            in_class = true;
                            self.pos += 1;
                        }
                        b']' if in_class => {
                            in_class = false;
                            self.pos += 1;
                        }
                        b'/' if !in_class => {
                            self.pos += 1;
                            // Flags: consume trailing ident-continue
                            // chars (ECMAScript regex flags are all
                            // ASCII — `g`, `i`, `m`, `s`, `u`, `y` —
                            // but we use the Unicode-aware helper
                            // anyway so the cursor never contradicts
                            // itself across code paths).
                            while let Some((_, n)) = self.next_ident_char(false) {
                                self.pos += n;
                            }
                            break;
                        }
                        b'\n' | b'\r' => {
                            // Unterminated regex — bail out treating
                            // the rest of the line as the literal.
                            break;
                        }
                        _ => self.pos += 1,
                    }
                }
                self.prev_kind = PrevKind::ExprEnd;
                return Some(Event::Regex {
                    start,
                    end: self.pos,
                });
            }

            // String-like literals.
            if c == b'\'' || c == b'"' {
                let start = self.pos;
                let quote = c;
                self.pos += 1;
                while let Some(b) = self.peek(0) {
                    match b {
                        b'\\' => {
                            self.pos += 1;
                            if self.pos < self.src.len() {
                                self.pos += 1;
                            }
                        }
                        _ if b == quote => {
                            self.pos += 1;
                            break;
                        }
                        _ => self.pos += 1,
                    }
                }
                self.prev_kind = PrevKind::ExprEnd;
                return Some(Event::StringLike {
                    start,
                    end: self.pos,
                });
            }
            if c == b'`' {
                // Opening backtick of a template literal. Emit the
                // backtick itself as a Punct (so the rewriter copies
                // it verbatim), then push `TemplateText` so subsequent
                // `next_event` calls scan the template body until
                // they hit `${` or the matching closing backtick.
                self.pos += 1;
                self.modes.push(Mode::TemplateText);
                self.prev_kind = PrevKind::ExprEnd;
                return Some(Event::Punct);
            }

            // Identifiers / keywords (Unicode-aware).
            if let Some((_, first_len)) = self.next_ident_char(true) {
                let start = self.pos;
                self.pos += first_len;
                while let Some((_, n)) = self.next_ident_char(false) {
                    self.pos += n;
                }
                let text = &self.src[start..self.pos];
                // Classify keywords that influence our state machine:
                // the three declaration kinds (for name collection)
                // and the small set of expression-prefix keywords (for
                // regex disambiguation).
                let kw = match text {
                    b"const" => Some(KeywordKind::Const),
                    b"let" => Some(KeywordKind::Let),
                    b"var" => Some(KeywordKind::Var),
                    b"return" | b"typeof" | b"delete" | b"void" | b"throw" | b"new"
                    | b"instanceof" | b"in" | b"of" | b"yield" | b"await" | b"case" => {
                        Some(KeywordKind::ExprPrefix)
                    }
                    _ => None,
                };
                if let Some(kw) = kw {
                    // After `return` etc., `/` starts a regex. After
                    // `const`/`let`/`var`, the next identifier is a
                    // binding name (still "expression start" for the
                    // following ident — matters only for the unusual
                    // `const x = /re/;` case, which we handle
                    // anyway).
                    self.prev_kind = PrevKind::ExprStart;
                    return Some(Event::Keyword(kw));
                }
                self.prev_kind = PrevKind::ExprEnd;
                return Some(Event::Ident {
                    start,
                    end: self.pos,
                });
            }

            // Digits — treat whole numeric literal as an "ExprEnd"
            // token. We don't emit a dedicated Numeric event because
            // the rewrite pass doesn't care about numbers specifically
            // — it only needs to know they terminate an expression so
            // a following `/` is division.
            if c.is_ascii_digit() {
                let start = self.pos;
                while let Some(b) = self.peek(0) {
                    if b.is_ascii_alphanumeric() || b == b'.' || b == b'_' {
                        self.pos += 1;
                    } else {
                        break;
                    }
                }
                self.prev_kind = PrevKind::ExprEnd;
                // Emit as an Ident event — the rewrite pass tests
                // membership in `declared`, and numeric literals
                // never collide with declared identifiers.
                return Some(Event::Ident {
                    start,
                    end: self.pos,
                });
            }

            // Single-byte punctuation. Update `prev_kind` based on
            // which byte it is, and — when inside a template-literal
            // expression slot — keep the `{`/`}` brace depth in sync
            // so the closing `}` that matches the opening `${` pops
            // us back to template-text mode.
            self.pos += 1;
            match c {
                b')' | b']' => self.prev_kind = PrevKind::ExprEnd,
                b'{' => {
                    if let Some(Mode::TemplateExpr { brace_depth }) = self.modes.last_mut() {
                        *brace_depth += 1;
                    }
                    self.prev_kind = PrevKind::ExprStart;
                }
                b'}' => {
                    if let Some(Mode::TemplateExpr { brace_depth }) = self.modes.last_mut() {
                        if *brace_depth == 0 {
                            // This `}` matches the opening `${` — pop
                            // the expression mode and resume template
                            // text scanning. The next call to
                            // next_event will land in
                            // next_event_in_template_text.
                            self.modes.pop();
                        } else {
                            *brace_depth -= 1;
                        }
                    }
                    self.prev_kind = PrevKind::ExprEnd;
                }
                _ => self.prev_kind = PrevKind::ExprStart,
            }
            return Some(Event::Punct);
        }
        None
    }

    /// Scan forward inside the text portion of a template literal.
    /// Produces one `StringLike` event per text chunk, stopping at
    /// either the closing backtick (pop the mode and emit a `Punct`
    /// event for the backtick on the NEXT call, once the text chunk
    /// has been yielded) or `${` (pop-and-push: switch to
    /// `TemplateExpr` mode so the next calls yield code events).
    /// Empty text chunks (e.g. `${x}${y}` with nothing between the
    /// slots) still produce a zero-length `StringLike` so the
    /// rewriter's pass-through loop stays synchronized.
    fn next_event_in_template_text(&mut self) -> Option<Event> {
        self.event_start = self.pos;
        let start = self.pos;
        while self.pos < self.src.len() {
            let b = self.src[self.pos];
            match b {
                b'\\' => {
                    self.pos += 1;
                    if self.pos < self.src.len() {
                        self.pos += 1;
                    }
                }
                b'`' => {
                    // Closing backtick. Emit everything up to (but
                    // not including) the backtick as a StringLike,
                    // then pop template mode so the next call
                    // yields the backtick as a normal Punct event.
                    let end = self.pos;
                    self.modes.pop();
                    return Some(Event::StringLike { start, end });
                }
                b'$' if self.peek(1) == Some(b'{') => {
                    // Entering an expression slot. Emit the text up
                    // to the `$`, switch mode, then on the next call
                    // we'll yield Punct(`$`) from the main loop
                    // (which will also handle `{` and push brace
                    // depth).
                    let end = self.pos;
                    // Replace the TemplateText at the top with
                    // TemplateExpr{0} so the subsequent `{` starts
                    // the depth counter.
                    let top = self.modes.last_mut().expect("mode stack invariant");
                    *top = Mode::TemplateExpr { brace_depth: 0 };
                    self.prev_kind = PrevKind::ExprStart;
                    return Some(Event::StringLike { start, end });
                }
                _ => self.pos += 1,
            }
        }
        // Source ended mid-template — emit whatever we accumulated
        // and pop the mode so the caller stops looping.
        let end = self.pos;
        self.modes.pop();
        if end > start {
            Some(Event::StringLike { start, end })
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Identifier classification
// ---------------------------------------------------------------------------
//
// Unicode-aware: we use `unicode_ident::is_xid_start` /
// `is_xid_continue` for non-ASCII characters so macro bodies that
// declare e.g. `const __café = 1` or `const __中文 = 1` are
// classified correctly. For ASCII, we keep the fast byte-level
// checks — ECMAScript's `$` and `_` are allowed as identifier start
// and continue characters, which `unicode_ident` doesn't grant (it
// follows UAX #31 strictly).
//
// The fast-path vs. slow-path split lives on `LexCursor::next_ident_char`
// below so callers don't need to know whether they're looking at an
// ASCII or multi-byte character. All identifier-consuming loops in
// `LexCursor::next_event` and `LexCursor::peek_next_code_ident`
// advance via `next_ident_char` and `.1` (the byte length of the
// UTF-8 encoding), which is always 1 for ASCII and 2–4 for non-ASCII.

impl<'a> LexCursor<'a> {
    /// Peek the identifier character at `self.pos`, if any. Returns
    /// `(char, byte_length)` when the byte at `pos` (or the char
    /// starting at `pos`) is a valid ECMAScript identifier
    /// character. `is_start: true` applies the stricter `IdStart`
    /// rules (no digits); `is_start: false` applies `IdContinue`.
    ///
    /// For ASCII bytes this is a direct predicate check. For
    /// non-ASCII leading bytes we decode one `char` from the
    /// underlying slice — `LexCursor::new` took a `&str`, so the
    /// buffer is valid UTF-8 and decoding is always safe at a byte
    /// boundary that marks the start of a char.
    fn next_ident_char(&self, is_start: bool) -> Option<(char, usize)> {
        if self.pos >= self.src.len() {
            return None;
        }
        let b = self.src[self.pos];
        if b.is_ascii() {
            let ok = if is_start {
                b.is_ascii_alphabetic() || b == b'_' || b == b'$'
            } else {
                b.is_ascii_alphanumeric() || b == b'_' || b == b'$'
            };
            return if ok { Some((b as char, 1)) } else { None };
        }
        // Non-ASCII: decode exactly one char starting at `self.pos`.
        // The UTF-8 char length is determined by the leading byte,
        // not by validating a fixed 4-byte window — because if the
        // next char after this one is also multi-byte, a 4-byte
        // window might include half of it and `from_utf8` would
        // reject the whole slice as invalid.
        let char_len = if b < 0xc0 {
            // Continuation byte at self.pos — caller is mid-char,
            // which means the cursor got out of sync with UTF-8
            // boundaries. Defensive: stop the ident here.
            return None;
        } else if b < 0xe0 {
            2
        } else if b < 0xf0 {
            3
        } else {
            4
        };
        if self.pos + char_len > self.src.len() {
            return None;
        }
        let s = std::str::from_utf8(&self.src[self.pos..self.pos + char_len]).ok()?;
        let ch = s.chars().next()?;
        let ok = if is_start {
            unicode_ident::is_xid_start(ch)
        } else {
            unicode_ident::is_xid_continue(ch)
        };
        if ok { Some((ch, ch.len_utf8())) } else { None }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn rewrite(source: &str, declared: &[&str]) -> String {
        let set: HashSet<String> = declared.iter().map(|s| s.to_string()).collect();
        rewrite_identifiers(source, &set, "$7")
    }

    #[test]
    fn collects_simple_const_declaration() {
        let set = collect_declared_underscore_names("const __v = 1;");
        assert!(set.contains("__v"), "got: {:?}", set);
    }

    #[test]
    fn collects_let_and_var() {
        let set = collect_declared_underscore_names("let __a = 0; var __b;");
        assert!(set.contains("__a"));
        assert!(set.contains("__b"));
    }

    #[test]
    fn ignores_non_underscore_declarations() {
        let set = collect_declared_underscore_names("const v = 1; let x;");
        assert!(set.is_empty(), "got: {:?}", set);
    }

    #[test]
    fn collects_across_newline_between_keyword_and_name() {
        // Regression for the "const\n__x" edge case the old scanner missed.
        let set = collect_declared_underscore_names("const\n    __v = 1;");
        assert!(set.contains("__v"), "got: {:?}", set);
    }

    #[test]
    fn rewrites_simple_const_and_uses() {
        let out = rewrite("const __v = 1; __v + 2", &["__v"]);
        assert!(out.contains("__v$7"), "got: {}", out);
        // The original bare `__v` must not appear anymore (well, it
        // does appear as a prefix of the suffixed form, but never as
        // a standalone token).
        assert!(
            !out.split_whitespace()
                .any(|t| t == "__v" || t == "__v;" || t == "__v,"),
            "unrenamed __v in: {}",
            out
        );
    }

    #[test]
    fn string_literal_interior_is_not_rewritten() {
        let declared = &["__v"];
        // The string contains `__v` but we have no real declaration —
        // the old scanner would flag it via the `const __v` byte match
        // inside the string. The new scanner treats the string as
        // opaque text.
        let declared_set: HashSet<String> =
            collect_declared_underscore_names(r#"const s = "const __v = inside";"#);
        assert!(
            declared_set.is_empty(),
            "string-interior `const __v` should not register as a declaration: {:?}",
            declared_set
        );
        // Even if `__v` WERE declared elsewhere, the rewrite must not
        // touch the literal text inside the string.
        let out = rewrite(r#"const __v = 1; const s = "__v inside";"#, declared);
        assert!(
            out.contains("__v$7 = 1"),
            "real declaration should be renamed: {}",
            out
        );
        assert!(
            out.contains("\"__v inside\""),
            "string literal must be unchanged: {}",
            out
        );
    }

    #[test]
    fn single_quoted_string_interior_is_not_rewritten() {
        let out = rewrite("const __v = 1; const s = '__v inside';", &["__v"]);
        assert!(out.contains("'__v inside'"), "got: {}", out);
    }

    #[test]
    fn line_comment_interior_is_not_rewritten() {
        let out = rewrite("const __v = 1; // __v used here\n__v + 2", &["__v"]);
        assert!(out.contains("// __v used here"), "got: {}", out);
        // The real use after the comment should still be renamed.
        assert!(out.contains("__v$7 + 2"), "got: {}", out);
    }

    #[test]
    fn block_comment_interior_is_not_rewritten() {
        let out = rewrite("const __v = 1; /* see __v above */ __v + 2", &["__v"]);
        assert!(out.contains("/* see __v above */"), "got: {}", out);
        assert!(out.contains("__v$7 + 2"), "got: {}", out);
    }

    #[test]
    fn regex_literal_interior_is_not_rewritten() {
        // The `/` after `=` is in expression position, so it's a
        // regex — its contents should be opaque even if they mention
        // `__v`.
        let out = rewrite("const __v = 1; const re = /__v test/g;", &["__v"]);
        assert!(
            out.contains("/__v test/g"),
            "regex should be preserved: {}",
            out
        );
        // The real declaration is still renamed.
        assert!(out.contains("__v$7 = 1"), "got: {}", out);
    }

    #[test]
    fn division_after_ident_is_not_misread_as_regex() {
        // `a / __v / b` — the two `/` are divisions, not regex
        // delimiters. `__v` in the middle is a real use and must be
        // rewritten.
        let out = rewrite("const __v = 1; const x = a / __v / b;", &["__v"]);
        assert!(
            out.contains("a / __v$7 / b"),
            "expected division, got: {}",
            out
        );
    }

    #[test]
    fn template_literal_text_is_not_rewritten() {
        // `__v` in the text portion of a template literal is literal
        // text, not a code-context identifier.
        let out = rewrite("const __v = 1; const s = `literal __v here`;", &["__v"]);
        assert!(out.contains("`literal __v here`"), "got: {}", out);
        assert!(out.contains("__v$7 = 1"), "got: {}", out);
    }

    #[test]
    fn template_expression_slot_is_rewritten() {
        // `${__v}` — the `__v` inside the expression slot IS code
        // context and must be renamed.
        let out = rewrite("const __v = 1; const s = `${__v}`;", &["__v"]);
        assert!(out.contains("${__v$7}"), "got: {}", out);
    }

    #[test]
    fn nested_template_literals_work() {
        // `` `outer ${`inner ${__v}`}` `` — the innermost `${__v}` is
        // code context through two layers of template.
        let out = rewrite(
            "const __v = 1; const s = `outer ${`inner ${__v}`}`;",
            &["__v"],
        );
        assert!(out.contains("${__v$7}"), "got: {}", out);
    }

    #[test]
    fn empty_declared_set_is_identity() {
        let source = "const v = 1; const s = \"__v\";";
        let out = rewrite(source, &[]);
        assert_eq!(out, source);
    }

    #[test]
    fn multiple_declared_names_rewrite_independently() {
        let out = rewrite("const __a = 1; const __b = 2; __a + __b", &["__a", "__b"]);
        assert!(out.contains("__a$7 = 1"), "got: {}", out);
        assert!(out.contains("__b$7 = 2"), "got: {}", out);
        assert!(out.contains("__a$7 + __b$7"), "got: {}", out);
    }

    #[test]
    fn collection_and_rewrite_round_trip_on_macro_shape() {
        // This is the shape a `vec!`-style macro produces.
        let body = "{ const __v = []; __v.push(1); __v.push(2); __v }";
        let declared = collect_declared_underscore_names(body);
        assert!(declared.contains("__v"), "got: {:?}", declared);
        let out = rewrite_identifiers(body, &declared, "$3");
        assert!(out.contains("const __v$3 = []"), "got: {}", out);
        assert!(out.contains("__v$3.push(1)"), "got: {}", out);
        assert!(out.contains("__v$3.push(2)"), "got: {}", out);
    }

    #[test]
    fn declaration_inside_string_does_not_register() {
        // The old byte-scanner false positive.
        let declared = collect_declared_underscore_names(r#"const s = "const __fake = 1";"#);
        assert!(declared.is_empty(), "got: {:?}", declared);
    }

    #[test]
    fn declaration_inside_comment_does_not_register() {
        let declared = collect_declared_underscore_names("// const __fake = 1\nconst __real = 2;");
        assert!(declared.contains("__real"), "got: {:?}", declared);
        assert!(!declared.contains("__fake"), "got: {:?}", declared);
    }

    // ---------------------------------------------------------------
    // PR 13 — Unicode-aware identifier classification
    // ---------------------------------------------------------------

    #[test]
    fn collects_latin_accented_identifier() {
        // `__café` contains a multi-byte `é` (UTF-8: 0xc3 0xa9). The
        // old ASCII classifier stopped at the `é` byte and registered
        // `__caf` as the binding name, then corrupted occurrences of
        // `__café` in the output. The Unicode-aware cursor should
        // recognise the whole identifier.
        let declared = collect_declared_underscore_names("const __café = 1;");
        assert!(
            declared.contains("__café"),
            "expected `__café` to be collected, got: {:?}",
            declared
        );
        assert!(
            !declared.contains("__caf"),
            "cursor should not have truncated at the accented char: {:?}",
            declared
        );
    }

    #[test]
    fn collects_cjk_identifier() {
        // `__中文` — two CJK characters, each 3 UTF-8 bytes.
        let declared = collect_declared_underscore_names("const __中文 = 1;");
        assert!(
            declared.contains("__中文"),
            "expected `__中文` to be collected, got: {:?}",
            declared
        );
    }

    #[test]
    fn rewrites_accented_identifier() {
        let out = rewrite("const __café = 1; __café + 2", &["__café"]);
        assert!(
            out.contains("__café$7"),
            "expected suffix-renamed Unicode ident, got: {}",
            out
        );
        assert!(
            out.contains("__café$7 + 2"),
            "both occurrences should be rewritten, got: {}",
            out
        );
    }

    #[test]
    fn rewrites_cjk_identifier() {
        let out = rewrite("const __中文 = 1; __中文.test()", &["__中文"]);
        assert!(
            out.contains("__中文$7"),
            "expected suffix-renamed CJK ident, got: {}",
            out
        );
        assert!(
            out.contains("__中文$7.test()"),
            "second occurrence should also be rewritten: {}",
            out
        );
    }

    #[test]
    fn non_ascii_inside_string_literal_stays_literal() {
        // `café` inside a string must NOT trigger anything — we're
        // verifying the string-skip path still works correctly when
        // it contains multi-byte chars.
        let out = rewrite(r#"const __v = 1; const s = "café"; __v"#, &["__v"]);
        assert!(
            out.contains("\"café\""),
            "string literal contents should be preserved: {}",
            out
        );
        assert!(
            out.contains("__v$7"),
            "non-string `__v` use should still be rewritten: {}",
            out
        );
    }

    #[test]
    fn emoji_is_not_a_valid_identifier_character() {
        // Emoji (👋, `U+1F44B`) is not in UAX #31's `IdContinue` set,
        // so `__👋` should NOT be collected as a declaration. The
        // cursor must stop after `__` and observe a non-ident char.
        let declared = collect_declared_underscore_names("const __👋 = 1;");
        assert!(
            !declared.iter().any(|n| n.contains('👋')),
            "emoji should never appear in a collected identifier: {:?}",
            declared
        );
    }
}
