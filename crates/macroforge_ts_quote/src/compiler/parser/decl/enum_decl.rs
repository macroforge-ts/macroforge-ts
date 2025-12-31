//! Enum declaration parsing
//!
//! Handles:
//! - `enum Status { Active, Inactive }`
//! - `const enum Direction { Up, Down }`
//! - `export enum Color { Red = 0, Green = 1 }`

use super::*;

impl Parser {
    /// Parse enum declaration
    /// Handles: enum, const enum, export enum
    pub(crate) fn parse_enum_decl(&mut self, exported: bool, const_: bool) -> Option<IrNode> {
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
            return None;
        }
        self.consume()?;
        self.skip_whitespace();

        // Parse enum name
        let name = self.parse_ts_ident_or_placeholder()?;
        self.skip_whitespace();

        // Parse enum body
        if !self.at(SyntaxKind::LBrace) {
            return Some(IrNode::Raw("enum ".to_string()));
        }
        self.consume(); // consume {
        self.skip_whitespace();

        let members = self.parse_enum_members();

        self.skip_whitespace();
        self.expect(SyntaxKind::RBrace);

        Some(IrNode::EnumDecl {
            exported,
            declare: false,
            const_,
            name: Box::new(name),
            members,
        })
    }

    /// Parse enum members
    fn parse_enum_members(&mut self) -> Vec<IrNode> {
        let mut members = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            // Check for control flow (for loop, etc.)
            if self.at(SyntaxKind::HashOpen) {
                if let Some(node) = self.parse_control_block() {
                    members.push(node);
                }
                continue;
            }

            // Check for directives
            if self.at(SyntaxKind::DollarOpen) {
                if let Some(node) = self.parse_directive() {
                    members.push(node);
                }
                continue;
            }

            // Parse enum member
            if let Some(member) = self.parse_enum_member() {
                members.push(member);
            } else {
                // Consume unknown token to prevent infinite loop
                self.advance();
            }

            self.skip_whitespace();

            // Handle comma separator
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }
        }

        members
    }

    /// Parse a single enum member: `Name` or `Name = value`
    fn parse_enum_member(&mut self) -> Option<IrNode> {
        // Parse member name (can be identifier or placeholder)
        let name = self.parse_ts_ident_or_placeholder()?;
        self.skip_whitespace();

        // Check for initializer
        let init = if self.at(SyntaxKind::Eq) {
            self.consume();
            self.skip_whitespace();
            // Parse the initializer expression until comma or closing brace
            Some(Box::new(self.parse_ts_expr_until(&[
                SyntaxKind::Comma,
                SyntaxKind::RBrace,
            ])?))
        } else {
            None
        };

        Some(IrNode::EnumMember {
            name: Box::new(name),
            init,
        })
    }
}
