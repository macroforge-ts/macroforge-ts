use super::super::*;

impl Parser {
    /// Parse a TypeScript for/while loop.
    /// For-in and for-of loops are parsed structurally as ForInStmt/ForOfStmt.
    /// C-style for loops and while loops are parsed as TsLoopStmt (raw text with placeholders).
    pub(in super::super) fn parse_ts_loop_stmt(&mut self) -> Option<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        let keyword = self.current()?.text.clone();

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG] parse_ts_loop_stmt: keyword={:?}", keyword);
        }

        if keyword == "for" {
            // Try to parse as for-in or for-of first
            if let Some(structured) = self.try_parse_for_in_of() {
                return Some(structured);
            }
            // Fall back to raw parsing for C-style for loops
            return self.parse_loop_as_raw();
        }

        // While loops - parse as raw
        self.parse_loop_as_raw()
    }

    /// Try to parse a for-in or for-of loop.
    /// Returns None if it's a C-style for loop (with semicolons).
    fn try_parse_for_in_of(&mut self) -> Option<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        // Save position for backtracking
        let start_pos = self.pos;

        self.consume()?; // for
        self.skip_whitespace();

        // Check for await keyword (for-await-of)
        let has_await = if self.at(SyntaxKind::AwaitKw) {
            self.consume();
            self.skip_whitespace();
            true
        } else {
            false
        };

        if !self.at(SyntaxKind::LParen) {
            // Restore position and let raw parsing handle it
            self.pos = start_pos;
            return None;
        }
        self.consume(); // (
        self.skip_whitespace();

        // Parse the left-hand side (variable declaration or expression)
        let left = self.parse_for_loop_left()?;

        self.skip_whitespace();

        // Check for 'in' or 'of' keyword
        let is_for_in = self.at(SyntaxKind::InKw);
        let is_for_of = self.at(SyntaxKind::OfKw);

        if !is_for_in && !is_for_of {
            // This is a C-style for loop - restore position
            self.pos = start_pos;
            return None;
        }

        self.consume(); // in/of
        self.skip_whitespace();

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!(
                "[MF_DEBUG] parse_for_in_of: is_for_in={}, is_for_of={}, has_await={}",
                is_for_in, is_for_of, has_await
            );
        }

        // Parse the right-hand side (expression)
        let right = self.parse_ts_expr_until(&[SyntaxKind::RParen])?;

        self.skip_whitespace();
        self.expect(SyntaxKind::RParen);
        self.skip_whitespace();

        // Parse the body
        let body = if self.at(SyntaxKind::LBrace) {
            self.parse_block_stmt()?
        } else {
            self.parse_stmt()?
        };

        if is_for_in {
            Some(IrNode::ForInStmt {
                left: Box::new(left),
                right: Box::new(right),
                body: Box::new(body),
            })
        } else {
            Some(IrNode::ForOfStmt {
                await_: has_await,
                left: Box::new(left),
                right: Box::new(right),
                body: Box::new(body),
            })
        }
    }

    /// Parse the left-hand side of a for-in/for-of loop.
    /// This can be a variable declaration (const/let/var x) or an expression.
    fn parse_for_loop_left(&mut self) -> Option<IrNode> {
        match self.current_kind()? {
            SyntaxKind::ConstKw | SyntaxKind::LetKw | SyntaxKind::VarKw => {
                // Variable declaration
                let kind = match self.current_kind()? {
                    SyntaxKind::ConstKw => VarKind::Const,
                    SyntaxKind::LetKw => VarKind::Let,
                    SyntaxKind::VarKw => VarKind::Var,
                    _ => unreachable!(),
                };
                self.consume(); // const/let/var
                self.skip_whitespace();

                // Parse the binding pattern or identifier
                let name = self.parse_binding_pattern()?;

                Some(IrNode::VarDecl {
                    exported: false,
                    declare: false,
                    kind,
                    decls: vec![VarDeclarator {
                        name: Box::new(name),
                        type_ann: None,
                        init: None,
                        definite: false,
                    }],
                })
            }
            SyntaxKind::At => {
                // Placeholder - could be expression or identifier
                self.parse_interpolation()
            }
            SyntaxKind::LBracket => {
                // Array destructuring pattern
                self.parse_array_pattern()
            }
            SyntaxKind::LBrace => {
                // Object destructuring pattern
                self.parse_object_pattern()
            }
            _ => {
                // Expression (for reassignment like: for (x in obj))
                self.parse_ts_expr_until(&[SyntaxKind::InKw, SyntaxKind::OfKw])
            }
        }
    }

    /// Parse a binding pattern (identifier, array pattern, or object pattern).
    fn parse_binding_pattern(&mut self) -> Option<IrNode> {
        match self.current_kind()? {
            SyntaxKind::LBracket => self.parse_array_pattern(),
            SyntaxKind::LBrace => self.parse_object_pattern(),
            SyntaxKind::At => self.parse_interpolation(),
            _ => self.parse_ts_ident_or_placeholder(),
        }
    }

    /// Parse an array destructuring pattern: [a, b, ...rest]
    fn parse_array_pattern(&mut self) -> Option<IrNode> {
        self.consume()?; // [
        self.skip_whitespace();

        let mut elems: Vec<Option<IrNode>> = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBracket) {
            self.skip_whitespace();

            if self.at(SyntaxKind::Comma) {
                // Hole in array pattern
                elems.push(None);
                self.consume();
                continue;
            }

            if self.at(SyntaxKind::RBracket) {
                break;
            }

            // Check for rest pattern
            if self.at(SyntaxKind::DotDotDot) {
                self.consume();
                self.skip_whitespace();
                let arg = self.parse_binding_pattern()?;
                elems.push(Some(IrNode::RestPat {
                    arg: Box::new(arg),
                    type_ann: None,
                }));
            } else {
                // Regular element
                let elem = self.parse_binding_pattern()?;
                elems.push(Some(elem));
            }

            self.skip_whitespace();
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }
        }

        self.expect(SyntaxKind::RBracket);

        Some(IrNode::ArrayPat {
            elems,
            type_ann: None,
            optional: false,
        })
    }

    /// Parse an object destructuring pattern: { a, b: c, ...rest }
    fn parse_object_pattern(&mut self) -> Option<IrNode> {
        self.consume()?; // {
        self.skip_whitespace();

        let mut props: Vec<IrNode> = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBrace) {
            self.skip_whitespace();

            if self.at(SyntaxKind::RBrace) {
                break;
            }

            // Check for rest pattern
            if self.at(SyntaxKind::DotDotDot) {
                self.consume();
                self.skip_whitespace();
                let arg = self.parse_binding_pattern()?;
                props.push(IrNode::RestPat {
                    arg: Box::new(arg),
                    type_ann: None,
                });
            } else {
                // Regular property
                let key = self.parse_ts_ident_or_placeholder()?;
                self.skip_whitespace();

                if self.at(SyntaxKind::Colon) {
                    // Renamed binding: { a: b }
                    self.consume();
                    self.skip_whitespace();
                    let value = self.parse_binding_pattern()?;
                    props.push(IrNode::ObjectPatProp {
                        key: Box::new(key),
                        value: Some(Box::new(value)),
                    });
                } else {
                    // Shorthand: { a }
                    props.push(IrNode::ObjectPatProp {
                        key: Box::new(key),
                        value: None,
                    });
                }
            }

            self.skip_whitespace();
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }
        }

        self.expect(SyntaxKind::RBrace);

        Some(IrNode::ObjectPat {
            props,
            type_ann: None,
            optional: false,
        })
    }

    /// Parse a loop as raw text with placeholders (fallback for C-style for and while loops).
    fn parse_loop_as_raw(&mut self) -> Option<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        let keyword = self.consume()?.text; // for/while
        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG] parse_loop_as_raw: keyword={:?}", keyword);
        }
        let mut parts = vec![IrNode::Raw(keyword)];

        // Helper to add whitespace
        while let Some(t) = self.current() {
            if t.kind == SyntaxKind::Whitespace {
                parts.push(IrNode::Raw(self.consume()?.text));
            } else {
                break;
            }
        }

        // Parse the loop header in parens
        if !self.at(SyntaxKind::LParen) {
            // Try to recover - just return what we have
            return Some(IrNode::IdentBlock { parts });
        }

        let mut paren_depth = 0;
        loop {
            if self.at_eof() {
                break;
            }

            let kind = match self.current_kind() {
                Some(k) => k,
                None => break,
            };

            match kind {
                SyntaxKind::LParen => {
                    paren_depth += 1;
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
                SyntaxKind::RParen => {
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                    paren_depth -= 1;
                    if paren_depth == 0 {
                        break;
                    }
                }
                SyntaxKind::At => {
                    if let Some(placeholder) = self.parse_interpolation() {
                        // Check for identifier suffix
                        if let Some(token) = self.current() {
                            if token.kind == SyntaxKind::Ident {
                                let suffix = token.text.clone();
                                self.consume();
                                let ident_placeholder = match placeholder {
                                    IrNode::Placeholder { expr, .. } => {
                                        IrNode::Placeholder { kind: PlaceholderKind::Ident, expr }
                                    }
                                    other => other,
                                };
                                parts.push(IrNode::IdentBlock {
                                    parts: vec![ident_placeholder, IrNode::Raw(suffix)],
                                });
                                continue;
                            }
                        }
                        parts.push(placeholder);
                    }
                }
                SyntaxKind::DoubleQuote => {
                    if let Some(node) = self.parse_string_literal() {
                        parts.push(node);
                    }
                }
                SyntaxKind::Backtick => {
                    if let Some(node) = self.parse_template_literal() {
                        parts.push(node);
                    }
                }
                _ => {
                    if let Some(t) = self.consume() {
                        parts.push(IrNode::Raw(t.text));
                    }
                }
            }
        }

        // Skip whitespace before body
        while let Some(t) = self.current() {
            if t.kind == SyntaxKind::Whitespace {
                parts.push(IrNode::Raw(self.consume()?.text));
            } else {
                break;
            }
        }

        // Parse the body block
        if self.at(SyntaxKind::LBrace) {
            let mut brace_depth = 0;
            loop {
                if self.at_eof() {
                    break;
                }

                let kind = match self.current_kind() {
                    Some(k) => k,
                    None => break,
                };

                match kind {
                    SyntaxKind::LBrace => {
                        brace_depth += 1;
                        if let Some(t) = self.consume() {
                            parts.push(IrNode::Raw(t.text));
                        }
                    }
                    SyntaxKind::RBrace => {
                        if let Some(t) = self.consume() {
                            parts.push(IrNode::Raw(t.text));
                        }
                        brace_depth -= 1;
                        if brace_depth == 0 {
                            break;
                        }
                    }
                    SyntaxKind::At => {
                        if let Some(placeholder) = self.parse_interpolation() {
                            // Check for identifier suffix
                            if let Some(token) = self.current() {
                                if token.kind == SyntaxKind::Ident {
                                    let suffix = token.text.clone();
                                    self.consume();
                                    let ident_placeholder = match placeholder {
                                        IrNode::Placeholder { expr, .. } => {
                                            IrNode::Placeholder { kind: PlaceholderKind::Ident, expr }
                                        }
                                        other => other,
                                    };
                                    parts.push(IrNode::IdentBlock {
                                        parts: vec![ident_placeholder, IrNode::Raw(suffix)],
                                    });
                                    continue;
                                }
                            }
                            parts.push(placeholder);
                        }
                    }
                    SyntaxKind::DoubleQuote => {
                        if let Some(node) = self.parse_string_literal() {
                            parts.push(node);
                        }
                    }
                    SyntaxKind::Backtick => {
                        if let Some(node) = self.parse_template_literal() {
                            parts.push(node);
                        }
                    }
                    _ => {
                        if let Some(t) = self.consume() {
                            parts.push(IrNode::Raw(t.text));
                        }
                    }
                }
            }
        }

        // Merge adjacent Raw nodes
        let merged = Self::merge_adjacent_text(parts);

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG] parse_loop_as_raw: collected {} parts", merged.len());
            for (i, part) in merged.iter().enumerate() {
                match part {
                    IrNode::Raw(text) => eprintln!("  part[{}]: Raw({:?})", i, &text[..text.len().min(50)]),
                    IrNode::Placeholder { kind, .. } => eprintln!("  part[{}]: Placeholder({:?})", i, kind),
                    IrNode::IdentBlock { parts } => eprintln!("  part[{}]: IdentBlock({} parts)", i, parts.len()),
                    IrNode::StringInterp { .. } => eprintln!("  part[{}]: StringInterp", i),
                    other => eprintln!("  part[{}]: {:?}", i, std::mem::discriminant(other)),
                }
            }
        }

        // Return as TsLoopStmt which will be handled as a structured statement
        Some(IrNode::TsLoopStmt { parts: merged })
    }
}
