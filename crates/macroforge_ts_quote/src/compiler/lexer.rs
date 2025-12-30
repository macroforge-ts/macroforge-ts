//! Lexer for the template language.
//!
//! The lexer tokenizes template input into a stream of tokens.
//! It handles the hybrid nature of templates: TypeScript code with embedded
//! control flow and interpolations.

use super::syntax::SyntaxKind;

/// Normalizes template spacing that Rust's tokenizer introduces.
///
/// Rust's proc_macro tokenizer adds spaces around punctuation, so:
/// - `@{expr}` becomes `@ { expr }`
/// - `{#if cond}` becomes `{ # if cond }`
///
/// This function collapses these back to the expected format.
pub fn normalize_template(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];

        // `@ {` → `@{` (with optional whitespace)
        if c == '@' && i + 1 < len {
            let mut j = i + 1;
            while j < len && chars[j].is_whitespace() {
                j += 1;
            }
            if j < len && chars[j] == '{' {
                result.push('@');
                result.push('{');
                i = j + 1;
                continue;
            }
        }

        // `{ #` → `{#`, `{ /` → `{/`, `{ :` → `{:`, `{ $` → `{$`, `{ |` → `{|`
        if c == '{' && i + 1 < len {
            let mut j = i + 1;
            while j < len && chars[j].is_whitespace() {
                j += 1;
            }
            if j < len && matches!(chars[j], '#' | '/' | ':' | '$' | '|') {
                result.push('{');
                result.push(chars[j]);
                i = j + 1;
                // Also skip whitespace after the control char
                while i < len && chars[i].is_whitespace() {
                    i += 1;
                }
                continue;
            }
        }

        // `| }` → `|}`
        if c == '|' && i + 1 < len {
            let mut j = i + 1;
            while j < len && chars[j].is_whitespace() {
                j += 1;
            }
            if j < len && chars[j] == '}' {
                result.push('|');
                result.push('}');
                i = j + 1;
                continue;
            }
        }

        result.push(c);
        i += 1;
    }

    result
}

/// A token produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The kind of token.
    pub kind: SyntaxKind,
    /// The text of the token.
    pub text: String,
    /// The byte offset where this token starts.
    pub start: usize,
}

/// Lexer state for context-sensitive tokenization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LexerMode {
    /// Normal template mode - looking for TypeScript and template constructs
    Normal,
    /// Inside a control block `{#...}`, `{/...}`, `{:...}`
    ControlBlock,
    /// Inside a directive `{$...}`
    Directive,
    /// Inside an interpolation `@{...}`
    Interpolation,
    /// Inside an ident block `...`
    IdentBlock,
    /// Inside a string literal
    StringLiteral,
    /// Inside a template literal (backticks)
    TemplateLiteral,
}

/// The lexer for template input.
pub struct Lexer {
    /// The normalized input text.
    input: String,
    /// Current byte position in the input.
    pos: usize,
    /// Stack of lexer modes for nested contexts.
    mode_stack: Vec<LexerMode>,
    /// Brace depth for tracking nested braces in interpolations.
    brace_depth: usize,
}

impl Lexer {
    /// Creates a new lexer for the given input.
    /// The input is normalized to collapse Rust tokenizer spacing.
    pub fn new(input: &str) -> Self {
        Self {
            input: normalize_template(input),
            pos: 0,
            mode_stack: vec![LexerMode::Normal],
            brace_depth: 0,
        }
    }

    /// Returns the current lexer mode.
    fn mode(&self) -> LexerMode {
        *self.mode_stack.last().unwrap_or(&LexerMode::Normal)
    }

    /// Pushes a new lexer mode onto the stack.
    fn push_mode(&mut self, mode: LexerMode) {
        self.mode_stack.push(mode);
    }

    /// Pops the current lexer mode from the stack.
    fn pop_mode(&mut self) {
        if self.mode_stack.len() > 1 {
            self.mode_stack.pop();
        }
    }

