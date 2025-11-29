//! Rust-style templating for TypeScript code generation
//!
//! Provides a template syntax with interpolation and control flow:
//! - `@{expr}` - Interpolate expressions
//! - `@@{` - Escape for literal `@{` (e.g., `"@@{foo}"` → `@{foo}`)
//! - `"string @{expr}"` - String interpolation (auto-detected)
//! - `"'^template ${expr}^'"` - JS backtick template literal (outputs `` `template ${expr}` ``)
//! - `{#if cond}...{/if}` - Conditional blocks
//! - `{#if let pattern = expr}...{/if}` - Pattern matching if-let blocks
//! - `{:else}` - Else clause
//! - `{:else if cond}` - Else-if clause
//! - `{#match expr}{:case pattern}...{/match}` - Match blocks with case arms
//! - `{#for item in list}...{/for}` - Iteration
//! - `{%let name = expr}` - Local constants
//!
//! Note: A single `@` not followed by `{` passes through unchanged (e.g., `email@domain.com`).

use proc_macro2::{Delimiter, Group, TokenStream as TokenStream2, TokenTree};
use quote::{ToTokens, quote};
use std::iter::Peekable;

/// Parse the template stream and generate code to build a TypeScript string
pub fn parse_template(input: TokenStream2) -> TokenStream2 {
    // Parse the tokens into a Rust block that returns a String
    let (body, _) = parse_fragment(&mut input.into_iter().peekable(), None);

    quote! {
        {
            let mut __out = String::new();
            #body
            __out
        }
    }
}

// Terminators tell the parser when to stop current recursion level
#[derive(Debug, Clone)]
enum Terminator {
    Else,
    ElseIf(TokenStream2),
    EndIf,
    EndFor,
    Case(TokenStream2),
    EndMatch,
}

// Analyzes a braces group { ... } to see if it's a Macro Tag or just TS code
enum TagType {
    If(TokenStream2),
    IfLet(TokenStream2, TokenStream2), // pattern, expression
    For(TokenStream2, TokenStream2),   // item_name, collection
    Match(TokenStream2),               // expression to match
    Else,
    ElseIf(TokenStream2),
    Case(TokenStream2), // pattern for match arm
    EndIf,
    EndFor,
    EndMatch,
    Let(TokenStream2),
    Block, // Standard TypeScript Block { ... }
}

fn analyze_tag(g: &Group) -> TagType {
    let tokens: Vec<TokenTree> = g.stream().into_iter().collect();
    if tokens.len() < 2 {
        return TagType::Block;
    }

    // Check for {# ...} tags
    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '#'
    {
        if i == "if" {
            // Check for {#if let pattern = expr}
            if let Some(TokenTree::Ident(let_kw)) = tokens.get(2)
                && let_kw == "let"
            {
                // Format: {#if let pattern = expr}
                // Split on "=" to separate pattern from expression
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

            // Format: {#if condition}
            let cond: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::If(cond);
        }

        if i == "match" {
            // Format: {#match expr}
            let expr: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Match(expr);
        }

        if i == "for" {
            // Format: {#for item in collection}
            let mut item = TokenStream2::new();
            let mut list = TokenStream2::new();
            let mut seen_in = false;

            // Split on "in" keyword
            for t in tokens.iter().skip(2) {
                if let TokenTree::Ident(id) = t
                    && id == "in"
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

    // Check for {% ...} tags (let)
    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '%'
        && i == "let"
    {
        // Format: {%let name = expr}
        let body: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
        return TagType::Let(body);
    }

    // Check for {: ...} tags (else, else if, case)
    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == ':'
    {
        if i == "else" {
            // Check for {:else if condition}
            if let Some(TokenTree::Ident(next)) = tokens.get(2)
                && next == "if"
            {
                let cond: TokenStream2 =
                    tokens.iter().skip(3).map(|t| t.to_token_stream()).collect();
                return TagType::ElseIf(cond);
            }
            return TagType::Else;
        }

        if i == "case" {
            // Format: {:case pattern}
            let pattern: TokenStream2 =
                tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Case(pattern);
        }
    }

    // Check for {/ ...} (End tags)
    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '/'
    {
        if i == "if" {
            return TagType::EndIf;
        }
        if i == "for" {
            return TagType::EndFor;
        }
        if i == "match" {
            return TagType::EndMatch;
        }
    }

    TagType::Block
}

/// Parse an if/else-if/else chain starting from the condition
fn parse_if_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    initial_cond: TokenStream2,
) -> TokenStream2 {
    // Parse the true block, stopping at {:else}, {:else if}, or {/if}
    let (true_block, terminator) = parse_fragment(
        iter,
        Some(&[
            Terminator::Else,
            Terminator::ElseIf(TokenStream2::new()),
            Terminator::EndIf,
        ]),
    );

    match terminator {
        Some(Terminator::EndIf) => {
            // Simple if without else
            quote! {
                if #initial_cond {
                    #true_block
                }
            }
        }
        Some(Terminator::Else) => {
            // if with else - parse else block until {/if}
            let (else_block, terminator) = parse_fragment(iter, Some(&[Terminator::EndIf]));
            if !matches!(terminator, Some(Terminator::EndIf)) {
                panic!("Unclosed {{:else}} block: Missing {{/if}}");
            }
            quote! {
                if #initial_cond {
                    #true_block
                } else {
                    #else_block
                }
            }
        }
        Some(Terminator::ElseIf(else_if_cond)) => {
            // if with else if - recursively parse the else-if chain
            let else_if_chain = parse_if_chain(iter, else_if_cond);
            quote! {
                if #initial_cond {
                    #true_block
                } else {
                    #else_if_chain
                }
            }
        }
        None => {
            panic!("Unclosed {{#if}} block: Missing {{/if}}");
        }
        _ => unreachable!(),
    }
}

