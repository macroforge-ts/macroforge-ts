//! Expression-level control flow parsing.
//!
//! This module implements parsing for control blocks (`{#if}`, `{#for}`, `{#while}`, `{#match}`)
//! when they appear in expression context. Unlike statement-level control blocks that produce
//! `Vec<IrNode>` bodies, expression-level control blocks produce single expressions that
//! evaluate to values.
//!
//! # Examples
//!
//! ```text
//! // If expression (all branches required)
//! const status = {#if cond} "active" {:else} "inactive" {/if};
//!
//! // For expression (produces iterator)
//! const items = [{#for x in list} x.name {/for}];
//!
//! // While expression (produces iterator)
//! const vals = {#while cond} next_val() {/while};
//!
//! // Match expression
//! const val = {#match x}{:case Some(v)} v {:case None} 0 {/match};
//! ```

use super::errors::{ParseError, ParseErrorKind, ParseResult};
use crate::compiler::ir::{IrNode, MatchArmExpr};
use crate::compiler::parser::Parser;
use crate::compiler::syntax::SyntaxKind;

impl Parser {
    /// Parses an if expression: `{#if cond} expr {:else if cond} expr {:else} expr {/if}`
    ///
    /// Unlike statement-level `{#if}`, expression-level if **requires** an `{:else}` branch
    /// because all branches must produce a value.
    pub(super) fn parse_if_expr(&mut self) -> ParseResult<IrNode> {
        // Push TemplateControlBlock context FIRST to preserve parent context (e.g., ObjectLiteral).
        // This must happen before consuming any tokens so that the closing `}` of the header
        // (in `{#if condition}`) doesn't pop the ObjectLiteral context.
        self.push_context(super::super::Context::template_control_block([
            SyntaxKind::BraceColonElseBrace,
            SyntaxKind::BraceColonElseIf,
            SyntaxKind::BraceSlashIfBrace,
        ]));

        // Consume {#if
        self.consume().ok_or_else(|| {
            self.pop_context(); // Clean up on error
            ParseError::unexpected_eof(self.current_byte_offset(), "if-expression opening")
        })?;

        self.skip_whitespace();

        // Parse condition until }
        let condition_str = self.collect_rust_until(SyntaxKind::RBrace);
        if self.expect(SyntaxKind::RBrace).is_none() {
            self.pop_context(); // Clean up on error
            return Err(ParseError::new(ParseErrorKind::MissingClosingBrace, self.current_byte_offset())
                .with_context("if-expression condition"));
        }

        // Parse then expression
        let then_expr = self.parse_expr_until_control_continuation()?;

        // Parse else-if branches
        let mut else_if_branches = Vec::new();
        while self.at(SyntaxKind::BraceColonElseIf) {
            self.consume(); // {:else if
            self.skip_whitespace();

            let cond_str = self.collect_rust_until(SyntaxKind::RBrace);
            self.expect(SyntaxKind::RBrace).ok_or_else(|| {
                ParseError::new(ParseErrorKind::MissingClosingBrace, self.current_byte_offset())
                    .with_context("else-if condition")
            })?;

            let expr = self.parse_expr_until_control_continuation()?;
            let cond = Self::str_to_token_stream(&cond_str)
                .map_err(|e| e.with_context("else-if condition"))?;
            else_if_branches.push((cond, Box::new(expr)));
        }

        // Require {:else} in expression context
        if !self.at(SyntaxKind::BraceColonElseBrace) {
            self.pop_context(); // Clean up before error
            return Err(ParseError::new(ParseErrorKind::MissingElseBranch, self.current_byte_offset())
                .with_context("if-expression")
                .with_help("if-expressions require an {:else} branch to produce a value. Add {:else} expr before {/if}."));
        }

        // Parse else branch
        // Note: {:else} is a complete token (includes closing brace), no separate } to expect
        self.consume(); // {:else}
        let else_expr = self.parse_expr_until_control_continuation()?;

        // Skip whitespace before checking for closing tag
        self.skip_whitespace();

        // Pop the TemplateControlBlock context
        self.pop_context();

        // Expect {/if}
        if !self.at(SyntaxKind::BraceSlashIfBrace) {
            return Err(ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["{/if}"])
                .with_context("if-expression"));
        }
        self.consume(); // {/if}

        let condition = Self::str_to_token_stream(&condition_str)
            .map_err(|e| e.with_context("if-expression condition"))?;

