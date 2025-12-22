//! Rust-style templating for TypeScript code generation (AST-based)
//!
//! Provides a template syntax with interpolation and control flow:
//! - `@{expr}` - Interpolate expressions (calls `.to_string()`)
//! - `{| content |}` - Ident block: concatenates content without spaces (e.g., `{|get@{name}|}` → `getUser`)
//! - `{> "comment" <}` - Line comment: outputs `// comment` (string preserves whitespace)
//! - `{>> "comment" <<}` - Block comment: outputs `/* comment */` (string preserves whitespace)
//! - `///` or `/** */` - Rust doc comments in the template emit JSDoc blocks (`/** ... */`)
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
use quote::{ToTokens, format_ident, quote};
use std::collections::{HashMap, HashSet};
use std::iter::Peekable;

use swc_core::common::comments::CommentKind;
use swc_core::common::sync::Lrc;
use swc_core::common::{FileName, SourceMap, SourceMapper, Spanned};
use swc_core::ecma::ast::*;
use swc_core::ecma::parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};
use swc_core::ecma::visit::{Visit, VisitWith};

/// Generates unique placeholder IDs for dynamic segments.
struct IdGen {
    next: usize,
}

impl IdGen {
    /// Creates a new placeholder ID generator.
    fn new() -> Self {
        Self { next: 0 }
    }

    /// Returns the next unique placeholder ID.
    fn next(&mut self) -> usize {
        let id = self.next;
        self.next += 1;
        id
    }
}

#[derive(Clone, Debug)]
enum Segment {
    Static(String),
    Comment {
        style: CommentStyle,
        text: String,
        span: Span,
    },
    Interpolation {
        id: usize,
        expr: TokenStream2,
        span: Span,
    },
    StringInterp {
        id: usize,
        parts: Vec<StringPart>,
        span: Span,
    },
    TemplateInterp {
        id: usize,
        parts: Vec<StringPart>,
        span: Span,
    },
    IdentBlock {
        id: usize,
        parts: Vec<IdentPart>,
        span: Span,
    },
    Control {
        id: usize,
        node: ControlNode,
        span: Span,
    },
    Let {
        tokens: TokenStream2,
        span: Span,
    },
    Do {
        expr: TokenStream2,
        span: Span,
    },
    Typescript {
        id: usize,
        expr: TokenStream2,
        span: Span,
    },
    /// A nested brace block containing inner segments.
    /// Used to preserve the atomic structure of function bodies and object literals
    /// while still detecting interpolations and control flow inside.
    BraceBlock {
        id: usize,
        inner: Vec<Segment>,
        span: Span,
    },
}

#[derive(Clone, Debug)]
enum IdentPart {
    Static(String),
    Interpolation { expr: TokenStream2, span: Span },
}

#[derive(Clone, Debug)]
enum StringPart {
    Text(String),
    Expr(TokenStream2),
}

#[derive(Clone, Debug)]
enum ControlNode {
    If {
        cond: TokenStream2,
        then_branch: Vec<Segment>,
        else_branch: Option<Vec<Segment>>,
    },
    IfLet {
        pattern: TokenStream2,
        expr: TokenStream2,
        then_branch: Vec<Segment>,
        else_branch: Option<Vec<Segment>>,
    },
    For {
        pat: TokenStream2,
        iter: TokenStream2,
        body: Vec<Segment>,
    },
    While {
        cond: TokenStream2,
        body: Vec<Segment>,
    },
    WhileLet {
        pattern: TokenStream2,
        expr: TokenStream2,
        body: Vec<Segment>,
    },
    Match {
        expr: TokenStream2,
        cases: Vec<MatchCase>,
    },
}

#[derive(Clone, Debug)]
enum CommentStyle {
    DocBlock,
    Block,
    Line,
}

#[derive(Clone, Debug)]
struct MatchCase {
    pattern: TokenStream2,
    body: Vec<Segment>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PlaceholderUse {
    Expr,
    Ident,
    IdentName,
    Stmt,
    Type,
}

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

#[derive(Debug)]
enum TagType {
    If(TokenStream2),
    IfLet(TokenStream2, TokenStream2),
    For(TokenStream2, TokenStream2),
    Match(TokenStream2),
    While(TokenStream2),
    WhileLet(TokenStream2, TokenStream2),
    Else,
    ElseIf(TokenStream2),
    EndIf,
    EndFor,
    EndWhile,
    Case(TokenStream2),
    EndMatch,
    Let(TokenStream2),
    Do(TokenStream2),
    Typescript(TokenStream2),
    IdentBlock,
    DocComment(String),
    BlockComment(String),
    Block,
}

/// Formats the placeholder identifier for a given segment ID.
fn placeholder_name(id: usize) -> String {
    format!("__mf_hole_{id}")
}

/// Builds a template error with optional context.
fn template_error(span: Span, message: &str, context: Option<&str>) -> syn::Error {
    let full_message = if let Some(ctx) = context {
        format!("{}\n  --> in: {}", message, ctx)
    } else {
        message.to_string()
    };
    syn::Error::new(span, full_message)
}

/// Parses a template token stream into Rust that builds TypeScript AST output.
pub fn parse_template(input: TokenStream2) -> syn::Result<TokenStream2> {
    let mut ids = IdGen::new();
    let (segments, _) = parse_segments(&mut input.into_iter().peekable(), None, &mut ids, false)?;
    let stmts_ident = proc_macro2::Ident::new("__stmts", Span::call_site());
    let comments_ident = proc_macro2::Ident::new("__comments", Span::call_site());
    let pending_ident = proc_macro2::Ident::new("__pending_comments", Span::call_site());
    let pos_ident = proc_macro2::Ident::new("__mf_pos_counter", Span::call_site());

    let stmts_builder = compile_stmt_segments(
        &segments,
        &stmts_ident,
        &comments_ident,
        &pending_ident,
        &pos_ident,
    )?;

    Ok(quote! {
        {
            let mut __stmts: Vec<swc_core::ecma::ast::Stmt> = Vec::new();
            let mut __patches: Vec<macroforge_ts::ts_syn::abi::Patch> = Vec::new();
            let __comments = swc_core::common::comments::SingleThreadedComments::default();
            let mut __pending_comments: Vec<swc_core::common::comments::Comment> = Vec::new();
            let mut __mf_pos_counter: u32 = 1;
            #stmts_builder
            (__stmts, __patches, __comments)
        }
    })
}

/// Parses a token stream into template segments until an optional terminator.
fn parse_segments(
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
                {
                    if let Some(content) = parse_doc_attribute(g) {
                        iter.next();
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        segments.push(Segment::Comment {
                            style: CommentStyle::DocBlock,
                            text: content,
                            span: g.span(),
                        });
                        continue;
                    }
                }
                static_tokens.extend(TokenStream2::from(token.clone()));
                iter.next();
            }
            TokenTree::Punct(p) if p.as_char() == '@' => {
                iter.next();
                let is_group = iter.peek().is_some_and(|t| {
                    matches!(t, TokenTree::Group(g) if g.delimiter() == Delimiter::Brace)
                });
                if is_group {
                    flush_static(&mut segments, &mut static_tokens);
                    if let Some(TokenTree::Group(g)) = iter.next() {
                        let id = ids.next();
                        segments.push(Segment::Interpolation {
                            id,
                            expr: g.stream(),
                            span: g.span(),
                        });
                    }
                } else {
                    static_tokens.extend(TokenStream2::from(token.clone()));
                }
            }
            TokenTree::Literal(lit) => {
                if let Some(parts) = parse_backtick_template(&lit)? {
                    iter.next();
                    flush_static(&mut segments, &mut static_tokens);
                    let id = ids.next();
                    segments.push(Segment::TemplateInterp {
                        id,
                        parts,
                        span: lit.span(),
                    });
                } else if let Some(parts) = parse_string_interpolation(&lit)? {
                    iter.next();
                    flush_static(&mut segments, &mut static_tokens);
                    let id = ids.next();
                    segments.push(Segment::StringInterp {
                        id,
                        parts,
                        span: lit.span(),
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

                    let has_dynamic =
                        inner_segments.iter().any(|s| !matches!(s, Segment::Static(_)));

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
                        segments.push(Segment::Control { id, node, span });
                    }
                    TagType::IfLet(pattern, expr) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        let node = parse_if_let_chain(iter, pattern, expr, span, ids)?;
                        let id = ids.next();
                        segments.push(Segment::Control { id, node, span });
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
                            span,
                        });
                    }
                    TagType::Match(expr) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        let node = parse_match_arms(iter, expr, span, ids)?;
                        let id = ids.next();
                        segments.push(Segment::Control { id, node, span });
                    }
                    TagType::While(cond) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        let node = parse_while_loop(iter, cond, span, ids)?;
                        let id = ids.next();
                        segments.push(Segment::Control { id, node, span });
                    }
                    TagType::WhileLet(pattern, expr) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        let node = parse_while_let_loop(iter, pattern, expr, span, ids)?;
                        let id = ids.next();
                        segments.push(Segment::Control { id, node, span });
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
                        segments.push(Segment::Let { tokens, span });
                    }
                    TagType::Do(expr) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        segments.push(Segment::Do { expr, span });
                    }
                    TagType::Typescript(expr) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        let id = ids.next();
                        segments.push(Segment::Typescript { id, expr, span });
                    }
                    TagType::IdentBlock => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        let parts = parse_ident_block_parts(g)?;
                        let id = ids.next();
                        segments.push(Segment::IdentBlock { id, parts, span });
                    }
                    TagType::DocComment(content) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        segments.push(Segment::Comment {
                            style: CommentStyle::Block,
                            text: content,
                            span,
                        });
                    }
                    TagType::BlockComment(content) => {
                        iter.next();
                        flush_static(&mut segments, &mut static_tokens);
                        segments.push(Segment::Comment {
                            style: CommentStyle::Line,
                            text: content,
                            span,
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
                        let has_dynamic =
                            inner_segments.iter().any(|s| !matches!(s, Segment::Static(_)));

                        if has_dynamic {
                            // Wrap in braces and add segments
                            segments.push(Segment::Static("{".to_string()));
                            segments.extend(inner_segments);
                            segments.push(Segment::Static("}".to_string()));
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
                let has_dynamic = inner_segments.iter().any(|s| !matches!(s, Segment::Static(_)));

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

/// Flushes buffered static tokens into a static segment.
fn flush_static(segments: &mut Vec<Segment>, static_tokens: &mut TokenStream2) {
    if static_tokens.is_empty() {
        return;
    }
    let tokens = std::mem::take(static_tokens);
    let s = tokens_to_ts_string(tokens);
    if !s.trim().is_empty() {
        segments.push(Segment::Static(s));
    }
}

/// Parses an `{#if ...}` chain into a control node.
fn parse_if_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    cond: TokenStream2,
    span: Span,
    ids: &mut IdGen,
) -> syn::Result<ControlNode> {
    let (then_branch, terminator) = parse_segments(
        iter,
        Some(&[
            Terminator::Else,
            Terminator::ElseIf(TokenStream2::new()),
            Terminator::EndIf,
        ]),
        ids,
        false,
    )?;

    match terminator {
        Some(Terminator::Else) => {
            let (else_branch, terminator) =
                parse_segments(iter, Some(&[Terminator::EndIf]), ids, false)?;
            if !matches!(terminator, Some(Terminator::EndIf)) {
                return Err(template_error(
                    span,
                    "Unclosed {#if} block: Missing {/if}",
                    Some("{#if condition}...{:else}...{/if}"),
                ));
            }
            Ok(ControlNode::If {
                cond,
                then_branch,
                else_branch: Some(else_branch),
            })
        }
        Some(Terminator::ElseIf(next_cond)) => {
            let else_branch = vec![Segment::Control {
                id: ids.next(),
                node: parse_if_chain(iter, next_cond, span, ids)?,
                span,
            }];
            Ok(ControlNode::If {
                cond,
                then_branch,
                else_branch: Some(else_branch),
            })
        }
        Some(Terminator::EndIf) => Ok(ControlNode::If {
            cond,
            then_branch,
            else_branch: None,
        }),
        _ => Err(template_error(
            span,
            "Unclosed {#if} block: Missing {/if}",
            Some("{#if condition}...{/if}"),
        )),
    }
}

/// Parses an `{#if let ...}` chain into a control node.
fn parse_if_let_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    pattern: TokenStream2,
    expr: TokenStream2,
    span: Span,
    ids: &mut IdGen,
) -> syn::Result<ControlNode> {
    let (then_branch, terminator) = parse_segments(
        iter,
        Some(&[
            Terminator::Else,
            Terminator::ElseIf(TokenStream2::new()),
            Terminator::EndIf,
        ]),
        ids,
        false,
    )?;

    match terminator {
        Some(Terminator::Else) => {
            let (else_branch, terminator) =
                parse_segments(iter, Some(&[Terminator::EndIf]), ids, false)?;
            if !matches!(terminator, Some(Terminator::EndIf)) {
                return Err(template_error(
                    span,
                    "Unclosed {#if let} block: Missing {/if}",
                    Some("{#if let pattern = expr}...{:else}...{/if}"),
                ));
            }
            Ok(ControlNode::IfLet {
                pattern,
                expr,
                then_branch,
                else_branch: Some(else_branch),
            })
        }
        Some(Terminator::ElseIf(next_cond)) => {
            let else_branch = vec![Segment::Control {
                id: ids.next(),
                node: parse_if_chain(iter, next_cond, span, ids)?,
                span,
            }];
            Ok(ControlNode::IfLet {
                pattern,
                expr,
                then_branch,
                else_branch: Some(else_branch),
            })
        }
        Some(Terminator::EndIf) => Ok(ControlNode::IfLet {
            pattern,
            expr,
            then_branch,
            else_branch: None,
        }),
        _ => Err(template_error(
            span,
            "Unclosed {#if let} block: Missing {/if}",
            Some("{#if let pattern = expr}...{/if}"),
        )),
    }
}

/// Parses the arms for a `{#match ...}` block.
fn parse_match_arms(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    expr: TokenStream2,
    span: Span,
    ids: &mut IdGen,
) -> syn::Result<ControlNode> {
    let mut cases = Vec::new();
    let mut pending_pattern: Option<TokenStream2> = None;

    loop {
        let (body, terminator) = parse_segments(
            iter,
            Some(&[Terminator::Case(TokenStream2::new()), Terminator::EndMatch]),
            ids,
            false,
        )?;

        match terminator {
            Some(Terminator::Case(pattern)) => {
                if let Some(prev) = pending_pattern.take() {
                    cases.push(MatchCase {
                        pattern: prev,
                        body,
                    });
                } else if !body.is_empty() {
                    return Err(template_error(
                        span,
                        "Unexpected content before {:case}",
                        None,
                    ));
                }
                pending_pattern = Some(pattern);
            }
            Some(Terminator::EndMatch) => {
                if let Some(prev) = pending_pattern.take() {
                    cases.push(MatchCase {
                        pattern: prev,
                        body,
                    });
                } else if !body.is_empty() {
                    return Err(template_error(
                        span,
                        "Unexpected content before {/match}",
                        None,
                    ));
                }
                break;
            }
            None => {
                return Err(template_error(
                    span,
                    "Unclosed {#match} block: Missing {/match}",
                    Some("{#match expr}{:case pattern}...{/match}"),
                ));
            }
            Some(other) => {
                return Err(template_error(
                    span,
                    &format!("Unexpected terminator in {{#match}}: {other:?}"),
                    None,
                ));
            }
        }
    }

    Ok(ControlNode::Match { expr, cases })
}

/// Parses a `{#while ...}` block.
fn parse_while_loop(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    cond: TokenStream2,
    span: Span,
    ids: &mut IdGen,
) -> syn::Result<ControlNode> {
    let (body, terminator) = parse_segments(iter, Some(&[Terminator::EndWhile]), ids, false)?;
    if !matches!(terminator, Some(Terminator::EndWhile)) {
        return Err(template_error(
            span,
            "Unclosed {#while} block: Missing {/while}",
            Some("{#while condition}...{/while}"),
        ));
    }
    Ok(ControlNode::While { cond, body })
}

/// Parses a `{#while let ...}` block.
fn parse_while_let_loop(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    pattern: TokenStream2,
    expr: TokenStream2,
    span: Span,
    ids: &mut IdGen,
) -> syn::Result<ControlNode> {
    let (body, terminator) = parse_segments(iter, Some(&[Terminator::EndWhile]), ids, false)?;
    if !matches!(terminator, Some(Terminator::EndWhile)) {
        return Err(template_error(
            span,
            "Unclosed {#while let} block: Missing {/while}",
            Some("{#while let pattern = expr}...{/while}"),
        ));
    }
    Ok(ControlNode::WhileLet {
        pattern,
        expr,
        body,
    })
}

/// Compiles statement-level segments into Rust code that builds SWC statements.
fn compile_stmt_segments(
    segments: &[Segment],
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
) -> syn::Result<TokenStream2> {
    let context_map = classify_placeholders_module(segments)?;
    let mut output = TokenStream2::new();
    let mut run: Vec<&Segment> = Vec::new();

    for segment in segments {
        match segment {
            Segment::Control { id, node, span: _ } => {
                let is_stmt = matches!(context_map.get(id), Some(PlaceholderUse::Stmt));
                if is_stmt {
                    if !run.is_empty() {
                        output.extend(flush_stmt_run(
                            &run,
                            &context_map,
                            out_ident,
                            comments_ident,
                            pending_ident,
                            pos_ident,
                        )?);
                        run.clear();
                    }
                    output.extend(compile_stmt_control(
                        node,
                        out_ident,
                        comments_ident,
                        pending_ident,
                        pos_ident,
                    )?);
                } else {
                    run.push(segment);
                }
            }
            Segment::Typescript { id, expr, span } => {
                if !run.is_empty() {
                    output.extend(flush_stmt_run(
                        &run,
                        &context_map,
                        out_ident,
                        comments_ident,
                        pending_ident,
                        pos_ident,
                    )?);
                    run.clear();
                }
                if matches!(context_map.get(id), Some(PlaceholderUse::Stmt) | None) {
                    output.extend(compile_ts_injection(
                        expr,
                        out_ident,
                        comments_ident,
                        pending_ident,
                    ));
                } else {
                    return Err(template_error(
                        *span,
                        "{$typescript} is only valid at statement boundaries",
                        None,
                    ));
                }
            }
            Segment::Comment { style, text, .. } => {
                if !run.is_empty() {
                    output.extend(flush_stmt_run(
                        &run,
                        &context_map,
                        out_ident,
                        comments_ident,
                        pending_ident,
                        pos_ident,
                    )?);
                    run.clear();
                }
                let comment = comment_expr(style, text);
                output.extend(quote! {
                    #pending_ident.push(#comment);
                });
            }
            Segment::Let { tokens, .. } => {
                if !run.is_empty() {
                    output.extend(flush_stmt_run(
                        &run,
                        &context_map,
                        out_ident,
                        comments_ident,
                        pending_ident,
                        pos_ident,
                    )?);
                    run.clear();
                }
                output.extend(quote! { let #tokens; });
            }
            Segment::Do { expr, .. } => {
                if !run.is_empty() {
                    output.extend(flush_stmt_run(
                        &run,
                        &context_map,
                        out_ident,
                        comments_ident,
                        pending_ident,
                        pos_ident,
                    )?);
                    run.clear();
                }
                output.extend(quote! { #expr; });
            }
            // BraceBlock is handled like other segments - added to the run and processed
            // by build_template_and_bindings, which inlines the content with { }
            _ => run.push(segment),
        }
    }

    if !run.is_empty() {
        output.extend(flush_stmt_run(
            &run,
            &context_map,
            out_ident,
            comments_ident,
            pending_ident,
            pos_ident,
        )?);
    }

    Ok(output)
}

/// Compiles statement-level control nodes into Rust control flow.
fn compile_stmt_control(
    node: &ControlNode,
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
) -> syn::Result<TokenStream2> {
    match node {
        ControlNode::If {
            cond,
            then_branch,
            else_branch,
        } => {
            let then_code = compile_stmt_segments(
                then_branch,
                out_ident,
                comments_ident,
                pending_ident,
                pos_ident,
            )?;
            let else_code = if let Some(branch) = else_branch {
                compile_stmt_segments(branch, out_ident, comments_ident, pending_ident, pos_ident)?
            } else {
                TokenStream2::new()
            };
            Ok(quote! {
                if #cond {
                    #then_code
                } else {
                    #else_code
                }
            })
        }
        ControlNode::IfLet {
            pattern,
            expr,
            then_branch,
            else_branch,
        } => {
            let then_code = compile_stmt_segments(
                then_branch,
                out_ident,
                comments_ident,
                pending_ident,
                pos_ident,
            )?;
            let else_code = if let Some(branch) = else_branch {
                compile_stmt_segments(branch, out_ident, comments_ident, pending_ident, pos_ident)?
            } else {
                TokenStream2::new()
            };
            Ok(quote! {
                if let #pattern = #expr {
                    #then_code
                } else {
                    #else_code
                }
            })
        }
        ControlNode::For { pat, iter, body } => {
            let body_code =
                compile_stmt_segments(body, out_ident, comments_ident, pending_ident, pos_ident)?;
            Ok(quote! {
                for #pat in #iter {
                    #body_code
                }
            })
        }
        ControlNode::While { cond, body } => {
            let body_code =
                compile_stmt_segments(body, out_ident, comments_ident, pending_ident, pos_ident)?;
            Ok(quote! {
                while #cond {
                    #body_code
                }
            })
        }
        ControlNode::WhileLet {
            pattern,
            expr,
            body,
        } => {
            let body_code =
                compile_stmt_segments(body, out_ident, comments_ident, pending_ident, pos_ident)?;
            Ok(quote! {
                while let #pattern = #expr {
                    #body_code
                }
            })
        }
        ControlNode::Match { expr, cases } => {
            let mut arms = TokenStream2::new();
            for case in cases {
                let body_code = compile_stmt_segments(
                    &case.body,
                    out_ident,
                    comments_ident,
                    pending_ident,
                    pos_ident,
                )?;
                let pattern = &case.pattern;
                arms.extend(quote! {
                    #pattern => { #body_code }
                });
            }
            Ok(quote! {
                match #expr {
                    #arms
                }
            })
        }
    }
}

