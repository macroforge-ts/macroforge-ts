//! JSDoc directive parsing utilities.
//!
//! Pure string-parsing functions for locating the JSDoc block that belongs to
//! a declaration and extracting `@name(args)` directives from its body. Shared
//! by lowering and the expansion host.

use std::collections::HashSet;

/// Modifiers that may sit between a declaration's JSDoc and the start of the
/// declaration's own span.
const DECLARATION_MODIFIERS: &[&str] = &["export", "declare", "abstract", "default", "async"];

/// A `/** … */` block, as 0-based byte offsets into the source. `start` is at
/// the opening `/**` and `end` just past the closing `*/`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsDocBlock {
    pub start: usize,
    pub end: usize,
}

impl JsDocBlock {
    /// The text between `/**` and `*/`.
    pub fn body<'source>(&self, source: &'source str) -> &'source str {
        &source[self.start + 3..self.end - 2]
    }
}

/// The JSDoc block that belongs to the declaration starting at `target_start`
/// (0-based): the last one before it, provided only whitespace and
/// declaration modifiers separate the two. A comment that documents an
/// earlier declaration is never returned.
pub fn adjacent_jsdoc(source: &str, target_start: usize) -> Option<JsDocBlock> {
    adjacent_jsdoc_after(source, 0, target_start)
}

/// [`adjacent_jsdoc`] for a declaration whose previous sibling ends at
/// `floor`: the comment cannot start before it, so only the gap between the
/// two is searched.
pub fn adjacent_jsdoc_after(source: &str, floor: usize, target_start: usize) -> Option<JsDocBlock> {
    let search_area = source.get(..target_start)?;
    let floor = floor.min(target_start);
    let close = floor + search_area.get(floor..)?.rfind("*/")?;
    let start = comment_start(search_area, floor, close)?;
    let end = close + 2;
    let only_modifiers_between = search_area[end..]
        .split_whitespace()
        .all(|word| DECLARATION_MODIFIERS.contains(&word));
    only_modifiers_between.then_some(JsDocBlock { start, end })
}

/// Where the block comment closed by the `*/` at `close` opens, at or after
/// `floor`. A comment cannot contain `*/`, so it is the first `/**` after the
/// previous `*/`; the last `/**` before `close` could be text inside the
/// comment, such as an example that itself shows a JSDoc.
fn comment_start(source: &str, floor: usize, close: usize) -> Option<usize> {
    let floor = floor.min(close);
    let previous_close = source[floor..close]
        .rfind("*/")
        .map_or(floor, |at| floor + at + 2);
    source[previous_close..close]
        .find("/**")
        .map(|offset| previous_close + offset)
}

/// Whether a JSDoc body is a `/** import macro … */` directive: the body is
/// the import itself. Its module paths (`"@testground/macro"`) would
/// otherwise read as `@` directives, while prose or an example that merely
/// mentions one is ordinary documentation.
pub fn is_macro_import_comment(body: &str) -> bool {
    body.trim_start()
        .trim_start_matches('*')
        .trim_start()
        .get(..MACRO_IMPORT.len())
        .is_some_and(|head| head.eq_ignore_ascii_case(MACRO_IMPORT))
}

/// What a `/** import macro … */` directive starts with, in any case.
pub const MACRO_IMPORT: &str = "import macro";

/// Whether `source` could hold an `import macro` directive anywhere: a scan
/// cheap enough to run before parsing for one.
pub fn may_have_macro_import(source: &str) -> bool {
    source
        .as_bytes()
        .windows(MACRO_IMPORT.len())
        .any(|window| window.eq_ignore_ascii_case(MACRO_IMPORT.as_bytes()))
}

/// The JSDoc block stacked directly above `block`, separated from it by
/// whitespace alone and starting at or after `floor`.
pub fn stacked_jsdoc_above(source: &str, floor: usize, block: JsDocBlock) -> Option<JsDocBlock> {
    let before = source[..block.start].trim_end();
    let close = before.strip_suffix("*/")?.len();
    if close < floor {
        return None;
    }
    let start = comment_start(before, floor, close)?;
    Some(JsDocBlock {
        start,
        end: close + 2,
    })
}

/// Where a scan of a directive's arguments stands: how many brackets of each
/// kind are open, and the quote of the string literal it is inside. A bracket
/// or a comma in a string is text, as the `(` of `@default(":(")` is.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Nesting {
    parens: u32,
    braces: u32,
    brackets: u32,
    quote: Option<char>,
    escaped: bool,
}

