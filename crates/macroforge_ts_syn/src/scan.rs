//! Scanning TypeScript text a letter at a time, telling its string literals
//! and brackets from the rest.

/// Where a scan stands against the string literals of the text: a quote opens
/// one, a backslash escapes the letter after it, and the opening quote closes
/// it. A `\\` before a quote escapes the backslash, not the quote.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Quoting {
    quote: Option<char>,
    escaped: bool,
}

impl Quoting {
    /// Takes the next letter of the text, and says whether it belongs to a
    /// string literal, the quotes around it among it.
    pub fn feed(&mut self, letter: char) -> bool {
        match self.quote {
            Some(quote) => {
                if self.escaped {
                    self.escaped = false;
                } else if letter == '\\' {
                    self.escaped = true;
                } else if letter == quote {
                    self.quote = None;
                }
                true
            }
            None if matches!(letter, '"' | '\'' | '`') => {
                self.quote = Some(letter);
                true
            }
            None => false,
        }
    }

    /// Whether the scan stands inside a string literal.
    pub fn in_string(&self) -> bool {
        self.quote.is_some()
    }

    /// Ends a line: a `"` or `'` string does not run past one.
    fn end_line(&mut self) {
        if self.quote != Some('`') {
            self.quote = None;
        }
        self.escaped = false;
    }
}

/// Where a scan of a directive's arguments stands: how many brackets of each
/// kind are open, and the string literal it is inside. A bracket or a comma
/// in a string is text, as the `(` of `@default(":(")` is.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Nesting {
    parens: u32,
    braces: u32,
    brackets: u32,
    quoting: Quoting,
}

impl Nesting {
    /// Takes the next character of the text scanned.
    pub fn feed(&mut self, letter: char) {
        if self.quoting.feed(letter) {
            return;
        }
        match letter {
            '(' => self.parens = self.parens.saturating_add(1),
            ')' => self.parens = self.parens.saturating_sub(1),
            '{' => self.braces = self.braces.saturating_add(1),
            '}' => self.braces = self.braces.saturating_sub(1),
            '[' => self.brackets = self.brackets.saturating_add(1),
            ']' => self.brackets = self.brackets.saturating_sub(1),
            _ => {}
        }
    }

    /// Takes a whole line. A `"` or `'` string does not run past its line.
    pub(crate) fn feed_line(&mut self, line: &str) {
        line.chars().for_each(|letter| self.feed(letter));
        self.quoting.end_line();
    }

    /// Whether the scan stands outside every bracket and string.
    pub fn is_top_level(&self) -> bool {
        !self.quoting.in_string() && !self.has_open_bracket()
    }

    pub(crate) fn has_open_bracket(&self) -> bool {
        self.parens > 0 || self.braces > 0 || self.brackets > 0
    }
}

/// A piece of TypeScript text, as [`tokens`] splits it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    Ident(&'a str),
    /// Anything that is not an identifier: punctuation, numbers, whitespace,
    /// whole string literals and the text of a template literal.
    Other(&'a str),
}

impl<'a> Token<'a> {
    /// The text of the token as written.
    pub fn text(&self) -> &'a str {
        match self {
            Token::Ident(text) | Token::Other(text) => text,
        }
    }
}

/// `s` split into identifiers and the text between them. The text of a
/// template literal is text, and the types in its `${}` are read as any
/// others are.
pub fn tokens(s: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    // How many `{` are open in each `${` of a template literal entered.
    let mut substitutions: Vec<usize> = Vec::new();
    let mut in_template_text = false;
    let mut rest = s;
    while let Some(first) = rest.chars().next() {
        let (len, ident) = if std::mem::take(&mut in_template_text) {
            let (len, enters) = template_text_len(rest);
            if enters {
                substitutions.push(0);
            }
            (len, false)
        } else if is_ident_start(first) {
            let len = rest
                .find(|c: char| !is_ident_continue(c))
                .unwrap_or(rest.len());
            (len, true)
        } else if first == '`' {
            let (len, enters) = template_text_len(rest.get(1..).unwrap_or_default());
            if enters {
                substitutions.push(0);
            }
            (len + 1, false)
        } else if matches!(first, '"' | '\'') {
            (string_literal_len(rest).unwrap_or(rest.len()), false)
        } else {
            match (first, substitutions.last_mut()) {
                ('{', Some(open)) => *open += 1,
                ('}', Some(&mut 0)) => {
                    substitutions.pop();
                    in_template_text = true;
                }
                ('}', Some(open)) => *open -= 1,
                _ => {}
            }
            (first.len_utf8(), false)
        };
        let Some((token, tail)) = rest.split_at_checked(len) else {
            break;
        };
        rest = tail;
        tokens.push(if ident {
            Token::Ident(token)
        } else {
            Token::Other(token)
        });
    }
    tokens
}

/// The length of the text of a template literal at the start of `s`, after
/// its opening backtick or a `}` closing one of its substitutions: through
/// the closing backtick, or through the `${` of the next substitution, which
/// the second value says it entered.
fn template_text_len(s: &str) -> (usize, bool) {
    let mut escaped = false;
    let mut letters = s.char_indices().peekable();
    while let Some((index, letter)) = letters.next() {
        match letter {
            _ if escaped => escaped = false,
            '\\' => escaped = true,
            '`' => return (index + 1, false),
            '$' if letters.peek().is_some_and(|(_, next)| *next == '{') => {
                return (index + 2, true);
            }
            _ => {}
        }
    }
    (s.len(), false)
}

/// The length of the string literal `s` opens with, through its closing
/// quote. `None` when it never closes.
pub fn string_literal_len(s: &str) -> Option<usize> {
    let mut quoting = Quoting::default();
    s.char_indices().find_map(|(index, letter)| {
        quoting.feed(letter);
        (!quoting.in_string()).then_some(index + letter.len_utf8())
    })
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_' || c == '$'
}

fn is_ident_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}
