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

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    fn parse_test(tokens: proc_macro2::TokenStream) -> syn::Result<Vec<Segment>> {
        let mut iter = tokens.into_iter().peekable();
        let mut ids = IdGen { next: 0 };
        let (segments, _) = parse_segments(&mut iter, None, &mut ids, false)?;
        Ok(segments)
    }

    #[test]
    fn test_empty_input() {
        let result = parse_test(quote! {});
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_static_only() {
        let result = parse_test(quote! { const x = 42; });
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert_eq!(segments.len(), 1);
        assert!(matches!(&segments[0], Segment::Static(_)));
    }

    #[test]
    fn test_basic_interpolation() {
        let result = parse_test(quote! { value: @{x} });
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(segments.len() >= 2);
        assert!(segments.iter().any(|s| matches!(s, Segment::Interpolation { .. })));
    }

    #[test]
    fn test_string_interpolation() {
        let result = parse_test(quote! { "hello @{name}" });
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(segments.iter().any(|s| matches!(s, Segment::StringInterp { .. })));
    }

    #[test]
    fn test_backtick_template() {
        let result = parse_test(quote! { "'^template @{expr}^'" });
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(segments.iter().any(|s| matches!(s, Segment::TemplateInterp { .. })));
    }

    #[test]
    fn test_if_control() {
        let tokens: proc_macro2::TokenStream = "{#if condition} then {/if}".parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(segments.iter().any(|s| matches!(s, Segment::Control { .. })));
    }

    #[test]
    fn test_if_else() {
        let tokens: proc_macro2::TokenStream = "{#if x} yes {:else} no {/if}".parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_let() {
        let tokens: proc_macro2::TokenStream = "{#if let Some(x) = opt} @{x} {/if}".parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_for_loop() {
        let tokens: proc_macro2::TokenStream = "{#for item in items} @{item} {/for}".parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_while_loop() {
        let tokens: proc_macro2::TokenStream = "{#while condition} body {/while}".parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_while_let() {
        let tokens: proc_macro2::TokenStream = "{#while let Some(x) = iter.next()} @{x} {/while}".parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_match_block() {
        let tokens: proc_macro2::TokenStream = "{#match value} {:case Some(x)} @{x} {:case None} none {/match}".parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_let_binding() {
        let result = parse_test(quote! {
            {$let x = 42}
        });
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(segments.iter().any(|s| matches!(s, Segment::Let { .. })));
    }

    #[test]
    fn test_do_expression() {
        let result = parse_test(quote! {
            {$do println!("test")}
        });
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(segments.iter().any(|s| matches!(s, Segment::Do { .. })));
    }

    #[test]
    fn test_typescript_block() {
        let result = parse_test(quote! {
            {$typescript stream}
        });
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(segments.iter().any(|s| matches!(s, Segment::Typescript { .. })));
    }

    #[test]
    fn test_ident_block() {
        let result = parse_test(quote! {
            {|get@{name}|}
        });
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(segments.iter().any(|s| matches!(s, Segment::IdentBlock { .. })));
    }

    #[test]
    fn test_line_comment() {
        let result = parse_test(quote! {
            {> "comment text" <}
        });
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(segments.iter().any(|s| matches!(s, Segment::Comment { style: CommentStyle::Line, .. })));
    }

    #[test]
    fn test_block_comment() {
        let result = parse_test(quote! {
            {>> "comment text" <<}
        });
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(segments.iter().any(|s| matches!(s, Segment::Comment { style: CommentStyle::Block, .. })));
    }

    #[test]
    fn test_nested_braces() {
        let result = parse_test(quote! {
            function() { @{body} }
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_mixed_content() {
        let tokens: proc_macro2::TokenStream = r#"
            const x = @{value};
            {#if condition}
                "string @{interp}"
            {/if}
        "#.parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_unclosed_if() {
        let tokens: proc_macro2::TokenStream = "{#if x} content".parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_unexpected_else() {
        let tokens: proc_macro2::TokenStream = "{:else}".parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_unexpected_endif() {
        let tokens: proc_macro2::TokenStream = "{/if}".parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_interpolations_only_mode() {
        let tokens = quote! { { @{x} } };
        let mut iter = tokens.into_iter().peekable();
        let mut ids = IdGen { next: 0 };
        let result = parse_segments(&mut iter, None, &mut ids, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nested_control_flow() {
        let tokens: proc_macro2::TokenStream = "{#if outer} {#if inner} nested {/if} {/if}".parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_interpolations() {
        let result = parse_test(quote! {
            @{a} @{b} @{c}
        });
        assert!(result.is_ok());
        let segments = result.unwrap();
        let interp_count = segments.iter().filter(|s| matches!(s, Segment::Interpolation { .. })).count();
        assert_eq!(interp_count, 3);
    }

    #[test]
    fn test_parenthesis_with_interpolation() {
        let result = parse_test(quote! {
            (@{x}, @{y})
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_bracket_with_interpolation() {
        let result = parse_test(quote! {
            [@{x}, @{y}]
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_at_escape() {
        let result = parse_test(quote! {
            @
        });
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(matches!(&segments[0], Segment::Static(_)));
    }

    #[test]
    fn test_complex_nested_structure() {
        let tokens: proc_macro2::TokenStream = r#"
            function test() {
                {#for item in items}
                    {#if item.active}
                        const x = @{item.name};
                        {#match item.status}
                            {:case Status::Ready} "ready"
                            {:case _} "other"
                        {/match}
                    {/if}
                {/for}
            }
        "#.parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_brace_block_detection() {
        let result = parse_test(quote! {
            { static content }
        });
        assert!(result.is_ok());
        let segments = result.unwrap();
        // Should be treated as static since no interpolations
        assert!(segments.iter().all(|s| matches!(s, Segment::Static(_))));
    }

    #[test]
    fn test_brace_block_with_interpolation() {
        let result = parse_test(quote! {
            { content @{x} }
        });
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(segments.iter().any(|s| matches!(s, Segment::BraceBlock { .. })));
    }

    #[test]
    fn test_doc_comment_detection() {
        let result = parse_test(quote! {
            #[doc = "test"]
        });
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert!(segments.iter().any(|s| matches!(s, Segment::Comment { style: CommentStyle::DocBlock, .. })));
    }

    #[test]
    fn test_consecutive_control_structures() {
        let tokens: proc_macro2::TokenStream = "{#if a} a {/if} {#if b} b {/if} {#if c} c {/if}".parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_control_blocks() {
        let tokens: proc_macro2::TokenStream = "{#if x}{/if} {#for i in x}{/for} {#while x}{/while}".parse().unwrap();
        let result = parse_test(tokens);
        assert!(result.is_ok());
    }
}