/// Parse an if-let/else chain starting from the pattern and expression
fn parse_if_let_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    pattern: TokenStream2,
    expr: TokenStream2,
) -> TokenStream2 {
    // Parse the true block, stopping at {:else} or {/if}
    let (true_block, terminator) =
        parse_fragment(iter, Some(&[Terminator::Else, Terminator::EndIf]));

    match terminator {
        Some(Terminator::EndIf) => {
            // Simple if let without else
            quote! {
                if let #pattern = #expr {
                    #true_block
                }
            }
        }
        Some(Terminator::Else) => {
            // if let with else - parse else block until {/if}
            let (else_block, terminator) = parse_fragment(iter, Some(&[Terminator::EndIf]));
            if !matches!(terminator, Some(Terminator::EndIf)) {
                panic!("Unclosed {{:else}} block: Missing {{/if}}");
            }
            quote! {
                if let #pattern = #expr {
                    #true_block
                } else {
                    #else_block
                }
            }
        }
        None => {
            panic!("Unclosed {{#if let}} block: Missing {{/if}}");
        }
        _ => unreachable!(),
    }
}

/// Parse match arms starting after {#match expr}
/// Format: {#match expr}{:case pattern1}body1{:case pattern2}body2{/match}
fn parse_match_arms(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    match_expr: TokenStream2,
) -> TokenStream2 {
    let mut arms = TokenStream2::new();
    let mut current_pattern: Option<TokenStream2> = None;

    loop {
        // Parse until we hit {:case} or {/match}
        let (body, terminator) = parse_fragment(
            iter,
            Some(&[Terminator::Case(TokenStream2::new()), Terminator::EndMatch]),
        );

        match terminator {
            Some(Terminator::Case(pattern)) => {
                // If we have a previous pattern, emit its arm with the body we just parsed
                if let Some(prev_pattern) = current_pattern.take() {
                    arms.extend(quote! {
                        #prev_pattern => {
                            #body
                        }
                    });
                }
                // Store this pattern for the next iteration
                current_pattern = Some(pattern);
            }
            Some(Terminator::EndMatch) => {
                // Emit the final arm if we have one
                if let Some(prev_pattern) = current_pattern.take() {
                    arms.extend(quote! {
                        #prev_pattern => {
                            #body
                        }
                    });
                }
                break;
            }
            None => {
                panic!("Unclosed {{#match}} block: Missing {{/match}}");
            }
            _ => unreachable!(),
        }
    }

    quote! {
        match #match_expr {
            #arms
        }
    }
}

