//! Intermediate Representation (IR) for code generation.
//!
//! The IR is a simplified, flattened representation of the template that's
//! easy to generate code from. It combines information from the parser's
//! CST and the semantic analyzer's placeholder classifications.

use super::semantic::{PlaceholderKind, SemanticAnalysis};
use super::syntax::{SyntaxKind, SyntaxNode};

/// A node in the IR.
#[derive(Debug, Clone)]
pub enum IrNode {
    /// Literal text to emit verbatim.
    Text(String),

    /// A placeholder that will be substituted.
    Placeholder {
        /// Classification (expr, type, ident, stmt).
        kind: PlaceholderKind,
        /// The Rust expression tokens.
        rust_expr: String,
    },

    /// Identifier block: concatenated parts forming a single identifier.
    IdentBlock {
        /// Parts (text or placeholders) to concatenate.
        parts: Vec<IrNode>,
    },

    /// String interpolation: a string with embedded expressions.
    StringInterp {
        /// Quote style: '"', '\'', or '`'.
        quote: char,
        /// Parts of the string.
        parts: Vec<IrNode>,
    },

    /// If/else-if/else control flow.
    If {
        /// The condition (Rust expression).
        condition: String,
        /// Body when condition is true.
        then_body: Vec<IrNode>,
        /// Else-if branches: (condition, body).
        else_if_branches: Vec<(String, Vec<IrNode>)>,
        /// Else branch body.
        else_body: Option<Vec<IrNode>>,
    },

    /// For loop.
    For {
        /// Loop variable pattern.
        pattern: String,
        /// Iterator expression.
        iterator: String,
        /// Loop body.
        body: Vec<IrNode>,
    },

    /// While loop.
    While {
        /// Loop condition.
        condition: String,
        /// Loop body.
        body: Vec<IrNode>,
    },

    /// Match expression.
    Match {
        /// Expression to match.
        expr: String,
        /// Match arms: (pattern, guard, body).
        arms: Vec<(String, Option<String>, Vec<IrNode>)>,
    },

    /// Let binding directive.
    Let {
        /// Variable name.
        name: String,
        /// Whether it's mutable.
        mutable: bool,
        /// Optional type annotation (e.g., "Expr" in `name: Expr`).
        type_hint: Option<String>,
        /// Initial value expression.
        value: String,
    },

    /// Do directive (execute Rust code).
    Do {
        /// Rust code to execute.
        code: String,
    },

    /// TypeScript injection directive.
    TypeScript {
        /// TokenStream to inject.
        stream: String,
    },

    /// A comment (preserved for debugging).
    Comment {
        /// Comment text.
        text: String,
    },
}

/// The complete IR for a template.
#[derive(Debug)]
pub struct Ir {
    /// Root nodes of the template.
    pub nodes: Vec<IrNode>,
}

/// Lowers a CST and semantic analysis result to IR.
pub struct IrLowering {
    /// Semantic analysis results.
    analysis: SemanticAnalysis,
    /// Current placeholder ID counter (mirrors semantic analysis).
    placeholder_id: usize,
}

impl IrLowering {
    /// Creates a new IR lowering pass.
    pub fn new(analysis: SemanticAnalysis) -> Self {
        Self {
            analysis,
            placeholder_id: 0,
        }
    }

    /// Lowers the CST to IR.
    pub fn lower(mut self, root: &SyntaxNode) -> Ir {
        let nodes = self.lower_node(root);
        Ir { nodes }
    }

