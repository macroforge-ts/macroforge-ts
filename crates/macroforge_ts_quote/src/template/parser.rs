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
use super::spacing::{Pos, same_line_gap, spacing_between};
use super::tag::{TagType, analyze_tag};

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
///
/// Control-flow and binding tags emit nothing, so they are zero-width: the
/// token after one is spaced from the tag's end. A tag on its own line takes
/// that line's newline with it; a tag inside a line passes on the gap before
/// it, so the tokens around it stay apart.
#[derive(Clone, Default)]
pub struct SpacingContext {
    /// End position of the previous token (for calculating gaps).
    pub prev_end: Option<Pos>,
    /// The template's left edge: indentation is measured from it, so output
    /// does not inherit how deeply the template sits in its Rust source.
    pub base_col: usize,
    /// The gap before a skipped tag, owed to the next token if it follows on
    /// the tag's line.
    inline_gap: Option<String>,
    /// The column of the tag a block body follows. The body's first line is
    /// indented past the tag only to nest it in the template, so that offset
    /// joins the base for the rest of the body.
    tag_col: Option<usize>,
}

impl SpacingContext {
    /// A context for a template whose first token starts at `base_col`.
    pub fn new(base_col: usize) -> Self {
        Self {
            base_col,
            ..Self::default()
        }
    }