        Ok(IrNode::IfExpr {
            condition,
            then_expr: Box::new(then_expr),
            else_if_branches,
            else_expr: Box::new(else_expr),
        })
    }

    /// Parses a for expression: `{#for pattern in iterator} expr {/for}`
    ///
    /// Produces an iterator via `.into_iter().map(|pattern| expr)`.
    pub(super) fn parse_for_expr(&mut self) -> ParseResult<IrNode> {
        // Push TemplateControlBlock context FIRST to preserve parent context (e.g., ObjectLiteral).
        // This must happen before consuming any tokens so that the closing `}` of the header
        // (in `{#for pattern in iter}`) doesn't pop the ObjectLiteral context.
        self.push_context(super::super::Context::template_control_block([
            SyntaxKind::BraceSlashForBrace,
        ]));

        // Consume {#for
        self.consume().ok_or_else(|| {
            self.pop_context(); // Clean up on error
            ParseError::unexpected_eof(self.current_byte_offset(), "for-expression opening")
        })?;

        self.skip_whitespace();

        // Parse pattern until `in` keyword
        let mut pattern_str = String::new();
        while !self.at_eof() && !self.at(SyntaxKind::InKw) && !self.at(SyntaxKind::RBrace) {
            if let Some(token) = self.consume() {
                pattern_str.push_str(&token.text);
            }
        }

        // Expect `in` keyword
        if self.expect(SyntaxKind::InKw).is_none() {
            self.pop_context(); // Clean up on error
            return Err(ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["in"])
                .with_context("for-expression"));
        }

        self.skip_whitespace();

        // Parse iterator until }
        let iterator_str = self.collect_rust_until(SyntaxKind::RBrace);
        if self.expect(SyntaxKind::RBrace).is_none() {
            self.pop_context(); // Clean up on error
            return Err(ParseError::new(ParseErrorKind::MissingClosingBrace, self.current_byte_offset())
                .with_context("for-expression iterator"));
        }

        // Parse body expression
        let body_expr = self.parse_expr_until_control_continuation()?;

        // Skip whitespace before checking for closing tag
        self.skip_whitespace();

        // Pop the TemplateControlBlock context
        self.pop_context();

        // Expect {/for}
        if !self.at(SyntaxKind::BraceSlashForBrace) {
            return Err(ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["{/for}"])
                .with_context("for-expression"));
        }
        self.consume(); // {/for}

        let pattern = Self::str_to_token_stream(pattern_str.trim())
            .map_err(|e| e.with_context("for-expression pattern"))?;
        let iterator = Self::str_to_token_stream(&iterator_str)
            .map_err(|e| e.with_context("for-expression iterator"))?;

        Ok(IrNode::ForExpr {
            pattern,
            iterator,
            body_expr: Box::new(body_expr),
        })
    }

    /// Parses a while expression: `{#while condition} expr {/while}`
    ///
    /// Produces an iterator via `std::iter::from_fn(|| if cond { Some(expr) } else { None })`.
    pub(super) fn parse_while_expr(&mut self) -> ParseResult<IrNode> {
        // Push TemplateControlBlock context FIRST to preserve parent context (e.g., ObjectLiteral).
        // This must happen before consuming any tokens so that the closing `}` of the header
        // (in `{#while condition}`) doesn't pop the ObjectLiteral context.
        self.push_context(super::super::Context::template_control_block([
            SyntaxKind::BraceSlashWhileBrace,
        ]));

        // Consume {#while
        self.consume().ok_or_else(|| {
            self.pop_context(); // Clean up on error
            ParseError::unexpected_eof(self.current_byte_offset(), "while-expression opening")
        })?;

        self.skip_whitespace();

        // Parse condition until }
        let condition_str = self.collect_rust_until(SyntaxKind::RBrace);
        if self.expect(SyntaxKind::RBrace).is_none() {
            self.pop_context(); // Clean up on error
            return Err(ParseError::new(ParseErrorKind::MissingClosingBrace, self.current_byte_offset())
                .with_context("while-expression condition"));
        }

        // Parse body expression
        let body_expr = self.parse_expr_until_control_continuation()?;

        // Skip whitespace before checking for closing tag
        self.skip_whitespace();

        // Pop the TemplateControlBlock context
        self.pop_context();

        // Expect {/while}
        if !self.at(SyntaxKind::BraceSlashWhileBrace) {
            return Err(ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["{/while}"])
                .with_context("while-expression"));
        }
        self.consume(); // {/while}

        let condition = Self::str_to_token_stream(&condition_str)
            .map_err(|e| e.with_context("while-expression condition"))?;

        Ok(IrNode::WhileExpr {
            condition,
            body_expr: Box::new(body_expr),
        })
    }

    /// Parses a match expression: `{#match expr}{:case pattern} expr {:case pattern} expr {/match}`
    pub(super) fn parse_match_expr(&mut self) -> ParseResult<IrNode> {
        // Push TemplateControlBlock context FIRST to preserve parent context (e.g., ObjectLiteral).
        // This must happen before consuming any tokens so that the closing `}` of the header
        // (in `{#match expr}`) doesn't pop the ObjectLiteral context.
        self.push_context(super::super::Context::template_control_block([
            SyntaxKind::BraceSlashMatchBrace,
            SyntaxKind::BraceColonCase,
        ]));

        // Consume {#match
        self.consume().ok_or_else(|| {
            self.pop_context(); // Clean up on error
            ParseError::unexpected_eof(self.current_byte_offset(), "match-expression opening")
        })?;

        self.skip_whitespace();

        // Parse match expression until }
        let expr_str = self.collect_rust_until(SyntaxKind::RBrace);
        if self.expect(SyntaxKind::RBrace).is_none() {
            self.pop_context(); // Clean up on error
            return Err(ParseError::new(ParseErrorKind::MissingClosingBrace, self.current_byte_offset())
                .with_context("match-expression scrutinee"));
        }

        // Parse arms
        let mut arms = Vec::new();
        self.skip_whitespace();

        while self.at(SyntaxKind::BraceColonCase) {
            self.consume(); // {:case
            self.skip_whitespace();

            // Parse pattern until }
            let pattern_str = self.collect_rust_until(SyntaxKind::RBrace);
            self.expect(SyntaxKind::RBrace).ok_or_else(|| {
                ParseError::new(ParseErrorKind::MissingClosingBrace, self.current_byte_offset())
                    .with_context("match arm pattern")
            })?;

            // Parse body expression
            let body_expr = self.parse_expr_until_control_continuation()?;

            let pattern = Self::str_to_token_stream(&pattern_str)
                .map_err(|e| e.with_context("match arm pattern"))?;

            arms.push(MatchArmExpr {
                pattern,
                guard: None, // TODO: Support guards with `if cond` syntax
                body_expr: Box::new(body_expr),
            });

            self.skip_whitespace();
        }

        // Pop the TemplateControlBlock context
        self.pop_context();

        // Expect {/match}
        if !self.at(SyntaxKind::BraceSlashMatchBrace) {
            return Err(ParseError::new(ParseErrorKind::UnexpectedToken, self.current_byte_offset())
                .with_expected(&["{/match}"])
                .with_context("match-expression"));
        }
        self.consume(); // {/match}

        let expr = Self::str_to_token_stream(&expr_str)
            .map_err(|e| e.with_context("match-expression scrutinee"))?;

        Ok(IrNode::MatchExpr { expr, arms })
    }

    /// Parses an expression until a control flow continuation token.
    ///
    /// Control continuations are: `{:else}`, `{:else if}`, `{/if}`, `{/for}`, `{/while}`, `{/match}`, `{:case}`
    ///
    /// This relies on the TemplateControlBlock context being pushed by the caller, which
    /// preserves the parent context (e.g., ObjectLiteral) so `is_object_literal()` works
    /// correctly by searching the entire context stack.
    fn parse_expr_until_control_continuation(&mut self) -> ParseResult<IrNode> {
        self.skip_whitespace();

        // Define terminators for expression parsing in control flow context
        let terminators = &[
            SyntaxKind::BraceColonElseBrace,
            SyntaxKind::BraceColonElseIf,
            SyntaxKind::BraceSlashIfBrace,
            SyntaxKind::BraceSlashForBrace,
            SyntaxKind::BraceSlashWhileBrace,
            SyntaxKind::BraceSlashMatchBrace,
            SyntaxKind::BraceColonCase,
        ];

        // Check if we're inside an object literal by searching the context stack.
        // is_inside_object_literal() searches the entire stack, so nested contexts
        // (like TemplateControlBlock) can still detect the parent ObjectLiteral.
        let in_object_literal = self.is_inside_object_literal();
        self.parse_expression_until_in_context(terminators, in_object_literal)
    }
}

#[cfg(test)]
mod tests {
    // Integration tests will be in src/test.rs
}
