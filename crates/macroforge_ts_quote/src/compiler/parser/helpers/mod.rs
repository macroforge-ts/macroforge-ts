mod collect;
mod parse;

use super::*;

// =========================================================================
// Helper Parsing Functions
// =========================================================================

impl Parser {
    // current_text is now defined in expr/primary.rs

    pub(super) fn wrap_with_doc(&mut self, node: IrNode) -> ParseResult<IrNode> {
        if let Some(doc) = self.pending_doc.take() {
            let span = node.span();
            Ok(IrNode::Documented {
                span,
                doc,
                inner: Box::new(node),
            })
        } else {
            Ok(node)
        }
    }

    /// Determines if the current `{` starts a block statement rather than an object literal.
    /// Uses lookahead to check the token after `{`.
    /// Only returns true for unambiguous block patterns at the true module level.
    pub(super) fn looks_like_block_stmt(&self) -> bool {
        // Only check at the true module level (context stack has only the initial context)
        if self.context_stack.len() > 1 {
            return false;
        }

        // We're currently at `{` - look at the next token(s)
        let mut lookahead_pos = self.pos + 1;

        // Skip whitespace
        while lookahead_pos < self.tokens.len() {
            if self.tokens[lookahead_pos].kind == SyntaxKind::Whitespace {
                lookahead_pos += 1;
            } else {
                break;
            }
        }

        if lookahead_pos >= self.tokens.len() {
            return false;
        }

        let next_kind = self.tokens[lookahead_pos].kind;

        // Statement keywords that can ONLY appear in block statements (not object literals)
        // Be conservative - only include keywords that are definitely block-only
        matches!(
            next_kind,
            SyntaxKind::ConstKw
                | SyntaxKind::LetKw
                | SyntaxKind::VarKw
                | SyntaxKind::IfKw
                | SyntaxKind::ForKw
                | SyntaxKind::WhileKw
                | SyntaxKind::ReturnKw
                | SyntaxKind::ThrowKw
                | SyntaxKind::RBrace // empty block {}
        )
    }

    /// Check if `const` is followed by `enum` (for `const enum` declarations)
    pub(super) fn peek_is_enum(&self) -> bool {
        // We're at `const` - look ahead for `enum`
        let mut lookahead_pos = self.pos + 1;

        // Skip whitespace
        while lookahead_pos < self.tokens.len() {
            if self.tokens[lookahead_pos].kind == SyntaxKind::Whitespace {
                lookahead_pos += 1;
            } else {
                break;
            }
        }

        if lookahead_pos >= self.tokens.len() {
            return false;
        }

        self.tokens[lookahead_pos].kind == SyntaxKind::EnumKw
    }

    pub(super) fn placeholder_kind(&self) -> PlaceholderKind {
        let ctx_kind = self.current_context_kind();
        let kind = match ctx_kind {
            ContextKind::TypeAnnotation | ContextKind::TypeAssertion | ContextKind::GenericParams => {
                PlaceholderKind::Type
            }
            // In interface member context, placeholders are typically identifiers (property names)
            ContextKind::Identifier | ContextKind::InterfaceMember => PlaceholderKind::Ident,
            ContextKind::Statement => PlaceholderKind::Stmt,
            ContextKind::Expression(_) | ContextKind::Parameters => PlaceholderKind::Expr,
        };

        #[cfg(debug_assertions)]
        if std::env::var("MF_DEBUG_PARSER").is_ok() {
            eprintln!(
                "[MF_DEBUG_PARSER] placeholder_kind: ctx={:?}, kind={:?}, stack={:?}",
                ctx_kind, kind, self.context_stack
            );
        }

        kind
    }

    /// Merges adjacent text nodes (StrLit, Ident).
    pub(super) fn merge_adjacent_text(nodes: Vec<IrNode>) -> Vec<IrNode> {
        let mut result = Vec::with_capacity(nodes.len());
        let mut pending_text = String::new();

        for node in nodes {
            match node {
                IrNode::StrLit { value, .. } | IrNode::Ident { value, .. } => {
                    pending_text.push_str(&value);
                }
                other => {
                    if !pending_text.is_empty() {
                        result.push(IrNode::StrLit {
                            span: IrSpan::empty(),
                            value: std::mem::take(&mut pending_text),
                        });
                    }
                    result.push(other);
                }
            }
        }

        if !pending_text.is_empty() {
            result.push(IrNode::StrLit {
                span: IrSpan::empty(),
                value: pending_text,
            });
        }

        result
    }
}