    /// Lowers a syntax node to IR nodes.
    fn lower_node(&mut self, node: &SyntaxNode) -> Vec<IrNode> {
        match node.kind() {
            SyntaxKind::Root => self.lower_children(node),

            SyntaxKind::Interpolation => {
                let id = self.next_placeholder_id();
                if let Some(info) = self.analysis.placeholders.get(&id) {
                    vec![IrNode::Placeholder {
                        kind: info.kind,
                        rust_expr: info.tokens.clone(),
                    }]
                } else {
                    // Fallback: extract tokens from node
                    let text = node.text().to_string();
                    let rust_expr = self.extract_interpolation_content(&text);
                    vec![IrNode::Placeholder {
                        kind: PlaceholderKind::Expr,
                        rust_expr,
                    }]
                }
            }

            SyntaxKind::IdentBlock => {
                let parts = self.lower_children(node);
                vec![IrNode::IdentBlock { parts }]
            }

            SyntaxKind::StringInterp => {
                let parts = self.lower_children(node);
                vec![IrNode::StringInterp {
                    quote: '"',
                    parts,
                }]
            }

            SyntaxKind::TemplateLiteral => {
                let parts = self.lower_children(node);
                vec![IrNode::StringInterp {
                    quote: '`',
                    parts,
                }]
            }

            SyntaxKind::IfBlock | SyntaxKind::IfLetBlock => {
                self.lower_if_block(node)
            }

            SyntaxKind::ForBlock => {
                self.lower_for_block(node)
            }

            SyntaxKind::WhileBlock | SyntaxKind::WhileLetBlock => {
                self.lower_while_block(node)
            }

            SyntaxKind::MatchBlock => {
                self.lower_match_block(node)
            }

            SyntaxKind::LetDirective => {
                self.lower_let_directive(node)
            }

            SyntaxKind::DoDirective => {
                self.lower_do_directive(node)
            }

            SyntaxKind::TypeScriptDirective => {
                self.lower_typescript_directive(node)
            }

            SyntaxKind::LineComment | SyntaxKind::BlockComment => {
                let text = self.extract_comment_text(node);
                vec![IrNode::Comment { text }]
            }

            SyntaxKind::DocComment => {
                let text = self.extract_comment_text(node);
                vec![IrNode::Comment { text }]
            }

            SyntaxKind::BraceBlock
            | SyntaxKind::TypeAnnotation
            | SyntaxKind::TypeAssertion
            | SyntaxKind::TsTypeParams
            | SyntaxKind::TsExpr
            | SyntaxKind::TsParam
            | SyntaxKind::TsObject
            | SyntaxKind::ElseClause
            | SyntaxKind::ElseIfClause
            | SyntaxKind::MatchCase => {
                // These are structural nodes - lower their children
                self.lower_children(node)
            }

            // Tokens that contribute text
            _ => {
                let text = self.collect_text_from_node(node);
                if text.is_empty() {
                    vec![]
                } else {
                    vec![IrNode::Text(text)]
                }
            }
        }
    }

    /// Lowers all children of a node.
    fn lower_children(&mut self, node: &SyntaxNode) -> Vec<IrNode> {
        let mut result = Vec::new();

        for child in node.children_with_tokens() {
            match child {
                rowan::NodeOrToken::Node(child_node) => {
                    result.extend(self.lower_node(&child_node));
                }
                rowan::NodeOrToken::Token(token) => {
                    // Skip certain structural tokens
                    if !self.is_structural_token(token.kind()) {
                        let text = token.text().to_string();
                        if !text.is_empty() {
                            result.push(IrNode::Text(text));
                        }
                    }
                }
            }
        }

        // Merge adjacent text nodes
        self.merge_text_nodes(result)
    }