impl Nesting {
    /// Takes the next character of the text scanned.
    pub fn feed(&mut self, letter: char) {
        if let Some(quote) = self.quote {
            if self.escaped {
                self.escaped = false;
            } else if letter == '\\' {
                self.escaped = true;
            } else if letter == quote {
                self.quote = None;
            }
            return;
        }
        match letter {
            '"' | '\'' | '`' => self.quote = Some(letter),
            '(' => self.parens += 1,
            ')' => self.parens = self.parens.saturating_sub(1),
            '{' => self.braces += 1,
            '}' => self.braces = self.braces.saturating_sub(1),
            '[' => self.brackets += 1,
            ']' => self.brackets = self.brackets.saturating_sub(1),
            _ => {}
        }
    }

    /// Takes a whole line. A `"` or `'` string does not run past its line.
    fn feed_line(&mut self, line: &str) {
        line.chars().for_each(|letter| self.feed(letter));
        if self.quote != Some('`') {
            self.quote = None;
        }
        self.escaped = false;
    }

    /// Whether the scan stands outside every bracket and string.
    pub fn is_top_level(&self) -> bool {
        self.quote.is_none() && !self.has_open_bracket()
    }

    /// Whether the scan stands inside a string literal.
    pub fn in_string(&self) -> bool {
        self.quote.is_some()
    }

    fn has_open_bracket(&self) -> bool {
        self.parens > 0 || self.braces > 0 || self.brackets > 0
    }
}

/// Parse the macro directives of a JSDoc comment body as `(name, args)` pairs.
/// A directive opens a line, several may follow one another on it
/// (`@derive(X) @default(Y)`), parentheses are optional (`@default`) and
/// arguments may span lines. Lines inside a ``` fence are skipped.
pub fn parse_all_macro_directives(
    comment_body: &str,
    valid_annotations: Option<&HashSet<String>>,
) -> Vec<(String, String)> {
    read_comment_directives(comment_body, valid_annotations).0
}

/// The names of the directives in a JSDoc comment body whose argument list
/// never closes, as `@default(5` and `@endec({ rename: "x"`. Such a text is
/// no directive, and [`parse_all_macro_directives`] leaves it out.
pub fn unclosed_directives(
    comment_body: &str,
    valid_annotations: Option<&HashSet<String>>,
) -> Vec<String> {
    read_comment_directives(comment_body, valid_annotations).1
}

/// The directives of a comment body, and the names of those whose argument
/// list never closes.
fn read_comment_directives(
    comment_body: &str,
    valid_annotations: Option<&HashSet<String>>,
) -> (Vec<(String, String)>, Vec<String>) {
    let mut results = Vec::new();
    let mut unclosed = Vec::new();

    let lines: Vec<&str> = comment_body
        .lines()
        .map(|line| line.trim().trim_start_matches('*').trim())
        .filter(|line| !line.is_empty())
        .collect();

    let mut accumulated = String::new();
    let mut nesting = Nesting::default();

    let mut in_fence = false;
    for line in &lines {
        let in_continuation = nesting.has_open_bracket();

        // A fenced block is example code; its `@` lines are not directives.
        if !in_continuation && line.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }

        if in_continuation {
            accumulated.push(' ');
            accumulated.push_str(line);
            nesting.feed_line(line);
            if !nesting.has_open_bracket() {
                read_directives(&accumulated, valid_annotations, &mut results, &mut unclosed);
                accumulated.clear();
                nesting = Nesting::default();
            }
            continue;
        }

        if !line.starts_with('@') {
            continue;
        }

        nesting.feed_line(line);
        if nesting.has_open_bracket() {
            accumulated = line.to_string();
            continue;
        }

        nesting = Nesting::default();
        read_directives(line, valid_annotations, &mut results, &mut unclosed);
    }

    if !accumulated.is_empty() {
        read_directives(&accumulated, valid_annotations, &mut results, &mut unclosed);
    }

    (results, unclosed)
}

/// Parse the directives a text fragment opens with: `@name` or `@name(args)`,
/// one after another. The run ends at the first text that is not a directive,
/// so a tag mentioned in prose (`@returns the enclosing @derive`) is not one.
pub fn parse_directives_from_text(
    text: &str,
    valid_annotations: Option<&HashSet<String>>,
    results: &mut Vec<(String, String)>,
) {
    read_directives(text, valid_annotations, results, &mut Vec::new());
}

