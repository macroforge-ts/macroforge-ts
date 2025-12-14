//! Rust-style templating for TypeScript code generation
//!
//! Provides a template syntax with interpolation and control flow:
//! - `@{expr}` - Interpolate expressions (calls `.to_string()`)
//! - `{| content |}` - Ident block: concatenates content without spaces (e.g., `{|get@{name}|}` → `getUser`)
//! - `{> "comment" <}` - Block comment: outputs `/* comment */` (string preserves whitespace)
//! - `{>> "doc" <<}` - Doc comment: outputs `/** doc */` (string preserves whitespace)
//! - `@@{` - Escape for literal `@{` (e.g., `"@@{foo}"` → `@{foo}`)
//! - `"string @{expr}"` - String interpolation (auto-detected)
//! - `"'^template ${expr}^'"` - JS backtick template literal (outputs `` `template ${expr}` ``)
//! - `{#if cond}...{/if}` - Conditional blocks
//! - `{#if let pattern = expr}...{/if}` - Pattern matching if-let blocks
//! - `{:else}` - Else clause
//! - `{:else if cond}` - Else-if clause
//! - `{#match expr}{:case pattern}...{/match}` - Match blocks with case arms
//! - `{#for item in list}...{/for}` - Iteration
//! - `{#while cond}...{/while}` - While loop
//! - `{#while let pattern = expr}...{/while}` - While-let pattern matching loop
//! - `{$let name = expr}` - Local constants
//! - `{$let mut name = expr}` - Mutable local binding
//! - `{$do expr}` - Execute side-effectful expression (discard result)
//! - `{$typescript stream}` - Inject a TsStream, preserving its source and runtime_patches (imports)
//!
//! Note: A single `@` not followed by `{` passes through unchanged (e.g., `email@domain.com`).

use proc_macro2::{Delimiter, Group, Span, TokenStream as TokenStream2, TokenTree};
use quote::{ToTokens, quote};
use std::iter::Peekable;

/// Creates a syntax error with contextual information about position in the template
fn template_error(span: Span, message: &str, context: Option<&str>) -> syn::Error {
    let full_message = if let Some(ctx) = context {
        format!("{}\n  --> in: {}", message, ctx)
    } else {
        message.to_string()
    };
    syn::Error::new(span, full_message)
}

/// Parse the template stream and generate code to build a TypeScript string
/// Returns a tuple of (String, Vec<Patch>) to support TsStream injection via {$typescript}
pub fn parse_template(input: TokenStream2) -> syn::Result<TokenStream2> {
    // Parse the tokens into a Rust block that returns a String or a templating error
    let (body, _) = parse_fragment(&mut input.into_iter().peekable(), None)?;

    Ok(quote! {
        {
            let mut __out = String::new();
            let mut __patches: Vec<macroforge_ts::ts_syn::abi::Patch> = Vec::new();
            #body
            (__out, __patches)
        }
    })
}

// Terminators tell the parser when to stop current recursion level
#[derive(Debug, Clone)]
enum Terminator {
    Else,
    ElseIf(TokenStream2),
    EndIf,
    EndFor,
    EndWhile,
    Case(TokenStream2),
    EndMatch,
}

// Analyzes a braces group { ... } to see if it's a Macro Tag or just TS code
enum TagType {
    If(TokenStream2),
    IfLet(TokenStream2, TokenStream2),    // pattern, expression
    While(TokenStream2),                  // {#while cond}
    WhileLet(TokenStream2, TokenStream2), // {#while let pattern = expr}
    For(TokenStream2, TokenStream2),      // item_name, collection
    Match(TokenStream2),                  // expression to match
    Else,
    ElseIf(TokenStream2),
    Case(TokenStream2), // pattern for match arm
    EndIf,
    EndFor,
    EndWhile,
    EndMatch,
    Let(TokenStream2),
    LetMut(TokenStream2),     // {$let mut name = expr}
    Do(TokenStream2),         // {$do expr} - side-effectful expression
    Typescript(TokenStream2), // {$typescript stream_expr} - inject TsStream with patches
    IdentBlock,                // {| ... |} - identifier block with no internal spacing
    BlockComment(String),      // {> "string" <} - block comment /* string */
    DocComment(String),        // {>> "string" <<} - doc comment /** string */
    Block,                     // Standard TypeScript Block { ... }
}

