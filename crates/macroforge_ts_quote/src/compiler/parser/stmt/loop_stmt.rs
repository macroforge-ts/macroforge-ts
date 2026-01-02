use super::super::expr::errors::{ParseError, ParseErrorKind, ParseResult};
use super::super::*;

impl Parser {
    /// Parse a TypeScript for/while loop.
    /// For-in and for-of loops are parsed structurally as ForInStmt/ForOfStmt.
    /// C-style for loops and while loops are parsed as TsLoopStmt (raw text with placeholders).
    pub(in super::super) fn parse_ts_loop_stmt(&mut self) -> ParseResult<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        let keyword = self.current()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "loop statement"))?
            .text.clone();

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG] parse_ts_loop_stmt: keyword={:?}", keyword);
        }

        if keyword == "for" {
            // Try to parse as for-in or for-of first
            if let Some(structured) = self.try_parse_for_in_of()? {
                return Ok(structured);
            }
            // Fall back to raw parsing for C-style for loops
            return self.parse_loop_as_raw();
        }

        // While loops - parse as raw
        self.parse_loop_as_raw()
    }

    /// Try to parse a for-in or for-of loop.
    /// Returns Ok(None) if it's a C-style for loop (with semicolons).
    fn try_parse_for_in_of(&mut self) -> ParseResult<Option<IrNode>> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        // Save position for backtracking
        let start_pos = self.pos;

        self.consume(); // for
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
            return Ok(None);
        }
        self.consume(); // (
        self.skip_whitespace();

        // Parse the left-hand side (variable declaration or expression)
        let left = match self.parse_for_loop_left() {
            Ok(node) => node,
            Err(_) => {
                self.pos = start_pos;
                return Ok(None);
            }
        };

        self.skip_whitespace();

        // Check for 'in' or 'of' keyword
        let is_for_in = self.at(SyntaxKind::InKw);
        let is_for_of = self.at(SyntaxKind::OfKw);

        if !is_for_in && !is_for_of {
            // This is a C-style for loop - restore position
            self.pos = start_pos;
            return Ok(None);
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
        let right = self
            .parse_ts_expr_until(&[SyntaxKind::RParen])
            .map_err(|e| e.with_context("parsing for loop iterable"))?;

        self.skip_whitespace();
        self.expect(SyntaxKind::RParen);
        self.skip_whitespace();

        // Parse the body
        let body = if self.at(SyntaxKind::LBrace) {
            self.parse_block_stmt()
                .map_err(|e| e.with_context("parsing for loop body"))?
        } else {
            self.parse_stmt()
                .map_err(|e| e.with_context("parsing for loop body"))?
        };

        if is_for_in {
            Ok(Some(IrNode::ForInStmt {
                left: Box::new(left),
                right: Box::new(right),
                body: Box::new(body),
            }))
        } else {
            Ok(Some(IrNode::ForOfStmt {
                await_: has_await,
                left: Box::new(left),
                right: Box::new(right),
                body: Box::new(body),
            }))
        }
    }

    /// Parse the left-hand side of a for-in/for-of loop.
    /// This can be a variable declaration (const/let/var x) or an expression.
    fn parse_for_loop_left(&mut self) -> ParseResult<IrNode> {
        let kind = self
            .current_kind()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "for loop left-hand side"))?;

        match kind {
            SyntaxKind::ConstKw | SyntaxKind::LetKw | SyntaxKind::VarKw => {
                // Variable declaration
                let var_kind = match kind {
                    SyntaxKind::ConstKw => VarKind::Const,
                    SyntaxKind::LetKw => VarKind::Let,
                    SyntaxKind::VarKw => VarKind::Var,
                    _ => unreachable!(),
                };
                self.consume(); // const/let/var
                self.skip_whitespace();

                // Parse the binding pattern or identifier using expr/ implementation
                let name = self
                    .parse_for_loop_binding()
                    .map_err(|e| e.with_context("parsing for loop variable binding"))?;

                Ok(IrNode::VarDecl {
                    exported: false,
                    declare: false,
                    kind: var_kind,
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
                // Array destructuring pattern - use simple collection
                self.parse_for_loop_array_pattern()
            }
            SyntaxKind::LBrace => {
                // Object destructuring pattern - use simple collection
                self.parse_for_loop_object_pattern()
            }
            _ => {
                // Expression (for reassignment like: for (x in obj))
                self.parse_ts_expr_until(&[SyntaxKind::InKw, SyntaxKind::OfKw])
                    .map_err(|e| e.with_context("parsing for loop left-hand side expression"))
            }
        }
    }

    /// Parse a simple binding for for-loop left-hand side
    fn parse_for_loop_binding(&mut self) -> ParseResult<IrNode> {
        let kind = self
            .current_kind()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "for loop binding"))?;

        match kind {
            SyntaxKind::LBracket => self
                .parse_for_loop_array_pattern()
                .map_err(|e| e.with_context("parsing for loop array pattern")),
            SyntaxKind::LBrace => self
                .parse_for_loop_object_pattern()
                .map_err(|e| e.with_context("parsing for loop object pattern")),
            SyntaxKind::At => self.parse_interpolation(),
            _ => self.parse_ts_ident_or_placeholder().ok_or_else(|| {
                ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                    .with_context("parsing for loop binding identifier")
            }),
        }
    }

    /// Parse array pattern for for-loop: [a, b, ...rest]
    fn parse_for_loop_array_pattern(&mut self) -> ParseResult<IrNode> {
        self.consume(); // [
        self.skip_whitespace();

        let mut elems: Vec<IrNode> = Vec::new();

        while !self.at_eof() && !self.at(SyntaxKind::RBracket) {
            self.skip_whitespace();

            if self.at(SyntaxKind::Comma) {
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
                let arg = self
                    .parse_for_loop_binding()
                    .map_err(|e| e.with_context("parsing rest element in array pattern"))?;
                elems.push(IrNode::RestPat {
                    arg: Box::new(arg),
                    type_ann: None,
                });
            } else {
                let elem = self
                    .parse_for_loop_binding()
                    .map_err(|e| e.with_context("parsing element in array pattern"))?;
                elems.push(elem);
            }

            self.skip_whitespace();
            if self.at(SyntaxKind::Comma) {
                self.consume();
            }
        }

        self.expect(SyntaxKind::RBracket).ok_or_else(|| {
            ParseError::new(ParseErrorKind::MissingClosingBracket, self.current_byte_offset())
                .with_context("parsing array pattern")
        })?;

        Ok(IrNode::ArrayPat {
            elems: elems.into_iter().map(Some).collect(),
            type_ann: None,
            optional: false,
        })
    }

    /// Parse object pattern for for-loop: { a, b: c, ...rest }
    fn parse_for_loop_object_pattern(&mut self) -> ParseResult<IrNode> {
        self.consume(); // {
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
                let arg = self
                    .parse_for_loop_binding()
                    .map_err(|e| e.with_context("parsing rest element in object pattern"))?;
                props.push(IrNode::RestPat {
                    arg: Box::new(arg),
                    type_ann: None,
                });
            } else {
                // Regular property
                let key = self.parse_ts_ident_or_placeholder().ok_or_else(|| {
                    ParseError::new(ParseErrorKind::ExpectedIdentifier, self.current_byte_offset())
                        .with_context("parsing object pattern property key")
                })?;
                self.skip_whitespace();

                if self.at(SyntaxKind::Colon) {
                    // Renamed binding: { a: b }
                    self.consume();
                    self.skip_whitespace();
                    let value = self
                        .parse_for_loop_binding()
                        .map_err(|e| e.with_context("parsing object pattern property value"))?;
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

        self.expect(SyntaxKind::RBrace).ok_or_else(|| {
            ParseError::new(ParseErrorKind::MissingClosingBrace, self.current_byte_offset())
                .with_context("parsing object pattern")
        })?;

        Ok(IrNode::ObjectPat {
            props,
            type_ann: None,
            optional: false,
        })
    }

    /// Parse a loop as raw text with placeholders (fallback for C-style for and while loops).
    fn parse_loop_as_raw(&mut self) -> ParseResult<IrNode> {
        #[cfg(debug_assertions)]
        let debug_parser = std::env::var("MF_DEBUG_PARSER").is_ok();

        let keyword = self.consume()
            .ok_or_else(|| ParseError::unexpected_eof(self.current_byte_offset(), "loop keyword"))?
            .text;

        #[cfg(debug_assertions)]
        if debug_parser {
            eprintln!("[MF_DEBUG] parse_loop_as_raw: keyword={:?}", keyword);
        }
        let mut parts = vec![IrNode::Raw(keyword)];

        // Helper to add whitespace
        while let Some(t) = self.current() {
            if t.kind == SyntaxKind::Whitespace {
                if let Some(tok) = self.consume() {
                    parts.push(IrNode::Raw(tok.text));
                }
            } else {
                break;
            }
        }

        // Parse the loop header in parens
        if !self.at(SyntaxKind::LParen) {
            // Try to recover - just return what we have
            return Ok(IrNode::IdentBlock { parts });
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
                    let placeholder = self.parse_interpolation()?;
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
                SyntaxKind::DoubleQuote => {
                    // Use the new parse_string_literal from expr/primary.rs
                    let node = self.parse_string_literal()?;
                    parts.push(node);
                }
                SyntaxKind::Backtick => {
                    // Use the new parse_template_literal from expr/primary.rs
                    let node = self.parse_template_literal()?;
                    parts.push(node);
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
                if let Some(tok) = self.consume() {
                    parts.push(IrNode::Raw(tok.text));
                }
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
                        let placeholder = self.parse_interpolation()?;
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
                    SyntaxKind::DoubleQuote => {
                        let node = self.parse_string_literal()?;
                        parts.push(node);
                    }
                    SyntaxKind::Backtick => {
                        let node = self.parse_template_literal()?;
                        parts.push(node);
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
        Ok(IrNode::TsLoopStmt { parts: merged })
    }
}