/// Reads the directives `text` opens with into `results`, and the name of
/// one whose argument list never closes, which ends the run, into `unclosed`.
fn read_directives(
    text: &str,
    valid_annotations: Option<&HashSet<String>>,
    results: &mut Vec<(String, String)>,
    unclosed: &mut Vec<String>,
) {
    let is_valid = |name: &str| {
        valid_annotations.is_none_or(|valid| valid.contains(&name.to_ascii_lowercase()))
    };
    let mut remaining = text.trim_start();
    while let Some(after_at) = remaining.strip_prefix('@') {
        let name_end = after_at
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(after_at.len());
        if name_end == 0 {
            return;
        }
        let name = &after_at[..name_end];
        let after_name = &after_at[name_end..];

        let trimmed_after_name = after_name.trim_start();
        let (args, rest) = if let Some(args_and_rest) = trimmed_after_name.strip_prefix('(') {
            let Some(close) = closing_paren(args_and_rest) else {
                if is_valid(name) {
                    unclosed.push(name.to_string());
                }
                return;
            };
            (args_and_rest[..close].trim(), &args_and_rest[close + 1..])
        } else {
            ("", after_name)
        };

        if is_valid(name) {
            let normalized_name = if name.eq_ignore_ascii_case("derive") {
                "Derive".to_string()
            } else {
                name.to_string()
            };
            results.push((normalized_name, args.to_string()));
        }
        remaining = rest.trim_start();
    }
}

/// The offset of the `)` that closes a directive's argument list, given the
/// text after its `(`. Parentheses, braces and brackets nest, and none of
/// them counts inside a string literal.
fn closing_paren(args: &str) -> Option<usize> {
    let mut nesting = Nesting::default();
    for (offset, letter) in args.char_indices() {
        if letter == ')' && nesting.is_top_level() {
            return Some(offset);
        }
        nesting.feed(letter);
    }
    None
}

#[cfg(test)]
mod adjacency_tests {
    use super::*;

    #[test]
    fn finds_the_comment_directly_above_a_declaration() {
        let source = "/** @derive(Debug) */\nexport interface Box {}";
        let block = adjacent_jsdoc(source, source.find("interface").unwrap_or_default());
        assert_eq!(
            block.map(|found| found.body(source)),
            Some(" @derive(Debug) ")
        );
    }

    #[test]
    fn ignores_a_comment_that_belongs_to_an_earlier_declaration() {
        let source = "/** @derive(Debug) */\ninterface Box {}\n\nexport type Other = string;";
        let target = source.find("type Other").unwrap_or_default();
        assert_eq!(adjacent_jsdoc(source, target), None);
    }

    #[test]
    fn walks_stacked_comments() {
        let source = "/** first */\n/** second */\ninterface Box {}";
        let target = source.find("interface").unwrap_or_default();
        let nearest = adjacent_jsdoc(source, target);
        assert_eq!(nearest.map(|block| block.body(source)), Some(" second "));
        let above = nearest.and_then(|block| stacked_jsdoc_above(source, 0, block));
        assert_eq!(above.map(|block| block.body(source)), Some(" first "));
    }

    #[test]
    fn a_floor_keeps_the_previous_siblings_comments_out() {
        let source =
            "class A {\n  /** @endec(skip) */\n  a: string;\n  /** b */ /** @x */\n  b: string;\n}";
        let floor = source.find("a: string;").unwrap_or_default() + "a: string;".len();
        let target = source.find("b: string").unwrap_or_default();
        let nearest = adjacent_jsdoc_after(source, floor, target);
        assert_eq!(nearest.map(|block| block.body(source)), Some(" @x "));
        let above = nearest.and_then(|block| stacked_jsdoc_above(source, floor, block));
        assert_eq!(above.map(|block| block.body(source)), Some(" b "));
        let first = above.and_then(|block| stacked_jsdoc_above(source, floor, block));
        assert_eq!(first, None);
    }

    #[test]
    fn an_example_showing_a_jsdoc_stays_inside_its_comment() {
        // The example's inner comments end in `*\u{200b}/`, so they don't close
        // the outer one; its `/**` must not be taken as the comment's start.
        let source = "/**\n * Collects modules.\n *\n * ```ts\n * /** @derive(Form) *\u{200b}/\n * class F {}\n * ```\n */\nexport function collect() {}";
        let target = source.find("export").unwrap_or_default();
        let block = adjacent_jsdoc(source, target);
        assert_eq!(block.map(|found| found.start), Some(0));
        let directives =
            parse_all_macro_directives(block.map_or("", |found| found.body(source)), None);
        assert!(
            directives.is_empty(),
            "the example is not a directive: {directives:?}"
        );
    }