    /// Emit spacing to bridge the gap from prev_end to the current span.
    pub fn emit_spacing_to(&mut self, span: Span) -> TokenStream2 {
        let curr_start = Pos::from_span_start(span);
        let inline_gap = self.inline_gap.take();
        self.settle_indent(curr_start);
        let spacing = match self.prev_end {
            Some(prev) if curr_start.line > prev.line => {
                spacing_between(prev, curr_start, self.base_col)
            }
            Some(prev) => inline_gap.unwrap_or_default() + &same_line_gap(prev, curr_start),
            None => String::new(),
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
        self.inline_gap = None;
    }

    /// Update prev_end to a specific position.
    pub fn set_prev_end(&mut self, pos: Pos) {
        self.prev_end = Some(pos);
        self.inline_gap = None;
    }

    /// Joins a block body's nesting offset to the base once the body's first
    /// line, starting at `start`, shows how far it is indented past its tag.
    fn settle_indent(&mut self, start: Pos) {
        if let Some(prev) = self.prev_end
            && start.line > prev.line
            && let Some(tag_col) = self.tag_col.take()
        {
            self.base_col += start.col.saturating_sub(tag_col);
        }
    }

    /// Passes over a tag that emits nothing, keeping the gap before it when
    /// it sits inside a line.
    pub fn skip_tag(&mut self, span: Span) {
        let tag_start = Pos::from_span_start(span);
        self.settle_indent(tag_start);
        self.inline_gap = self
            .prev_end
            .filter(|prev| prev.line == tag_start.line)
            .map(|prev| same_line_gap(prev, tag_start));
        self.prev_end = Some(Pos::from_span_end(span));
    }

    /// The context a block's body starts in, just after its opening tag.
    pub fn after_tag(&mut self, span: Span) -> Self {
        self.settle_indent(Pos::from_span_start(span));
        let mut body = self.clone();
        body.skip_tag(span);
        body.tag_col = Some(Pos::from_span_start(span).col);
        body
    }

    /// The context the next branch of a block starts in: the block's opening
    /// context, moved to just after the branch's tag at `self`'s position.
    pub fn next_branch(&mut self, opening: &Self) {
        let branch_tag_end = self.prev_end;
        *self = opening.clone();
        self.prev_end = branch_tag_end;
    }

    /// Continues after a block whose body ended in `body`, at its closing tag.
    pub fn resume_after(&mut self, body: &Self) {
        self.prev_end = body.prev_end;
        self.inline_gap = None;
    }
}

/// Parses a whole template, measuring indentation from its first token.
pub fn parse_template(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
) -> syn::Result<TokenStream2> {
    let base_col = iter
        .peek()
        .map(|token| Pos::from_span_start(token.span()).col)
        .unwrap_or_default();
    let mut ctx = SpacingContext::new(base_col);
    let (output, _) = parse_fragment_with_ctx(iter, None, &mut ctx)?;
    Ok(output)
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
                        let mut body = ctx.after_tag(span);
                        output.extend(parse_if_chain(iter, cond, span, &mut body)?);
                        ctx.resume_after(&body);
                    }
                    TagType::IfLet(pattern, expr) => {
                        iter.next(); // Consume {#if let}
                        let mut body = ctx.after_tag(span);
                        output.extend(parse_if_let_chain(iter, pattern, expr, span, &mut body)?);
                        ctx.resume_after(&body);
                    }
                    TagType::For(item, list) => {
                        iter.next(); // Consume {#for}

                        let mut body_ctx = ctx.after_tag(span);
                        let (body, terminator) = parse_fragment_with_ctx(
                            iter,
                            Some(&[Terminator::EndFor]),
                            &mut body_ctx,
                        )?;
                        ctx.resume_after(&body_ctx);
                        if !matches!(terminator, Some(Terminator::EndFor)) {
                            return Err(syn::Error::new(
                                span,
                                "Unclosed {#for} block: Missing {/for}",
                            ));
                        }

                        // Each iteration starts with its body's own leading spacing.
                        output.extend(quote! {
                            for #item in #list {
                                #body
                            }
                        });
                    }
                    TagType::Match(expr) => {
                        iter.next(); // Consume {#match}
                        let mut body = ctx.after_tag(span);
                        output.extend(parse_match_arms(iter, expr, span, &mut body)?);
                        ctx.resume_after(&body);
                    }
                    TagType::While(cond) => {
                        iter.next(); // Consume {#while}
                        let mut body = ctx.after_tag(span);
                        output.extend(parse_while_chain(iter, cond, span, &mut body)?);
                        ctx.resume_after(&body);
                    }
                    TagType::WhileLet(pattern, expr) => {
                        iter.next(); // Consume {#while let}
                        let mut body = ctx.after_tag(span);
                        output.extend(parse_while_let_chain(iter, pattern, expr, span, &mut body)?);
                        ctx.resume_after(&body);
                    }
                    TagType::Do(body) => {
                        iter.next(); // Consume {$do ...}
                        // Execute the Rust expression (for side effects)
                        output.extend(quote! {
                            #body;
                        });
                        ctx.skip_tag(span);
                    }
                    TagType::LineComment(body) => {
                        iter.next(); // Consume
                        output.extend(ctx.emit_spacing_to(span));
                        let comment_text = comment_text(body, span);
                        output.extend(quote! {
                            {
                                let __comment: String = #comment_text;
                                __out.push_str("// ");
                                __out.push_str(&__comment.replace('\n', "\n// "));
                                __out.push_str("\n");
                            }
                        });
                        ctx.advance_past(span);
                    }
                    TagType::BlockComment(body) => {
                        iter.next(); // Consume
                        output.extend(ctx.emit_spacing_to(span));
                        let comment_text = comment_text(body, span);
                        output.extend(quote! {
                            {
                                let __comment: String = #comment_text;
                                __out.push_str("/* ");
                                __out.push_str(&__comment.replace("*/", "* /"));
                                __out.push_str(" */");
                            }
                        });
                        ctx.advance_past(span);
                    }
                    TagType::Else => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::Else))
                        {
                            iter.next(); // Consume
                            ctx.skip_tag(span);
                            return Ok((output, Some(Terminator::Else)));
                        }
                        return Err(syn::Error::new(span, "Unexpected {:else}"));
                    }
                    TagType::ElseIf(cond) => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::ElseIf(_)))
                        {
                            iter.next(); // Consume
                            ctx.skip_tag(span);
                            return Ok((output, Some(Terminator::ElseIf(cond))));
                        }
                        return Err(syn::Error::new(span, "Unexpected {:else if}"));
                    }
                    TagType::EndIf => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndIf))
                        {
                            iter.next(); // Consume
                            ctx.skip_tag(span);
                            return Ok((output, Some(Terminator::EndIf)));
                        }
                        return Err(syn::Error::new(span, "Unexpected {/if}"));
                    }
                    TagType::EndFor => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndFor))
                        {
                            iter.next(); // Consume
                            ctx.skip_tag(span);
                            return Ok((output, Some(Terminator::EndFor)));
                        }
                        return Err(syn::Error::new(span, "Unexpected {/for}"));
                    }
                    TagType::EndWhile => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndWhile))
                        {
                            iter.next(); // Consume
                            ctx.skip_tag(span);
                            return Ok((output, Some(Terminator::EndWhile)));
                        }
                        return Err(syn::Error::new(span, "Unexpected {/while}"));
                    }
                    TagType::Case(pattern) => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::Case(_)))
                        {
                            iter.next(); // Consume
                            ctx.skip_tag(span);
                            return Ok((output, Some(Terminator::Case(pattern))));
                        }
                        return Err(syn::Error::new(span, "Unexpected {:case}"));
                    }
                    TagType::EndMatch => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndMatch))
                        {
                            iter.next(); // Consume
                            ctx.skip_tag(span);
                            return Ok((output, Some(Terminator::EndMatch)));
                        }
                        return Err(syn::Error::new(span, "Unexpected {/match}"));
                    }
                    TagType::Let(body) => {
                        iter.next(); // Consume {$let ...}
                        output.extend(quote! {
                            let #body;
                        });
                        ctx.skip_tag(span);
                    }
                    TagType::LetMut(body) => {
                        iter.next(); // Consume {$mut ...}
                        output.extend(quote! {
                            let mut #body;
                        });
                        ctx.skip_tag(span);
                    }
                    TagType::TypeScript(body) => {
                        iter.next(); // Consume {$typescript ...}
                        output.extend(ctx.emit_spacing_to(span));
                        // The body is a TsStream: its source joins the output at the
                        // indentation it lands on, and the rest of it (patches, suffixes,
                        // diagnostics) is carried along.
                        output.extend(quote! {
                            {
                                let mut __ts_stream: macroforge_ts::ts_syn::TsStream = #body;
                                __ts_stream.splice_source_into(&mut __out);
                                __carried = __carried.merge(__ts_stream);
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
                            let sp = spacing_between(prev, close_start, ctx.base_col);
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
                let (inner_parsed, _) =
                    parse_fragment_with_ctx(&mut g.stream().into_iter().peekable(), None, ctx)?;
                output.extend(inner_parsed);

                // Emit spacing before closing delimiter
                let close_start = Pos::from_span_start(g.span_close());
                if let Some(prev) = ctx.prev_end {
                    let sp = spacing_between(prev, close_start, ctx.base_col);
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
                output.extend(ctx.emit_spacing_to(hash_span));

                let doc = match iter.peek() {
                    Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => {
                        doc_text(group.stream()).map(|text| (text, group.span()))
                    }
                    _ => None,
                };
                match doc {
                    Some((text, group_span)) => {
                        iter.next(); // Consume the [doc = "..."] group
                        let pushes =
                            super::interpolation::interpolate_text(&text, group_span, false);
                        output.extend(quote! {
                            {
                                let __comment: String = {
                                    let mut __out = String::new();
                                    #pushes
                                    __out
                                };
                                __out.push_str("/**");
                                __out.push_str(&__comment.replace("*/", "* /"));
                                __out.push_str(" */");
                            }
                        });
                        ctx.advance_past(group_span);
                    }
                    None => {
                        // Not a doc attribute - just emit the '#' as-is
                        output.extend(quote! { __out.push_str("#"); });
                        ctx.advance_past(hash_span);
                    }
                }
            }

            // Case 6: Plain tokens (identifiers, punctuation, literals)
            _ => {
                let Some(t) = iter.next() else {
                    break;
                };
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

/// An expression building a comment's text. A body that is one string
/// literal gives the string's value, with `@{expr}` interpolated; any other
/// body is printed as written.
fn comment_text(body: TokenStream2, span: Span) -> TokenStream2 {
    let mut tokens = body.clone().into_iter();
    if let (Some(TokenTree::Literal(lit)), None) = (tokens.next(), tokens.next())
        && let Ok(text) = syn::parse2::<syn::LitStr>(TokenTree::Literal(lit).into())
    {
        let pushes = super::interpolation::interpolate_text(&text.value(), span, false);
        return quote! {
            {
                let mut __out = String::new();
                #pushes
                __out
            }
        };
    }
    let printed = body.to_string();
    quote! { String::from(#printed) }
}

/// The text of a `doc = "..."` attribute body, with its escapes decoded.
fn doc_text(body: TokenStream2) -> Option<String> {
    let doc = syn::parse2::<syn::MetaNameValue>(body).ok()?;
    if !doc.path.is_ident("doc") {
        return None;
    }
    match doc.value {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(text),
            ..
        }) => Some(text.value()),
        _ => None,
    }
}
