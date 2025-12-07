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
pub fn parse_template(input: TokenStream2) -> syn::Result<TokenStream2> {
    // Parse the tokens into a Rust block that returns a String or a templating error
    let (body, _) = parse_fragment(&mut input.into_iter().peekable(), None)?;

    Ok(quote! {
        {
            let mut __out = String::new();
            #body
            __out
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
    open_span: proc_macro2::Span,
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
                return Err(syn::Error::new(
                    open_span,
                    "Unclosed {:else} block: Missing {/if}",
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
        None => Err(syn::Error::new(
            open_span,
            "Unclosed {#if} block: Missing {/if}",
        )),
        _ => unreachable!(),
    }
}

/// Parse an if-let/else chain starting from the pattern and expression
fn parse_if_let_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    pattern: TokenStream2,
    expr: TokenStream2,
    open_span: proc_macro2::Span,
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
                return Err(syn::Error::new(
                    open_span,
                    "Unclosed {:else} block: Missing {/if}",
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
        None => Err(syn::Error::new(
            open_span,
            "Unclosed {#if let} block: Missing {/if}",
        )),
        _ => unreachable!(),
    }
}

/// Parse match arms starting after {#match expr}
/// Format: {#match expr}{:case pattern1}body1{:case pattern2}body2{/match}
fn parse_match_arms(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    match_expr: TokenStream2,
    open_span: proc_macro2::Span,
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
                return Err(syn::Error::new(
                    open_span,
                    "Unclosed {#match} block: Missing {/match}",
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

/// Check if a string is a TypeScript keyword that usually requires a space
fn is_ts_keyword(s: &str) -> bool {
    let s = s.trim_matches('r').trim_matches('"').trim_matches('\'');
    [
        "return",
        "throw",
        "await",
        "yield",
        "typeof",
        "void",
        "new",
        "case",
        "else",
        "var",
        "let",
        "const",
        "import",
        "export",
        "default",
        "extends",
        "implements",
        "interface",
        "class",
        "enum",
        "function",
        "in",
        "instanceof",
        "of",
        "as",
        "is",
        "from",
        "keyof",
        "unique",
        "readonly",
    ]
    .contains(&s)
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

                // Spacing logic after interpolation
                let next = iter.peek();
                let next_char = match next {
                    Some(TokenTree::Punct(p)) => Some(p.as_char()),
                    _ => None,
                };

                let mut add_space = true;
                if matches!(next_char, Some(c) if ".,;:?()[]{}<>!".contains(c)) {
                    add_space = false;
                } else if let Some(TokenTree::Group(g)) = next {
                    match g.delimiter() {
                        Delimiter::Parenthesis | Delimiter::Bracket => add_space = false,
                        _ => {}
                    }
                }

                // No space if followed by @ (concatenation)
                if matches!(next, Some(TokenTree::Punct(p)) if p.as_char() == '@') {
                    add_space = false;
                }

                // But FORCE space if next is a Keyword (e.g. @{val} as Type)
                // Otherwise default to concatenation for identifiers (make@{Name} -> makeName)
                if let Some(TokenTree::Ident(i)) = next {
                    if is_ts_keyword(&i.to_string()) {
                        add_space = true;
                    } else {
                        // If next is regular identifier, assume concatenation
                        add_space = false;
                    }
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
                            return Err(syn::Error::new(
                                span,
                                "Unclosed {#for} block: Missing {/for}",
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
                    TagType::Else => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::Else))
                        {
                            iter.next(); // Consume
                            return Ok((output, Some(Terminator::Else)));
                        }
                        return Err(syn::Error::new(span, "Unexpected {:else}"));
                    }
                    TagType::ElseIf(cond) => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::ElseIf(_)))
                        {
                            iter.next(); // Consume
                            return Ok((output, Some(Terminator::ElseIf(cond))));
                        }
                        return Err(syn::Error::new(span, "Unexpected {:else if}"));
                    }
                    TagType::EndIf => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndIf))
                        {
                            iter.next(); // Consume
                            return Ok((output, Some(Terminator::EndIf)));
                        }
                        return Err(syn::Error::new(span, "Unexpected {/if}"));
                    }
                    TagType::EndFor => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndFor))
                        {
                            iter.next(); // Consume
                            return Ok((output, Some(Terminator::EndFor)));
                        }
                        return Err(syn::Error::new(span, "Unexpected {/for}"));
                    }
                    TagType::Case(pattern) => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::Case(_)))
                        {
                            iter.next(); // Consume
                            return Ok((output, Some(Terminator::Case(pattern))));
                        }
                        return Err(syn::Error::new(span, "Unexpected {:case}"));
                    }
                    TagType::EndMatch => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndMatch))
                        {
                            iter.next(); // Consume
                            return Ok((output, Some(Terminator::EndMatch)));
                        }
                        return Err(syn::Error::new(span, "Unexpected {/match}"));
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
                let next_is_at = matches!(next, Some(TokenTree::Punct(p)) if p.as_char() == '@');
                let next_char = match next {
                    Some(TokenTree::Punct(p)) => Some(p.as_char()),
                    _ => None,
                };

                // Emit token string
                output.extend(quote! {
                    __out.push_str(#s);
                });

                // Decide whether to append a space
                let mut add_space = true;

                if is_joint {
                    add_space = false;
                } else if next_is_at {
                    // Interpolation logic:
                    // `make@{name}` (concatenation) -> No space
                    // `obj.@{prop}` (member access) -> No space
                    // `val = @{expr}` (assignment) -> Space
                    if is_ident {
                        // If it's a keyword (e.g. "as @{Type}", "instanceof @{Type}"), we MUST keep the space.
                        // If it's a regular ident (e.g. "get@{Prop}"), we assume concatenation.
                        add_space = is_ts_keyword(&s);
                    }
                    if let Some('.') = punct_char {
                        add_space = false;
                    }
                } else if is_ident {
                    // Identifiers usually need space, EXCEPT when followed by:
                    // - Member access (.)
                    // - Function calls ( () )
                    // - Indexing ( [] )
                    // - Generics ( < )
                    // - Punctuation delimiters ( , ; : ? )
                    // - End of expression/block ( ) ] } )
                    // - Unary/Post-fix operators ( ! )

                    if matches!(next_char, Some(c) if ".,;:?()[]{}<>!".contains(c)) {
                        add_space = false;
                    } else if let Some(TokenTree::Group(g)) = next {
                        match g.delimiter() {
                            Delimiter::Parenthesis | Delimiter::Bracket => add_space = false,
                            _ => {}
                        }
                    }

                    // Special handling for interpolation @
                    if next_is_at {
                        // Check if this identifier is a keyword that requires a space
                        // e.g. `return @{val}` vs `make@{Name}`
                        if !is_ts_keyword(&s) {
                            add_space = false;
                        }
                    }
                } else if let Some(c) = punct_char {
                    // Punctuation specific rules
                    match c {
                        '.' => add_space = false,             // obj.prop
                        '!' => add_space = false,             // !unary or non-null!
                        '(' | '[' | '{' => add_space = false, // Openers: (expr)
                        '<' => add_space = false,             // Generics: Type<T>, or x<y (compact)
                        '@' => add_space = false,             // Decorator: @Dec
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
