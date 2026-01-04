//! Tag type analysis for template control flow.

use proc_macro2::{Group, TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;

/// Represents the type of a brace-delimited tag in the template.
#[derive(Debug)]
pub enum TagType {
    /// `{#if condition}`
    If(TokenStream2),
    /// `{#if let pattern = expr}`
    IfLet(TokenStream2, TokenStream2),
    /// `{#for item in collection}`
    For(TokenStream2, TokenStream2),
    /// `{#while condition}`
    While(TokenStream2),
    /// `{#while let pattern = expr}`
    WhileLet(TokenStream2, TokenStream2),
    /// `{#match expr}`
    Match(TokenStream2),
    /// `{:else}`
    Else,
    /// `{:else if condition}`
    ElseIf(TokenStream2),
    /// `{:case pattern}`
    Case(TokenStream2),
    /// `{/if}`
    EndIf,
    /// `{/for}`
    EndFor,
    /// `{/while}`
    EndWhile,
    /// `{/match}`
    EndMatch,
    /// `{%let name = expr}` or `{$let name = expr}`
    Let(TokenStream2),
    /// `{$let mut name = expr}` - mutable let binding
    LetMut(TokenStream2),
    /// `{$do expr}` - execute Rust expression
    Do(TokenStream2),
    /// `{$typescript expr}` - inject TsStream into output
    TypeScript(TokenStream2),
    /// `{> comment <}` - line comment (emitted as // comment)
    LineComment(TokenStream2),
    /// `{>> comment <<}` - block comment (emitted as /* comment */)
    BlockComment(TokenStream2),
    /// Regular TypeScript block `{ ... }`
    Block,
}

/// Analyze a braced group to determine if it's a template tag or a regular block.
pub fn analyze_tag(g: &Group) -> TagType {
    let tokens: Vec<TokenTree> = g.stream().into_iter().collect();
    if tokens.len() < 2 {
        return TagType::Block;
    }

    // Check for {# ...} tags (if, match, for)
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

        if i == "while" {
            // Check for {#while let pattern = expr}
            if let Some(TokenTree::Ident(let_kw)) = tokens.get(2)
                && let_kw == "let"
            {
                // Format: {#while let pattern = expr}
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

            // Format: {#while condition}
            let cond: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::While(cond);
        }
    }

    // Check for {% ...} tags (let) - alternate syntax
    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '%'
        && i == "let"
    {
        // Format: {%let name = expr}
        let body: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
        return TagType::Let(body);
    }

    // Check for {$ ...} tags (directives: let, do, typescript)
    if let TokenTree::Punct(p) = &tokens[0]
        && p.as_char() == '$'
    {
        if let Some(TokenTree::Ident(i)) = tokens.get(1) {
            if i == "let" {
                // Check for {$let mut ...} - mutable binding
                if let Some(TokenTree::Ident(mut_kw)) = tokens.get(2)
                    && mut_kw == "mut"
                {
                    // Format: {$let mut name = expr}
                    let body: TokenStream2 = tokens.iter().skip(3).map(|t| t.to_token_stream()).collect();
                    return TagType::LetMut(body);
                }
                // Format: {$let name = expr}
                let body: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
                return TagType::Let(body);
            }
            if i == "do" {
                // Format: {$do expr}
                let body: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
                return TagType::Do(body);
            }
            if i == "typescript" {
                // Format: {$typescript expr} - inject TsStream
                let body: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
                return TagType::TypeScript(body);
            }
        }
    }

    // Check for {> ... <} tags (line comment) or {>> ... <<} (block comment)
    if let TokenTree::Punct(p) = &tokens[0]
        && p.as_char() == '>'
    {
        // Check for {>> ... <<} (block comment)
        if let Some(TokenTree::Punct(p2)) = tokens.get(1)
            && p2.as_char() == '>'
        {
            // Extract content between >> and << (skip first 2 tokens, trim last 2)
            let content_end = tokens.len().saturating_sub(2);
            let body: TokenStream2 = tokens.iter().skip(2).take(content_end.saturating_sub(2)).map(|t| t.to_token_stream()).collect();
            return TagType::BlockComment(body);
        }
        // Line comment {> ... <} - extract content between > and <
        let content_end = tokens.len().saturating_sub(1);
        let body: TokenStream2 = tokens.iter().skip(1).take(content_end.saturating_sub(1)).map(|t| t.to_token_stream()).collect();
        return TagType::LineComment(body);
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