    /// Returns the remaining input from the current position.
    fn remaining(&self) -> &str {
        &self.input[self.pos..]
    }

    /// Peeks at the next character without consuming it.
    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    /// Advances the position by n bytes.
    fn advance(&mut self, n: usize) {
        self.pos += n;
    }

    /// Consumes characters while the predicate is true.
    fn consume_while<F: Fn(char) -> bool>(&mut self, pred: F) {
        while let Some(c) = self.peek() {
            if pred(c) {
                self.advance(c.len_utf8());
            } else {
                break;
            }
        }
    }

    /// Tokenizes the entire input.
    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.pos < self.input.len() {
            if let Some(token) = self.next_token() {
                tokens.push(token);
            }
        }
        tokens
    }

    /// Returns the next token, or None if at EOF.
    fn next_token(&mut self) -> Option<Token> {
        if self.pos >= self.input.len() {
            return None;
        }

        let start = self.pos;

        // Check for Rust doc attr first (needs special text handling)
        if let Some((len, content)) = self.try_match_rust_doc_attr() {
            self.advance(len);
            return Some(Token {
                kind: SyntaxKind::RustDocAttr,
                text: content,
                start,
            });
        }

        let kind = match self.mode() {
            LexerMode::Normal => self.lex_normal(),
            LexerMode::ControlBlock => self.lex_control_block(),
            LexerMode::Directive => self.lex_directive(),
            LexerMode::Interpolation => self.lex_interpolation(),
            LexerMode::IdentBlock => self.lex_ident_block(),
            LexerMode::StringLiteral => self.lex_string_literal(),
            LexerMode::TemplateLiteral => self.lex_template_literal(),
        };

        let text = self.input[start..self.pos].to_string();
        Some(Token { kind, text, start })
    }

    /// Lexes in normal mode (TypeScript + template constructs).
    fn lex_normal(&mut self) -> SyntaxKind {
        let remaining = self.remaining();

        // Check for template constructs first
        if remaining.starts_with("@@{") {
            self.advance(3);
            return SyntaxKind::AtAt;
        }

        if remaining.starts_with("@{") {
            self.advance(2);
            self.push_mode(LexerMode::Interpolation);
            self.brace_depth = 1;
            return SyntaxKind::At;
        }

        if remaining.starts_with("{#") {
            self.advance(2);
            self.push_mode(LexerMode::ControlBlock);
            return SyntaxKind::HashOpen;
        }

        if remaining.starts_with("{/") {
            self.advance(2);
            self.push_mode(LexerMode::ControlBlock);
            return SyntaxKind::SlashOpen;
        }

        if remaining.starts_with("{:") {
            self.advance(2);
            self.push_mode(LexerMode::ControlBlock);
            return SyntaxKind::ColonOpen;
        }

        if remaining.starts_with("{$") {
            self.advance(2);
            self.push_mode(LexerMode::Directive);
            return SyntaxKind::DollarOpen;
        }

        if remaining.starts_with("{|") {
            self.advance(2);
            self.push_mode(LexerMode::IdentBlock);
            return SyntaxKind::PipeOpen;
        }

        if remaining.starts_with("{>>") {
            self.advance(3);
            return SyntaxKind::CommentBlockOpen;
        }

        if remaining.starts_with("{>") {
            self.advance(2);
            return SyntaxKind::CommentLineOpen;
        }

        if remaining.starts_with("<<}") {
            self.advance(3);
            return SyntaxKind::CommentBlockClose;
        }

        if remaining.starts_with("<}") {
            self.advance(2);
            return SyntaxKind::CommentLineClose;
        }

        // Check for doc comments
        if remaining.starts_with("///") {
            self.advance(3);
            return SyntaxKind::DocCommentPrefix;
        }

        if remaining.starts_with("/**") {
            self.advance(3);
            return SyntaxKind::JsDocOpen;
        }

        if remaining.starts_with("*/") {
            self.advance(2);
            return SyntaxKind::JsDocClose;
        }

        // Check for TypeScript keywords (for type position detection)
        if let Some(kw) = self.try_lex_ts_keyword() {
            kw
        } else {
            // Single character tokens
            let c = self.peek().unwrap();
            match c {
                '{' => {
                    self.advance(1);
                    SyntaxKind::LBrace
                }
                '}' => {
                    self.advance(1);
                    SyntaxKind::RBrace
                }
                '(' => {
                    self.advance(1);
                    SyntaxKind::LParen
                }
                ')' => {
                    self.advance(1);
                    SyntaxKind::RParen
                }
                '[' => {
                    self.advance(1);
                    SyntaxKind::LBracket
                }
                ']' => {
                    self.advance(1);
                    SyntaxKind::RBracket
                }
                '<' => {
                    self.advance(1);
                    SyntaxKind::Lt
                }
                '>' => {
                    self.advance(1);
                    SyntaxKind::Gt
                }
                ':' => {
                    self.advance(1);
                    SyntaxKind::Colon
                }
                ';' => {
                    self.advance(1);
                    SyntaxKind::Semicolon
                }
                ',' => {
                    self.advance(1);
                    SyntaxKind::Comma
                }
                '=' => {
                    self.advance(1);
                    SyntaxKind::Eq
                }
                '?' => {
                    self.advance(1);
                    SyntaxKind::Question
                }
                '.' => {
                    self.advance(1);
                    SyntaxKind::Dot
                }
                '"' => {
                    self.advance(1);
                    self.push_mode(LexerMode::StringLiteral);
                    SyntaxKind::DoubleQuote
                }
                '\'' => {
                    self.advance(1);
                    SyntaxKind::SingleQuote
                }
                '`' => {
                    self.advance(1);
                    self.push_mode(LexerMode::TemplateLiteral);
                    SyntaxKind::Backtick
                }
                '@' => {
                    // Single @ not followed by { - just text
                    self.advance(1);
                    SyntaxKind::Text
                }
                _ if c.is_whitespace() => {
                    self.consume_while(|c| c.is_whitespace());
                    SyntaxKind::Whitespace
                }
                _ if c.is_alphabetic() || c == '_' => {
                    self.consume_while(|c| c.is_alphanumeric() || c == '_');
                    SyntaxKind::Ident
                }
                _ => {
                    // Consume as text until we hit a special character
                    self.consume_text_until_special();
                    SyntaxKind::Text
                }
            }
        }
    }

    /// Tries to match a Rust doc attribute pattern: `# [ doc = "..." ]`
    /// Returns (total_length, content) if matched, None otherwise.
    fn try_match_rust_doc_attr(&self) -> Option<(usize, String)> {
        let remaining = self.remaining();
        let bytes = remaining.as_bytes();
        let mut i = 0;

        // Match `#`
        if bytes.get(i) != Some(&b'#') {
            return None;
        }
        i += 1;

        // Skip whitespace
        while bytes.get(i).map(|b| b.is_ascii_whitespace()).unwrap_or(false) {
            i += 1;
        }

        // Match `[`
        if bytes.get(i) != Some(&b'[') {
            return None;
        }
        i += 1;

        // Skip whitespace
        while bytes.get(i).map(|b| b.is_ascii_whitespace()).unwrap_or(false) {
            i += 1;
        }

        // Match `doc`
        if !remaining[i..].starts_with("doc") {
            return None;
        }
        i += 3;

        // Skip whitespace
        while bytes.get(i).map(|b| b.is_ascii_whitespace()).unwrap_or(false) {
            i += 1;
        }

        // Match `=`
        if bytes.get(i) != Some(&b'=') {
            return None;
        }
        i += 1;

        // Skip whitespace
        while bytes.get(i).map(|b| b.is_ascii_whitespace()).unwrap_or(false) {
            i += 1;
        }

        // Match `"` and consume string content
        if bytes.get(i) != Some(&b'"') {
            return None;
        }
        i += 1;
        let content_start = i;

        // Consume until closing `"`
        while i < bytes.len() {
            if bytes[i] == b'\\' && i + 1 < bytes.len() {
                i += 2; // Skip escape sequence
            } else if bytes[i] == b'"' {
                break;
            } else {
                i += 1;
            }
        }
        let content_end = i;
        let content = remaining[content_start..content_end].to_string();

        // Skip closing quote
        if bytes.get(i) == Some(&b'"') {
            i += 1;
        }

        // Skip whitespace
        while bytes.get(i).map(|b| b.is_ascii_whitespace()).unwrap_or(false) {
            i += 1;
        }

        // Match `]`
        if bytes.get(i) != Some(&b']') {
            return None;
        }
        i += 1;

        Some((i, content))
    }

    /// Tries to lex a TypeScript keyword.
    fn try_lex_ts_keyword(&mut self) -> Option<SyntaxKind> {
        let remaining = self.remaining();

        // Check each keyword with word boundary
        // Ordered longest-first to avoid prefix matching issues (e.g., "typeof" before "type")
        let keywords = [
            // Declaration keywords
            ("function", SyntaxKind::FunctionKw),
            ("interface", SyntaxKind::InterfaceKw),
            ("implements", SyntaxKind::ImplementsKw),
            ("protected", SyntaxKind::ProtectedKw),
            ("satisfies", SyntaxKind::SatisfiesKw),
            ("abstract", SyntaxKind::AbstractKw),
            ("readonly", SyntaxKind::ReadonlyKw),
            ("private", SyntaxKind::PrivateKw),
            ("declare", SyntaxKind::DeclareKw),
            ("extends", SyntaxKind::ExtendsKw),
            ("finally", SyntaxKind::FinallyKw),
            ("default", SyntaxKind::DefaultKw),
            ("public", SyntaxKind::PublicKw),
            ("static", SyntaxKind::StaticKw),
            ("return", SyntaxKind::ReturnKw),
            ("typeof", SyntaxKind::TypeofKw), // Must come before "type"
            ("export", SyntaxKind::ExportKw),
            ("import", SyntaxKind::ImportKw),
            ("keyof", SyntaxKind::KeyofKw),
            ("infer", SyntaxKind::InferKw),
            ("const", SyntaxKind::ConstKw),
            ("async", SyntaxKind::AsyncKw),
            ("await", SyntaxKind::AwaitKw),
            ("yield", SyntaxKind::YieldKw),
            ("throw", SyntaxKind::ThrowKw),
            ("catch", SyntaxKind::CatchKw),
            ("class", SyntaxKind::ClassKw),
            ("from", SyntaxKind::FromKw),
            ("type", SyntaxKind::TypeKw),
            ("let", SyntaxKind::LetKw),
            ("var", SyntaxKind::VarKw),
            ("new", SyntaxKind::NewKw),
            ("try", SyntaxKind::TryKw),
            ("get", SyntaxKind::GetKw),
            ("set", SyntaxKind::SetKw),
            ("as", SyntaxKind::AsKw),
            ("is", SyntaxKind::IsKw),
            ("of", SyntaxKind::OfKw),
        ];

        for (kw, kind) in keywords {
            if remaining.starts_with(kw) {
                // Check for word boundary
                let next_char = remaining.chars().nth(kw.len());
                if next_char
                    .map(|c| !c.is_alphanumeric() && c != '_')
                    .unwrap_or(true)
                {
                    self.advance(kw.len());
                    return Some(kind);
                }
            }
        }

        None
    }

    /// Consumes text until a special template character is found.
    fn consume_text_until_special(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                '@' | '{' | '}' | '"' | '\'' | '`' | ':' | ';' | '(' | ')' | '[' | ']' | '<'
                | '>' | ',' | '=' | '?' | '.' => break,
                _ if c.is_whitespace() => break,
                _ if c.is_alphabetic() || c == '_' => break,
                _ => self.advance(c.len_utf8()),
            }
        }
    }

    /// Lexes inside a control block `{#...}`, `{/...}`, or `{:...}`.
    fn lex_control_block(&mut self) -> SyntaxKind {
        let remaining = self.remaining();

        // Skip whitespace
        if let Some(c) = self.peek()
            && c.is_whitespace()
        {
            self.consume_while(|c| c.is_whitespace());
            return SyntaxKind::Whitespace;
        }

        // Check for closing brace
        if remaining.starts_with("}") {
            self.advance(1);
            self.pop_mode();
            return SyntaxKind::RBrace;
        }

        // Check for control keywords
        let keywords = [
            ("if", SyntaxKind::IfKw),
            ("else", SyntaxKind::ElseKw),
            ("for", SyntaxKind::ForKw),
            ("while", SyntaxKind::WhileKw),
            ("match", SyntaxKind::MatchKw),
            ("case", SyntaxKind::CaseKw),
            ("let", SyntaxKind::LetKw),
            ("in", SyntaxKind::InKw),
        ];

        for (kw, kind) in keywords {
            if remaining.starts_with(kw) {
                let next_char = remaining.chars().nth(kw.len());
                if next_char
                    .map(|c| !c.is_alphanumeric() && c != '_')
                    .unwrap_or(true)
                {
                    self.advance(kw.len());
                    return kind;
                }
            }
        }

        // Tokenize granularly so we can detect keywords like "in"
        // Only consume one "unit" at a time (identifier, operator, etc.)
        if let Some(c) = self.peek() {
            if c.is_alphabetic() || c == '_' {
                // Identifier - consume just this one identifier
                self.consume_while(|c| c.is_alphanumeric() || c == '_');
                SyntaxKind::Ident
            } else {
                // Single character operators/punctuation
                match c {
                    '(' | ')' | '[' | ']' | '{' | ',' | '.' | ':' | ';' | '+' | '-' | '*' | '/'
                    | '%' | '&' | '|' | '^' | '!' | '=' | '<' | '>' | '?' | '@' | '#' | '$'
                    | '~' => {
                        self.advance(1);
                        SyntaxKind::Text
                    }
                    '"' => {
                        // String literal - consume the whole thing
                        self.advance(1);
                        while let Some(c) = self.peek() {
                            if c == '\\' {
                                self.advance(2);
                            } else if c == '"' {
                                self.advance(1);
                                break;
                            } else {
                                self.advance(c.len_utf8());
                            }
                        }
                        SyntaxKind::Text
                    }
                    '\'' => {
                        // Char literal
                        self.advance(1);
                        while let Some(c) = self.peek() {
                            if c == '\\' {
                                self.advance(2);
                            } else if c == '\'' {
                                self.advance(1);
                                break;
                            } else {
                                self.advance(c.len_utf8());
                            }
                        }
                        SyntaxKind::Text
                    }
                    _ => {
                        // Numbers or other characters
                        if c.is_numeric() {
                            self.consume_while(|c| c.is_alphanumeric() || c == '_' || c == '.');
                        } else {
                            self.advance(c.len_utf8());
                        }
                        SyntaxKind::Text
                    }
                }
            }
        } else {
            SyntaxKind::Error
        }
    }

    /// Lexes inside a directive `{$...}`.
    fn lex_directive(&mut self) -> SyntaxKind {
        // Skip whitespace
        if let Some(c) = self.peek()
            && c.is_whitespace()
        {
            self.consume_while(|c| c.is_whitespace());
            return SyntaxKind::Whitespace;
        }

        // Check for closing brace
        if self.remaining().starts_with("}") {
            self.advance(1);
            self.pop_mode();
            return SyntaxKind::RBrace;
        }

        // Check for directive keywords
        let keywords = [
            ("let", SyntaxKind::LetKw),
            ("mut", SyntaxKind::MutKw),
            ("do", SyntaxKind::DoKw),
            ("typescript", SyntaxKind::TypeScriptKw),
        ];

        for (kw, kind) in keywords {
            if self.remaining().starts_with(kw) {
                let next_char = self.remaining().chars().nth(kw.len());
                if next_char
                    .map(|c| !c.is_alphanumeric() && c != '_')
                    .unwrap_or(true)
                {
                    self.advance(kw.len());
                    return kind;
                }
            }
        }

        // Everything else is Rust tokens
        self.consume_rust_tokens();
        SyntaxKind::RustTokens
    }

    /// Lexes inside an interpolation `@{...}`.
    fn lex_interpolation(&mut self) -> SyntaxKind {
        // Track brace depth to find the matching close brace
        while let Some(c) = self.peek() {
            match c {
                '{' => {
                    self.brace_depth += 1;
                    self.advance(1);
                }
                '}' => {
                    self.brace_depth -= 1;
                    if self.brace_depth == 0 {
                        self.advance(1);
                        self.pop_mode();
                        return SyntaxKind::RBrace;
                    }
                    self.advance(1);
                }
                _ => {
                    self.advance(c.len_utf8());
                }
            }
        }

        // If we get here, we didn't find the closing brace
        SyntaxKind::RustTokens
    }

    /// Lexes inside an ident block `{|...|}`
    fn lex_ident_block(&mut self) -> SyntaxKind {
        let remaining = self.remaining();

        // Check for closing |}
        if remaining.starts_with("|}") {
            self.advance(2);
            self.pop_mode();
            return SyntaxKind::PipeClose;
        }

        // Check for interpolation
        if remaining.starts_with("@{") {
            self.advance(2);
            self.push_mode(LexerMode::Interpolation);
            self.brace_depth = 1;
            return SyntaxKind::At;
        }

        // Consume text until @{ or |}
        let start = self.pos;
        while self.pos < self.input.len() {
            let r = self.remaining();
            if r.starts_with("|}") || r.starts_with("@{") {
                break;
            }
            if let Some(c) = self.peek() {
                self.advance(c.len_utf8());
            }
        }

        if self.pos > start {
            SyntaxKind::Text
        } else {
            SyntaxKind::Error
        }
    }

    /// Lexes inside a string literal.
    fn lex_string_literal(&mut self) -> SyntaxKind {
        let remaining = self.remaining();

        // Check for closing quote
        if remaining.starts_with("\"") {
            self.advance(1);
            self.pop_mode();
            return SyntaxKind::DoubleQuote;
        }

        // Check for interpolation
        if remaining.starts_with("@{") {
            self.advance(2);
            self.push_mode(LexerMode::Interpolation);
            self.brace_depth = 1;
            return SyntaxKind::At;
        }

        // Check for escape
        if remaining.starts_with("\\") && remaining.len() >= 2 {
            self.advance(2);
            return SyntaxKind::Text;
        }

        // Consume text until @{, ", or escape
        let start = self.pos;
        while self.pos < self.input.len() {
            let r = self.remaining();
            if r.starts_with("\"") || r.starts_with("@{") || r.starts_with("\\") {
                break;
            }
            if let Some(c) = self.peek() {
                self.advance(c.len_utf8());
            }
        }

        if self.pos > start {
            SyntaxKind::Text
        } else {
            SyntaxKind::Error
        }
    }

    /// Lexes inside a template literal.
    fn lex_template_literal(&mut self) -> SyntaxKind {
        let remaining = self.remaining();

        // Check for closing backtick
        if remaining.starts_with("`") {
            self.advance(1);
            self.pop_mode();
            return SyntaxKind::Backtick;
        }

        // Check for ${...} expression
        if remaining.starts_with("${") {
            self.advance(2);
            self.push_mode(LexerMode::Interpolation);
            self.brace_depth = 1;
            return SyntaxKind::At; // Reuse AT for template expressions
        }

        // Check for @{...} interpolation
        if remaining.starts_with("@{") {
            self.advance(2);
            self.push_mode(LexerMode::Interpolation);
            self.brace_depth = 1;
            return SyntaxKind::At;
        }

        // Check for escape
        if remaining.starts_with("\\") && remaining.len() >= 2 {
            self.advance(2);
            return SyntaxKind::Text;
        }

        // Consume text until ${, @{, `, or escape
        let start = self.pos;
        while self.pos < self.input.len() {
            let r = self.remaining();
            if r.starts_with("`")
                || r.starts_with("${")
                || r.starts_with("@{")
                || r.starts_with("\\")
            {
                break;
            }
            if let Some(c) = self.peek() {
                self.advance(c.len_utf8());
            }
        }

        if self.pos > start {
            SyntaxKind::Text
        } else {
            SyntaxKind::Error
        }
    }

    /// Consumes Rust tokens until a closing brace at depth 0.
    fn consume_rust_tokens(&mut self) {
        let mut depth = 0;
        while let Some(c) = self.peek() {
            match c {
                '{' => {
                    depth += 1;
                    self.advance(1);
                }
                '}' => {
                    if depth == 0 {
                        // Don't consume the closing brace
                        break;
                    }
                    depth -= 1;
                    self.advance(1);
                }
                '"' => {
                    // Skip string literals
                    self.advance(1);
                    while let Some(c) = self.peek() {
                        if c == '\\' {
                            self.advance(2);
                        } else if c == '"' {
                            self.advance(1);
                            break;
                        } else {
                            self.advance(c.len_utf8());
                        }
                    }
                }
                '\'' => {
                    // Skip char literals
                    self.advance(1);
                    while let Some(c) = self.peek() {
                        if c == '\\' {
                            self.advance(2);
                        } else if c == '\'' {
                            self.advance(1);
                            break;
                        } else {
                            self.advance(c.len_utf8());
                        }
                    }
                }
                _ => {
                    self.advance(c.len_utf8());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Vec<(SyntaxKind, String)> {
        let tokens = Lexer::new(input).tokenize();
        tokens.iter().map(|t| (t.kind, t.text.clone())).collect()
    }

    #[test]
    fn test_simple_text() {
        let tokens = lex("hello world");
        assert_eq!(tokens[0].0, SyntaxKind::Ident);
        assert_eq!(tokens[0].1, "hello");
    }

    #[test]
    fn test_normalize_spaced_interpolation() {
        // Rust tokenizer produces `@ { expr }` from `@{expr}`
        // We collapse `@ {` to `@{` but preserve internal whitespace
        let normalized = normalize_template("@ { expr }");
        assert_eq!(normalized, "@{ expr }");
    }

    #[test]
    fn test_normalize_spaced_control_block() {
        let normalized = normalize_template("{ # if cond }");
        assert_eq!(normalized, "{#if cond }");
    }

    #[test]
    fn test_interpolation() {
        let tokens = lex("@{expr}");
        assert_eq!(tokens[0].0, SyntaxKind::At);
        // The lexer consumes interpolation content and closing brace together
        assert_eq!(tokens[1].0, SyntaxKind::RBrace);
        // Verify the content was consumed
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn test_control_block() {
        let tokens = lex("{#if cond}");
        assert_eq!(tokens[0].0, SyntaxKind::HashOpen);
        assert_eq!(tokens[1].0, SyntaxKind::IfKw);
    }

    #[test]
    fn test_type_annotation() {
        let tokens = lex("const x: number");
        // Find the colon
        let colon = tokens.iter().find(|(k, _)| *k == SyntaxKind::Colon);
        assert!(colon.is_some());
    }

    #[test]
    fn test_as_keyword() {
        let tokens = lex("value as Type");
        let as_kw = tokens.iter().find(|(k, _)| *k == SyntaxKind::AsKw);
        assert!(as_kw.is_some());
    }

    #[test]
    fn test_for_block_tokens() {
        let tokens = lex("{#for item in items}");
        // Should have: HASH_OPEN, FOR_KW, WHITESPACE, IDENT(item), WHITESPACE, IN_KW, WHITESPACE, IDENT(items), RBRACE
        let in_kw = tokens.iter().find(|(k, _)| *k == SyntaxKind::InKw);
        assert!(in_kw.is_some(), "IN_KW token not found in: {:?}", tokens);
    }
}