    /// Checks if a token is structural (should not contribute to output).
    fn is_structural_token(&self, kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SyntaxKind::HashOpen
                | SyntaxKind::SlashOpen
                | SyntaxKind::ColonOpen
                | SyntaxKind::DollarOpen
                | SyntaxKind::PipeOpen
                | SyntaxKind::PipeClose
                | SyntaxKind::CommentLineOpen
                | SyntaxKind::CommentLineClose
                | SyntaxKind::CommentBlockOpen
                | SyntaxKind::CommentBlockClose
                | SyntaxKind::At
                | SyntaxKind::AtAt
        )
    }

    /// Merges adjacent text nodes.
    fn merge_text_nodes(&self, nodes: Vec<IrNode>) -> Vec<IrNode> {
        let mut result = Vec::new();
        let mut pending_text = String::new();

        for node in nodes {
            match node {
                IrNode::Text(text) => {
                    pending_text.push_str(&text);
                }
                other => {
                    if !pending_text.is_empty() {
                        result.push(IrNode::Text(std::mem::take(&mut pending_text)));
                    }
                    result.push(other);
                }
            }
        }

        if !pending_text.is_empty() {
            result.push(IrNode::Text(pending_text));
        }

        result
    }

    /// Collects text from a node and its children (for simple text nodes).
    fn collect_text_from_node(&self, node: &SyntaxNode) -> String {
        node.text().to_string()
    }

    /// Extracts content from an interpolation (strips @{ and }).
    fn extract_interpolation_content(&self, text: &str) -> String {
        text.strip_prefix("@{")
            .and_then(|s| s.strip_suffix("}"))
            .map(|s| s.trim().to_string())
            .or_else(|| text.strip_prefix("@").map(|s| s.trim().to_string()))
            .unwrap_or_else(|| text.to_string())
    }

    /// Extracts text from a comment node.
    fn extract_comment_text(&self, node: &SyntaxNode) -> String {
        let text = node.text().to_string();
        // Strip comment delimiters
        text.trim().to_string()
    }

    /// Gets the next placeholder ID.
    fn next_placeholder_id(&mut self) -> usize {
        let id = self.placeholder_id;
        self.placeholder_id += 1;
        id
    }

    /// Lowers an if block.
    fn lower_if_block(&mut self, node: &SyntaxNode) -> Vec<IrNode> {
        let mut condition = String::new();
        let mut then_body = Vec::new();
        let mut else_if_branches = Vec::new();
        let mut else_body = None;
        let mut phase = 0; // 0=before condition, 1=in condition, 2=in body

        for child in node.children_with_tokens() {
            match child {
                rowan::NodeOrToken::Token(token) => {
                    let kind = token.kind();
                    match kind {
                        SyntaxKind::IfKw if phase == 0 => {
                            // Only match the opening IF_KW, not the closing one
                            phase = 1;
                        }
                        SyntaxKind::RBrace if phase == 1 => {
                            phase = 2;
                        }
                        SyntaxKind::SlashOpen => {
                            // Hit the closing {/if} - stop processing body
                            break;
                        }
                        _ if phase == 1 => {
                            condition.push_str(token.text());
                        }
                        _ if phase == 2 && !self.is_structural_token(kind) => {
                            // Body tokens (text content)
                            then_body.push(IrNode::Text(token.text().to_string()));
                        }
                        _ => {}
                    }
                }
                rowan::NodeOrToken::Node(child_node) => {
                    match child_node.kind() {
                        SyntaxKind::ElseIfClause => {
                            let (cond, body) = self.lower_else_if_clause(&child_node);
                            else_if_branches.push((cond, body));
                        }
                        SyntaxKind::ElseClause => {
                            else_body = Some(self.lower_else_clause(&child_node));
                        }
                        SyntaxKind::Interpolation if phase == 1 => {
                            // Interpolation in condition - must increment ID to stay in sync
                            // with semantic analysis, and append the rust expr to condition
                            let id = self.next_placeholder_id();
                            if let Some(info) = self.analysis.placeholders.get(&id) {
                                condition.push_str(&info.tokens);
                            } else {
                                let text = child_node.text().to_string();
                                condition.push_str(&self.extract_interpolation_content(&text));
                            }
                        }
                        _ if phase == 2 => {
                            then_body.extend(self.lower_node(&child_node));
                        }
                        _ => {}
                    }
                }
            }
        }

        vec![IrNode::If {
            condition: condition.trim().to_string(),
            then_body: self.merge_text_nodes(then_body),
            else_if_branches,
            else_body,
        }]
    }

    /// Lowers an else-if clause, returning (condition, body).
    fn lower_else_if_clause(&mut self, node: &SyntaxNode) -> (String, Vec<IrNode>) {
        let mut condition = String::new();
        let mut body = Vec::new();
        let mut phase = 0; // 0=before condition, 1=in condition, 2=in body

        for child in node.children_with_tokens() {
            match child {
                rowan::NodeOrToken::Token(token) => {
                    let kind = token.kind();
                    match kind {
                        SyntaxKind::IfKw if phase == 0 => {
                            phase = 1;
                        }
                        SyntaxKind::RBrace if phase == 1 => {
                            phase = 2;
                        }
                        _ if phase == 1 => {
                            condition.push_str(token.text());
                        }
                        _ if phase == 2 && !self.is_structural_token(kind) => {
                            body.push(IrNode::Text(token.text().to_string()));
                        }
                        _ => {}
                    }
                }
                rowan::NodeOrToken::Node(child_node) => {
                    match child_node.kind() {
                        SyntaxKind::Interpolation if phase == 1 => {
                            // Interpolation in condition - increment ID and append rust expr
                            let id = self.next_placeholder_id();
                            if let Some(info) = self.analysis.placeholders.get(&id) {
                                condition.push_str(&info.tokens);
                            } else {
                                let text = child_node.text().to_string();
                                condition.push_str(&self.extract_interpolation_content(&text));
                            }
                        }
                        _ if phase == 2 => {
                            body.extend(self.lower_node(&child_node));
                        }
                        _ => {}
                    }
                }
            }
        }

        (condition.trim().to_string(), self.merge_text_nodes(body))
    }

    /// Lowers an else clause, returning the body.
    fn lower_else_clause(&mut self, node: &SyntaxNode) -> Vec<IrNode> {
        let mut body = Vec::new();
        let mut in_body = false;

        for child in node.children_with_tokens() {
            match child {
                rowan::NodeOrToken::Token(token) => {
                    let kind = token.kind();
                    if kind == SyntaxKind::RBrace && !in_body {
                        in_body = true;
                    } else if in_body && !self.is_structural_token(kind) {
                        body.push(IrNode::Text(token.text().to_string()));
                    }
                }
                rowan::NodeOrToken::Node(child_node) if in_body => {
                    body.extend(self.lower_node(&child_node));
                }
                _ => {}
            }
        }

        self.merge_text_nodes(body)
    }

    /// Lowers a for block.
    fn lower_for_block(&mut self, node: &SyntaxNode) -> Vec<IrNode> {
        let mut pattern = String::new();
        let mut iterator = String::new();
        let mut body = Vec::new();
        let mut phase = 0; // 0=before pattern, 1=in pattern, 2=in iterator, 3=in body

        for child in node.children_with_tokens() {
            match child {
                rowan::NodeOrToken::Token(token) => {
                    let kind = token.kind();
                    match kind {
                        SyntaxKind::ForKw if phase == 0 => {
                            // Only match the opening FOR_KW, not the closing one in {/for}
                            phase = 1;
                        }
                        SyntaxKind::InKw if phase == 1 => {
                            phase = 2;
                        }
                        SyntaxKind::RBrace if phase == 2 => {
                            phase = 3;
                        }
                        SyntaxKind::SlashOpen => {
                            // Hit the closing {/for} - stop processing body
                            break;
                        }
                        _ if phase == 1 => {
                            pattern.push_str(token.text());
                        }
                        _ if phase == 2 => {
                            iterator.push_str(token.text());
                        }
                        _ if phase == 3 && !self.is_structural_token(kind) => {
                            // Body tokens (text content)
                            body.push(IrNode::Text(token.text().to_string()));
                        }
                        _ => {}
                    }
                }
                rowan::NodeOrToken::Node(child_node) => {
                    match child_node.kind() {
                        SyntaxKind::Interpolation if phase == 1 => {
                            // Interpolation in pattern - increment ID and append rust expr
                            let id = self.next_placeholder_id();
                            if let Some(info) = self.analysis.placeholders.get(&id) {
                                pattern.push_str(&info.tokens);
                            } else {
                                let text = child_node.text().to_string();
                                pattern.push_str(&self.extract_interpolation_content(&text));
                            }
                        }
                        SyntaxKind::Interpolation if phase == 2 => {
                            // Interpolation in iterator - increment ID and append rust expr
                            let id = self.next_placeholder_id();
                            if let Some(info) = self.analysis.placeholders.get(&id) {
                                iterator.push_str(&info.tokens);
                            } else {
                                let text = child_node.text().to_string();
                                iterator.push_str(&self.extract_interpolation_content(&text));
                            }
                        }
                        _ if phase == 3 => {
                            body.extend(self.lower_node(&child_node));
                        }
                        _ => {}
                    }
                }
            }
        }

        vec![IrNode::For {
            pattern: pattern.trim().to_string(),
            iterator: iterator.trim().to_string(),
            body: self.merge_text_nodes(body),
        }]
    }

    /// Lowers a while block.
    fn lower_while_block(&mut self, node: &SyntaxNode) -> Vec<IrNode> {
        let mut condition = String::new();
        let mut body = Vec::new();
        let mut phase = 0; // 0=before condition, 1=in condition, 2=in body

        for child in node.children_with_tokens() {
            match child {
                rowan::NodeOrToken::Token(token) => {
                    let kind = token.kind();
                    match kind {
                        SyntaxKind::WhileKw if phase == 0 => {
                            phase = 1;
                        }
                        SyntaxKind::RBrace if phase == 1 => {
                            phase = 2;
                        }
                        SyntaxKind::SlashOpen => {
                            // Hit the closing {/while} - stop processing body
                            break;
                        }
                        _ if phase == 1 => {
                            condition.push_str(token.text());
                        }
                        _ if phase == 2 && !self.is_structural_token(kind) => {
                            // Body tokens (text content)
                            body.push(IrNode::Text(token.text().to_string()));
                        }
                        _ => {}
                    }
                }
                rowan::NodeOrToken::Node(child_node) => {
                    match child_node.kind() {
                        SyntaxKind::Interpolation if phase == 1 => {
                            // Interpolation in condition - increment ID and append rust expr
                            let id = self.next_placeholder_id();
                            if let Some(info) = self.analysis.placeholders.get(&id) {
                                condition.push_str(&info.tokens);
                            } else {
                                let text = child_node.text().to_string();
                                condition.push_str(&self.extract_interpolation_content(&text));
                            }
                        }
                        _ if phase == 2 => {
                            body.extend(self.lower_node(&child_node));
                        }
                        _ => {}
                    }
                }
            }
        }

        vec![IrNode::While {
            condition: condition.trim().to_string(),
            body: self.merge_text_nodes(body),
        }]
    }

    /// Lowers a match block.
    fn lower_match_block(&mut self, node: &SyntaxNode) -> Vec<IrNode> {
        let mut expr = String::new();
        let mut arms = Vec::new();
        let mut phase = 0; // 0=before expr, 1=in expr, 2=in arms

        for child in node.children_with_tokens() {
            match child {
                rowan::NodeOrToken::Token(token) => {
                    match token.kind() {
                        SyntaxKind::MatchKw if phase == 0 => {
                            phase = 1;
                        }
                        SyntaxKind::RBrace if phase == 1 => {
                            phase = 2;
                        }
                        _ if phase == 1 => {
                            expr.push_str(token.text());
                        }
                        _ => {}
                    }
                }
                rowan::NodeOrToken::Node(child_node) => {
                    match child_node.kind() {
                        SyntaxKind::Interpolation if phase == 1 => {
                            // Interpolation in expr - increment ID and append rust expr
                            let id = self.next_placeholder_id();
                            if let Some(info) = self.analysis.placeholders.get(&id) {
                                expr.push_str(&info.tokens);
                            } else {
                                let text = child_node.text().to_string();
                                expr.push_str(&self.extract_interpolation_content(&text));
                            }
                        }
                        SyntaxKind::MatchCase if phase == 2 => {
                            let (pattern, guard, body) = self.lower_match_case(&child_node);
                            arms.push((pattern, guard, body));
                        }
                        _ => {}
                    }
                }
            }
        }

        vec![IrNode::Match {
            expr: expr.trim().to_string(),
            arms,
        }]
    }

    /// Lowers a match case, returning (pattern, guard, body).
    fn lower_match_case(&mut self, node: &SyntaxNode) -> (String, Option<String>, Vec<IrNode>) {
        let mut pattern = String::new();
        let mut body = Vec::new();
        let mut phase = 0; // 0=before pattern, 1=in pattern, 2=in body

        for child in node.children_with_tokens() {
            match child {
                rowan::NodeOrToken::Token(token) => {
                    let kind = token.kind();
                    match kind {
                        SyntaxKind::CaseKw if phase == 0 => {
                            phase = 1;
                        }
                        SyntaxKind::RBrace if phase == 1 => {
                            phase = 2;
                        }
                        SyntaxKind::ColonOpen | SyntaxKind::SlashOpen if phase == 2 => {
                            // Hit the next case or closing {/match} - stop processing body
                            break;
                        }
                        SyntaxKind::ColonOpen if phase == 0 => {
                            // Opening {: of this case - skip
                        }
                        _ if phase == 1 => {
                            pattern.push_str(token.text());
                        }
                        _ if phase == 2 && !self.is_structural_token(kind) => {
                            // Body tokens (text content)
                            body.push(IrNode::Text(token.text().to_string()));
                        }
                        _ => {}
                    }
                }
                rowan::NodeOrToken::Node(child_node) => {
                    match child_node.kind() {
                        SyntaxKind::Interpolation if phase == 1 => {
                            // Interpolation in pattern - increment ID and append rust expr
                            let id = self.next_placeholder_id();
                            if let Some(info) = self.analysis.placeholders.get(&id) {
                                pattern.push_str(&info.tokens);
                            } else {
                                let text = child_node.text().to_string();
                                pattern.push_str(&self.extract_interpolation_content(&text));
                            }
                        }
                        _ if phase == 2 => {
                            body.extend(self.lower_node(&child_node));
                        }
                        _ => {}
                    }
                }
            }
        }

        (
            pattern.trim().to_string(),
            None, // Guards not yet supported
            self.merge_text_nodes(body),
        )
    }

    /// Lowers a let directive.
    fn lower_let_directive(&mut self, node: &SyntaxNode) -> Vec<IrNode> {
        let text = node.text().to_string();
        // Parse: {$let [mut] name = value}
        let content = self.strip_directive_delimiters(&text, "let");

        let (mutable, rest) = if let Some(stripped) = content.trim_start().strip_prefix("mut ") {
            (true, stripped.to_string())
        } else {
            (false, content.to_string())
        };

        if let Some(eq_pos) = rest.find('=') {
            let name_part = rest[..eq_pos].trim();
            let value = rest[eq_pos + 1..].trim().to_string();

            // Parse optional type annotation: `name: Type` or just `name`
            let (name, type_hint) = if let Some(colon_pos) = name_part.find(':') {
                let n = name_part[..colon_pos].trim().to_string();
                let t = name_part[colon_pos + 1..].trim().to_string();
                (n, Some(t))
            } else {
                (name_part.to_string(), None)
            };

            vec![IrNode::Let {
                name,
                mutable,
                type_hint,
                value,
            }]
        } else {
            // Invalid let directive: missing '=' - skip
            vec![]
        }
    }

    /// Lowers a do directive.
    fn lower_do_directive(&mut self, node: &SyntaxNode) -> Vec<IrNode> {
        let text = node.text().to_string();
        let code = self.strip_directive_delimiters(&text, "do");
        vec![IrNode::Do {
            code: code.trim().to_string(),
        }]
    }

    /// Lowers a typescript directive.
    fn lower_typescript_directive(&mut self, node: &SyntaxNode) -> Vec<IrNode> {
        let text = node.text().to_string();
        let stream = self.strip_directive_delimiters(&text, "typescript");
        vec![IrNode::TypeScript {
            stream: stream.trim().to_string(),
        }]
    }

    /// Strips directive delimiters: {$keyword ...} -> ...
    fn strip_directive_delimiters(&self, text: &str, keyword: &str) -> String {
        let prefix = format!("{{${} ", keyword);
        let alt_prefix = format!("{{${}", keyword);

        let stripped = text
            .strip_prefix(prefix.as_str())
            .or_else(|| text.strip_prefix(alt_prefix.as_str()))
            .unwrap_or(text);

        stripped
            .strip_suffix('}')
            .unwrap_or(stripped)
            .to_string()
    }
}

