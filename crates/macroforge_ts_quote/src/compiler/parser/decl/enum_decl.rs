//! Enum declaration parsing
//!
//! Handles:
//! - `enum Status { Active, Inactive }`
//! - `const enum Direction { Up, Down }`
//! - `export enum Color { Red = 0, Green = 1 }`

use super::super::expr::errors::{ParseError, ParseResult};
use super::*;

impl Parser {
    /// Parse enum declaration
    /// Handles: enum, const enum, export enum
    pub(crate) fn parse_enum_decl(&mut self, exported: bool, const_: bool) -> ParseResult<IrNode> {
        // If we're at "const", consume it
        let const_ = if self.at(SyntaxKind::ConstKw) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            const_
        };

        // Consume "enum"
        if !self.at(SyntaxKind::EnumKw) {
            return Err(ParseError::unexpected_eof(self.current_byte_offset(), "enum keyword"))
                .map_err(|e| e.with_context("parsing enum declaration"));
        }
        self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "enum keyword"))?;
        self.skip_whitespace();

        // Parse enum name
        let name = self.parse_ts_ident_or_placeholder()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "enum name")
                .with_context("parsing enum declaration"))?;
        self.skip_whitespace();

        // Parse enum body
        if !self.at(SyntaxKind::LBrace) {
            return Ok(IrNode::Raw("enum ".to_string()));
        }
        self.consume(); // consume {
        self.skip_whitespace();

        let members = self.parse_enum_members()?;

        self.skip_whitespace();
        self.expect(SyntaxKind::RBrace);

        Ok(IrNode::EnumDecl {
            exported,
            declare: false,
            const_,
            name: Box::new(name),
            members,
        })
    }

    /// Parse enum members
    fn parse_enum_members(&mut self) -> ParseResult<Vec<IrNode>> {
        let mut members = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            // Check for control flow (for loop, etc.)
            if let Some(kind) = self.current_kind() {
                match kind {
                    SyntaxKind::BraceHashIf
                    | SyntaxKind::BraceHashFor
                    | SyntaxKind::BraceHashWhile
                    | SyntaxKind::BraceHashMatch => {
                        members.push(self.parse_control_block(kind)?);
                        continue;
                    }
                    _ => {}
                }
            }

            // Check for directives
            if self.at(SyntaxKind::DollarOpen) {
                if let Some(node) = self.parse_directive() {
                    members.push(node);
                }
                continue;
            }

            // Parse enum member
            match self.parse_enum_member() {
                Ok(member) => members.push(member),
                Err(e) => {
                    // If we get an error parsing member, propagate it
                    // unless we're at a position where we can recover
                    if self.at(SyntaxKind::Comma) || self.at(SyntaxKind::RBrace) {
                        // Skip and continue
                    } else {
                        return Err(e.with_context("parsing enum members"));
                    }
                }
            }

            self.skip_whitespace();

            // Handle comma separator
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }
        }

        Ok(members)
    }

    /// Parse a single enum member: `Name` or `Name = value`
    fn parse_enum_member(&mut self) -> ParseResult<IrNode> {
        // Parse member name (can be identifier or placeholder)
        let name = self.parse_ts_ident_or_placeholder()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "enum member name")
                .with_context("parsing enum member"))?;
        self.skip_whitespace();

        // Check for initializer
        let init = if self.at(SyntaxKind::Eq) {
            self.consume();
            self.skip_whitespace();
            // Parse the initializer expression until comma or closing brace
            Some(Box::new(
                self.parse_ts_expr_until(&[SyntaxKind::Comma, SyntaxKind::RBrace])
                    .map_err(|e| e.with_context("parsing enum member initializer"))?,
            ))
        } else {
            None
        };

        Ok(IrNode::EnumMember {
            name: Box::new(name),
            init,
        })
    }
}
