//! Svelte-style templating for TypeScript code generation
//!
//! Provides a template syntax with interpolation and control flow:
//! - `#{expr}` - Interpolate expressions
//! - `{#if cond}...{/if}` - Conditional blocks
//! - `{:else}` - Else clause
//! - `{#each list as item}...{/each}` - Iteration

use proc_macro2::{Delimiter, TokenStream as TokenStream2, TokenTree, Group};
use quote::{quote, ToTokens};
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
#[derive(Debug, PartialEq, Clone, Copy)]
enum Terminator {
    Else,
    EndIf,
    EndEach,
}

// Analyzes a braces group { ... } to see if it's a Macro Tag or just TS code
enum TagType {
    If(TokenStream2),
    Each(TokenStream2, TokenStream2), // collection, item_name
    Else,
    EndIf,
    EndEach,
    Block, // Standard TypeScript Block { ... }
}

fn analyze_tag(g: &Group) -> TagType {
    let tokens: Vec<TokenTree> = g.stream().into_iter().collect();
    if tokens.len() < 2 {
        return TagType::Block;
    }

    // Check for {# ...}
    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '#'
    {
        if i == "if" {
            // Format: {#if condition}
            let cond: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::If(cond);
        }
        if i == "each" {
            // Format: {#each users as user}
            let mut list = TokenStream2::new();
            let mut item = TokenStream2::new();
            let mut seen_as = false;

            // Simple splitter on "as" keyword
            for t in tokens.iter().skip(2) {
                 if let TokenTree::Ident(id) = t
                     && id == "as" && !seen_as
                 {
                     seen_as = true;
                     continue;
                 }
                 if !seen_as {
                     t.to_tokens(&mut list);
                 } else {
                     t.to_tokens(&mut item);
                 }
            }
            return TagType::Each(list, item);
        }
    }

    // Check for {: ...} (Else)
    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == ':' && i == "else"
    {
        return TagType::Else;
    }

    // Check for {/ ...} (End tags)
    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '/'
    {
        if i == "if" { return TagType::EndIf; }
        if i == "each" { return TagType::EndEach; }
    }

    TagType::Block
}

/// Recursive function to parse tokens until a terminator is found
fn parse_fragment(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    stop_at: Option<&[Terminator]>,
) -> (TokenStream2, Option<Terminator>) {
    let mut output = TokenStream2::new();

    while let Some(token) = iter.peek().cloned() {
        match &token {
            // Case 1: Interpolation #{ expr }
            TokenTree::Punct(p) if p.as_char() == '#' => {
                // Check if the NEXT token is a Group { ... }
                let p_clone = p.clone();
                iter.next(); // Consume '#'

                // Look ahead
                let is_group = matches!(iter.peek(), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace);

                if is_group {
                    // It IS interpolation: #{ expr }
                    if let Some(TokenTree::Group(g)) = iter.next() {
                         let content = g.stream();
                         output.extend(quote! {
                            __out.push_str(&#content.to_string());
                        });
                    }
                } else {
                    // It is just a literal '#' (like in private class fields #foo)
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

                        // Recursively parse True block
                        let (true_block, terminator) = parse_fragment(iter, Some(&[Terminator::Else, Terminator::EndIf]));

                        match terminator {
                            Some(Terminator::Else) => {
                                // Hit {:else}, parse False block
                                let (false_block, term2) = parse_fragment(iter, Some(&[Terminator::EndIf]));
                                if term2 != Some(Terminator::EndIf) {
                                     panic!("Unclosed {{#if}} block: Missing {{/if}} after {{:else}}");
                                }
                                output.extend(quote! {
                                    if #cond {
                                        #true_block
                                    } else {
                                        #false_block
                                    }
                                });
                            }
                            Some(Terminator::EndIf) => {
                                output.extend(quote! {
                                    if #cond {
                                        #true_block
                                    }
                                });
                            }
                            _ => panic!("Unclosed {{#if}} block: Missing {{/if}} or {{:else}}"),
                        }
                    }
                    TagType::Each(list, item) => {
                        iter.next(); // Consume {#each}

                        let (body, terminator) = parse_fragment(iter, Some(&[Terminator::EndEach]));
                        if terminator != Some(Terminator::EndEach) {
                            panic!("Unclosed {{#each}} block: Missing {{/each}}");
                        }

                        output.extend(quote! {
                            for #item in #list {
                                #body
                            }
                        });
                    }
                    TagType::Else => {
                        if let Some(stops) = stop_at
                            && stops.contains(&Terminator::Else)
                        {
                            iter.next(); // Consume
                            return (output, Some(Terminator::Else));
                        }
                        panic!("Unexpected {{:else}}");
                    }
                    TagType::EndIf => {
                        if let Some(stops) = stop_at
                            && stops.contains(&Terminator::EndIf)
                        {
                            iter.next(); // Consume
                            return (output, Some(Terminator::EndIf));
                        }
                        panic!("Unexpected {{/if}}");
                    }
                    TagType::EndEach => {
                        if let Some(stops) = stop_at
                            && stops.contains(&Terminator::EndEach)
                        {
                            iter.next(); // Consume
                            return (output, Some(Terminator::EndEach));
                        }
                        panic!("Unexpected {{/each}}");
                    }
                    TagType::Block => {
                         // Regular TS Block { ... }
                         // Recurse to allow macros inside standard TS objects
                         iter.next(); // Consume
                         let inner_stream = g.stream();

                         output.extend(quote! { __out.push_str("{"); });
                         let (inner_parsed, _) = parse_fragment(&mut inner_stream.into_iter().peekable(), None);
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
                let (inner_parsed, _) = parse_fragment(&mut g.stream().into_iter().peekable(), None);
                output.extend(inner_parsed);
                output.extend(quote! { __out.push_str(#close); });
            }

            // Case 4: Plain Text
            _ => {
                let t = iter.next().unwrap();
                // Simple stringification. For better TS formatting, you might want custom logic.
                let s = t.to_string();
                output.extend(quote! {
                    __out.push_str(#s);
                    __out.push_str(" ");
                });
            }
        }
    }

    (output, None)
}
