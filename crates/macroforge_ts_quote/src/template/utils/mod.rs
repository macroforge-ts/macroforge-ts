use crate::template::{ControlNode, IdGen, PlaceholderUse, Segment, TagType};
use proc_macro2::{Delimiter, Group, TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;

/// Assigns a precedence rank for placeholder usage kinds.
pub(crate) fn use_rank(use_kind: &PlaceholderUse) -> usize {
    match use_kind {
        PlaceholderUse::Stmt => 5,
        PlaceholderUse::Type => 4,
        PlaceholderUse::Ident => 3,
        PlaceholderUse::IdentName => 2,
        PlaceholderUse::Expr => 1,
    }
}

/// Appends a template part, inserting spaces between tokens when needed.
pub(crate) fn append_part(out: &mut String, part: &str) {
    if out.is_empty() {
        out.push_str(part);
        return;
    }

    let last_char = out.chars().last().unwrap_or(' ');
    let first_char = part.chars().next().unwrap_or(' ');

    // Don't add space if:
    // - Last char is whitespace
    // - First char is opening bracket (no space before)
    // - Last char is opening bracket (no space after)
    // - First char is closing bracket, colon, semicolon, comma, or dot (no space before)
    // - Last char is a dot (no space after dots for member access)
    let no_space_before = matches!(
        first_char,
        '(' | '[' | ')' | ']' | '}' | ':' | ';' | ',' | '.'
    );
    let no_space_after = matches!(last_char, '(' | '[' | '.');

    let needs_space = !last_char.is_whitespace() && !no_space_before && !no_space_after;

    if needs_space {
        out.push(' ');
    }
    out.push_str(part);
}

/// Converts a token stream into a TypeScript-like string.
pub(crate) fn tokens_to_ts_string(tokens: TokenStream2) -> String {
    let mut output = String::new();
    let mut iter = tokens.into_iter().peekable();

    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Group(g) => {
                let inner = tokens_to_ts_string(g.stream());
                let (open, close) = match g.delimiter() {
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::Brace => ("{", "}"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::None => ("", ""),
                };
                output.push_str(open);
                output.push_str(&inner);
                output.push_str(close);
            }
            TokenTree::Ident(ident) => {
                output.push_str(&ident.to_string());
                // Only add space after ident if the next token needs it
                // Don't add space before: (, [, ., :, ;, ,
                let next_needs_space = match iter.peek() {
                    Some(TokenTree::Punct(p)) => {
                        !matches!(p.as_char(), '(' | '[' | '.' | ':' | ';' | ',' | ')' | ']')
                    }
                    Some(TokenTree::Group(g)) => {
                        // No space before groups
                        g.delimiter() == Delimiter::Brace
                    }
                    None => false,
                    _ => true,
                };
                if next_needs_space {
                    output.push(' ');
                }
            }
            TokenTree::Punct(p) => {
                output.push(p.as_char());
                // Add space after standalone punct, but not after certain punctuation
                if p.spacing() == proc_macro2::Spacing::Alone {
                    // @ is for interpolations - never add space after it
                    let no_space_after =
                        matches!(p.as_char(), '.' | '(' | '[' | ':' | ';' | ',' | '@');
                    if !no_space_after {
                        output.push(' ');
                    }
                }
            }
            TokenTree::Literal(lit) => output.push_str(&lit.to_string()),
        }
    }

    output
}