    #[test]
    fn a_bracket_in_a_string_literal_is_text() {
        let directives = |body: &str| parse_all_macro_directives(body, None);
        let pair = |name: &str, args: &str| (name.to_string(), args.to_string());
        assert_eq!(
            directives(r#" @default(":(") @endec({ rename: "a)" }) "#),
            vec![
                pair("default", r#"":(""#),
                pair("endec", r#"{ rename: "a)" }"#)
            ]
        );
        assert_eq!(
            directives(" @endec({ default: \"[\" })\n * @debug({ rename: 'it\\'s {' }) "),
            vec![
                pair("endec", r#"{ default: "[" }"#),
                pair("debug", r"{ rename: 'it\'s {' }"),
            ]
        );
        // Arguments run over lines until their brackets close.
        assert_eq!(
            directives(" @endec({ rename: \"a\",\n *   skip: true })\n * @default(1) "),
            vec![
                pair("endec", r#"{ rename: "a", skip: true }"#),
                pair("default", "1"),
            ]
        );
        // An apostrophe in prose opens no string that outlives its line.
        assert_eq!(
            directives(" @remarks the user's (first\n * @default(1) "),
            vec![pair("remarks", ""), pair("default", "1")]
        );
    }

    #[test]
    fn a_tag_mentioned_in_prose_is_not_a_directive() {
        let body = "\n * @returns names from the enclosing @derive, or null\n * @see {@link find} - For `@derive` decorators\n ";
        let valid: HashSet<String> = ["derive".to_string()].into_iter().collect();
        assert_eq!(parse_all_macro_directives(body, Some(&valid)), Vec::new());
    }

    #[test]
    fn a_fenced_example_is_not_a_directive() {
        let body = "\n * @example\n * ```ts\n * @endec({ skip: true })\n * ```\n * @debug\n ";
        let names: Vec<String> = parse_all_macro_directives(body, None)
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        assert_eq!(names, ["example", "debug"]);
    }

    #[test]
    fn directives_may_follow_one_another_on_a_line() {
        assert_eq!(
            parse_all_macro_directives(" @derive(Debug) @default(1) ", None),
            [
                ("Derive".to_string(), "Debug".to_string()),
                ("default".to_string(), "1".to_string())
            ]
        );
    }

    /// An argument list that never closes is told from a directive without
    /// one, and from a tag of prose, by the names a macro reads.
    #[test]
    fn a_directive_whose_arguments_never_close_is_named() {
        let valid: HashSet<String> = ["default", "endec", "derive"]
            .into_iter()
            .map(str::to_string)
            .collect();
        for (body, name) in [
            (" @default(5 ", "default"),
            (" @default(] ", "default"),
            (" @default({ a: ) ", "default"),
            ("\n * @endec({\n *   rename: \"x\"\n ", "endec"),
            (" @derive(Debug ", "derive"),
            (" @default(\"a)\" ", "default"),
        ] {
            assert_eq!(unclosed_directives(body, Some(&valid)), [name], "{body}");
            assert_eq!(
                parse_all_macro_directives(body, Some(&valid)),
                Vec::new(),
                "{body}"
            );
        }
        // The directives before it are read, and it ends the run.
        assert_eq!(
            parse_all_macro_directives(" @derive(Debug) @default(5 ", Some(&valid)),
            [("Derive".to_string(), "Debug".to_string())]
        );
        assert_eq!(
            unclosed_directives(" @derive(Debug) @default(5 ", Some(&valid)),
            ["default"]
        );
        for closed in [
            " @default(5) ",
            " @default ",
            " @see (the docs ",
            " prose with @default(5 ",
        ] {
            assert_eq!(
                unclosed_directives(closed, Some(&valid)),
                Vec::<String>::new(),
                "{closed}"
            );
        }
    }

    #[test]
    fn only_an_import_macro_directive_is_an_import_comment() {
        assert!(is_macro_import_comment(
            " import macro { $vec } from \"./vec\"; "
        ));
        assert!(!is_macro_import_comment(
            "\n * Parses import macro comments like `import macro { A } from \"pkg\"`.\n "
        ));
    }
}
