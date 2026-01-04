//! Core template parser with span-based spacing preservation.

use proc_macro2::{Delimiter, Span, TokenStream as TokenStream2, TokenTree};
use quote::quote;
use std::iter::Peekable;

use super::control_flow::{
    parse_if_chain, parse_if_let_chain, parse_match_arms, parse_while_chain, parse_while_let_chain,
};
use super::interpolation::{
    interpolate_string_literal, is_backtick_template, is_string_literal, process_backtick_template,
};
use super::spacing::{spacing_between, Pos};
use super::tag::{analyze_tag, TagType};

/// Terminators tell the parser when to stop current recursion level.
#[derive(Debug, Clone)]
pub enum Terminator {
    Else,
    ElseIf(TokenStream2),
    EndIf,
    EndFor,
    EndWhile,
    Case(TokenStream2),
    EndMatch,
}

/// Context for tracking spacing during parsing.
#[derive(Default)]
pub struct SpacingContext {
    /// End position of the previous token (for calculating gaps).
    pub prev_end: Option<Pos>,
}

impl SpacingContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Emit spacing to bridge the gap from prev_end to the current span.
    pub fn emit_spacing_to(&mut self, span: Span) -> TokenStream2 {
        let curr_start = Pos::from_span_start(span);

        let spacing = if let Some(prev) = self.prev_end {
            spacing_between(prev, curr_start)
        } else {
            String::new()
        };

        if spacing.is_empty() {
            TokenStream2::new()
        } else {
            quote! { __out.push_str(#spacing); }
        }
    }

    /// Update prev_end to the end of the given span.
    pub fn advance_past(&mut self, span: Span) {
        self.prev_end = Some(Pos::from_span_end(span));
    }

    /// Update prev_end to a specific position.
    pub fn set_prev_end(&mut self, pos: Pos) {
        self.prev_end = Some(pos);
    }
}

/// Recursive function to parse tokens until a terminator is found.
pub fn parse_fragment(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    stop_at: Option<&[Terminator]>,
) -> syn::Result<(TokenStream2, Option<Terminator>)> {
    let mut ctx = SpacingContext::new();
    parse_fragment_with_ctx(iter, stop_at, &mut ctx)
}

/// Parse fragment with explicit spacing context (for nested parsing).
pub fn parse_fragment_with_ctx(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    stop_at: Option<&[Terminator]>,
    ctx: &mut SpacingContext,
) -> syn::Result<(TokenStream2, Option<Terminator>)> {
    let mut output = TokenStream2::new();

    while let Some(token) = iter.peek().cloned() {
        match &token {
            // Case 1: Interpolation @{ expr }
            TokenTree::Punct(p) if p.as_char() == '@' => {
                let at_span = p.span();
                iter.next(); // Consume '@'

                // Look ahead for { ... }
                let is_group = matches!(iter.peek(), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace);

                if is_group {
                    // Emit spacing before the interpolation
                    output.extend(ctx.emit_spacing_to(at_span));

                    // It IS interpolation: @{ expr }
                    if let Some(TokenTree::Group(g)) = iter.next() {
                        let content = g.stream();
                        output.extend(quote! {
                            __out.push_str(&macroforge_ts::ts_syn::ToTsString::to_ts_string(&#content));
                        });
                        // Track end of the closing brace
                        ctx.advance_past(g.span_close());
                    }
                } else {
                    // It is just a literal '@' - emit with spacing
                    output.extend(ctx.emit_spacing_to(at_span));
                    let s = p.to_string();
                    output.extend(quote! { __out.push_str(#s); });
                    ctx.advance_past(at_span);
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
                        ctx.advance_past(span);
                    }
                    TagType::IfLet(pattern, expr) => {
                        iter.next(); // Consume {#if let}
                        output.extend(parse_if_let_chain(iter, pattern, expr, span)?);
                        ctx.advance_past(span);
                    }
                    TagType::For(item, list) => {
                        iter.next(); // Consume {#for}

                        // Capture spacing from before the {#for} tag
                        let leading_spacing = ctx.emit_spacing_to(span);
                        ctx.advance_past(span);

                        let (body, terminator) = parse_fragment(iter, Some(&[Terminator::EndFor]))?;
                        if !matches!(terminator, Some(Terminator::EndFor)) {
                            return Err(syn::Error::new(
                                span,
                                "Unclosed {#for} block: Missing {/for}",
                            ));
                        }

                        // Emit leading spacing before each iteration
                        output.extend(quote! {
                            for #item in #list {
                                #leading_spacing
                                #body
                            }
                        });
                    }
                    TagType::Match(expr) => {
                        iter.next(); // Consume {#match}
                        output.extend(parse_match_arms(iter, expr, span)?);
                        ctx.advance_past(span);
                    }
                    TagType::While(cond) => {
                        iter.next(); // Consume {#while}
                        output.extend(parse_while_chain(iter, cond, span)?);
                        ctx.advance_past(span);
                    }
                    TagType::WhileLet(pattern, expr) => {
                        iter.next(); // Consume {#while let}
                        output.extend(parse_while_let_chain(iter, pattern, expr, span)?);
                        ctx.advance_past(span);
                    }
                    TagType::Do(body) => {
                        iter.next(); // Consume {$do ...}
                        // Execute the Rust expression (for side effects)
                        output.extend(quote! {
                            #body;
                        });
                        ctx.advance_past(span);
                    }
                    TagType::LineComment(body) => {
                        iter.next(); // Consume
                        output.extend(ctx.emit_spacing_to(span));
                        let comment_text = body.to_string();
                        output.extend(quote! {
                            __out.push_str("// ");
                            __out.push_str(#comment_text);
                            __out.push_str("\n");
                        });
                        ctx.advance_past(span);
                    }
                    TagType::BlockComment(body) => {
                        iter.next(); // Consume
                        output.extend(ctx.emit_spacing_to(span));
                        let comment_text = body.to_string();
                        output.extend(quote! {
                            __out.push_str("/* ");
                            __out.push_str(#comment_text);
                            __out.push_str(" */");
                        });
                        ctx.advance_past(span);
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
                    TagType::EndWhile => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndWhile))
                        {
                            iter.next(); // Consume
                            return Ok((output, Some(Terminator::EndWhile)));
                        }
                        return Err(syn::Error::new(span, "Unexpected {/while}"));
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
                        iter.next(); // Consume {$let ...} or {%let ...}
                        output.extend(quote! {
                            let #body;
                        });
                        ctx.advance_past(span);
                    }
                    TagType::LetMut(body) => {
                        iter.next(); // Consume {$mut ...}
                        output.extend(quote! {
                            let mut #body;
                        });
                        ctx.advance_past(span);
                    }
                    TagType::TypeScript(body) => {
                        iter.next(); // Consume {$typescript ...}
                        output.extend(ctx.emit_spacing_to(span));
                        // The body is a TsStream - extract source and collect patches
                        output.extend(quote! {
                            {
                                let __ts_stream = #body;
                                __out.push_str(&macroforge_ts::ts_syn::ToTsString::to_ts_string(&__ts_stream));
                                __patches.extend(__ts_stream.runtime_patches);
                            }
                        });
                        ctx.advance_past(span);
                    }
                    TagType::Block => {
                        // Regular TS Block { ... }
                        iter.next(); // Consume
                        let inner_stream = g.stream();

                        // Emit spacing before the block
                        output.extend(ctx.emit_spacing_to(g.span_open()));
                        output.extend(quote! { __out.push_str("{"); });

                        // Set context to after opening brace
                        ctx.set_prev_end(Pos::from_span_end(g.span_open()));

                        // Parse inner content
                        let (inner_parsed, _) = parse_fragment_with_ctx(
                            &mut inner_stream.into_iter().peekable(),
                            None,
                            ctx,
                        )?;
                        output.extend(inner_parsed);

                        // Emit spacing before closing brace
                        let close_start = Pos::from_span_start(g.span_close());
                        if let Some(prev) = ctx.prev_end {
                            let sp = spacing_between(prev, close_start);
                            if !sp.is_empty() {
                                output.extend(quote! { __out.push_str(#sp); });
                            }
                        }

                        output.extend(quote! { __out.push_str("}"); });
                        ctx.advance_past(g.span_close());
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

                // Emit spacing before the group
                output.extend(ctx.emit_spacing_to(g.span_open()));
                output.extend(quote! { __out.push_str(#open); });

                // Set context to after opening delimiter
                ctx.set_prev_end(Pos::from_span_end(g.span_open()));

                // Parse inner content
                let (inner_parsed, _) = parse_fragment_with_ctx(
                    &mut g.stream().into_iter().peekable(),
                    None,
                    ctx,
                )?;
                output.extend(inner_parsed);

                // Emit spacing before closing delimiter
                let close_start = Pos::from_span_start(g.span_close());
                if let Some(prev) = ctx.prev_end {
                    let sp = spacing_between(prev, close_start);
                    if !sp.is_empty() {
                        output.extend(quote! { __out.push_str(#sp); });
                    }
                }

                output.extend(quote! { __out.push_str(#close); });
                ctx.advance_past(g.span_close());
            }

            // Case 4a: Backtick template literals "'^...^'" -> `...`
            TokenTree::Literal(lit) if is_backtick_template(lit) => {
                let span = lit.span();
                iter.next(); // Consume

                output.extend(ctx.emit_spacing_to(span));
                let processed = process_backtick_template(lit);
                output.extend(processed);
                ctx.advance_past(span);
            }

            // Case 4b: String literals with interpolation
            TokenTree::Literal(lit) if is_string_literal(lit) => {
                let span = lit.span();
                iter.next(); // Consume

                output.extend(ctx.emit_spacing_to(span));
                let interpolated = interpolate_string_literal(lit);
                output.extend(interpolated);
                ctx.advance_past(span);
            }

            // Case 5: Rust doc attribute #[doc = "..."] -> JSDoc /** ... */
            // Rust's tokenizer converts /** ... */ to #[doc = "..."] attributes
            TokenTree::Punct(p) if p.as_char() == '#' => {
                let hash_span = p.span();
                iter.next(); // Consume '#'

                // Check if next token is a bracket group [doc = "..."]
                let doc_content = if let Some(TokenTree::Group(g)) = iter.peek() {
                    if g.delimiter() == Delimiter::Bracket {
                        let mut inner = g.stream().into_iter().peekable();
                        // Check for `doc = "content"`
                        if let Some(TokenTree::Ident(ident)) = inner.next() {
                            if ident.to_string() == "doc" {
                                // Skip optional whitespace and match `=`
                                if let Some(TokenTree::Punct(eq)) = inner.next() {
                                    if eq.as_char() == '=' {
                                        // Get the string content
                                        if let Some(TokenTree::Literal(lit)) = inner.next() {
                                            let lit_str = lit.to_string();
                                            // Remove surrounding quotes and unescape
                                            if lit_str.starts_with('"') && lit_str.ends_with('"') {
                                                let content = &lit_str[1..lit_str.len() - 1];
                                                let unescaped = content
                                                    .replace("\\\"", "\"")
                                                    .replace("\\\\", "\\");
                                                Some(unescaped)
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(unescaped) = doc_content {
                    // Consume the bracket group and emit as JSDoc
                    let g = iter.next().unwrap();
                    let group_span = g.span();
                    output.extend(ctx.emit_spacing_to(hash_span));
                    output.extend(quote! {
                        __out.push_str("/**");
                        __out.push_str(#unescaped);
                        __out.push_str(" */");
                    });
                    ctx.advance_past(group_span);
                } else {
                    // Not a doc attribute - just emit the '#' as-is
                    output.extend(ctx.emit_spacing_to(hash_span));
                    output.extend(quote! { __out.push_str("#"); });
                    ctx.advance_past(hash_span);
                }
            }

            // Case 6: Plain tokens (identifiers, punctuation, literals)
            _ => {
                let t = iter.next().unwrap();
                let span = t.span();
                let s = t.to_string();

                // Emit spacing before this token
                output.extend(ctx.emit_spacing_to(span));

                // Emit the token
                output.extend(quote! {
                    __out.push_str(#s);
                });

                ctx.advance_past(span);
            }
        }
    }

    Ok((output, None))
}