fn analyze_tag(g: &Group) -> TagType {
    let tokens: Vec<TokenTree> = g.stream().into_iter().collect();

    // Check for {| ... |} ident block - must have at least | and |
    if tokens.len() >= 2
        && let (Some(TokenTree::Punct(first)), Some(TokenTree::Punct(last))) =
            (tokens.first(), tokens.last())
        && first.as_char() == '|'
        && last.as_char() == '|'
    {
        return TagType::IdentBlock;
    }

    // Check for {>> "string" <<} doc comment - must have >> string <<
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
        // Extract the string literal in the middle
        if let Some(TokenTree::Literal(lit)) = tokens.get(2) {
            let content = extract_string_literal(lit);
            return TagType::DocComment(content);
        }
        // Fallback: join remaining tokens as string (for backwards compat)
        let content = tokens_to_spaced_string(&tokens[2..tokens.len() - 2]);
        return TagType::DocComment(content);
    }

    // Check for {> "string" <} block comment - must have > string <
    if tokens.len() >= 3
        && let (Some(TokenTree::Punct(first)), Some(TokenTree::Punct(last))) =
            (tokens.first(), tokens.last())
        && first.as_char() == '>'
        && last.as_char() == '<'
    {
        // Extract the string literal in the middle
        if let Some(TokenTree::Literal(lit)) = tokens.get(1) {
            let content = extract_string_literal(lit);
            return TagType::BlockComment(content);
        }
        // Fallback: join remaining tokens as string (for backwards compat)
        let content = tokens_to_spaced_string(&tokens[1..tokens.len() - 1]);
        return TagType::BlockComment(content);
    }

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

        if i == "while" {
            // Check for {#while let pattern = expr}
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

            // Simple {#while condition}
            let cond: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::While(cond);
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

    // Check for {$ ...} tags (let, let mut, do, typescript)
    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '$'
    {
        if i == "let" {
            // Check for {$let mut name = expr}
            if let Some(TokenTree::Ident(mut_kw)) = tokens.get(2)
                && mut_kw == "mut"
            {
                let body: TokenStream2 =
                    tokens.iter().skip(3).map(|t| t.to_token_stream()).collect();
                return TagType::LetMut(body);
            }
            // Format: {$let name = expr}
            let body: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Let(body);
        }
        if i == "do" {
            // Format: {$do expr} - execute side-effectful expression
            let expr: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Do(expr);
        }
        if i == "typescript" {
            // Format: {$typescript stream_expr}
            let expr: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Typescript(expr);
        }
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
        if i == "while" {
            return TagType::EndWhile;
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
    open_span: Span,
) -> syn::Result<TokenStream2> {
    // Parse the true block, stopping at {:else}, {:else if}, or {/if}
    let (true_block, terminator) = parse_fragment(
        iter,
        Some(&[
            Terminator::Else,
            Terminator::ElseIf(TokenStream2::new()),
            Terminator::EndIf,
        ]),
    )?;

    match terminator {
        Some(Terminator::EndIf) => {
            // Simple if without else
            Ok(quote! {
                if #initial_cond {
                    #true_block
                }
            })
        }
        Some(Terminator::Else) => {
            // if with else - parse else block until {/if}
            let (else_block, terminator) = parse_fragment(iter, Some(&[Terminator::EndIf]))?;
            if !matches!(terminator, Some(Terminator::EndIf)) {
                return Err(template_error(
                    open_span,
                    "Unclosed {:else} block: Missing {/if}",
                    Some("{:else}..."),
                ));
            }
            Ok(quote! {
                if #initial_cond {
                    #true_block
                } else {
                    #else_block
                }
            })
        }
        Some(Terminator::ElseIf(else_if_cond)) => {
            // if with else if - recursively parse the else-if chain
            // For the recursive call, we should ideally find the span of the else-if tag.
            // But keeping open_span is acceptable for now as it points to the start of the whole chain.
            let else_if_chain = parse_if_chain(iter, else_if_cond, open_span)?;
            Ok(quote! {
                if #initial_cond {
                    #true_block
                } else {
                    #else_if_chain
                }
            })
        }
        None => Err(template_error(
            open_span,
            "Unclosed {#if} block: Missing {/if}",
            Some("{#if condition}..."),
        )),
        _ => unreachable!(),
    }
}