/// Convenience function to lower a CST to IR.
pub fn lower(root: &SyntaxNode, analysis: SemanticAnalysis) -> Ir {
    IrLowering::new(analysis).lower(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::parser::Parser;
    use crate::compiler::semantic::analyze;

    fn lower_template(input: &str) -> Ir {
        let parser = Parser::new(input);
        let green = parser.parse();
        let root = SyntaxNode::new_root(green);
        let analysis = analyze(&root);
        lower(&root, analysis)
    }

    #[test]
    fn test_lower_simple_text() {
        let ir = lower_template("hello world");
        assert!(!ir.nodes.is_empty());
        // Should have text node(s)
        let has_text = ir.nodes.iter().any(|n| matches!(n, IrNode::Text(_)));
        assert!(has_text);
    }

    #[test]
    fn test_lower_interpolation() {
        let ir = lower_template("@{expr}");
        assert_eq!(ir.nodes.len(), 1);
        match &ir.nodes[0] {
            IrNode::Placeholder { rust_expr, .. } => {
                assert_eq!(rust_expr, "expr");
            }
            _ => panic!("Expected Placeholder"),
        }
    }

    #[test]
    fn test_lower_mixed_content() {
        let ir = lower_template("const x = @{value}");
        // Should have text and placeholder
        let has_text = ir.nodes.iter().any(|n| matches!(n, IrNode::Text(_)));
        let has_placeholder = ir.nodes.iter().any(|n| matches!(n, IrNode::Placeholder { .. }));
        assert!(has_text);
        assert!(has_placeholder);
    }

    #[test]
    fn test_lower_type_placeholder() {
        let ir = lower_template("const x: @{MyType} = 1");
        // Find the placeholder
        let placeholder = ir.nodes.iter().find_map(|n| {
            if let IrNode::Placeholder { kind, rust_expr, .. } = n {
                Some((kind, rust_expr))
            } else {
                None
            }
        });
        assert!(placeholder.is_some());
        let (kind, expr) = placeholder.unwrap();
        assert_eq!(*kind, PlaceholderKind::Type);
        assert_eq!(expr, "MyType");
    }

    #[test]
    fn test_lower_for_block() {
        let ir = lower_template("{#for item in items}@{item}{/for}");
        assert_eq!(ir.nodes.len(), 1);
        match &ir.nodes[0] {
            IrNode::For { pattern, iterator, body } => {
                assert_eq!(pattern, "item");
                assert_eq!(iterator, "items");
                assert!(!body.is_empty());
            }
            _ => panic!("Expected For"),
        }
    }

    #[test]
    fn test_lower_if_block() {
        let ir = lower_template("{#if cond}yes{/if}");
        assert_eq!(ir.nodes.len(), 1);
        match &ir.nodes[0] {
            IrNode::If { condition, then_body, else_body, .. } => {
                assert_eq!(condition, "cond");
                assert!(!then_body.is_empty());
                assert!(else_body.is_none());
            }
            _ => panic!("Expected If"),
        }
    }

    #[test]
    fn test_lower_if_else_block() {
        let ir = lower_template("{#if cond}yes{:else}no{/if}");
        assert_eq!(ir.nodes.len(), 1);
        match &ir.nodes[0] {
            IrNode::If { condition, then_body, else_body, .. } => {
                assert_eq!(condition, "cond");
                assert!(!then_body.is_empty());
                assert!(else_body.is_some());
            }
            _ => panic!("Expected If"),
        }
    }

    #[test]
    fn test_placeholder_info_preserved() {
        let ir = lower_template("const x: @{T} = @{v}");
        // Should have 2 placeholders in nodes
        let placeholder_count = ir.nodes.iter().filter(|n| matches!(n, IrNode::Placeholder { .. })).count();
        assert_eq!(placeholder_count, 2);
    }
}