/// Compiles expression-level segments into a single SWC expression.
fn compile_expr_segments(segments: &[Segment]) -> syn::Result<TokenStream2> {
    let context_map = classify_placeholders_expr(segments)?;
    let template_result = build_template_and_bindings(segments, &context_map)?;
    let ident_name_ids = collect_ident_name_ids(segments.iter(), &context_map);
    let quote_ts = quote_ts(&template_result.template, quote!(Expr), &template_result.bindings);
    let QuoteTsResult { bindings, expr } = quote_ts;

    // Generate type placeholder fix if there are any
    let type_fix = generate_type_placeholder_fix(&template_result.type_placeholders);

    if ident_name_ids.is_empty() && type_fix.is_empty() {
        Ok(quote! {{
            #bindings
            #expr
        }})
    } else {
        let expr_ident = proc_macro2::Ident::new("__mf_expr", Span::call_site());
        let fix_block = ident_name_fix_block(&expr_ident, &ident_name_ids);
        Ok(quote! {{
            #bindings
            let mut #expr_ident = #expr;
            #fix_block
            #type_fix
            #expr_ident
        }})
    }
}

/// Compiles control nodes in expression context into Rust expressions.
fn compile_control_expr(node: &ControlNode, span: Span) -> syn::Result<TokenStream2> {
    match node {
        ControlNode::If {
            cond,
            then_branch,
            else_branch,
        } => {
            let then_expr = compile_expr_segments(then_branch)?;
            let else_expr = if let Some(branch) = else_branch {
                compile_expr_segments(branch)?
            } else {
                return Err(template_error(
                    span,
                    "Expression-level {#if} requires an {:else} branch",
                    None,
                ));
            };
            Ok(quote! {
                if #cond {
                    #then_expr
                } else {
                    #else_expr
                }
            })
        }
        ControlNode::IfLet {
            pattern,
            expr,
            then_branch,
            else_branch,
        } => {
            let then_expr = compile_expr_segments(then_branch)?;
            let else_expr = if let Some(branch) = else_branch {
                compile_expr_segments(branch)?
            } else {
                return Err(template_error(
                    span,
                    "Expression-level {#if let} requires an {:else} branch",
                    None,
                ));
            };
            Ok(quote! {
                if let #pattern = #expr {
                    #then_expr
                } else {
                    #else_expr
                }
            })
        }
        ControlNode::Match { expr, cases } => {
            let mut arms = TokenStream2::new();
            for case in cases {
                let pattern = &case.pattern;
                let body_expr = compile_expr_segments(&case.body)?;
                arms.extend(quote! {
                    #pattern => #body_expr,
                });
            }
            Ok(quote! {
                match #expr {
                    #arms
                }
            })
        }
        ControlNode::For { .. } | ControlNode::While { .. } | ControlNode::WhileLet { .. } => {
            Err(template_error(
                span,
                "Loop constructs are not allowed in expression context",
                None,
            ))
        }
    }
}

