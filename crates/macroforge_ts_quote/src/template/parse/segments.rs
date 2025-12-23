use crate::template::{
    CommentStyle, ControlNode, IdGen, Segment, TagType, Terminator, template_error,
};
use proc_macro2::{Delimiter, TokenStream as TokenStream2, TokenTree};
use std::iter::Peekable;

use super::{
    parse_backtick_template, parse_doc_attribute, parse_ident_block_parts, parse_if_chain,
    parse_if_let_chain, parse_match_arms, parse_string_interpolation, parse_while_let_loop,
    parse_while_loop,
};
use super::super::flush::flush_static;
use super::super::utils::{analyze_tag, try_extract_object_prop_loop};

/// Parses a token stream into template segments until an optional terminator.
pub fn parse_segments(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    stop_at: Option<&[Terminator]>,
    ids: &mut IdGen,
    interpolations_only: bool,
) -> syn::Result<(Vec<Segment>, Option<Terminator>)> {
    let mut segments = Vec::new();
    let mut static_tokens = TokenStream2::new();

    while let Some(token) = iter.peek().cloned() {
        match &token {
            TokenTree::Punct(p) if p.as_char() == '#' => {
                let mut lookahead = iter.clone();
                lookahead.next();
                if let Some(TokenTree::Group(g)) = lookahead.peek()
                    && g.delimiter() == Delimiter::Bracket
                    && let Some(content) = parse_doc_attribute(g) {
                        iter.next();
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        segments.push(Segment::Comment {
                            style: CommentStyle::DocBlock,
                            text: content,
                        });
                        continue;
                    }
                static_tokens.extend(TokenStream2::from(token.clone()));
                iter.next();
            }
            TokenTree::Punct(p) if p.as_char() == '@' => {
                iter.next();
                let is_group = iter.peek().is_some_and(
                    |t| matches!(t, TokenTree::Group(g) if g.delimiter() == Delimiter::Brace),
                );
                if is_group {
                    flush_static(&mut segments, &mut static_tokens);
                    if let Some(TokenTree::Group(g)) = iter.next() {
                        let id = ids.next();
                        segments.push(Segment::Interpolation {
                            id,
                            expr: g.stream(),
                        });
                    }
                } else {
                    static_tokens.extend(TokenStream2::from(token.clone()));
                }
            }
            TokenTree::Literal(lit) => {
                if let Some(parts) = parse_backtick_template(lit)? {
                    iter.next();
                    flush_static(&mut segments, &mut static_tokens);
                    let id = ids.next();
                    segments.push(Segment::TemplateInterp {
                        id,
                        parts,
                    });
                } else if let Some(parts) = parse_string_interpolation(lit)? {
                    iter.next();
                    flush_static(&mut segments, &mut static_tokens);
                    let id = ids.next();
                    segments.push(Segment::StringInterp {
                        id,
                        parts,
                    });
                } else {
                    static_tokens.extend(TokenStream2::from(token.clone()));
                    iter.next();
                }
            }
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => {
                let span = g.span();

                // In interpolation-only mode, treat ALL brace groups uniformly
                // (recursively parse for interpolations, skip control flow detection)
                if interpolations_only {
                    iter.next();
                    flush_static(&mut segments, &mut static_tokens);

                    let (inner_segments, _) =
                        parse_segments(&mut g.stream().into_iter().peekable(), None, ids, true)?;

                    let has_dynamic = inner_segments
                        .iter()
                        .any(|s| !matches!(s, Segment::Static(_)));

                    if has_dynamic {
                        segments.push(Segment::Static("{".to_string()));
                        segments.extend(inner_segments);
                        segments.push(Segment::Static("}".to_string()));
                    } else {
                        static_tokens.extend(TokenStream2::from(token.clone()));
                    }
                    continue;
                }

                let tag = analyze_tag(g);
                match tag {
                    TagType::If(cond) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        let node = parse_if_chain(iter, cond, span, ids)?;
                        let id = ids.next();
                        segments.push(Segment::Control { id, node });
                    }
                    TagType::IfLet(pattern, expr) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        let node = parse_if_let_chain(iter, pattern, expr, span, ids)?;
                        let id = ids.next();
                        segments.push(Segment::Control { id, node });
                    }
                    TagType::For(pat, iter_expr) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        let (body, terminator) =
                            parse_segments(iter, Some(&[Terminator::EndFor]), ids, false)?;
                        if !matches!(terminator, Some(Terminator::EndFor)) {
                            return Err(template_error(
                                span,
                                "Unclosed {#for} block: Missing {/for}",
                                Some("{#for item in collection}..."),
                            ));
                        }
                        let id = ids.next();
                        segments.push(Segment::Control {
                            id,
                            node: ControlNode::For {
                                pat,
                                iter: iter_expr,
                                body,
                            },
                        });
                    }
                    TagType::Match(expr) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        let node = parse_match_arms(iter, expr, span, ids)?;
                        let id = ids.next();
                        segments.push(Segment::Control { id, node });
                    }
                    TagType::While(cond) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        let node = parse_while_loop(iter, cond, span, ids)?;
                        let id = ids.next();
                        segments.push(Segment::Control { id, node });
                    }
                    TagType::WhileLet(pattern, expr) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        let node = parse_while_let_loop(iter, pattern, expr, span, ids)?;
                        let id = ids.next();
                        segments.push(Segment::Control { id, node });
                    }
                    TagType::Else => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::Else))
                        {
                            iter.next();
                            flush_static(&mut segments, &mut static_tokens);
                            return Ok((segments, Some(Terminator::Else)));
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
                            iter.next();
                            flush_static(&mut segments, &mut static_tokens);
                            return Ok((segments, Some(Terminator::ElseIf(cond))));
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
                            iter.next();
                            flush_static(&mut segments, &mut static_tokens);
                            return Ok((segments, Some(Terminator::EndIf)));
                        }
                        return Err(template_error(span, "Unexpected {/if}", None));
                    }
                    TagType::EndFor => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndFor))
                        {
                            iter.next();
                            flush_static(&mut segments, &mut static_tokens);
                            return Ok((segments, Some(Terminator::EndFor)));
                        }
                        return Err(template_error(span, "Unexpected {/for}", None));
                    }
                    TagType::EndWhile => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndWhile))
                        {
                            iter.next();
                            flush_static(&mut segments, &mut static_tokens);
                            return Ok((segments, Some(Terminator::EndWhile)));
                        }
                        return Err(template_error(span, "Unexpected {/while}", None));
                    }
                    TagType::Case(pattern) => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::Case(_)))
                        {
                            iter.next();
                            flush_static(&mut segments, &mut static_tokens);
                            return Ok((segments, Some(Terminator::Case(pattern))));
                        }
                        return Err(template_error(span, "Unexpected {:case}", None));
                    }
                    TagType::EndMatch => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndMatch))
                        {
                            iter.next();
                            flush_static(&mut segments, &mut static_tokens);
                            return Ok((segments, Some(Terminator::EndMatch)));
                        }
                        return Err(template_error(span, "Unexpected {/match}", None));
                    }
                    TagType::Let(tokens) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        segments.push(Segment::Let { tokens });
                    }
                    TagType::Do(expr) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        segments.push(Segment::Do { expr });
                    }
                    TagType::Typescript(expr) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        let id = ids.next();
                        segments.push(Segment::Typescript { id, expr });
                    }
                    TagType::IdentBlock => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        let parts = parse_ident_block_parts(g)?;
                        let id = ids.next();
                        segments.push(Segment::IdentBlock { id, parts });
                    }
                    TagType::DocComment(content) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        segments.push(Segment::Comment {
                            style: CommentStyle::Block,
                            text: content,
                        });
                    }
                    TagType::BlockComment(content) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        segments.push(Segment::Comment {
                            style: CommentStyle::Line,
                            text: content,
                        });
                    }
                    TagType::Block => {
                        // Regular brace group (function body, object literal, etc.)
                        // Recursively parse to detect interpolations inside.
                        // Control flow is still allowed at the parent level.
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);

                        let (inner_segments, _) = parse_segments(
                            &mut g.stream().into_iter().peekable(),
                            None,
                            ids,
                            false, // Full parsing - control flow is handled at correct level
                        )?;

                        // Check if there are any dynamic segments
                        let has_dynamic = inner_segments
                            .iter()
                            .any(|s| !matches!(s, Segment::Static(_)));

                        if has_dynamic {
                            // Check if this is an object literal with a property loop pattern:
                            // { {#for (key, val) in items} @{key}: @{val}, {/for} }
                            if let Some(obj_prop_loop) =
                                try_extract_object_prop_loop(&inner_segments, ids)
                            {
                                segments.push(obj_prop_loop);
                            } else {
                                let id = ids.next();
                                segments.push(Segment::BraceBlock {
                                    id,
                                    inner: inner_segments,
                                });
                            }
                        } else {
                            // No dynamic content, just add the whole group as static
                            static_tokens.extend(TokenStream2::from(token.clone()));
                        }
                    }
                }
            }
            // Handle parenthesis and bracket groups - recursively process their contents
            TokenTree::Group(g)
                if g.delimiter() == Delimiter::Parenthesis
                    || g.delimiter() == Delimiter::Bracket =>
            {
                iter.next();
                flush_static(&mut segments, &mut static_tokens);

                // Recursively parse the group's contents
                let (inner_segments, _) =
                    parse_segments(&mut g.stream().into_iter().peekable(), None, ids, false)?;

                // Check if there are any interpolations in the inner segments
                let has_dynamic = inner_segments
                    .iter()
                    .any(|s| !matches!(s, Segment::Static(_)));

                if has_dynamic {
                    // There are interpolations, we need to handle this specially
                    // Emit opening delimiter
                    let (open, close) = match g.delimiter() {
                        Delimiter::Parenthesis => ("(", ")"),
                        Delimiter::Bracket => ("[", "]"),
                        _ => unreachable!(),
                    };
                    segments.push(Segment::Static(open.to_string()));
                    segments.extend(inner_segments);
                    segments.push(Segment::Static(close.to_string()));
                } else {
                    // No interpolations, just add the group as static
                    static_tokens.extend(TokenStream2::from(token.clone()));
                }
            }
            _ => {
                static_tokens.extend(TokenStream2::from(token.clone()));
                iter.next();
            }
        }
    }

    flush_static(&mut segments, &mut static_tokens);
    Ok((segments, None))
}