/// Classifies a brace-delimited group as a template tag or plain block.
pub(crate) fn analyze_tag(g: &Group) -> TagType {
    let tokens: Vec<TokenTree> = g.stream().into_iter().collect();

    if tokens.len() >= 2
        && let (Some(TokenTree::Punct(first)), Some(TokenTree::Punct(last))) =
            (tokens.first(), tokens.last())
        && first.as_char() == '|'
        && last.as_char() == '|'
    {
        return TagType::IdentBlock;
    }

    if tokens.len() >= 5
        && let (Some(TokenTree::Punct(p1)), Some(TokenTree::Punct(p2))) =
            (tokens.first(), tokens.get(1))
        && p1.as_char() == '>'
        && p2.as_char() == '>'
        && let (Some(TokenTree::Punct(p3)), Some(TokenTree::Punct(p4))) =
            (tokens.get(tokens.len() - 2), tokens.last())
        && p3.as_char() == '<'
        && p4.as_char() == '<'
    {
        if let Some(TokenTree::Literal(lit)) = tokens.get(2) {
            let content = extract_string_literal(lit);
            return TagType::DocComment(content);
        }
        let content = tokens_to_spaced_string(&tokens[2..tokens.len() - 2]);
        return TagType::DocComment(content);
    }

    if tokens.len() >= 3
        && let (Some(TokenTree::Punct(first)), Some(TokenTree::Punct(last))) =
            (tokens.first(), tokens.last())
        && first.as_char() == '>'
        && last.as_char() == '<'
    {
        if let Some(TokenTree::Literal(lit)) = tokens.get(1) {
            let content = extract_string_literal(lit);
            return TagType::BlockComment(content);
        }
        let content = tokens_to_spaced_string(&tokens[1..tokens.len() - 1]);
        return TagType::BlockComment(content);
    }

    if tokens.len() < 2 {
        return TagType::Block;
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '#'
    {
        if i == "if" {
            if let Some(TokenTree::Ident(let_kw)) = tokens.get(2)
                && let_kw == "let"
            {
                let mut pattern = TokenStream2::new();
                let mut expr = TokenStream2::new();
                let mut seen_eq = false;

                for t in tokens.iter().skip(3) {
                    if let TokenTree::Punct(eq) = t
                        && eq.as_char() == '='
                        && !seen_eq
                    {
                        seen_eq = true;
                        continue;
                    }
                    if !seen_eq {
                        t.to_tokens(&mut pattern);
                    } else {
                        t.to_tokens(&mut expr);
                    }
                }
                return TagType::IfLet(pattern, expr);
            }

            let cond: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::If(cond);
        }

        if i == "match" {
            let expr: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Match(expr);
        }

        if i == "while" {
            if let Some(TokenTree::Ident(let_kw)) = tokens.get(2)
                && let_kw == "let"
            {
                let mut pattern = TokenStream2::new();
                let mut expr = TokenStream2::new();
                let mut seen_eq = false;

                for t in tokens.iter().skip(3) {
                    if let TokenTree::Punct(eq) = t
                        && eq.as_char() == '='
                        && !seen_eq
                    {
                        seen_eq = true;
                        continue;
                    }
                    if !seen_eq {
                        t.to_tokens(&mut pattern);
                    } else {
                        t.to_tokens(&mut expr);
                    }
                }
                return TagType::WhileLet(pattern, expr);
            }

            let cond: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::While(cond);
        }

        if i == "for" {
            let mut item = TokenStream2::new();
            let mut list = TokenStream2::new();
            let mut seen_in = false;

            for t in tokens.iter().skip(2) {
                if let TokenTree::Ident(ident) = t
                    && ident == "in"
                    && !seen_in
                {
                    seen_in = true;
                    continue;
                }
                if !seen_in {
                    t.to_tokens(&mut item);
                } else {
                    t.to_tokens(&mut list);
                }
            }

            return TagType::For(item, list);
        }
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == ':'
    {
        if i == "else" {
            if tokens.len() >= 4
                && let Some(TokenTree::Ident(if_kw)) = tokens.get(2)
                && if_kw == "if"
            {
                let cond: TokenStream2 =
                    tokens.iter().skip(3).map(|t| t.to_token_stream()).collect();
                return TagType::ElseIf(cond);
            }
            return TagType::Else;
        }

        if i == "case" {
            let pattern: TokenStream2 =
                tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Case(pattern);
        }
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '/'
    {
        if i == "if" {
            return TagType::EndIf;
        }
        if i == "for" {
            return TagType::EndFor;
        }
        if i == "while" {
            return TagType::EndWhile;
        }
        if i == "match" {
            return TagType::EndMatch;
        }
    }

    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '$'
    {
        if i == "let" {
            let rest: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Let(rest);
        }
        if i == "do" {
            let rest: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Do(rest);
        }
        if i == "typescript" {
            let rest: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Typescript(rest);
        }
    }

    TagType::Block
}

/// Renders a group token into a string, preserving delimiters.
pub(crate) fn group_to_string(g: &Group) -> String {
    let inner = tokens_to_ts_string(g.stream());
    let (open, close) = match g.delimiter() {
        Delimiter::Parenthesis => ("(", ")"),
        Delimiter::Brace => ("{", "}"),
        Delimiter::Bracket => ("[", "]"),
        Delimiter::None => ("", ""),
    };
    format!("{}{}{}", open, inner, close)
}

/// Joins tokens into a space-separated string.
pub(crate) fn tokens_to_spaced_string(tokens: &[TokenTree]) -> String {
    let mut result = String::new();
    for (i, token) in tokens.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push_str(&token.to_string());
    }
    result
}

/// Extracts and unescapes the contents of a Rust string literal token.
pub(crate) fn extract_string_literal(lit: &proc_macro2::Literal) -> String {
    let s = lit.to_string();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        let inner = &s[1..s.len() - 1];
        return unescape_string(inner);
    }
    if s.starts_with("r\"") && s.ends_with("\"") {
        return s[2..s.len() - 1].to_string();
    }
    if s.starts_with("r#\"")
        && let Some(idx) = s.rfind("\"") {
            return s[3..idx].to_string();
        }
    s
}

/// Performs a minimal unescape pass for Rust string literal escapes.
pub(crate) fn unescape_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            result.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => result.push('\n'),
            Some('t') => result.push('\t'),
            Some('r') => result.push('\r'),
            Some('0') => result.push('\0'),
            Some('"') => result.push('"'),
            Some('\\') => result.push('\\'),
            Some(other) => {
                result.push('\\');
                result.push(other);
            }
            None => result.push('\\'),
        }
    }
    result
}
/// Tries to extract an object property loop pattern from brace block inner segments.
///
/// Matches the pattern: `{ {#for (key, val) in items} @{key}: @{val}, {/for} }`
/// where the for loop contains key-value property assignments.
///
/// Returns `Some(ObjectPropLoop)` if the pattern is detected, `None` otherwise.
pub(crate) fn try_extract_object_prop_loop(
    inner_segments: &[Segment],
    ids: &mut IdGen,
) -> Option<Segment> {
    // Find the for loop control node
    let mut for_node = None;
    let mut other_dynamic = false;

    for seg in inner_segments {
        match seg {
            Segment::Control {
                node: ControlNode::For { pat, iter, body },
                ..
            } => {
                if for_node.is_some() {
                    // Multiple for loops - not a simple object prop loop
                    return None;
                }
                for_node = Some((pat.clone(), iter.clone(), body.clone()));
            }
            Segment::Static(_) => {
                // Static content (whitespace, commas) is OK
            }
            _ => {
                // Other dynamic segments (interpolations outside the loop, etc.)
                other_dynamic = true;
            }
        }
    }

    // If there's other dynamic content, fall back to normal handling
    if other_dynamic {
        return None;
    }

    let (pat, iter, body) = for_node?;

    // Check if the body matches the key-value property pattern: @{key}: @{val},
    // We expect: Static(""), Interpolation(key), Static(": "), Interpolation(val), Static(",")
    // Or variations with whitespace
    let mut key_expr = None;
    let mut value_expr = None;
    let mut found_colon = false;

    for seg in &body {
        match seg {
            Segment::Static(s) => {
                let trimmed = s.trim();
                if trimmed.contains(':') {
                    found_colon = true;
                }
                // Allow whitespace, colons, commas
            }
            Segment::Interpolation { expr, .. } => {
                if !found_colon {
                    // This is before the colon, so it's the key
                    if key_expr.is_some() {
                        // Multiple keys - not a simple pattern
                        return None;
                    }
                    key_expr = Some(expr.clone());
                } else {
                    // This is after the colon, so it's the value
                    if value_expr.is_some() {
                        // Multiple values - not a simple pattern
                        return None;
                    }
                    value_expr = Some(expr.clone());
                }
            }
            _ => {
                // Other segment types in the body - not a simple pattern
                return None;
            }
        }
    }

    // Must have both key and value
    let key_expr = key_expr?;
    let value_expr = value_expr?;

    if !found_colon {
        return None;
    }

    let id = ids.next();
    Some(Segment::ObjectPropLoop {
        id,
        pat,
        iter,
        key_expr,
        value_expr,
    })
}