/// Parse an if-let/else chain starting from the pattern and expression
fn parse_if_let_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    pattern: TokenStream2,
    expr: TokenStream2,
    open_span: Span,
) -> syn::Result<TokenStream2> {
    // Parse the true block, stopping at {:else} or {/if}
    let (true_block, terminator) =
        parse_fragment(iter, Some(&[Terminator::Else, Terminator::EndIf]))?;

    match terminator {
        Some(Terminator::EndIf) => {
            // Simple if let without else
            Ok(quote! {
                if let #pattern = #expr {
                    #true_block
                }
            })
        }
        Some(Terminator::Else) => {
            // if let with else - parse else block until {/if}
            let (else_block, terminator) = parse_fragment(iter, Some(&[Terminator::EndIf]))?;
            if !matches!(terminator, Some(Terminator::EndIf)) {
                return Err(template_error(
                    open_span,
                    "Unclosed {:else} block in {#if let}: Missing {/if}",
                    Some("{#if let pattern = expr}{:else}..."),
                ));
            }
            Ok(quote! {
                if let #pattern = #expr {
                    #true_block
                } else {
                    #else_block
                }
            })
        }
        None => Err(template_error(
            open_span,
            "Unclosed {#if let} block: Missing {/if}",
            Some("{#if let pattern = expr}..."),
        )),
        _ => unreachable!(),
    }
}

/// Parse a while loop starting from the condition
fn parse_while_loop(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    cond: TokenStream2,
    open_span: Span,
) -> syn::Result<TokenStream2> {
    let (body, terminator) = parse_fragment(iter, Some(&[Terminator::EndWhile]))?;

    if !matches!(terminator, Some(Terminator::EndWhile)) {
        return Err(template_error(
            open_span,
            "Unclosed {#while} block: Missing {/while}",
            Some("{#while condition}..."),
        ));
    }

    Ok(quote! {
        while #cond {
            #body
        }
    })
}

/// Parse a while-let loop starting from the pattern and expression
fn parse_while_let_loop(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    pattern: TokenStream2,
    expr: TokenStream2,
    open_span: Span,
) -> syn::Result<TokenStream2> {
    let (body, terminator) = parse_fragment(iter, Some(&[Terminator::EndWhile]))?;

    if !matches!(terminator, Some(Terminator::EndWhile)) {
        return Err(template_error(
            open_span,
            "Unclosed {#while let} block: Missing {/while}",
            Some("{#while let pattern = expr}..."),
        ));
    }

    Ok(quote! {
        while let #pattern = #expr {
            #body
        }
    })
}

/// Parse match arms starting after {#match expr}
/// Format: {#match expr}{:case pattern1}body1{:case pattern2}body2{/match}
fn parse_match_arms(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    match_expr: TokenStream2,
    open_span: Span,
) -> syn::Result<TokenStream2> {
    let mut arms = TokenStream2::new();
    let mut current_pattern: Option<TokenStream2> = None;

    loop {
        // Parse until we hit {:case} or {/match}
        let (body, terminator) = parse_fragment(
            iter,
            Some(&[Terminator::Case(TokenStream2::new()), Terminator::EndMatch]),
        )?;

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
                return Err(template_error(
                    open_span,
                    "Unclosed {#match} block: Missing {/match}",
                    Some("{#match expr}{:case pattern}...{/match}"),
                ));
            }
            _ => unreachable!(),
        }
    }

    Ok(quote! {
        match #match_expr {
            #arms
        }
    })
}