/// Emits a run of statement segments as parsed SWC statements, attaching comments.
fn flush_stmt_run(
    run: &[&Segment],
    context_map: &HashMap<usize, PlaceholderUse>,
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
) -> syn::Result<TokenStream2> {
    let template_result = build_template_and_bindings(run.iter().copied(), context_map)?;
    if template_result.template.trim().is_empty() {
        return Ok(TokenStream2::new());
    }
    let ident_name_ids = collect_ident_name_ids(run.iter().copied(), context_map);
    let ident_name_fix =
        ident_name_fix_block(&proc_macro2::Ident::new("__mf_stmt", Span::call_site()), &ident_name_ids);

    // Generate type placeholder fix for replacing __MfTypeN with actual types
    let type_fix = generate_type_placeholder_fix(&template_result.type_placeholders);

    // Collect BraceBlocks that need block substitution
    let block_compilations = collect_block_compilations(
        run,
        context_map,
        comments_ident,
        pending_ident,
        pos_ident,
    )?;

    let mut output = TokenStream2::new();

    // If we have type placeholders, use runtime parsing with full TypeScript support.
    // This is needed because SWC's quote! macro doesn't support $placeholder in type positions.
    if !template_result.type_placeholders.is_empty() {
        let template_str = syn::LitStr::new(&template_result.template, Span::call_site());

        // Generate binding initializations
        let mut binding_inits = TokenStream2::new();
        for binding in &template_result.bindings {
            let name = &binding.name;
            let ty = &binding.ty;
            let expr = &binding.expr;
            binding_inits.extend(quote! { let #name: #ty = #expr; });
        }

        // Generate type placeholder initializations
        for tp in &template_result.type_placeholders {
            let field_name = format_ident!("__mf_type_{}", tp.id);
            let expr = &tp.expr;
            binding_inits.extend(quote! { let #field_name: String = (#expr).to_string(); });
        }

        // Generate visitor struct fields
        let mut visitor_fields = Vec::new();
        let mut visitor_inits = Vec::new();

        // Ident substitution fields
        for binding in &template_result.bindings {
            let name = &binding.name;
            let placeholder = name.to_string();
            let field_name = format_ident!("binding_{}", name);
            let ty = &binding.ty;
            visitor_fields.push(quote! { #field_name: #ty });
            visitor_inits.push(quote! { #field_name: #name.clone() });
        }

        // Type substitution fields
        for tp in &template_result.type_placeholders {
            let field_name = format_ident!("__mf_type_{}", tp.id);
            visitor_fields.push(quote! { #field_name: String });
            visitor_inits.push(quote! { #field_name: #field_name.clone() });
        }

        // Generate ident match arms - only for Ident-typed bindings (not Expr)
        // Template uses $__mf_hole_N format, which in TypeScript parses as ident with sym="$__mf_hole_N"
        let ident_arms: Vec<_> = template_result.bindings.iter()
            .filter(|binding| binding.ty.to_string() == "Ident")
            .map(|binding| {
                let name = &binding.name;
                // Include $ prefix since template uses $__mf_hole_N format
                let placeholder = format!("${}", name.to_string());
                let field_name = format_ident!("binding_{}", name);
                quote! {
                    #placeholder => {
                        ident.sym = self.#field_name.sym.clone();
                    }
                }
            }).collect();

        // Generate expr match arms - for Expr-typed bindings
        // When the placeholder appears as an expression (e.g., function name), replace the entire expr
        let expr_arms: Vec<_> = template_result.bindings.iter()
            .filter(|binding| binding.ty.to_string() == "Expr")
            .map(|binding| {
                let name = &binding.name;
                // Include $ prefix since template uses $__mf_hole_N format
                let placeholder = format!("${}", name.to_string());
                let field_name = format_ident!("binding_{}", name);
                quote! {
                    #placeholder => Some(self.#field_name.clone()),
                }
            }).collect();

        // Use swc_core directly - macroforge_ts has it as a direct dependency
        // just like macroforge_ts_syn's macros (ident!, fn_expr!, etc.) do
        let swc_core_path = quote!(swc_core);

        // Generate type match arms
        // Uses types from `use #swc_core_path::ecma::ast::*;` and common imports
        let type_arms: Vec<_> = template_result.type_placeholders.iter().map(|tp| {
            let marker = format!("__MfTypeMarker{}", tp.id);
            let field_name = format_ident!("__mf_type_{}", tp.id);
            let swc_path = &swc_core_path;
            quote! {
                #marker => {
                    *ty = TsType::TsTypeRef(TsTypeRef {
                        span: #swc_path::common::DUMMY_SP,
                        type_name: TsEntityName::Ident(Ident::new(
                            self.#field_name.clone().into(),
                            #swc_path::common::DUMMY_SP,
                            Default::default(),
                        )),
                        type_params: None,
                    });
                }
            }
        }).collect();
        output.extend(quote! {{
            #binding_inits
            use #swc_core_path::common::{FileName, SourceMap, sync::Lrc};
            use #swc_core_path::ecma::parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};
            use #swc_core_path::ecma::visit::{VisitMut, VisitMutWith};
            use #swc_core_path::ecma::ast::*;

            let __mf_cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
            let __mf_fm = __mf_cm.new_source_file(
                FileName::Custom("template.ts".into()).into(),
                #template_str.to_string(),
            );
            let __mf_syntax = Syntax::Typescript(TsSyntax {
                tsx: true,
                decorators: true,
                ..Default::default()
            });
            let __mf_lexer = Lexer::new(__mf_syntax, EsVersion::latest(), StringInput::from(&*__mf_fm), None);
            let mut __mf_parser = Parser::new_from(__mf_lexer);
            let __mf_module = __mf_parser.parse_module().expect("Failed to parse TypeScript template");

            struct __MfSubstitutor {
                #(#visitor_fields,)*
            }

            impl VisitMut for __MfSubstitutor {
                fn visit_mut_ident(&mut self, ident: &mut Ident) {
                    let name = ident.sym.as_ref();
                    match name {
                        #(#ident_arms)*
                        _ => {}
                    }
                }

                fn visit_mut_expr(&mut self, expr: &mut Expr) {
                    // First check if this is an ident placeholder that should be replaced
                    let replacement = if let Expr::Ident(ident) = &*expr {
                        match ident.sym.as_ref() {
                            #(#expr_arms)*
                            _ => None
                        }
                    } else {
                        None
                    };
                    // Apply replacement if found, otherwise continue visiting children
                    if let Some(new_expr) = replacement {
                        *expr = new_expr;
                    } else {
                        expr.visit_mut_children_with(self);
                    }
                }

                fn visit_mut_ts_type(&mut self, ty: &mut TsType) {
                    if let TsType::TsTypeRef(type_ref) = ty {
                        if let TsEntityName::Ident(ident) = &type_ref.type_name {
                            match ident.sym.as_ref() {
                                #(#type_arms)*
                                _ => {}
                            }
                        }
                    }
                    ty.visit_mut_children_with(self);
                }
            }

            let mut __mf_substitutor = __MfSubstitutor {
                #(#visitor_inits,)*
            };

            for __mf_item in __mf_module.body {
                let mut __mf_stmt = match __mf_item {
                    ModuleItem::Stmt(s) => s,
                    ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export)) => Stmt::Decl(export.decl),
                    _ => continue,
                };

                __mf_stmt.visit_mut_with(&mut __mf_substitutor);

                let __mf_pos = #swc_core_path::common::BytePos(#pos_ident);
                #pos_ident += 1;
                {
                    struct __MfSpanFix {
                        span: #swc_core_path::common::Span,
                    }
                    impl VisitMut for __MfSpanFix {
                        fn visit_mut_span(&mut self, span: &mut #swc_core_path::common::Span) {
                            *span = self.span;
                        }
                    }
                    let mut __mf_span_fix = __MfSpanFix {
                        span: #swc_core_path::common::Span::new(__mf_pos, __mf_pos),
                    };
                    __mf_stmt.visit_mut_with(&mut __mf_span_fix);
                }
                if !#pending_ident.is_empty() {
                    use #swc_core_path::common::comments::Comments;
                    for __mf_comment in #pending_ident.drain(..) {
                        #comments_ident.add_leading(__mf_pos, __mf_comment);
                    }
                }
                #out_ident.push(__mf_stmt);
            }
        }});
        return Ok(output);
    }

    // Standard path: no type placeholders, use quote! for better performance
    let (module, cm) = parse_ts_module_with_source(&template_result.template)?;

    for item in module.body {
        match item {
            ModuleItem::Stmt(stmt) => {
                let snippet = cm.span_to_snippet(stmt.span()).map_err(|e| {
                    syn::Error::new(Span::call_site(), format!("TypeScript span error: {e:?}"))
                })?;
                let snippet = snippet.trim();
                if snippet.is_empty() {
                    continue;
                }
                let quote_ts = quote_ts(snippet, quote!(Stmt), &template_result.bindings);
                let QuoteTsResult { bindings, expr } = quote_ts;

                // Generate block replacement code if we have blocks to replace
                let block_replacement = if block_compilations.is_empty() {
                    TokenStream2::new()
                } else {
                    let mut block_replacements = TokenStream2::new();
                    for (block_id, block_code) in &block_compilations {
                        let marker = format!("__mf_block_{}", block_id);
                        block_replacements.extend(quote! {
                            (#marker, {
                                let mut __mf_block_stmts: Vec<swc_core::ecma::ast::Stmt> = Vec::new();
                                #block_code
                                __mf_block_stmts
                            }),
                        });
                    }
                    quote! {
                        {
                            use swc_core::ecma::visit::{VisitMut, VisitMutWith};
                            use swc_core::ecma::ast::{Stmt, Expr, ExprStmt, Ident};

                            struct __MfBlockReplacer {
                                blocks: std::collections::HashMap<String, Vec<Stmt>>,
                            }

                            impl VisitMut for __MfBlockReplacer {
                                fn visit_mut_block_stmt(&mut self, block: &mut swc_core::ecma::ast::BlockStmt) {
                                    // First, check if this block contains a marker statement
                                    let marker_id = block.stmts.iter().find_map(|stmt| {
                                        if let Stmt::Expr(ExprStmt { expr, .. }) = stmt {
                                            if let Expr::Ident(ident) = &**expr {
                                                let name = ident.sym.as_ref();
                                                if name.starts_with("__mf_block_") {
                                                    return Some(name.to_string());
                                                }
                                            }
                                        }
                                        None
                                    });

                                    if let Some(marker) = marker_id {
                                        if let Some(compiled_stmts) = self.blocks.remove(&marker) {
                                            block.stmts = compiled_stmts;
                                            return; // Don't recurse into replaced block
                                        }
                                    }

                                    // Recurse into children
                                    block.visit_mut_children_with(self);
                                }
                            }

                            let mut __mf_block_replacer = __MfBlockReplacer {
                                blocks: [#block_replacements].into_iter().collect(),
                            };
                            __mf_stmt.visit_mut_with(&mut __mf_block_replacer);
                        }
                    }
                };

                output.extend(quote! {{
                    #bindings
                    let mut __mf_stmt = #expr;
                    #ident_name_fix
                    #type_fix
                    #block_replacement
                    let __mf_pos = swc_core::common::BytePos(#pos_ident);
                    #pos_ident += 1;
                    {
                        use swc_core::ecma::visit::{VisitMut, VisitMutWith};
                        struct __MfSpanFix {
                            span: swc_core::common::Span,
                        }
                        impl VisitMut for __MfSpanFix {
                            fn visit_mut_span(&mut self, span: &mut swc_core::common::Span) {
                                *span = self.span;
                            }
                        }
                        let mut __mf_span_fix = __MfSpanFix {
                            span: swc_core::common::Span::new(__mf_pos, __mf_pos),
                        };
                        __mf_stmt.visit_mut_with(&mut __mf_span_fix);
                    }
                    if !#pending_ident.is_empty() {
                        use swc_core::common::comments::Comments;
                        for __mf_comment in #pending_ident.drain(..) {
                            #comments_ident.add_leading(__mf_pos, __mf_comment);
                        }
                    }
                    #out_ident.push(__mf_stmt);
                }});
            }
            ModuleItem::ModuleDecl(decl) => {
                // Handle export declarations (export function, export const, etc.)
                // Since ts_template outputs Vec<Stmt>, we convert exports to their inner declarations
                match &decl {
                    ModuleDecl::ExportDecl(_) => {
                        // Get the FULL snippet including "export" keyword
                        // We quote as ModuleItem (which supports TypeScript) then extract the Decl
                        let snippet = cm.span_to_snippet(decl.span()).map_err(|e| {
                            syn::Error::new(Span::call_site(), format!("TypeScript span error: {e:?}"))
                        })?;
                        let snippet = snippet.trim();
                        if snippet.is_empty() {
                            continue;
                        }

                        // Quote as ModuleItem - this supports full TypeScript syntax including
                        // type annotations in function parameters (unlike quote!(... as Stmt))
                        let quote_ts = quote_ts(snippet, quote!(ModuleItem), &template_result.bindings);
                        let QuoteTsResult {
                            bindings: quote_bindings,
                            expr,
                        } = quote_ts;

                        // Generate block replacement code if we have blocks to replace
                        let block_replacement = if block_compilations.is_empty() {
                            TokenStream2::new()
                        } else {
                            let mut block_replacements = TokenStream2::new();
                            for (block_id, block_code) in &block_compilations {
                                let marker = format!("__mf_block_{}", block_id);
                                block_replacements.extend(quote! {
                                    (#marker, {
                                        let mut __mf_block_stmts: Vec<swc_core::ecma::ast::Stmt> = Vec::new();
                                        #block_code
                                        __mf_block_stmts
                                    }),
                                });
                            }
                            quote! {
                                {
                                    use swc_core::ecma::visit::{VisitMut, VisitMutWith};
                                    use swc_core::ecma::ast::{Stmt, Expr, ExprStmt, Ident};

                                    struct __MfBlockReplacer {
                                        blocks: std::collections::HashMap<String, Vec<Stmt>>,
                                    }

                                    impl VisitMut for __MfBlockReplacer {
                                        fn visit_mut_block_stmt(&mut self, block: &mut swc_core::ecma::ast::BlockStmt) {
                                            let marker_id = block.stmts.iter().find_map(|stmt| {
                                                if let Stmt::Expr(ExprStmt { expr, .. }) = stmt {
                                                    if let Expr::Ident(ident) = &**expr {
                                                        let name = ident.sym.as_ref();
                                                        if name.starts_with("__mf_block_") {
                                                            return Some(name.to_string());
                                                        }
                                                    }
                                                }
                                                None
                                            });

                                            if let Some(marker) = marker_id {
                                                if let Some(compiled_stmts) = self.blocks.remove(&marker) {
                                                    block.stmts = compiled_stmts;
                                                    return;
                                                }
                                            }
                                            block.visit_mut_children_with(self);
                                        }
                                    }

                                    let mut __mf_block_replacer = __MfBlockReplacer {
                                        blocks: [#block_replacements].into_iter().collect(),
                                    };
                                    __mf_stmt.visit_mut_with(&mut __mf_block_replacer);
                                }
                            }
                        };

                        output.extend(quote! {{
                            #quote_bindings
                            // Quote as ModuleItem, then extract the inner Decl and convert to Stmt
                            let __mf_module_item = #expr;
                            let mut __mf_stmt = match __mf_module_item {
                                swc_core::ecma::ast::ModuleItem::ModuleDecl(
                                    swc_core::ecma::ast::ModuleDecl::ExportDecl(export)
                                ) => swc_core::ecma::ast::Stmt::Decl(export.decl),
                                swc_core::ecma::ast::ModuleItem::Stmt(s) => s,
                                _ => panic!("unexpected module item type in ts_template"),
                            };
                            #block_replacement
                            #ident_name_fix
                            #type_fix
                            let __mf_pos = swc_core::common::BytePos(#pos_ident);
                            #pos_ident += 1;
                            {
                                use swc_core::ecma::visit::{VisitMut, VisitMutWith};
                                struct __MfSpanFix {
                                    span: swc_core::common::Span,
                                }
                                impl VisitMut for __MfSpanFix {
                                    fn visit_mut_span(&mut self, span: &mut swc_core::common::Span) {
                                        *span = self.span;
                                    }
                                }
                                let mut __mf_span_fix = __MfSpanFix {
                                    span: swc_core::common::Span::new(__mf_pos, __mf_pos),
                                };
                                __mf_stmt.visit_mut_with(&mut __mf_span_fix);
                            }
                            if !#pending_ident.is_empty() {
                                use swc_core::common::comments::Comments;
                                for __mf_comment in #pending_ident.drain(..) {
                                    #comments_ident.add_leading(__mf_pos, __mf_comment);
                                }
                            }
                            #out_ident.push(__mf_stmt);
                        }});
                    }
                    ModuleDecl::ExportDefaultDecl(_) | ModuleDecl::ExportDefaultExpr(_) => {
                        return Err(template_error(
                            Span::call_site(),
                            "Export default declarations are not supported in ts_template. Use export without default.",
                            None,
                        ));
                    }
                    _ => {
                        return Err(template_error(
                            Span::call_site(),
                            "Import declarations are not supported in ts_template",
                            None,
                        ));
                    }
                }
            }
        }
    }

    Ok(output)
}

/// Collects BraceBlocks that need block substitution and compiles their inner segments.
fn collect_block_compilations(
    run: &[&Segment],
    context_map: &HashMap<usize, PlaceholderUse>,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
) -> syn::Result<Vec<(usize, TokenStream2)>> {
    let mut compilations = Vec::new();
    let out_ident = proc_macro2::Ident::new("__mf_block_stmts", Span::call_site());

    fn collect_from_segment(
        seg: &Segment,
        context_map: &HashMap<usize, PlaceholderUse>,
        out_ident: &proc_macro2::Ident,
        comments_ident: &proc_macro2::Ident,
        pending_ident: &proc_macro2::Ident,
        pos_ident: &proc_macro2::Ident,
        compilations: &mut Vec<(usize, TokenStream2)>,
    ) -> syn::Result<()> {
        if let Segment::BraceBlock { id, inner, .. } = seg {
            // Check if this block has statement-level control flow
            fn has_stmt_level_control(
                segments: &[Segment],
                context_map: &HashMap<usize, PlaceholderUse>,
            ) -> bool {
                for s in segments {
                    match s {
                        Segment::Control { id, .. } => {
                            if matches!(context_map.get(id), Some(PlaceholderUse::Stmt)) {
                                return true;
                            }
                        }
                        Segment::BraceBlock { inner, .. } => {
                            if has_stmt_level_control(inner, context_map) {
                                return true;
                            }
                        }
                        _ => {}
                    }
                }
                false
            }

            if has_stmt_level_control(inner, context_map) {
                // Compile the inner segments
                let compiled = compile_stmt_segments(
                    inner,
                    out_ident,
                    comments_ident,
                    pending_ident,
                    pos_ident,
                )?;
                compilations.push((*id, compiled));
            }

            // Recurse into inner segments to find nested BraceBlocks
            for inner_seg in inner {
                collect_from_segment(
                    inner_seg,
                    context_map,
                    out_ident,
                    comments_ident,
                    pending_ident,
                    pos_ident,
                    compilations,
                )?;
            }
        }
        Ok(())
    }

    for seg in run {
        collect_from_segment(
            seg,
            context_map,
            &out_ident,
            comments_ident,
            pending_ident,
            pos_ident,
            &mut compilations,
        )?;
    }

    Ok(compilations)
}

struct BindingSpec {
    name: proc_macro2::Ident,
    ty: TokenStream2,
    expr: TokenStream2,
}

/// Type placeholder that needs post-processing via visitor (since SWC quote! doesn't support $ in types)
struct TypePlaceholder {
    id: usize,
    expr: TokenStream2,
}

struct QuoteTsResult {
    bindings: TokenStream2,
    expr: TokenStream2,
}

/// Result of building template and bindings
struct TemplateAndBindings {
    template: String,
    bindings: Vec<BindingSpec>,
    type_placeholders: Vec<TypePlaceholder>,
}

/// Builds the placeholder template string and binding list for a segment run.
fn build_template_and_bindings(
    segments: impl IntoIterator<Item = impl std::borrow::Borrow<Segment>>,
    context_map: &HashMap<usize, PlaceholderUse>,
) -> syn::Result<TemplateAndBindings> {
    let mut template = String::new();
    let mut bindings = Vec::new();
    let mut type_placeholders = Vec::new();
    let mut seen = HashSet::new();

    for seg in segments {
        match seg.borrow() {
            Segment::Static(s) => append_part(&mut template, s),
            Segment::Comment { span, .. } => {
                return Err(template_error(
                    *span,
                    "Comments must appear between TypeScript statements",
                    None,
                ));
            }
            Segment::Interpolation { id, expr, span: _ } => {
                let use_kind = context_map.get(id).cloned().unwrap_or(PlaceholderUse::Expr);

                if matches!(use_kind, PlaceholderUse::Type) {
                    // For type placeholders, use a unique marker identifier that quote! can parse
                    // We'll replace it with the actual type using a visitor after quote! parsing
                    let marker = format!("__MfTypeMarker{}", id);
                    append_part(&mut template, &marker);
                    if seen.insert(*id) {
                        type_placeholders.push(TypePlaceholder {
                            id: *id,
                            expr: expr.clone(),
                        });
                    }
                } else {
                    // Non-type placeholders use $ prefix for quote! substitution
                    let name = placeholder_name(*id);
                    append_part(&mut template, &format!("${name}"));
                    if seen.insert(*id) {
                        let ty = placeholder_type_tokens(&use_kind);
                        let bind_ident = proc_macro2::Ident::new(&name, Span::call_site());
                        bindings.push(BindingSpec {
                            name: bind_ident,
                            ty,
                            expr: expr.clone(),
                        });
                    }
                }
            }
            Segment::StringInterp { id, parts, span: _ } => {
                let name = placeholder_name(*id);
                append_part(&mut template, &format!("${name}"));
                if seen.insert(*id) {
                    let expr = build_string_interp_expr(parts, *id);
                    let ty = placeholder_type_tokens(&PlaceholderUse::Expr);
                    let bind_ident = proc_macro2::Ident::new(&name, Span::call_site());
                    bindings.push(BindingSpec {
                        name: bind_ident,
                        ty,
                        expr,
                    });
                }
            }
            Segment::TemplateInterp { id, parts, span: _ } => {
                let name = placeholder_name(*id);
                append_part(&mut template, &format!("${name}"));
                if seen.insert(*id) {
                    let expr = build_template_interp_expr(parts, *id);
                    let ty = placeholder_type_tokens(&PlaceholderUse::Expr);
                    let bind_ident = proc_macro2::Ident::new(&name, Span::call_site());
                    bindings.push(BindingSpec {
                        name: bind_ident,
                        ty,
                        expr,
                    });
                }
            }
            Segment::IdentBlock { id, parts, span: _ } => {
                let name = placeholder_name(*id);
                append_part(&mut template, &format!("${name}"));
                if seen.insert(*id) {
                    let ident_expr = compile_ident_block(parts);
                    let ty = placeholder_type_tokens(&PlaceholderUse::Ident);
                    let bind_ident = proc_macro2::Ident::new(&name, Span::call_site());
                    bindings.push(BindingSpec {
                        name: bind_ident,
                        ty,
                        expr: ident_expr,
                    });
                }
            }
            Segment::Control { id, node, span } => {
                let name = placeholder_name(*id);
                let use_kind = context_map.get(id).cloned().unwrap_or(PlaceholderUse::Expr);
                if matches!(use_kind, PlaceholderUse::Stmt) {
                    return Err(template_error(
                        *span,
                        "Statement-level control blocks must stand alone",
                        None,
                    ));
                }
                if matches!(use_kind, PlaceholderUse::Ident | PlaceholderUse::Type) {
                    return Err(template_error(
                        *span,
                        "Control blocks cannot appear in identifier/type positions",
                        None,
                    ));
                }
                append_part(&mut template, &format!("${name}"));
                if seen.insert(*id) {
                    let expr = compile_control_expr(node, *span)?;
                    let ty = placeholder_type_tokens(&PlaceholderUse::Expr);
                    let bind_ident = proc_macro2::Ident::new(&name, Span::call_site());
                    bindings.push(BindingSpec {
                        name: bind_ident,
                        ty,
                        expr,
                    });
                }
            }
            Segment::Typescript { id, span, .. } => {
                let use_kind = context_map.get(id).cloned().unwrap_or(PlaceholderUse::Stmt);
                if !matches!(use_kind, PlaceholderUse::Stmt) {
                    return Err(template_error(
                        *span,
                        "{$typescript} is only valid at statement boundaries",
                        None,
                    ));
                }
                return Err(template_error(
                    *span,
                    "{$typescript} must be placed as its own statement",
                    None,
                ));
            }
            Segment::Let { span, .. } | Segment::Do { span, .. } => {
                return Err(template_error(
                    *span,
                    "{$let} and {$do} cannot appear inside TypeScript expressions",
                    None,
                ));
            }
            Segment::BraceBlock { id, inner, span: _ } => {
                // Check if any inner segment contains statement-level control flow
                fn has_stmt_level_control(
                    segments: &[Segment],
                    context_map: &HashMap<usize, PlaceholderUse>,
                ) -> bool {
                    for s in segments {
                        match s {
                            Segment::Control { id, .. } => {
                                if matches!(context_map.get(id), Some(PlaceholderUse::Stmt)) {
                                    return true;
                                }
                            }
                            Segment::BraceBlock { inner, .. } => {
                                if has_stmt_level_control(inner, context_map) {
                                    return true;
                                }
                            }
                            _ => {}
                        }
                    }
                    false
                }

                let has_stmt_control = has_stmt_level_control(inner, context_map);

                if has_stmt_control {
                    // For blocks with statement-level control, emit a placeholder
                    // that will be replaced with the compiled block body after parsing
                    let block_marker = format!("__mf_block_{}", id);
                    append_part(&mut template, &format!("{{ {}; }}", block_marker));
                    // Note: The BraceBlock is NOT added to bindings here.
                    // It will be compiled and substituted in flush_stmt_run.
                } else {
                    // For blocks without statement-level control, inline the content
                    append_part(&mut template, "{");
                    let inner_result = build_template_and_bindings(inner.iter(), context_map)?;
                    append_part(&mut template, &inner_result.template);
                    bindings.extend(inner_result.bindings);
                    type_placeholders.extend(inner_result.type_placeholders);
                    append_part(&mut template, "}");
                }
            }
        }
    }

    Ok(TemplateAndBindings {
        template,
        bindings,
        type_placeholders,
    })
}

/// Collects placeholder IDs used in `IdentName` positions.
fn collect_ident_name_ids<'a>(
    segments: impl IntoIterator<Item = &'a Segment>,
    context_map: &HashMap<usize, PlaceholderUse>,
) -> Vec<usize> {
    let mut ids = HashSet::new();

    fn collect_from_segment(
        seg: &Segment,
        context_map: &HashMap<usize, PlaceholderUse>,
        ids: &mut HashSet<usize>,
    ) {
        let id = match seg {
            Segment::Interpolation { id, .. }
            | Segment::StringInterp { id, .. }
            | Segment::TemplateInterp { id, .. }
            | Segment::IdentBlock { id, .. }
            | Segment::Control { id, .. }
            | Segment::Typescript { id, .. } => Some(*id),
            Segment::BraceBlock { inner, .. } => {
                for inner_seg in inner {
                    collect_from_segment(inner_seg, context_map, ids);
                }
                None
            }
            _ => None,
        };
        if let Some(id) = id
            && matches!(context_map.get(&id), Some(PlaceholderUse::IdentName))
        {
            ids.insert(id);
        }
    }

    for seg in segments {
        collect_from_segment(seg, context_map, &mut ids);
    }

    let mut ids: Vec<_> = ids.into_iter().collect();
    ids.sort_unstable();
    ids
}

/// Rewrites placeholder `IdentName`s (e.g. `$__mf_hole_0`) into real identifiers.
fn ident_name_fix_block(
    target_ident: &proc_macro2::Ident,
    placeholder_ids: &[usize],
) -> TokenStream2 {
    if placeholder_ids.is_empty() {
        return TokenStream2::new();
    }

    let fields: Vec<TokenStream2> = placeholder_ids
        .iter()
        .map(|id| {
            let field_ident = proc_macro2::Ident::new(&placeholder_name(*id), Span::call_site());
            quote! { #field_ident: swc_core::ecma::ast::IdentName }
        })
        .collect();
    let inits: Vec<TokenStream2> = placeholder_ids
        .iter()
        .map(|id| {
            let field_ident = proc_macro2::Ident::new(&placeholder_name(*id), Span::call_site());
            quote! { #field_ident: swc_core::ecma::ast::IdentName::from(#field_ident.clone()) }
        })
        .collect();
    let arms: Vec<TokenStream2> = placeholder_ids
        .iter()
        .map(|id| {
            let marker = format!("${}", placeholder_name(*id));
            let field_ident = proc_macro2::Ident::new(&placeholder_name(*id), Span::call_site());
            quote! {
                #marker => {
                    *ident = self.#field_ident.clone();
                }
            }
        })
        .collect();
    let member_arms = arms.clone();
    let prop_arms = arms;

    quote! {
        {
            use swc_core::ecma::visit::{VisitMut, VisitMutWith};
            struct __MfIdentNameFix {
                #(#fields,)*
            }
            impl VisitMut for __MfIdentNameFix {
                fn visit_mut_member_prop(&mut self, prop: &mut swc_core::ecma::ast::MemberProp) {
                    if let swc_core::ecma::ast::MemberProp::Ident(ident) = prop {
                        match ident.sym.as_ref() {
                            #(#member_arms)*
                            _ => {}
                        }
                    }
                    prop.visit_mut_children_with(self);
                }

                fn visit_mut_prop_name(&mut self, prop: &mut swc_core::ecma::ast::PropName) {
                    if let swc_core::ecma::ast::PropName::Ident(ident) = prop {
                        match ident.sym.as_ref() {
                            #(#prop_arms)*
                            _ => {}
                        }
                    }
                    prop.visit_mut_children_with(self);
                }
            }
            let mut __mf_ident_fix = __MfIdentNameFix {
                #(#inits,)*
            };
            #target_ident.visit_mut_with(&mut __mf_ident_fix);
        }
    }
}

/// Generates code to replace type placeholders like __MfType0 with actual type references.
///
/// SWC's quote! macro doesn't support $ placeholders in type positions, so we use
/// regular identifiers and replace them via a visitor after quoting.
fn generate_type_placeholder_fix(type_placeholders: &[TypePlaceholder]) -> TokenStream2 {
    if type_placeholders.is_empty() {
        return TokenStream2::new();
    }

    let fields: Vec<_> = type_placeholders
        .iter()
        .map(|tp| {
            let field_name = format_ident!("__mf_type_{}", tp.id);
            quote! { #field_name: String }
        })
        .collect();

    let inits: Vec<_> = type_placeholders
        .iter()
        .map(|tp| {
            let field_name = format_ident!("__mf_type_{}", tp.id);
            let expr = &tp.expr;
            // Convert the expression to a String for the type name
            quote! { #field_name: (#expr).to_string() }
        })
        .collect();

    let arms: Vec<_> = type_placeholders
        .iter()
        .map(|tp| {
            let marker = format!("__MfTypeMarker{}", tp.id);
            let field_name = format_ident!("__mf_type_{}", tp.id);
            quote! {
                #marker => {
                    *ty = swc_core::ecma::ast::TsType::TsTypeRef(swc_core::ecma::ast::TsTypeRef {
                        span: swc_core::common::DUMMY_SP,
                        type_name: swc_core::ecma::ast::TsEntityName::Ident(
                            swc_core::ecma::ast::Ident::new(
                                self.#field_name.clone().into(),
                                swc_core::common::DUMMY_SP,
                                Default::default(),
                            )
                        ),
                        type_params: None,
                    });
                }
            }
        })
        .collect();

    quote! {
        {
            use swc_core::ecma::visit::{VisitMut, VisitMutWith};
            struct __MfTypeFix {
                #(#fields,)*
            }
            impl VisitMut for __MfTypeFix {
                fn visit_mut_ts_type(&mut self, ty: &mut swc_core::ecma::ast::TsType) {
                    if let swc_core::ecma::ast::TsType::TsTypeRef(type_ref) = ty {
                        if let swc_core::ecma::ast::TsEntityName::Ident(ident) = &type_ref.type_name {
                            match ident.sym.as_ref() {
                                #(#arms)*
                                _ => {}
                            }
                        }
                    }
                    ty.visit_mut_children_with(self);
                }
            }
            let mut __mf_type_fix = __MfTypeFix {
                #(#inits,)*
            };
            __mf_stmt.visit_mut_with(&mut __mf_type_fix);
        }
    }
}

/// Maps placeholder usage to the SWC AST type used for binding in quote!.
///
/// NOTE: SWC's quote! macro only supports Ident, Expr, and Pat for variable
/// substitution. For Type placeholders, we use Ident and later convert them
/// to TsType via a post-processing visitor.
fn placeholder_type_tokens(use_kind: &PlaceholderUse) -> TokenStream2 {
    match use_kind {
        PlaceholderUse::Expr | PlaceholderUse::Stmt => quote!(Expr),
        PlaceholderUse::Ident => quote!(Ident),
        PlaceholderUse::IdentName => quote!(Ident),
        // Use Ident for types too - SWC quote! doesn't support TsType
        // The Ident will be placed in type position and needs no conversion
        // because TypeScript type references are just identifiers
        PlaceholderUse::Type => quote!(Ident),
    }
}

/// Builds an SWC identifier from an ident block's parts.
fn compile_ident_block(parts: &[IdentPart]) -> TokenStream2 {
    let mut stmts = TokenStream2::new();
    let mut part_index = 0usize;

    for part in parts {
        match part {
            IdentPart::Static(s) => {
                stmts.extend(quote! {
                    __mf_name.push_str(#s);
                });
            }
            IdentPart::Interpolation { expr, .. } => {
                let var =
                    proc_macro2::Ident::new(&format!("__mf_part_{part_index}"), Span::call_site());
                part_index += 1;
                stmts.extend(quote! {
                    let #var: swc_core::ecma::ast::Ident = #expr;
                    __mf_name.push_str(&#var.sym.to_string());
                });
            }
        }
    }

    quote! {
        {
            let mut __mf_name = String::new();
            #stmts
            swc_core::ecma::ast::Ident::new(__mf_name.into(), swc_core::common::DUMMY_SP)
        }
    }
}

/// Emits Rust code that injects a TsStream into the statement list.
fn compile_ts_injection(
    expr: &TokenStream2,
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
) -> TokenStream2 {
    quote! {
        {
            let mut __mf_stream = #expr;
            __patches.extend(__mf_stream.runtime_patches.drain(..));
            let __mf_source = __mf_stream.source().to_string();
            let mut __mf_parser = macroforge_ts::ts_syn::TsStream::from_string(__mf_source);
            let __mf_module: swc_core::ecma::ast::Module = __mf_parser.parse().expect("Failed to parse injected TsStream");
            let mut __mf_first = true;
            for __mf_item in __mf_module.body {
                if let swc_core::ecma::ast::ModuleItem::Stmt(stmt) = __mf_item {
                    if __mf_first {
                        __mf_first = false;
                        if !#pending_ident.is_empty() {
                            use swc_core::common::comments::Comments;
                            use swc_core::common::Spanned;
                            let __mf_pos = stmt.span().lo();
                            for __mf_comment in #pending_ident.drain(..) {
                                #comments_ident.add_leading(__mf_pos, __mf_comment);
                            }
                        }
                    }
                    #out_ident.push(stmt);
                }
            }
        }
    }
}

/// Builds a SWC comment literal for the pending comment buffer.
fn comment_expr(style: &CommentStyle, text: &str) -> TokenStream2 {
    let mut content = text.trim().to_string();
    let kind = match style {
        CommentStyle::DocBlock => {
            if !content.starts_with('*') {
                content = format!("* {}", content.trim_start());
            }
            CommentKind::Block
        }
        CommentStyle::Block => {
            if !content.starts_with(' ') {
                content = format!(" {}", content.trim_start());
            }
            CommentKind::Block
        }
        CommentStyle::Line => {
            if !content.starts_with(' ') {
                content = format!(" {}", content.trim_start());
            }
            CommentKind::Line
        }
    };

    content = content.trim_end().to_string();
    if !matches!(kind, CommentKind::Line) && !content.ends_with(' ') {
        content.push(' ');
    }

    let kind_tokens = match kind {
        CommentKind::Line => quote!(swc_core::common::comments::CommentKind::Line),
        CommentKind::Block => quote!(swc_core::common::comments::CommentKind::Block),
    };
    let text_lit = syn::LitStr::new(&content, Span::call_site());
    quote! {
        swc_core::common::comments::Comment {
            kind: #kind_tokens,
            span: swc_core::common::DUMMY_SP,
            text: #text_lit.into(),
        }
    }
}

/// Placeholder source kind for parsing.
#[derive(Clone, Copy)]
enum PlaceholderSourceKind {
    Module,
    Expr,
}

/// Builds a placeholder-only source string and placeholder ID map.
fn build_placeholder_source(
    segments: &[Segment],
    kind: PlaceholderSourceKind,
) -> (String, HashMap<String, usize>) {
    let mut src = String::new();
    let mut map = HashMap::new();

    for seg in segments {
        match seg {
            Segment::Static(s) => append_part(&mut src, s),
            Segment::Comment { .. } => {}
            Segment::Interpolation { id, .. }
            | Segment::StringInterp { id, .. }
            | Segment::TemplateInterp { id, .. }
            | Segment::IdentBlock { id, .. }
            | Segment::Control { id, .. }
            | Segment::Typescript { id, .. } => {
                let name = placeholder_name(*id);
                append_part(&mut src, &name);
                if matches!(kind, PlaceholderSourceKind::Module)
                    && matches!(seg, Segment::Control { .. } | Segment::Typescript { .. })
                {
                    src.push(';');
                }
                map.insert(name, *id);
            }
            Segment::Let { .. } | Segment::Do { .. } => {
                // Rust-only constructs are ignored for TS parsing.
            }
            Segment::BraceBlock { inner, .. } => {
                // Recursively process inner segments with braces
                append_part(&mut src, "{");
                let (inner_src, inner_map) = build_placeholder_source(inner, kind);
                append_part(&mut src, &inner_src);
                map.extend(inner_map);
                append_part(&mut src, "}");
            }
        }
    }

    (src, map)
}

/// Classifies placeholder usage by parsing the segments as a module.
/// Falls back to wrapping in a class if the template contains class body members.
fn classify_placeholders_module(
    segments: &[Segment],
) -> syn::Result<HashMap<usize, PlaceholderUse>> {
    let (source, map) = build_placeholder_source(segments, PlaceholderSourceKind::Module);
    if source.trim().is_empty() {
        return Ok(HashMap::new());
    }

    // Try parsing as a module first
    if let Ok(module) = parse_ts_module(&source) {
        let mut finder = PlaceholderFinder::new(map);
        module.visit_with(&mut finder);
        return Ok(finder.into_map());
    }

    // If module parsing fails, try wrapping in a class (for class body members like static methods)
    let wrapped_source = format!("class __MfWrapper {{ {} }}", source);
    if let Ok(module) = parse_ts_module(&wrapped_source) {
        let mut finder = PlaceholderFinder::new(map);
        module.visit_with(&mut finder);
        return Ok(finder.into_map());
    }

    // If both fail, try parsing as an expression
    if let Ok(expr) = parse_ts_expr(&source) {
        let mut finder = PlaceholderFinder::new(map);
        expr.visit_with(&mut finder);
        return Ok(finder.into_map());
    }

    // Fall back to returning an empty map (all placeholders will use default classification)
    Ok(HashMap::new())
}

/// Classifies placeholder usage by parsing the segments as an expression.
fn classify_placeholders_expr(segments: &[Segment]) -> syn::Result<HashMap<usize, PlaceholderUse>> {
    let (source, map) = build_placeholder_source(segments, PlaceholderSourceKind::Expr);
    if source.trim().is_empty() {
        return Ok(HashMap::new());
    }
    let expr = parse_ts_expr(&source)?;
    let mut finder = PlaceholderFinder::new(map);
    expr.visit_with(&mut finder);
    Ok(finder.into_map())
}

struct PlaceholderFinder {
    ids: HashMap<String, usize>,
    uses: HashMap<usize, PlaceholderUse>,
}

impl PlaceholderFinder {
    /// Creates a placeholder finder from a placeholder name map.
    fn new(ids: HashMap<String, usize>) -> Self {
        Self {
            ids,
            uses: HashMap::new(),
        }
    }

    /// Records a placeholder usage, keeping the most specific kind.
    fn record(&mut self, name: &str, use_kind: PlaceholderUse) {
        if let Some(id) = self.ids.get(name).copied() {
            let entry = self.uses.entry(id).or_insert(use_kind.clone());
            if use_rank(&use_kind) > use_rank(entry) {
                *entry = use_kind;
            }
        }
    }

    /// Returns the collected placeholder usage map.
    fn into_map(self) -> HashMap<usize, PlaceholderUse> {
        self.uses
    }
}

impl Visit for PlaceholderFinder {
    fn visit_stmt(&mut self, stmt: &Stmt) {
        if let Stmt::Expr(expr_stmt) = stmt {
            if let Expr::Ident(ident) = &*expr_stmt.expr {
                self.record(ident.sym.as_ref(), PlaceholderUse::Stmt);
                return;
            }
        }
        stmt.visit_children_with(self);
    }

    fn visit_expr(&mut self, expr: &Expr) {
        if let Expr::Ident(ident) = expr {
            self.record(ident.sym.as_ref(), PlaceholderUse::Expr);
        }
        expr.visit_children_with(self);
    }

    fn visit_member_prop(&mut self, prop: &MemberProp) {
        if let MemberProp::Ident(ident) = prop {
            self.record(ident.sym.as_ref(), PlaceholderUse::IdentName);
        }
        prop.visit_children_with(self);
    }

    fn visit_prop_name(&mut self, prop: &PropName) {
        if let PropName::Ident(ident) = prop {
            self.record(ident.sym.as_ref(), PlaceholderUse::IdentName);
        }
        prop.visit_children_with(self);
    }

    fn visit_labeled_stmt(&mut self, stmt: &LabeledStmt) {
        self.record(stmt.label.sym.as_ref(), PlaceholderUse::Ident);
        stmt.visit_children_with(self);
    }

    fn visit_pat(&mut self, pat: &Pat) {
        if let Pat::Ident(binding) = pat {
            self.record(binding.id.sym.as_ref(), PlaceholderUse::Ident);
        }
        pat.visit_children_with(self);
    }

    fn visit_ts_type_ref(&mut self, ty: &TsTypeRef) {
        if let TsEntityName::Ident(ident) = &ty.type_name {
            self.record(ident.sym.as_ref(), PlaceholderUse::Type);
        }
        ty.visit_children_with(self);
    }
}

/// Assigns a precedence rank for placeholder usage kinds.
fn use_rank(use_kind: &PlaceholderUse) -> usize {
    match use_kind {
        PlaceholderUse::Stmt => 5,
        PlaceholderUse::Type => 4,
        PlaceholderUse::Ident => 3,
        PlaceholderUse::IdentName => 2,
        PlaceholderUse::Expr => 1,
    }
}

/// Parses a TypeScript module from a source string.
fn parse_ts_module(source: &str) -> syn::Result<Module> {
    let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(
        FileName::Custom("template.ts".into()).into(),
        source.to_string(),
    );
    let syntax = Syntax::Typescript(TsSyntax {
        tsx: true,
        decorators: true,
        ..Default::default()
    });
    let lexer = Lexer::new(syntax, EsVersion::latest(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);
    parser
        .parse_module()
        .map_err(|e| syn::Error::new(Span::call_site(), format!("TypeScript parse error: {e:?}")))
}

/// Parses a TypeScript module and returns its SourceMap for span lookups.
fn parse_ts_module_with_source(source: &str) -> syn::Result<(Module, Lrc<SourceMap>)> {
    let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(
        FileName::Custom("template.ts".into()).into(),
        source.to_string(),
    );
    let syntax = Syntax::Typescript(TsSyntax {
        tsx: true,
        decorators: true,
        ..Default::default()
    });
    let lexer = Lexer::new(syntax, EsVersion::latest(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().map_err(|e| {
        syn::Error::new(Span::call_site(), format!("TypeScript parse error: {e:?}"))
    })?;
    Ok((module, cm))
}

/// Parses a TypeScript expression from a source string.
fn parse_ts_expr(source: &str) -> syn::Result<Box<Expr>> {
    let cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(
        FileName::Custom("template.ts".into()).into(),
        source.to_string(),
    );
    let syntax = Syntax::Typescript(TsSyntax {
        tsx: true,
        decorators: true,
        ..Default::default()
    });
    let lexer = Lexer::new(syntax, EsVersion::latest(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);
    parser
        .parse_expr()
        .map_err(|e| syn::Error::new(Span::call_site(), format!("TypeScript parse error: {e:?}")))
}

/// Builds a `swc_core::quote!` invocation with placeholder bindings.
/// NOTE: This does NOT support TypeScript type annotations. Use `quote_ts_with_types` for that.
fn quote_ts(template: &str, output_type: TokenStream2, bindings: &[BindingSpec]) -> QuoteTsResult {
    let mut final_template = template.to_string();
    if output_type.to_string().contains("Expr") {
        let trimmed = final_template.trim();
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            final_template = format!("({})", trimmed);
        }
    }

    let lit = syn::LitStr::new(&final_template, Span::call_site());
    let mut binding_inits = TokenStream2::new();
    binding_inits.extend(quote! { use swc_core::ecma::ast::*; });

    let mut quote_bindings = Vec::new();
    for binding in bindings {
        let name = &binding.name;
        let ty = &binding.ty;
        let expr = &binding.expr;
        binding_inits.extend(quote! { let #name: #ty = #expr; });
        quote_bindings.push(quote! { #name: #ty = #name.clone() });
    }

    let quote_call = if quote_bindings.is_empty() {
        quote! { swc_core::quote!(#lit as #output_type) }
    } else {
        quote! { swc_core::quote!(#lit as #output_type, #(#quote_bindings),*) }
    };

    QuoteTsResult {
        bindings: binding_inits,
        expr: quote_call,
    }
}

/// Builds code that parses TypeScript at runtime and substitutes placeholders.
/// This supports full TypeScript syntax including type annotations.
fn quote_ts_with_types(
    template: &str,
    output_type: TokenStream2,
    bindings: &[BindingSpec],
    type_placeholders: &[TypePlaceholder],
) -> QuoteTsResult {
    let lit = syn::LitStr::new(template, Span::call_site());
    let mut binding_inits = TokenStream2::new();
    binding_inits.extend(quote! { use swc_core::ecma::ast::*; });

    // Initialize regular bindings (non-type placeholders)
    for binding in bindings {
        let name = &binding.name;
        let ty = &binding.ty;
        let expr = &binding.expr;
        binding_inits.extend(quote! { let #name: #ty = #expr; });
    }

    // Initialize type placeholder values
    for tp in type_placeholders {
        let field_name = format_ident!("__mf_type_{}", tp.id);
        let expr = &tp.expr;
        binding_inits.extend(quote! { let #field_name: String = (#expr).to_string(); });
    }

    // Generate the substitution visitor struct fields and match arms
    let mut visitor_fields = Vec::new();
    let mut visitor_inits = Vec::new();
    let mut ident_arms = Vec::new();
    let mut type_arms = Vec::new();

    // Handle regular bindings (replace $__mf_hole_N with actual values)
    for binding in bindings {
        let name = &binding.name;
        let placeholder = name.to_string();
        let field_name = format_ident!("binding_{}", name);
        let ty = &binding.ty;

        visitor_fields.push(quote! { #field_name: #ty });
        visitor_inits.push(quote! { #field_name: #name.clone() });

        // For Ident bindings, replace in both Ident and BindingIdent positions
        ident_arms.push(quote! {
            #placeholder => {
                ident.sym = self.#field_name.sym.clone();
            }
        });
    }

    // Handle type placeholders (replace __MfTypeMarkerN with actual type names)
    for tp in type_placeholders {
        let marker = format!("__MfTypeMarker{}", tp.id);
        let field_name = format_ident!("__mf_type_{}", tp.id);

        visitor_fields.push(quote! { #field_name: String });
        visitor_inits.push(quote! { #field_name: #field_name.clone() });

        type_arms.push(quote! {
            #marker => {
                *ty = TsType::TsTypeRef(TsTypeRef {
                    span: swc_core::common::DUMMY_SP,
                    type_name: TsEntityName::Ident(Ident::new(
                        self.#field_name.clone().into(),
                        swc_core::common::DUMMY_SP,
                        Default::default(),
                    )),
                    type_params: None,
                });
            }
        });
    }

    let output_type_str = output_type.to_string();
    let is_stmt = output_type_str.contains("Stmt");
    let is_module_item = output_type_str.contains("ModuleItem");

    // Generate the parsing and substitution code
    let parse_code = if is_module_item {
        quote! {
            use swc_core::common::{FileName, SourceMap, sync::Lrc};
            use swc_core::ecma::parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};
            use swc_core::ecma::ast::EsVersion;
            use swc_core::ecma::visit::{VisitMut, VisitMutWith};

            let __mf_cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
            let __mf_fm = __mf_cm.new_source_file(
                FileName::Custom("template.ts".into()).into(),
                #lit.to_string(),
            );
            let __mf_syntax = Syntax::Typescript(TsSyntax {
                tsx: true,
                decorators: true,
                ..Default::default()
            });
            let __mf_lexer = Lexer::new(__mf_syntax, EsVersion::latest(), StringInput::from(&*__mf_fm), None);
            let mut __mf_parser = Parser::new_from(__mf_lexer);
            let __mf_module = __mf_parser.parse_module().expect("Failed to parse TypeScript template");

            // Create visitor for substitution
            struct __MfSubstitutor {
                #(#visitor_fields,)*
            }

            impl VisitMut for __MfSubstitutor {
                fn visit_mut_ident(&mut self, ident: &mut Ident) {
                    let name = ident.sym.as_ref();
                    if name.starts_with("$__mf_hole_") || name.starts_with("__mf_hole_") {
                        let lookup = if name.starts_with("$") { name } else { &format!("${}", name) };
                        match lookup {
                            #(#ident_arms)*
                            _ => {}
                        }
                    }
                }

                fn visit_mut_ts_type(&mut self, ty: &mut TsType) {
                    if let TsType::TsTypeRef(type_ref) = ty {
                        if let TsEntityName::Ident(ident) = &type_ref.type_name {
                            match ident.sym.as_ref() {
                                #(#type_arms)*
                                _ => {}
                            }
                        }
                    }
                    ty.visit_mut_children_with(self);
                }
            }

            let mut __mf_substitutor = __MfSubstitutor {
                #(#visitor_inits,)*
            };

            let mut __mf_items: Vec<ModuleItem> = __mf_module.body;
            for __mf_item in &mut __mf_items {
                __mf_item.visit_mut_with(&mut __mf_substitutor);
            }
            __mf_items.into_iter().next().expect("Expected at least one module item")
        }
    } else if is_stmt {
        quote! {
            use swc_core::common::{FileName, SourceMap, sync::Lrc};
            use swc_core::ecma::parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};
            use swc_core::ecma::ast::EsVersion;
            use swc_core::ecma::visit::{VisitMut, VisitMutWith};

            let __mf_cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
            let __mf_fm = __mf_cm.new_source_file(
                FileName::Custom("template.ts".into()).into(),
                #lit.to_string(),
            );
            let __mf_syntax = Syntax::Typescript(TsSyntax {
                tsx: true,
                decorators: true,
                ..Default::default()
            });
            let __mf_lexer = Lexer::new(__mf_syntax, EsVersion::latest(), StringInput::from(&*__mf_fm), None);
            let mut __mf_parser = Parser::new_from(__mf_lexer);
            let __mf_module = __mf_parser.parse_module().expect("Failed to parse TypeScript template");

            struct __MfSubstitutor {
                #(#visitor_fields,)*
            }

            impl VisitMut for __MfSubstitutor {
                fn visit_mut_ident(&mut self, ident: &mut Ident) {
                    let name = ident.sym.as_ref();
                    if name.starts_with("$__mf_hole_") || name.starts_with("__mf_hole_") {
                        let lookup = if name.starts_with("$") { name } else { &format!("${}", name) };
                        match lookup {
                            #(#ident_arms)*
                            _ => {}
                        }
                    }
                }

                fn visit_mut_ts_type(&mut self, ty: &mut TsType) {
                    if let TsType::TsTypeRef(type_ref) = ty {
                        if let TsEntityName::Ident(ident) = &type_ref.type_name {
                            match ident.sym.as_ref() {
                                #(#type_arms)*
                                _ => {}
                            }
                        }
                    }
                    ty.visit_mut_children_with(self);
                }
            }

            let mut __mf_substitutor = __MfSubstitutor {
                #(#visitor_inits,)*
            };

            // Convert ModuleItems to Stmts
            let mut __mf_stmts: Vec<Stmt> = Vec::new();
            for __mf_item in __mf_module.body {
                match __mf_item {
                    ModuleItem::Stmt(mut s) => {
                        s.visit_mut_with(&mut __mf_substitutor);
                        __mf_stmts.push(s);
                    }
                    ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(mut export)) => {
                        export.visit_mut_with(&mut __mf_substitutor);
                        __mf_stmts.push(Stmt::Decl(export.decl));
                    }
                    _ => {}
                }
            }
            __mf_stmts.into_iter().next().expect("Expected at least one statement")
        }
    } else {
        // For Expr or other types, fall back to parsing as module and extracting
        quote! {
            use swc_core::common::{FileName, SourceMap, sync::Lrc};
            use swc_core::ecma::parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};
            use swc_core::ecma::ast::EsVersion;
            use swc_core::ecma::visit::{VisitMut, VisitMutWith};

            let __mf_cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
            let __mf_fm = __mf_cm.new_source_file(
                FileName::Custom("template.ts".into()).into(),
                #lit.to_string(),
            );
            let __mf_syntax = Syntax::Typescript(TsSyntax {
                tsx: true,
                decorators: true,
                ..Default::default()
            });
            let __mf_lexer = Lexer::new(__mf_syntax, EsVersion::latest(), StringInput::from(&*__mf_fm), None);
            let mut __mf_parser = Parser::new_from(__mf_lexer);
            let mut __mf_expr = __mf_parser.parse_expr().expect("Failed to parse TypeScript expression");

            struct __MfSubstitutor {
                #(#visitor_fields,)*
            }

            impl VisitMut for __MfSubstitutor {
                fn visit_mut_ident(&mut self, ident: &mut Ident) {
                    let name = ident.sym.as_ref();
                    if name.starts_with("$__mf_hole_") || name.starts_with("__mf_hole_") {
                        let lookup = if name.starts_with("$") { name } else { &format!("${}", name) };
                        match lookup {
                            #(#ident_arms)*
                            _ => {}
                        }
                    }
                }

                fn visit_mut_ts_type(&mut self, ty: &mut TsType) {
                    if let TsType::TsTypeRef(type_ref) = ty {
                        if let TsEntityName::Ident(ident) = &type_ref.type_name {
                            match ident.sym.as_ref() {
                                #(#type_arms)*
                                _ => {}
                            }
                        }
                    }
                    ty.visit_mut_children_with(self);
                }
            }

            let mut __mf_substitutor = __MfSubstitutor {
                #(#visitor_inits,)*
            };
            __mf_expr.visit_mut_with(&mut __mf_substitutor);
            *__mf_expr
        }
    };

    QuoteTsResult {
        bindings: binding_inits,
        expr: parse_code,
    }
}

/// Appends a template part, inserting spaces between tokens when needed.
fn append_part(out: &mut String, part: &str) {
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
    let no_space_before = matches!(first_char, '(' | '[' | ')' | ']' | '}' | ':' | ';' | ',' | '.');
    let no_space_after = matches!(last_char, '(' | '[' | '.');

    let needs_space = !last_char.is_whitespace() && !no_space_before && !no_space_after;

    if needs_space {
        out.push(' ');
    }
    out.push_str(part);
}

/// Converts a token stream into a TypeScript-like string.
fn tokens_to_ts_string(tokens: TokenStream2) -> String {
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
                    let no_space_after = matches!(p.as_char(), '.' | '(' | '[' | ':' | ';' | ',' | '@');
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
fn analyze_tag(g: &Group) -> TagType {
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

/// Parses the contents of an ident block into static and interpolated parts.
fn parse_ident_block_parts(g: &Group) -> syn::Result<Vec<IdentPart>> {
    let mut tokens: Vec<TokenTree> = g.stream().into_iter().collect();
    if tokens.len() >= 2 {
        tokens.remove(0);
        tokens.pop();
    }

    let mut parts = Vec::new();
    let mut current = String::new();
    let mut iter = tokens.into_iter().peekable();

    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Punct(p) if p.as_char() == '@' => {
                if let Some(TokenTree::Group(g)) = iter.peek()
                    && g.delimiter() == Delimiter::Brace
                {
                    if !current.is_empty() {
                        parts.push(IdentPart::Static(std::mem::take(&mut current)));
                    }
                    let g = match iter.next() {
                        Some(TokenTree::Group(group)) => group,
                        _ => continue,
                    };
                    parts.push(IdentPart::Interpolation {
                        expr: g.stream(),
                        span: g.span(),
                    });
                } else {
                    current.push('@');
                }
            }
            TokenTree::Group(g) => {
                current.push_str(&group_to_string(&g));
            }
            TokenTree::Ident(ident) => current.push_str(&ident.to_string()),
            TokenTree::Punct(p) => current.push(p.as_char()),
            TokenTree::Literal(lit) => current.push_str(&lit.to_string()),
        }
    }

    if !current.is_empty() {
        parts.push(IdentPart::Static(current));
    }

    Ok(parts)
}

/// Extracts the string literal from a `#[doc = "..."]` attribute group.
fn parse_doc_attribute(g: &Group) -> Option<String> {
    let tokens: Vec<TokenTree> = g.stream().into_iter().collect();
    if tokens.len() < 3 {
        return None;
    }
    if let (TokenTree::Ident(ident), TokenTree::Punct(eq), TokenTree::Literal(lit)) =
        (&tokens[0], &tokens[1], &tokens[2])
    {
        if ident == "doc" && eq.as_char() == '=' {
            return Some(extract_string_literal(lit));
        }
    }
    None
}

/// Renders a group token into a string, preserving delimiters.
fn group_to_string(g: &Group) -> String {
    let inner = tokens_to_ts_string(g.stream());
    let (open, close) = match g.delimiter() {
        Delimiter::Parenthesis => ("(", ")"),
        Delimiter::Brace => ("{", "}"),
        Delimiter::Bracket => ("[", "]"),
        Delimiter::None => ("", ""),
    };
    format!("{}{}{}", open, inner, close)
}

/// Builds an SWC expression from an interpolated string literal.
fn build_string_interp_expr(parts: &[StringPart], id: usize) -> TokenStream2 {
    let mut template = String::new();
    let mut bindings = Vec::new();
    let mut expr_index = 0usize;

    template.push('`');
    for part in parts {
        match part {
            StringPart::Text(text) => {
                template.push_str(&escape_tpl_segment(text));
            }
            StringPart::Expr(expr) => {
                let name = format!("__mf_str_{id}_{expr_index}");
                expr_index += 1;
                template.push_str("${$");
                template.push_str(&name);
                template.push('}');
                let ident = proc_macro2::Ident::new(&name, Span::call_site());
                bindings.push(BindingSpec {
                    name: ident,
                    ty: quote!(Expr),
                    expr: quote! { macroforge_ts::ts_syn::to_ts_expr(#expr) },
                });
            }
        }
    }
    template.push('`');

    let quote_ts = quote_ts(&template, quote!(Expr), &bindings);
    let QuoteTsResult { bindings, expr } = quote_ts;
    quote! {{
        #bindings
        #expr
    }}
}

/// Builds an SWC expression from a backtick template literal.
fn build_template_interp_expr(parts: &[StringPart], id: usize) -> TokenStream2 {
    let mut template = String::new();
    let mut bindings = Vec::new();
    let mut expr_index = 0usize;

    template.push('`');
    for part in parts {
        match part {
            StringPart::Text(text) => {
                template.push_str(&escape_tpl_segment_allow_dollar(text));
            }
            StringPart::Expr(expr) => {
                let name = format!("__mf_tpl_{id}_{expr_index}");
                expr_index += 1;
                template.push_str("${$");
                template.push_str(&name);
                template.push('}');
                let ident = proc_macro2::Ident::new(&name, Span::call_site());
                bindings.push(BindingSpec {
                    name: ident,
                    ty: quote!(Expr),
                    expr: quote! { macroforge_ts::ts_syn::to_ts_expr(#expr) },
                });
            }
        }
    }
    template.push('`');

    let quote_ts = quote_ts(&template, quote!(Expr), &bindings);
    let QuoteTsResult { bindings, expr } = quote_ts;
    quote! {{
        #bindings
        #expr
    }}
}

/// Escapes a template segment while preserving `${...}` interpolation markers.
fn escape_tpl_segment(input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '`' => out.push_str("\\`"),
            '\\' => out.push_str("\\\\"),
            '$' => {
                if matches!(chars.peek(), Some('{')) {
                    chars.next();
                    out.push_str("\\${");
                } else {
                    out.push('$');
                }
            }
            _ => out.push(c),
        }
    }
    out
}

/// Escapes a template segment without touching dollar signs.
fn escape_tpl_segment_allow_dollar(input: &str) -> String {
    let mut out = String::new();
    for c in input.chars() {
        match c {
            '`' => out.push_str("\\`"),
            '\\' => out.push_str("\\\\"),
            _ => out.push(c),
        }
    }
    out
}

/// Joins tokens into a space-separated string.
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

/// Parses a backtick template literal encoded as a Rust string literal.
fn parse_backtick_template(lit: &proc_macro2::Literal) -> syn::Result<Option<Vec<StringPart>>> {
    let raw = lit.to_string();
    let span = lit.span();

    let content = if raw.starts_with("\"'^") && raw.ends_with("^'\"") && raw.len() >= 6 {
        Some(raw[3..raw.len() - 3].to_string())
    } else if raw.starts_with("r\"'^") && raw.ends_with("^'\"") {
        Some(raw[4..raw.len() - 3].to_string())
    } else if raw.starts_with("r#\"'^") && raw.ends_with("^'\"#") {
        Some(raw[5..raw.len() - 4].to_string())
    } else {
        None
    };

    let Some(content) = content else {
        return Ok(None);
    };

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

    let mut parts = Vec::new();
    let mut chars = content.chars().peekable();
    let mut current = String::new();
    let mut char_pos = 0usize;

    while let Some(c) = chars.next() {
        char_pos += 1;
        if c == '@' {
            match chars.peek() {
                Some('@') => {
                    chars.next();
                    char_pos += 1;
                    current.push('@');
                }
                Some('{') => {
                    chars.next();
                    char_pos += 1;
                    if !current.is_empty() {
                        parts.push(StringPart::Text(std::mem::take(&mut current)));
                    }

                    let mut expr_str = String::new();
                    let mut brace_depth = 1;
                    let expr_start_pos = char_pos;

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

                    if brace_depth != 0 {
                        return Err(template_error(
                            span,
                            &format!(
                                "Unclosed @{{}} interpolation at position {}",
                                expr_start_pos
                            ),
                            Some(&format!("@{{{}", expr_str)),
                        ));
                    }

                    match syn::parse_str::<syn::Expr>(&expr_str) {
                        Ok(expr) => parts.push(StringPart::Expr(expr.to_token_stream())),
                        Err(parse_err) => {
                            return Err(template_error(
                                span,
                                &format!(
                                    "Invalid Rust expression in template literal interpolation: {}",
                                    parse_err
                                ),
                                Some(&format!("@{{{}}}", expr_str)),
                            ));
                        }
                    }
                }
                _ => current.push('@'),
            }
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        parts.push(StringPart::Text(current));
    }

    Ok(Some(parts))
}

/// Parses a normal string literal that may contain `@{}` interpolations.
fn parse_string_interpolation(lit: &proc_macro2::Literal) -> syn::Result<Option<Vec<StringPart>>> {
    let raw = lit.to_string();
    let span = lit.span();

    let content = if raw.starts_with('"') && raw.ends_with('"') && raw.len() >= 2 {
        extract_string_literal(lit)
    } else if raw.starts_with("r\"") || raw.starts_with("r#") {
        extract_string_literal(lit)
    } else {
        return Ok(None);
    };

    if !content.contains('@') {
        return Ok(None);
    }

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

    let mut parts = Vec::new();
    let mut chars = content.chars().peekable();
    let mut current = String::new();
    let mut char_pos = 0usize;
    let mut has_expr = false;

    while let Some(c) = chars.next() {
        char_pos += 1;
        if c == '@' {
            match chars.peek() {
                Some('@') => {
                    chars.next();
                    char_pos += 1;
                    current.push('@');
                }
                Some('{') => {
                    chars.next();
                    char_pos += 1;
                    if !current.is_empty() {
                        parts.push(StringPart::Text(std::mem::take(&mut current)));
                    }

                    let mut expr_str = String::new();
                    let mut brace_depth = 1;
                    let expr_start_pos = char_pos;

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

                    if brace_depth != 0 {
                        return Err(template_error(
                            span,
                            &format!(
                                "Unclosed @{{}} interpolation at position {}",
                                expr_start_pos
                            ),
                            Some(&format!("@{{{}", expr_str)),
                        ));
                    }

                    match syn::parse_str::<syn::Expr>(&expr_str) {
                        Ok(expr) => {
                            has_expr = true;
                            parts.push(StringPart::Expr(expr.to_token_stream()));
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
                _ => current.push('@'),
            }
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        parts.push(StringPart::Text(current));
    }

    if !has_expr {
        if let Some(StringPart::Text(text)) = parts.get(0) {
            if parts.len() == 1 && text == &content {
                return Ok(None);
            }
        }
    }

    Ok(Some(parts))
}

/// Extracts and unescapes the contents of a Rust string literal token.
fn extract_string_literal(lit: &proc_macro2::Literal) -> String {
    let s = lit.to_string();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        let inner = &s[1..s.len() - 1];
        return unescape_string(inner);
    }
    if s.starts_with("r\"") && s.ends_with("\"") {
        return s[2..s.len() - 1].to_string();
    }
    if s.starts_with("r#\"") {
        if let Some(idx) = s.rfind("\"") {
            return s[3..idx].to_string();
        }
    }
    s
}

/// Performs a minimal unescape pass for Rust string literal escapes.
fn unescape_string(s: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    /// Test the actual token stream structure from @{...}
    #[test]
    fn test_token_stream_structure() {
        let tokens: TokenStream2 = quote! { @{name} };
        let tokens_vec: Vec<TokenTree> = tokens.into_iter().collect();

        eprintln!("Token count: {}", tokens_vec.len());
        for (i, tt) in tokens_vec.iter().enumerate() {
            match tt {
                TokenTree::Punct(p) => {
                    eprintln!("  Token {}: Punct('{}', spacing={:?})", i, p.as_char(), p.spacing());
                }
                TokenTree::Group(g) => {
                    eprintln!("  Token {}: Group(delimiter={:?}, content={:?})", i, g.delimiter(), g.stream().to_string());
                }
                TokenTree::Ident(id) => {
                    eprintln!("  Token {}: Ident({})", i, id);
                }
                TokenTree::Literal(lit) => {
                    eprintln!("  Token {}: Literal({})", i, lit);
                }
            }
        }

        // Should have 2 tokens: @ (Punct) and {name} (Group with Brace delimiter)
        assert_eq!(tokens_vec.len(), 2, "Should have exactly 2 tokens");
        assert!(matches!(&tokens_vec[0], TokenTree::Punct(p) if p.as_char() == '@'), "First token should be @");
        assert!(matches!(&tokens_vec[1], TokenTree::Group(g) if g.delimiter() == Delimiter::Brace), "Second token should be brace group");
    }

    /// Test the is_group check in isolation
    #[test]
    fn test_is_group_check() {
        let tokens: TokenStream2 = quote! { @{name} };
        let mut iter = tokens.into_iter().peekable();

        // First token should be @
        let first = iter.next();
        eprintln!("First token: {:?}", first);
        assert!(matches!(first, Some(TokenTree::Punct(p)) if p.as_char() == '@'));

        // After consuming @, peek should show the brace group
        let peeked = iter.peek();
        eprintln!("Peeked after @: {:?}", peeked);

        // Test the is_group check - same code as in parse_segments
        let is_group = iter.peek().is_some_and(|t| {
            matches!(t, TokenTree::Group(g) if g.delimiter() == Delimiter::Brace)
        });
        eprintln!("is_group result: {}", is_group);
        assert!(is_group, "is_group should be true");
    }

    /// Test that parse_segments correctly identifies @{...} as interpolation
    #[test]
    fn test_parse_at_brace_interpolation() {
        // Create tokens: foo @{bar} baz
        let tokens: TokenStream2 = quote! { foo @{bar} baz };
        let mut ids = IdGen::new();
        let (segments, _) =
            parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
                .expect("parse_segments should succeed");

        // Debug output
        eprintln!("Number of segments: {}", segments.len());
        for (i, seg) in segments.iter().enumerate() {
            match seg {
                Segment::Static(s) => eprintln!("  Segment {}: Static({:?})", i, s),
                Segment::Interpolation { id, .. } => eprintln!("  Segment {}: Interpolation(id={})", i, id),
                Segment::Control { id, .. } => eprintln!("  Segment {}: Control(id={})", i, id),
                _ => eprintln!("  Segment {}: Other", i),
            }
        }

        // We expect: Static("foo "), Interpolation, Static(" baz")
        let has_interpolation = segments.iter().any(|s| matches!(s, Segment::Interpolation { .. }));
        assert!(has_interpolation, "Should have at least one Interpolation segment");

        // Check that no Static segment contains '@'
        for seg in &segments {
            if let Segment::Static(s) = seg {
                assert!(!s.contains('@'), "Static segment should not contain '@': {:?}", s);
            }
        }
    }

    /// Test multiple @{...} interpolations
    #[test]
    fn test_multiple_interpolations() {
        let tokens: TokenStream2 = quote! { foo @{a} bar @{b} baz };
        let mut ids = IdGen::new();
        let (segments, _) =
            parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
                .expect("parse_segments should succeed");

        eprintln!("Number of segments: {}", segments.len());
        for (i, seg) in segments.iter().enumerate() {
            match seg {
                Segment::Static(s) => eprintln!("  Segment {}: Static({:?})", i, s),
                Segment::Interpolation { id, .. } => eprintln!("  Segment {}: Interpolation(id={})", i, id),
                _ => eprintln!("  Segment {}: Other", i),
            }
        }

        let interp_count = segments.iter().filter(|s| matches!(s, Segment::Interpolation { .. })).count();
        assert_eq!(interp_count, 2, "Should have exactly 2 interpolations");

        // Check no @ in static segments
        for seg in &segments {
            if let Segment::Static(s) = seg {
                assert!(!s.contains('@'), "Static segment should not contain '@': {:?}", s);
            }
        }
    }

    /// Test tokens_to_ts_string doesn't inject @
    #[test]
    fn test_tokens_to_ts_string_no_at() {
        let tokens: TokenStream2 = quote! { foo bar };
        let result = tokens_to_ts_string(tokens);
        eprintln!("tokens_to_ts_string result: {:?}", result);
        assert!(!result.contains('@'), "Should not contain @");
    }

    /// Test using from_str (like the actual macro does)
    #[test]
    fn test_from_str_tokenization() {
        use std::str::FromStr;

        let tokens = TokenStream2::from_str("@{name}").unwrap();
        let tokens_vec: Vec<TokenTree> = tokens.into_iter().collect();

        eprintln!("from_str Token count: {}", tokens_vec.len());
        for (i, tt) in tokens_vec.iter().enumerate() {
            match tt {
                TokenTree::Punct(p) => {
                    eprintln!("  Token {}: Punct('{}', spacing={:?})", i, p.as_char(), p.spacing());
                }
                TokenTree::Group(g) => {
                    eprintln!("  Token {}: Group(delimiter={:?}, content={:?})", i, g.delimiter(), g.stream().to_string());
                }
                TokenTree::Ident(id) => {
                    eprintln!("  Token {}: Ident({})", i, id);
                }
                TokenTree::Literal(lit) => {
                    eprintln!("  Token {}: Literal({})", i, lit);
                }
            }
        }

        // Should have 2 tokens like quote! does
        assert_eq!(tokens_vec.len(), 2, "Should have exactly 2 tokens");
    }

    /// Test parse_segments with from_str input
    #[test]
    fn test_parse_segments_from_str() {
        use std::str::FromStr;

        let tokens = TokenStream2::from_str("foo @{bar} baz").unwrap();
        let mut ids = IdGen::new();
        let (segments, _) =
            parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
                .expect("parse_segments should succeed");

        eprintln!("from_str segments count: {}", segments.len());
        for (i, seg) in segments.iter().enumerate() {
            match seg {
                Segment::Static(s) => eprintln!("  Segment {}: Static({:?})", i, s),
                Segment::Interpolation { id, .. } => eprintln!("  Segment {}: Interpolation(id={})", i, id),
                _ => eprintln!("  Segment {}: Other", i),
            }
        }

        // Check that no Static segment contains '@'
        for seg in &segments {
            if let Segment::Static(s) = seg {
                assert!(!s.contains('@'), "Static segment should not contain '@': {:?}", s);
            }
        }
    }

    /// Test build_template_and_bindings produces correct template string
    #[test]
    fn test_build_template_and_bindings() {
        use std::str::FromStr;

        let tokens = TokenStream2::from_str("foo @{bar} baz").unwrap();
        let mut ids = IdGen::new();
        let (segments, _) =
            parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
                .expect("parse_segments should succeed");

        // Create an empty context map (no special placeholder classification)
        let context_map = HashMap::new();

        let template_result = build_template_and_bindings(segments.iter(), &context_map)
            .expect("build_template_and_bindings should succeed");

        eprintln!("Template string: {:?}", template_result.template);
        eprintln!("Bindings count: {}", template_result.bindings.len());

        // Template should NOT contain '@'
        assert!(!template_result.template.contains('@'), "Template should not contain '@': {:?}", template_result.template);

        // Template should contain placeholder
        assert!(template_result.template.contains("__mf_hole_"), "Template should contain placeholder: {:?}", template_result.template);
    }

    /// Test the full template string for the derive_clone pattern
    #[test]
    fn test_derive_clone_full_template() {
        use std::str::FromStr;

        let code = r#"export function @{fn_name}(value: @{class_name}): @{class_name} {
            const cloned = Object.create(Object.getPrototypeOf(value));
            return cloned;
        }"#;

        let tokens = TokenStream2::from_str(code).unwrap();
        let mut ids = IdGen::new();
        let (segments, _) =
            parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
                .expect("parse_segments should succeed");

        let context_map = classify_placeholders_module(&segments)
            .expect("classify_placeholders_module should succeed");

        let template_result = build_template_and_bindings(segments.iter(), &context_map)
            .expect("build_template_and_bindings should succeed");

        eprintln!("Full template string:\n{}", template_result.template);
        eprintln!("\nBindings count: {}", template_result.bindings.len());

        // Template should NOT contain '@'
        assert!(!template_result.template.contains('@'), "Template should not contain '@'");
    }

    /// Test classify_placeholders_module
    #[test]
    fn test_classify_placeholders() {
        use std::str::FromStr;

        let tokens = TokenStream2::from_str("const x = @{bar};").unwrap();
        let mut ids = IdGen::new();
        let (segments, _) =
            parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
                .expect("parse_segments should succeed");

        eprintln!("Segments for classification:");
        for (i, seg) in segments.iter().enumerate() {
            match seg {
                Segment::Static(s) => eprintln!("  {}: Static({:?})", i, s),
                Segment::Interpolation { id, .. } => eprintln!("  {}: Interpolation(id={})", i, id),
                _ => eprintln!("  {}: Other", i),
            }
        }

        let context_map = classify_placeholders_module(&segments)
            .expect("classify_placeholders_module should succeed");

        eprintln!("Context map: {:?}", context_map);
    }

    /// Test full parse_template flow
    #[test]
    fn test_full_parse_template() {
        use std::str::FromStr;

        let tokens = TokenStream2::from_str("const x = @{bar};").unwrap();
        let result = parse_template(tokens);

        match result {
            Ok(output) => {
                let s = output.to_string();
                eprintln!("parse_template output (first 500 chars): {}", &s[..s.len().min(500)]);
                // If it succeeds, the template was parsed correctly
            }
            Err(e) => {
                eprintln!("parse_template error: {}", e);
                // This is the error we're trying to fix
                panic!("parse_template failed: {}", e);
            }
        }
    }

    /// Test case matching the actual failing code from derive_clone.rs
    #[test]
    fn test_derive_clone_like_template() {
        // Match the exact pattern from derive_clone.rs
        let tokens: TokenStream2 = quote! {
            export function @{fn_name}(value: @{class_name}): @{class_name} {
                const cloned = Object.create(Object.getPrototypeOf(value));
                return cloned;
            }
        };

        let mut ids = IdGen::new();
        let (segments, _) =
            parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
                .expect("parse_segments should succeed");

        eprintln!("derive_clone-like segments:");
        for (i, seg) in segments.iter().enumerate() {
            match seg {
                Segment::Static(s) => {
                    eprintln!("  {}: Static({:?})", i, s);
                    if s.contains('@') {
                        eprintln!("    ^^^ CONTAINS @ ^^^");
                    }
                }
                Segment::Interpolation { id, .. } => eprintln!("  {}: Interpolation(id={})", i, id),
                Segment::Control { id, .. } => eprintln!("  {}: Control(id={})", i, id),
                _ => eprintln!("  {}: Other", i),
            }
        }

        // Should have 3 interpolations
        let interp_count = segments.iter().filter(|s| matches!(s, Segment::Interpolation { .. })).count();
        assert_eq!(interp_count, 3, "Should have exactly 3 interpolations for @{{fn_name}}, @{{class_name}}, @{{class_name}}");

        // No Static segment should contain '@'
        for seg in &segments {
            if let Segment::Static(s) = seg {
                assert!(!s.contains('@'), "Static segment contains '@': {:?}", s);
            }
        }
    }

    /// Test with from_str matching derive_clone
    #[test]
    fn test_derive_clone_like_from_str() {
        use std::str::FromStr;

        let code = r#"export function @{fn_name}(value: @{class_name}): @{class_name} {
            const cloned = Object.create(Object.getPrototypeOf(value));
            return cloned;
        }"#;

        let tokens = TokenStream2::from_str(code).unwrap();

        let mut ids = IdGen::new();
        let (segments, _) =
            parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
                .expect("parse_segments should succeed");

        eprintln!("from_str derive_clone-like segments:");
        for (i, seg) in segments.iter().enumerate() {
            match seg {
                Segment::Static(s) => {
                    eprintln!("  {}: Static({:?})", i, s);
                    if s.contains('@') {
                        eprintln!("    ^^^ CONTAINS @ ^^^");
                    }
                }
                Segment::Interpolation { id, .. } => eprintln!("  {}: Interpolation(id={})", i, id),
                Segment::Control { id, .. } => eprintln!("  {}: Control(id={})", i, id),
                _ => eprintln!("  {}: Other", i),
            }
        }

        // Should have 3 interpolations
        let interp_count = segments.iter().filter(|s| matches!(s, Segment::Interpolation { .. })).count();
        assert_eq!(interp_count, 3, "Should have exactly 3 interpolations");
    }

    /// Test derive_clone with control flow - matches actual macro usage
    #[test]
    fn test_derive_clone_with_control_flow() {
        use std::str::FromStr;

        // This is the EXACT template from derive_clone.rs line 110-122
        let code = r#"export function @{fn_name}(value: @{class_name}): @{class_name} {
            const cloned = Object.create(Object.getPrototypeOf(value));

            {#if has_fields}
                {#for field in field_names}
                    cloned.@{field} = value.@{field};
                {/for}
            {/if}

            return cloned;
        }"#;

        eprintln!("\n=== Testing derive_clone with control flow ===");
        eprintln!("Input code:\n{}", code);

        let tokens = TokenStream2::from_str(code).unwrap();

        let mut ids = IdGen::new();
        let result = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false);

        match result {
            Ok((segments, _)) => {
                eprintln!("\nParsed segments:");
                for (i, seg) in segments.iter().enumerate() {
                    match seg {
                        Segment::Static(s) => {
                            eprintln!("  {}: Static({:?})", i, s);
                            if s.contains('@') {
                                eprintln!("    ^^^ CONTAINS @ - BUG! ^^^");
                            }
                            if s.contains('#') {
                                eprintln!("    ^^^ CONTAINS # - control flow not parsed! ^^^");
                            }
                        }
                        Segment::Interpolation { id, expr, .. } => {
                            eprintln!("  {}: Interpolation(id={}, expr={})", i, id, expr);
                        }
                        Segment::Control { id, node, .. } => {
                            eprintln!("  {}: Control(id={}, node={:?})", i, id, std::mem::discriminant(node));
                        }
                        _ => eprintln!("  {}: {:?}", i, seg),
                    }
                }

                // Build the template string
                let context_map = HashMap::new();
                let template_result = build_template_and_bindings(segments.iter(), &context_map);

                match template_result {
                    Ok(result) => {
                        eprintln!("\nFull template string:\n{}", result.template);
                        eprintln!("\nBindings: {}", result.bindings.len());
                        for b in &result.bindings {
                            eprintln!("  {} : {:?}", b.name, b.ty);
                        }
                    }
                    Err(e) => {
                        eprintln!("\nFailed to build template: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("\nFailed to parse segments: {}", e);
            }
        }
    }

    /// Debug: trace through "value: @{class_name}" token by token
    #[test]
    fn test_debug_colon_at_sequence() {
        use std::str::FromStr;

        // The parentheses case - this might be the issue!
        let code = "(value: @{class_name})";
        let tokens = TokenStream2::from_str(code).unwrap();
        let tokens_vec: Vec<TokenTree> = tokens.into_iter().collect();

        eprintln!("Tokens for 'value: @{{class_name}}':");
        for (i, tt) in tokens_vec.iter().enumerate() {
            match tt {
                TokenTree::Punct(p) => {
                    eprintln!("  {}: Punct('{}', spacing={:?})", i, p.as_char(), p.spacing());
                }
                TokenTree::Group(g) => {
                    eprintln!("  {}: Group(delimiter={:?}, content={:?})", i, g.delimiter(), g.stream().to_string());
                }
                TokenTree::Ident(id) => {
                    eprintln!("  {}: Ident({})", i, id);
                }
                TokenTree::Literal(lit) => {
                    eprintln!("  {}: Literal({})", i, lit);
                }
            }
        }

        // Now simulate what parse_segments does
        let tokens = TokenStream2::from_str(code).unwrap();
        let mut iter = tokens.into_iter().peekable();

        eprintln!("\nSimulating parse_segments:");
        while let Some(token) = iter.peek().cloned() {
            match &token {
                TokenTree::Punct(p) if p.as_char() == '@' => {
                    eprintln!("  Found @, consuming it...");
                    iter.next();
                    eprintln!("  After consuming @, peeking next token...");
                    let peeked = iter.peek();
                    eprintln!("  Peeked: {:?}", peeked);

                    let is_group = iter.peek().is_some_and(|t| {
                        let result = matches!(t, TokenTree::Group(g) if g.delimiter() == Delimiter::Brace);
                        eprintln!("  is_group check result: {}", result);
                        result
                    });

                    if is_group {
                        eprintln!("  -> IS a brace group, consuming it");
                        iter.next();
                    } else {
                        eprintln!("  -> NOT a brace group!");
                    }
                }
                _ => {
                    eprintln!("  Token: {:?}, consuming...", token);
                    iter.next();
                }
            }
        }
    }
}