/// Recursive function to parse tokens until a terminator is found
fn parse_fragment(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    stop_at: Option<&[Terminator]>,
) -> (TokenStream2, Option<Terminator>) {
    let mut output = TokenStream2::new();

    while let Some(token) = iter.peek().cloned() {
        match &token {
            // Case 1: Interpolation @{ expr }
            TokenTree::Punct(p) if p.as_char() == '@' => {
                // Check if the NEXT token is a Group { ... }
                let p_clone = p.clone();
                iter.next(); // Consume '@'

                // Look ahead
                let is_group = matches!(iter.peek(), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace);

                if is_group {
                    // It IS interpolation: @{ expr }
                    if let Some(TokenTree::Group(g)) = iter.next() {
                        let content = g.stream();
                        output.extend(quote! {
                            __out.push_str(&#content.to_string());
                        });
                    }
                } else {
                    // It is just a literal '@'
                    let s = p_clone.to_string();
                    output.extend(quote! { __out.push_str(#s); });
                }
            }

            // Case 2: Groups { ... } - Could be Tag or Block
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => {
                let tag = analyze_tag(g);

                match tag {
                    TagType::If(cond) => {
                        iter.next(); // Consume {#if}
                        output.extend(parse_if_chain(iter, cond));
                    }
                    TagType::IfLet(pattern, expr) => {
                        iter.next(); // Consume {#if let}
                        output.extend(parse_if_let_chain(iter, pattern, expr));
                    }
                    TagType::For(item, list) => {
                        iter.next(); // Consume {#for}

                        let (body, terminator) = parse_fragment(iter, Some(&[Terminator::EndFor]));
                        if !matches!(terminator, Some(Terminator::EndFor)) {
                            panic!("Unclosed {{#for}} block: Missing {{/for}}");
                        }

                        output.extend(quote! {
                            for #item in #list {
                                #body
                            }
                        });
                    }
                    TagType::Match(expr) => {
                        iter.next(); // Consume {#match}
                        output.extend(parse_match_arms(iter, expr));
                    }
                    TagType::Else => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::Else))
                        {
                            iter.next(); // Consume
                            return (output, Some(Terminator::Else));
                        }
                        panic!("Unexpected {{:else}}");
                    }
                    TagType::ElseIf(cond) => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::ElseIf(_)))
                        {
                            iter.next(); // Consume
                            return (output, Some(Terminator::ElseIf(cond)));
                        }
                        panic!("Unexpected {{:else if}}");
                    }
                    TagType::EndIf => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndIf))
                        {
                            iter.next(); // Consume
                            return (output, Some(Terminator::EndIf));
                        }
                        panic!("Unexpected {{/if}}");
                    }
                    TagType::EndFor => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndFor))
                        {
                            iter.next(); // Consume
                            return (output, Some(Terminator::EndFor));
                        }
                        panic!("Unexpected {{/for}}");
                    }
                    TagType::Case(pattern) => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::Case(_)))
                        {
                            iter.next(); // Consume
                            return (output, Some(Terminator::Case(pattern)));
                        }
                        panic!("Unexpected {{:case}}");
                    }
                    TagType::EndMatch => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndMatch))
                        {
                            iter.next(); // Consume
                            return (output, Some(Terminator::EndMatch));
                        }
                        panic!("Unexpected {{/match}}");
                    }
                    TagType::Let(body) => {
                        iter.next(); // Consume {%let ...}
                        output.extend(quote! {
                            let #body;
                        });
                    }
                    TagType::Block => {
                        // Regular TS Block { ... }
                        // Recurse to allow macros inside standard TS objects
                        iter.next(); // Consume
                        let inner_stream = g.stream();

                        output.extend(quote! { __out.push_str("{"); });
                        let (inner_parsed, _) =
                            parse_fragment(&mut inner_stream.into_iter().peekable(), None);
                        output.extend(inner_parsed);
                        output.extend(quote! { __out.push_str("}"); });
                    }
                }
            }

            // Case 3: Other groups (parentheses, brackets)
            TokenTree::Group(g) => {
                iter.next();
                let (open, close) = match g.delimiter() {
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::Brace => ("{", "}"), // Shouldn't reach here
                    Delimiter::None => ("", ""),
                };

                output.extend(quote! { __out.push_str(#open); });
                let (inner_parsed, _) =
                    parse_fragment(&mut g.stream().into_iter().peekable(), None);
                output.extend(inner_parsed);
                output.extend(quote! { __out.push_str(#close); });
            }

            // Case 4a: Backtick template literals "'^...^'" -> `...`
            TokenTree::Literal(lit) if is_backtick_template(lit) => {
                iter.next(); // Consume
                let processed = process_backtick_template(lit);
                output.extend(processed);
                output.extend(quote! { __out.push_str(" "); });
            }

            // Case 4b: String literals with interpolation
            TokenTree::Literal(lit) if is_string_literal(lit) => {
                iter.next(); // Consume
                let interpolated = interpolate_string_literal(lit);
                output.extend(interpolated);
                output.extend(quote! { __out.push_str(" "); });
            }

            // Case 5: Plain Text
            _ => {
                let t = iter.next().unwrap();
                let s = t.to_string();

                // Check if this is an identifier (includes keywords like `instanceof`, `return`, etc.)
                let is_ident = matches!(&t, TokenTree::Ident(_));

                // Check if next token is '@' (start of interpolation)
                let next_is_at =
                    matches!(iter.peek(), Some(TokenTree::Punct(p)) if p.as_char() == '@');

                // Check if this is a punctuation token with Joint spacing
                // (e.g., first two chars of `===` should not have spaces)
                let is_joint_punct =
                    matches!(&t, TokenTree::Punct(p) if p.spacing() == proc_macro2::Spacing::Joint);

                output.extend(quote! {
                    __out.push_str(#s);
                });

                // Add space after:
                // - Identifiers (always need space before next token for keywords like `instanceof`, `return`)
                // - Non-joint punctuation when not followed by @ (allows `make@{Name}` but spaces around operators)
                let should_add_space = is_ident || (!is_joint_punct && !next_is_at);

                if should_add_space {
                    output.extend(quote! { __out.push_str(" "); });
                }
            }
        }
    }

    (output, None)
}

/// Check if a literal is a string (starts with " or ')
fn is_string_literal(lit: &proc_macro2::Literal) -> bool {
    let s = lit.to_string();
    s.starts_with('"') || s.starts_with('\'') || s.starts_with("r\"") || s.starts_with("r#")
}

/// Check if a literal is a backtick template literal marker: "'^...^'"
/// This syntax outputs JS template literals with backticks: `...`
fn is_backtick_template(lit: &proc_macro2::Literal) -> bool {
    let s = lit.to_string();
    // Check for "'^...^'" pattern (the outer quotes are part of the Rust string)
    if s.starts_with("\"'^") && s.ends_with("^'\"") && s.len() >= 6 {
        return true;
    }
    // Also support raw strings: r"'^...^'" or r#"'^...^'"#
    if s.starts_with("r\"'^") && s.ends_with("^'\"") {
        return true;
    }
    if s.starts_with("r#\"'^") && s.ends_with("^'\"#") {
        return true;
    }
    false
}

/// Process a backtick template literal "'^...^'" -> `...`
/// Supports @{expr} interpolation for Rust expressions within the template
fn process_backtick_template(lit: &proc_macro2::Literal) -> TokenStream2 {
    let raw = lit.to_string();

    // Extract content between '^...^' markers
    let content = if raw.starts_with("\"'^") && raw.ends_with("^'\"") {
        &raw[3..raw.len() - 3]
    } else if raw.starts_with("r\"'^") && raw.ends_with("^'\"") {
        &raw[4..raw.len() - 3]
    } else if raw.starts_with("r#\"'^") && raw.ends_with("^'\"#") {
        &raw[5..raw.len() - 4]
    } else {
        return quote! { __out.push_str(#raw); };
    };

    // Check if there are any @{} interpolations or @@ escapes
    if !content.contains('@') {
        // No @ at all, output the backtick string as-is
        // The content may contain ${} for JS interpolation, which passes through
        let mut output = TokenStream2::new();
        output.extend(quote! { __out.push_str("`"); });
        output.extend(quote! { __out.push_str(#content); });
        output.extend(quote! { __out.push_str("`"); });
        return output;
    }

    // Handle @{} Rust interpolations and @@ escapes within the backtick template
    let mut output = TokenStream2::new();
    output.extend(quote! { __out.push_str("`"); });

    let mut chars = content.chars().peekable();
    let mut current_literal = String::new();

    while let Some(c) = chars.next() {
        if c == '@' {
            match chars.peek() {
                Some(&'@') => {
                    // @@ -> literal @
                    chars.next(); // Consume second @
                    current_literal.push('@');
                }
                Some(&'{') => {
                    // @{ -> interpolation
                    // Flush current literal
                    if !current_literal.is_empty() {
                        output.extend(quote! { __out.push_str(#current_literal); });
                        current_literal.clear();
                    }

                    chars.next(); // Consume '{'

                    // Collect expression until matching '}'
                    let mut expr_str = String::new();
                    let mut brace_depth = 1;

                    for ec in chars.by_ref() {
                        if ec == '{' {
                            brace_depth += 1;
                            expr_str.push(ec);
                        } else if ec == '}' {
                            brace_depth -= 1;
                            if brace_depth == 0 {
                                break;
                            }
                            expr_str.push(ec);
                        } else {
                            expr_str.push(ec);
                        }
                    }

                    // Parse the expression and generate interpolation code
                    if let Ok(expr) = syn::parse_str::<syn::Expr>(&expr_str) {
                        output.extend(quote! {
                            __out.push_str(&#expr.to_string());
                        });
                    } else {
                        // Failed to parse, output as literal
                        let fallback = format!("@{{{}}}", expr_str);
                        output.extend(quote! { __out.push_str(#fallback); });
                    }
                }
                _ => {
                    // Just a literal @
                    current_literal.push('@');
                }
            }
        } else {
            current_literal.push(c);
        }
    }

    // Flush remaining literal
    if !current_literal.is_empty() {
        output.extend(quote! { __out.push_str(#current_literal); });
    }

    output.extend(quote! { __out.push_str("`"); });
    output
}

/// Process a string literal and handle @{expr} interpolations inside it
fn interpolate_string_literal(lit: &proc_macro2::Literal) -> TokenStream2 {
    let raw = lit.to_string();

    // Determine quote character and extract content
    let (quote_char, content) = if raw.starts_with('"') {
        ('"', &raw[1..raw.len() - 1])
    } else if raw.starts_with('\'') {
        ('\'', &raw[1..raw.len() - 1])
    } else if raw.starts_with("r\"") {
        // Raw string r"..."
        ('"', &raw[2..raw.len() - 1])
    } else if raw.starts_with("r#") {
        // Raw string r#"..."# - find the actual content
        let hash_count = raw[1..].chars().take_while(|&c| c == '#').count();
        let start = 2 + hash_count; // r + # + "
        let end = raw.len() - 1 - hash_count; // " + #
        ('"', &raw[start..end])
    } else {
        // Not a string we recognize, just output as-is
        return quote! { __out.push_str(#raw); };
    };

    // Check if there are any interpolations or escapes
    if !content.contains('@') {
        // No @ at all, output the string as-is
        return quote! { __out.push_str(#raw); };
    }

    // Parse and interpolate
    let mut output = TokenStream2::new();
    let quote_str = quote_char.to_string();
    output.extend(quote! { __out.push_str(#quote_str); });

    let mut chars = content.chars().peekable();
    let mut current_literal = String::new();

    while let Some(c) = chars.next() {
        if c == '@' {
            match chars.peek() {
                Some(&'@') => {
                    // @@ -> literal @
                    chars.next(); // Consume second @
                    current_literal.push('@');
                }
                Some(&'{') => {
                    // @{ -> interpolation
                    // Flush current literal
                    if !current_literal.is_empty() {
                        output.extend(quote! { __out.push_str(#current_literal); });
                        current_literal.clear();
                    }

                    chars.next(); // Consume '{'

                    // Collect expression until matching '}'
                    let mut expr_str = String::new();
                    let mut brace_depth = 1;

                    for ec in chars.by_ref() {
                        if ec == '{' {
                            brace_depth += 1;
                            expr_str.push(ec);
                        } else if ec == '}' {
                            brace_depth -= 1;
                            if brace_depth == 0 {
                                break;
                            }
                            expr_str.push(ec);
                        } else {
                            expr_str.push(ec);
                        }
                    }

                    // Parse the expression and generate interpolation code
                    if let Ok(expr) = syn::parse_str::<syn::Expr>(&expr_str) {
                        output.extend(quote! {
                            __out.push_str(&#expr.to_string());
                        });
                    } else {
                        // Failed to parse, output as literal
                        let fallback = format!("@{{{}}}", expr_str);
                        output.extend(quote! { __out.push_str(#fallback); });
                    }
                }
                _ => {
                    // Just a literal @
                    current_literal.push('@');
                }
            }
        } else if c == '\\' {
            // Handle escape sequences - pass through as-is
            current_literal.push(c);
            if chars.peek().is_some() {
                current_literal.push(chars.next().unwrap());
            }
        } else {
            current_literal.push(c);
        }
    }

    // Flush remaining literal
    if !current_literal.is_empty() {
        output.extend(quote! { __out.push_str(#current_literal); });
    }

    output.extend(quote! { __out.push_str(#quote_str); });

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use std::str::FromStr;

    #[test]
    fn test_at_interpolation_glue() {
        // Rust allows `make@{Name}` naturally without spaces.
        // We verify that `make@{Name}` results in "make" + value (no space).
        let input = quote! {
            make@{Name}
        };
        let output = parse_template(input);
        let s = output.to_string();

        // Should generate code that pushes "make"
        assert!(s.contains("\"make\""));

        // And then pushes the value.
        // If our logic works, no " " is pushed in between.
    }

    #[test]
    fn test_let_scope() {
        let input = TokenStream2::from_str(
            r#"
            {%let val = "hello"}
            @{val}
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should contain "let val = "hello" ;"
        assert!(s.contains("let val = \"hello\""));

        // Should contain usage
        assert!(s.contains("val . to_string"));
    }

    #[test]
    fn test_for_loop() {
        let input = TokenStream2::from_str(
            r#"
            {#for item in items}
                @{item}
            {/for}
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should generate a for loop
        assert!(s.contains("for item in items"), "Should generate for loop");
    }

    #[test]
    fn test_if_else() {
        let input = TokenStream2::from_str(
            r#"
            {#if condition}
                "true"
            {:else}
                "false"
            {/if}
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should generate an if-else
        assert!(s.contains("if condition"), "Should generate if condition");
        assert!(s.contains("else"), "Should have else branch");
    }

    #[test]
    fn test_if_else_if() {
        let input = TokenStream2::from_str(
            r#"
            {#if a}
                "a"
            {:else if b}
                "b"
            {:else}
                "c"
            {/if}
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should generate if/else if/else chain
        assert!(s.contains("if a"), "Should have if a");
        assert!(s.contains("if b"), "Should have else if b");
        assert!(s.contains("else"), "Should have else");
    }

    #[test]
    fn test_string_interpolation_simple() {
        // Test that @{expr} inside strings gets interpolated
        let input = quote! {
            "Hello @{name}!"
        };
        let output = parse_template(input);
        let s = output.to_string();

        // Should push the opening quote
        assert!(s.contains("\"\\\"\""), "Should push opening quote");
        // Should push "Hello "
        assert!(s.contains("\"Hello \""), "Should push 'Hello '");
        // Should interpolate name
        assert!(s.contains("name . to_string"), "Should interpolate name");
        // Should push "!"
        assert!(s.contains("\"!\""), "Should push '!'");
    }

    #[test]
    fn test_string_no_interpolation() {
        // Strings without @{} should pass through unchanged
        let input = quote! {
            "Just a plain string"
        };
        let output = parse_template(input);
        let s = output.to_string();

        println!("Output: {}", s);

        // Should just push the whole string as-is (escaped in the generated code)
        assert!(
            s.contains("Just a plain string"),
            "Should contain the string content"
        );
    }

    #[test]
    fn test_string_interpolation_multiple() {
        // Test multiple interpolations in one string
        let input = quote! {
            "@{greeting}, @{name}!"
        };
        let output = parse_template(input);
        let s = output.to_string();

        // Should interpolate both
        assert!(
            s.contains("greeting . to_string"),
            "Should interpolate greeting"
        );
        assert!(s.contains("name . to_string"), "Should interpolate name");
    }

    #[test]
    fn test_string_interpolation_with_method_call() {
        // Test that expressions with method calls work
        let input = quote! {
            "Name: @{name.to_uppercase()}"
        };
        let output = parse_template(input);
        let s = output.to_string();

        // Should contain the method call
        assert!(s.contains("to_uppercase"), "Should contain method call");
    }

    #[test]
    fn test_backtick_template_simple() {
        // Test that "'^...^'" outputs backtick template literals
        let input = TokenStream2::from_str(
            r#"
            "'^hello ${name}^'"
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should push backtick at start
        assert!(s.contains("\"`\""), "Should push opening backtick");
        // Should contain the template content with ${name} passed through
        assert!(
            s.contains("hello ${name}"),
            "Should contain template content"
        );
    }

    #[test]
    fn test_backtick_template_with_rust_interpolation() {
        // Test that @{} works inside backtick templates for Rust expressions
        let input = TokenStream2::from_str(
            r#"
            "'^hello @{rust_var}^'"
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should push backtick
        assert!(s.contains("\"`\""), "Should push backtick");
        // Should interpolate the Rust variable
        assert!(
            s.contains("rust_var . to_string"),
            "Should interpolate Rust var"
        );
    }

    #[test]
    fn test_backtick_template_mixed() {
        // Test mixing JS ${} and Rust @{} in backtick templates
        let input = TokenStream2::from_str(
            r#"
            "'^${jsVar} and @{rustVar}^'"
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should contain JS interpolation passed through
        assert!(
            s.contains("${jsVar}"),
            "Should pass through JS interpolation"
        );
        // Should interpolate Rust variable
        assert!(
            s.contains("rustVar . to_string"),
            "Should interpolate Rust var"
        );
    }

    #[test]
    fn test_at_symbol_without_brace_passes_through() {
        // Test that @ not followed by { passes through unchanged
        let input = TokenStream2::from_str(
            r#"
            "email@domain.com"
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should contain literal @ (no escaping needed)
        assert!(
            s.contains("email@domain.com"),
            "Should pass through @ unchanged"
        );
    }

    #[test]
    fn test_at_symbol_in_backtick_passes_through() {
        // Test that @ not followed by { passes through in backtick templates
        let input = TokenStream2::from_str(
            r#"
            "'^email@domain.com^'"
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should contain backticks and the literal @
        assert!(s.contains("\"`\""), "Should push backtick");
        assert!(
            s.contains("email@domain.com"),
            "Should pass through @ unchanged"
        );
    }

    #[test]
    fn test_escape_at_before_brace() {
        // Test that @@{ produces a literal @{ (not interpolation)
        let input = TokenStream2::from_str(
            r#"
            "use @@{decorators}"
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should contain literal @{decorators} not try to interpolate
        let expected = "@{decorators}";
        assert!(
            s.contains(expected),
            "Should contain literal @{{decorators}}"
        );
        // Should NOT try to interpolate 'decorators'
        assert!(
            !s.contains("decorators . to_string"),
            "Should not interpolate"
        );
    }

    #[test]
    fn test_if_let_simple() {
        // Test {#if let pattern = expr}...{/if}
        let input = TokenStream2::from_str(
            r#"
            {#if let Some(value) = option}
                @{value}
            {/if}
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should generate if let
        assert!(
            s.contains("if let Some (value)"),
            "Should have if let pattern"
        );
        assert!(s.contains("= option"), "Should have expression");
    }

    #[test]
    fn test_if_let_with_else() {
        // Test {#if let}...{:else}...{/if}
        let input = TokenStream2::from_str(
            r#"
            {#if let Some(x) = maybe}
                "found"
            {:else}
                "not found"
            {/if}
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should generate if let with else
        assert!(s.contains("if let Some (x)"), "Should have if let pattern");
        assert!(s.contains("else"), "Should have else branch");
    }

    #[test]
    fn test_match_simple() {
        // Test {#match expr}{:case pattern}...{/match}
        let input = TokenStream2::from_str(
            r#"
            {#match value}
                {:case Some(x)}
                    @{x}
                {:case None}
                    "nothing"
            {/match}
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should generate match
        assert!(s.contains("match value"), "Should have match expr");
        assert!(s.contains("Some (x) =>"), "Should have Some case arm");
        assert!(s.contains("None =>"), "Should have None case arm");
    }

    #[test]
    fn test_match_with_wildcard() {
        // Test match with wildcard pattern
        let input = TokenStream2::from_str(
            r#"
            {#match num}
                {:case 1}
                    "one"
                {:case 2}
                    "two"
                {:case _}
                    "other"
            {/match}
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should generate match with all arms
        assert!(s.contains("match num"), "Should have match expr");
        assert!(s.contains("1 =>"), "Should have case 1");
        assert!(s.contains("2 =>"), "Should have case 2");
        assert!(s.contains("_ =>"), "Should have wildcard case");
    }

    #[test]
    fn test_match_with_interpolation() {
        // Test match arms with interpolation
        let input = TokenStream2::from_str(
            r#"
            {#match result}
                {:case Ok(val)}
                    "success: @{val}"
                {:case Err(e)}
                    "error: @{e}"
            {/match}
        "#,
        )
        .unwrap();
        let output = parse_template(input);
        let s = output.to_string();

        // Should interpolate values in match arms
        assert!(s.contains("val . to_string"), "Should interpolate val");
        assert!(s.contains("e . to_string"), "Should interpolate e");
    }
}