/// Parse tokens without adding any spaces - for {| |} ident blocks
fn parse_fragment_no_spacing(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
) -> syn::Result<TokenStream2> {
    let mut output = TokenStream2::new();

    while let Some(token) = iter.peek().cloned() {
        match &token {
            // Handle @{ expr } interpolation
            TokenTree::Punct(p) if p.as_char() == '@' => {
                iter.next(); // Consume '@'

                let is_group = matches!(iter.peek(), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace);

                if is_group {
                    if let Some(TokenTree::Group(g)) = iter.next() {
                        let content = g.stream();
                        output.extend(quote! {
                            __out.push_str(&#content.to_string());
                        });
                    }
                } else {
                    output.extend(quote! { __out.push_str("@"); });
                }
            }

            // Handle nested groups (but not ident blocks - those are already consumed)
            TokenTree::Group(g) => {
                iter.next();
                let (open, close) = match g.delimiter() {
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::Brace => ("{", "}"),
                    Delimiter::None => ("", ""),
                };
                output.extend(quote! { __out.push_str(#open); });
                let inner = parse_fragment_no_spacing(&mut g.stream().into_iter().peekable())?;
                output.extend(inner);
                output.extend(quote! { __out.push_str(#close); });
            }

            // All other tokens - just emit, no spacing
            _ => {
                let t = iter.next().unwrap();
                let s = t.to_string();
                output.extend(quote! { __out.push_str(#s); });
                // NO space added - that's the point of ident blocks
            }
        }
    }

    Ok(output)
}

/// Recursive function to parse tokens until a terminator is found
fn parse_fragment(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    stop_at: Option<&[Terminator]>,
) -> syn::Result<(TokenStream2, Option<Terminator>)> {
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

                // Spacing logic after interpolation: always add space unless followed by punctuation
                // Use {| |} for explicit no-space concatenation
                let next = iter.peek();
                let next_char = match next {
                    Some(TokenTree::Punct(p)) => Some(p.as_char()),
                    _ => None,
                };

                let mut add_space = true;

                // No space at end of stream (group delimiter like ) will follow)
                if next.is_none() {
                    add_space = false;
                }

                // No space before punctuation
                if matches!(next_char, Some(c) if ".,;:?()[]{}<>!".contains(c)) {
                    add_space = false;
                }

                // No space before ( or [ groups (function calls, indexing)
                if let Some(TokenTree::Group(g)) = next {
                    match g.delimiter() {
                        Delimiter::Parenthesis | Delimiter::Bracket => add_space = false,
                        _ => {}
                    }
                }

                // No space if followed by @ (for @{a}@{b} patterns inside {| |})
                if matches!(next, Some(TokenTree::Punct(p)) if p.as_char() == '@') {
                    add_space = false;
                }

                if add_space {
                    output.extend(quote! { __out.push_str(" "); });
                }
            }

            // Case 2: Groups { ... } - Could be Tag or Block
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => {
                let tag = analyze_tag(g);
                let span = g.span();

                match tag {
                    TagType::If(cond) => {
                        iter.next(); // Consume {#if}
                        output.extend(parse_if_chain(iter, cond, span)?);
                    }
                    TagType::IfLet(pattern, expr) => {
                        iter.next(); // Consume {#if let}
                        output.extend(parse_if_let_chain(iter, pattern, expr, span)?);
                    }
                    TagType::For(item, list) => {
                        iter.next(); // Consume {#for}

                        let (body, terminator) = parse_fragment(iter, Some(&[Terminator::EndFor]))?;
                        if !matches!(terminator, Some(Terminator::EndFor)) {
                            return Err(template_error(
                                span,
                                "Unclosed {#for} block: Missing {/for}",
                                Some("{#for item in collection}..."),
                            ));
                        }

                        output.extend(quote! {
                            for #item in #list {
                                #body
                            }
                        });
                    }
                    TagType::Match(expr) => {
                        iter.next(); // Consume {#match}
                        output.extend(parse_match_arms(iter, expr, span)?);
                    }
                    TagType::While(cond) => {
                        iter.next(); // Consume {#while}
                        output.extend(parse_while_loop(iter, cond, span)?);
                    }
                    TagType::WhileLet(pattern, expr) => {
                        iter.next(); // Consume {#while let}
                        output.extend(parse_while_let_loop(iter, pattern, expr, span)?);
                    }
                    TagType::Else => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::Else))
                        {
                            iter.next(); // Consume
                            return Ok((output, Some(Terminator::Else)));
                        }
                        return Err(template_error(
                            span,
                            "Unexpected {:else} - not inside an {#if} block",
                            None,
                        ));
                    }
                    TagType::ElseIf(cond) => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::ElseIf(_)))
                        {
                            iter.next(); // Consume
                            return Ok((output, Some(Terminator::ElseIf(cond))));
                        }
                        return Err(template_error(
                            span,
                            "Unexpected {:else if} - not inside an {#if} block",
                            None,
                        ));
                    }
                    TagType::EndIf => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndIf))
                        {
                            iter.next(); // Consume
                            return Ok((output, Some(Terminator::EndIf)));
                        }
                        return Err(template_error(
                            span,
                            "Unexpected {/if} - no matching {#if} block",
                            None,
                        ));
                    }
                    TagType::EndFor => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndFor))
                        {
                            iter.next(); // Consume
                            return Ok((output, Some(Terminator::EndFor)));
                        }
                        return Err(template_error(
                            span,
                            "Unexpected {/for} - no matching {#for} block",
                            None,
                        ));
                    }
                    TagType::EndWhile => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndWhile))
                        {
                            iter.next(); // Consume
                            return Ok((output, Some(Terminator::EndWhile)));
                        }
                        return Err(template_error(
                            span,
                            "Unexpected {/while} - no matching {#while} block",
                            None,
                        ));
                    }
                    TagType::Case(pattern) => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::Case(_)))
                        {
                            iter.next(); // Consume
                            return Ok((output, Some(Terminator::Case(pattern))));
                        }
                        return Err(template_error(
                            span,
                            "Unexpected {:case} - not inside a {#match} block",
                            None,
                        ));
                    }
                    TagType::EndMatch => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndMatch))
                        {
                            iter.next(); // Consume
                            return Ok((output, Some(Terminator::EndMatch)));
                        }
                        return Err(template_error(
                            span,
                            "Unexpected {/match} - no matching {#match} block",
                            None,
                        ));
                    }
                    TagType::Let(body) => {
                        iter.next(); // Consume {$let ...}
                        output.extend(quote! {
                            let #body;
                        });
                    }
                    TagType::LetMut(body) => {
                        iter.next(); // Consume {$let mut ...}
                        output.extend(quote! {
                            let mut #body;
                        });
                    }
                    TagType::Do(expr) => {
                        iter.next(); // Consume {$do ...}
                        output.extend(quote! {
                            #expr;
                        });
                    }
                    TagType::Typescript(expr) => {
                        iter.next(); // Consume {$typescript ...}
                        output.extend(quote! {
                            {
                                let __ts_stream = #expr;
                                __out.push_str(__ts_stream.source());
                                __patches.extend(__ts_stream.runtime_patches);
                            }
                        });
                    }
                    TagType::IdentBlock => {
                        iter.next(); // Consume {| ... |}

                        // Get the content between the | markers
                        let inner_tokens: Vec<TokenTree> = g.stream().into_iter().collect();
                        // Skip first | and last |, extract content in between
                        if inner_tokens.len() >= 2 {
                            let content: TokenStream2 = inner_tokens[1..inner_tokens.len() - 1]
                                .iter()
                                .map(|t| t.to_token_stream())
                                .collect();

                            // Parse with no-spacing mode
                            let inner_output =
                                parse_fragment_no_spacing(&mut content.into_iter().peekable())?;
                            output.extend(inner_output);
                        }
                        // No space added after the block - that's the point
                    }
                    TagType::BlockComment(content) => {
                        iter.next(); // Consume {> "..." <}
                        output.extend(quote! { __out.push_str("/* "); });
                        output.extend(quote! { __out.push_str(#content); });
                        output.extend(quote! { __out.push_str(" */"); });
                    }
                    TagType::DocComment(content) => {
                        iter.next(); // Consume {>> "..." <<}
                        output.extend(quote! { __out.push_str("/** "); });
                        output.extend(quote! { __out.push_str(#content); });
                        output.extend(quote! { __out.push_str(" */"); });
                    }
                    TagType::Block => {
                        // Regular TS Block { ... }
                        // Recurse to allow macros inside standard TS objects
                        iter.next(); // Consume
                        let inner_stream = g.stream();

                        output.extend(quote! { __out.push_str("{"); });
                        let (inner_parsed, _) =
                            parse_fragment(&mut inner_stream.into_iter().peekable(), None)?;
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
                    parse_fragment(&mut g.stream().into_iter().peekable(), None)?;
                output.extend(inner_parsed);
                output.extend(quote! { __out.push_str(#close); });
            }

            // Case 4a: Backtick template literals "'^...^'" -> `...`
            TokenTree::Literal(lit) if is_backtick_template(lit) => {
                iter.next(); // Consume
                let processed = process_backtick_template(lit)?;
                output.extend(processed);
                output.extend(quote! { __out.push_str(" "); });
            }

            // Case 4b: String literals with interpolation
            TokenTree::Literal(lit) if is_string_literal(lit) => {
                iter.next(); // Consume
                let interpolated = interpolate_string_literal(lit)?;
                output.extend(interpolated);
                output.extend(quote! { __out.push_str(" "); });
            }

            // Case 5: Plain Text
            _ => {
                let t = iter.next().unwrap();
                let s = t.to_string();

                // Analyze current token
                let is_ident = matches!(&t, TokenTree::Ident(_));
                let punct_char = if let TokenTree::Punct(p) = &t {
                    Some(p.as_char())
                } else {
                    None
                };
                let is_joint = if let TokenTree::Punct(p) = &t {
                    p.spacing() == proc_macro2::Spacing::Joint
                } else {
                    false
                };

                // Analyze next token
                let next = iter.peek();
                let next_char = match next {
                    Some(TokenTree::Punct(p)) => Some(p.as_char()),
                    _ => None,
                };

                // Emit token string
                output.extend(quote! {
                    __out.push_str(#s);
                });

                // Decide whether to append a space
                // Simplified: always add space unless followed by punctuation
                // Use {| |} for explicit no-space concatenation
                let mut add_space = true;

                // No space at end of stream (group delimiter like ) will follow)
                if next.is_none() || is_joint {
                    add_space = false;
                } else if is_ident {
                    // Identifiers need space, EXCEPT when followed by punctuation or groups
                    if matches!(next_char, Some(c) if ".,;:?()[]{}<>!".contains(c)) {
                        add_space = false;
                    } else if let Some(TokenTree::Group(g)) = next {
                        match g.delimiter() {
                            Delimiter::Parenthesis | Delimiter::Bracket => add_space = false,
                            _ => {}
                        }
                    }
                    // Always add space before @ interpolation (use {| |} for concatenation)
                } else if let Some(c) = punct_char {
                    // Punctuation specific rules
                    match c {
                        '.' => add_space = false,             // obj.prop
                        '!' => add_space = false,             // !unary or non-null!
                        '(' | '[' | '{' => add_space = false, // Openers: (expr)
                        '<' | '>' => add_space = false,       // Generics: Type<T> or T>(...) (compact)
                        '@' => add_space = false,             // Decorator: @Dec
                        '$' => add_space = false,             // Svelte runes: $state, $derived
                        _ => {}
                    }

                    // Never add space if next is a closing delimiter or separator
                    if matches!(next_char, Some(nc) if ".,;)]}>".contains(nc)) {
                        add_space = false;
                    }
                } else {
                    // Groups/Literals
                    // Prevent space if next is punctuation like . , ; ) ] >
                    if matches!(next_char, Some(nc) if ".,;)]}>".contains(nc)) {
                        add_space = false;
                    }
                }

                if add_space {
                    output.extend(quote! { __out.push_str(" "); });
                }
            }
        }
    }

    Ok((output, None))
}

/// Converts a slice of tokens to a string with spaces between tokens.
/// This preserves the logical structure while adding consistent spacing.
fn tokens_to_spaced_string(tokens: &[TokenTree]) -> String {
    let mut result = String::new();
    for (i, token) in tokens.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push_str(&token.to_string());
    }
    result
}

/// Extracts the content from a string literal token, removing quotes.
fn extract_string_literal(lit: &proc_macro2::Literal) -> String {
    let s = lit.to_string();
    // Handle regular strings "..."
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        // Unescape the string content
        let inner = &s[1..s.len() - 1];
        return unescape_string(inner);
    }
    // Handle raw strings r"..." or r#"..."#
    if s.starts_with("r\"") && s.ends_with('"') {
        return s[2..s.len() - 1].to_string();
    }
    if s.starts_with("r#\"") && s.ends_with("\"#") {
        return s[3..s.len() - 2].to_string();
    }
    // Fallback: return as-is
    s
}

/// Unescape common escape sequences in a string.
fn unescape_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
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
fn process_backtick_template(lit: &proc_macro2::Literal) -> syn::Result<TokenStream2> {
    let raw = lit.to_string();
    let span = lit.span();

    // Extract content between '^...^' markers
    let content = if raw.starts_with("\"'^") && raw.ends_with("^'\"") {
        &raw[3..raw.len() - 3]
    } else if raw.starts_with("r\"'^") && raw.ends_with("^'\"") {
        &raw[4..raw.len() - 3]
    } else if raw.starts_with("r#\"'^") && raw.ends_with("^'\"#") {
        &raw[5..raw.len() - 4]
    } else {
        return Ok(quote! { __out.push_str(#raw); });
    };

    // Check for common mistakes: control flow tags inside template strings
    if content.contains("{#") || content.contains("{/") || content.contains("{:") {
        return Err(template_error(
            span,
            "Template control flow tags cannot be used inside backtick template literals",
            Some(&format!(
                "\"'^{}...^'\"",
                content.chars().take(40).collect::<String>()
            )),
        ));
    }

    // Check if there are any @{} interpolations or @@ escapes
    if !content.contains('@') {
        // No @ at all, output the backtick string as-is
        // The content may contain ${} for JS interpolation, which passes through
        let mut output = TokenStream2::new();
        output.extend(quote! { __out.push_str("`"); });
        output.extend(quote! { __out.push_str(#content); });
        output.extend(quote! { __out.push_str("`"); });
        return Ok(output);
    }

    // Handle @{} Rust interpolations and @@ escapes within the backtick template
    let mut output = TokenStream2::new();
    output.extend(quote! { __out.push_str("`"); });

    let mut chars = content.chars().peekable();
    let mut current_literal = String::new();
    let mut char_pos = 0usize;

    while let Some(c) = chars.next() {
        char_pos += 1;
        if c == '@' {
            match chars.peek() {
                Some(&'@') => {
                    // @@ -> literal @
                    chars.next(); // Consume second @
                    char_pos += 1;
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
                    char_pos += 1;
                    let expr_start_pos = char_pos;

                    // Collect expression until matching '}'
                    let mut expr_str = String::new();
                    let mut brace_depth = 1;

                    for ec in chars.by_ref() {
                        char_pos += 1;
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

                    // Check for unclosed brace
                    if brace_depth != 0 {
                        return Err(template_error(
                            span,
                            &format!("Unclosed @{{}} interpolation at position {}", expr_start_pos),
                            Some(&format!("@{{{}", expr_str)),
                        ));
                    }

                    // Parse the expression and generate interpolation code
                    match syn::parse_str::<syn::Expr>(&expr_str) {
                        Ok(expr) => {
                            output.extend(quote! {
                                __out.push_str(&#expr.to_string());
                            });
                        }
                        Err(parse_err) => {
                            return Err(template_error(
                                span,
                                &format!(
                                    "Invalid Rust expression in backtick template interpolation: {}",
                                    parse_err
                                ),
                                Some(&format!("@{{{}}}", expr_str)),
                            ));
                        }
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
    Ok(output)
}

/// Process a string literal and handle @{expr} interpolations inside it
fn interpolate_string_literal(lit: &proc_macro2::Literal) -> syn::Result<TokenStream2> {
    let raw = lit.to_string();
    let span = lit.span();

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
        return Ok(quote! { __out.push_str(#raw); });
    };

    // Check if there are any interpolations or escapes
    if !content.contains('@') {
        // No @ at all, output the string as-is
        return Ok(quote! { __out.push_str(#raw); });
    }

    // Check for common mistakes: control flow tags inside strings
    if content.contains("{#") || content.contains("{/") || content.contains("{:") {
        return Err(template_error(
            span,
            "Template control flow tags cannot be used inside string literals",
            Some(&format!(
                "\"{}...\"",
                content.chars().take(40).collect::<String>()
            )),
        ));
    }

    // Parse and interpolate
    let mut output = TokenStream2::new();
    let quote_str = quote_char.to_string();
    output.extend(quote! { __out.push_str(#quote_str); });

    let mut chars = content.chars().peekable();
    let mut current_literal = String::new();
    let mut char_pos = 0usize;

    while let Some(c) = chars.next() {
        char_pos += 1;
        if c == '@' {
            match chars.peek() {
                Some(&'@') => {
                    // @@ -> literal @
                    chars.next(); // Consume second @
                    char_pos += 1;
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
                    char_pos += 1;
                    let expr_start_pos = char_pos;

                    // Collect expression until matching '}'
                    let mut expr_str = String::new();
                    let mut brace_depth = 1;

                    for ec in chars.by_ref() {
                        char_pos += 1;
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

                    // Check for unclosed brace
                    if brace_depth != 0 {
                        return Err(template_error(
                            span,
                            &format!("Unclosed @{{}} interpolation at position {}", expr_start_pos),
                            Some(&format!("@{{{}", expr_str)),
                        ));
                    }

                    // Parse the expression and generate interpolation code
                    match syn::parse_str::<syn::Expr>(&expr_str) {
                        Ok(expr) => {
                            output.extend(quote! {
                                __out.push_str(&#expr.to_string());
                            });
                        }
                        Err(parse_err) => {
                            return Err(template_error(
                                span,
                                &format!(
                                    "Invalid Rust expression in string interpolation: {}",
                                    parse_err
                                ),
                                Some(&format!("@{{{}}}", expr_str)),
                            ));
                        }
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
                char_pos += 1;
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

    Ok(output)
}
