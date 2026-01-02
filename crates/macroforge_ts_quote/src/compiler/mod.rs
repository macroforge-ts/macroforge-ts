//! Compiler infrastructure for the template language.
//!
//! This module provides a clean compiler architecture:
//! - Lexer: Tokenizes input into a token stream (with normalization)
//! - Parser: Parses tokens directly into IR with inline placeholder classification
//! - Codegen: Generates Rust TokenStream output from IR

mod codegen;
mod error_fmt;
mod ir;
mod lexer;
mod parser;
mod syntax;
#[cfg(test)]
mod tests;

use codegen::{Codegen, CodegenConfig};
use parser::Parser;
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;

/// A span entry with line and column information for precise lookup.
#[derive(Clone, Copy)]
struct SpanEntry {
    /// Line number (1-indexed, from proc_macro2)
    line: usize,
    /// Start column (0-indexed)
    col_start: usize,
    /// End column (0-indexed, exclusive)
    col_end: usize,
    /// The proc_macro2 span
    span: Span,
}

/// Maps line/column positions to proc_macro2 Spans for precise error highlighting.
struct SpanMap {
    /// Span entries sorted by (line, col_start) for efficient lookup.
    entries: Vec<SpanEntry>,
    /// Fallback span when no entry matches.
    fallback: Span,
}

impl SpanMap {
    /// Build a SpanMap from a TokenStream using line/column information.
    fn from_token_stream(stream: &TokenStream) -> Self {
        let mut entries = Vec::new();
        let fallback = stream
            .clone()
            .into_iter()
            .next()
            .map(|t| t.span())
            .unwrap_or_else(Span::call_site);

        Self::collect_spans(stream, &mut entries);

        // Sort by (line, col_start) for binary search
        entries.sort_by(|a, b| {
            a.line.cmp(&b.line).then(a.col_start.cmp(&b.col_start))
        });

        Self { entries, fallback }
    }

    /// Recursively collect spans from the token stream.
    fn collect_spans(stream: &TokenStream, entries: &mut Vec<SpanEntry>) {
        for token in stream.clone() {
            let span = token.span();
            let start = span.start();
            let end = span.end();

            // Add entry for this token
            entries.push(SpanEntry {
                line: start.line,
                col_start: start.column,
                col_end: if end.line == start.line { end.column } else { usize::MAX },
                span,
            });

            // Recurse into groups
            if let TokenTree::Group(group) = token {
                // Add open delimiter span
                let open_span = group.span_open();
                let open_start = open_span.start();
                entries.push(SpanEntry {
                    line: open_start.line,
                    col_start: open_start.column,
                    col_end: open_start.column + 1,
                    span: open_span,
                });

                // Process contents
                Self::collect_spans(&group.stream(), entries);

                // Add close delimiter span
                let close_span = group.span_close();
                let close_start = close_span.start();
                entries.push(SpanEntry {
                    line: close_start.line,
                    col_start: close_start.column,
                    col_end: close_start.column + 1,
                    span: close_span,
                });
            }
        }
    }

    /// Find the span for a given line and column.
    /// Line is 1-indexed (matching proc_macro2), column is 1-indexed (matching SourceLocation).
    fn span_at(&self, line: usize, column: usize) -> Span {
        // Convert column to 0-indexed for comparison
        let col = column.saturating_sub(1);

        // Find entries on this line
        let line_start = self.entries.partition_point(|e| e.line < line);
        let line_end = self.entries.partition_point(|e| e.line <= line);

        if line_start >= line_end {
            // No entries on this line, find closest line before
            if line_start > 0 {
                return self.entries[line_start - 1].span;
            }
            return self.fallback;
        }

        // Find the entry containing this column, or closest before
        let line_entries = &self.entries[line_start..line_end];

        // First try to find an entry that contains the column
        for entry in line_entries {
            if col >= entry.col_start && col < entry.col_end {
                return entry.span;
            }
        }

        // Otherwise find the closest entry before this column
        let mut best = None;
        for entry in line_entries {
            if entry.col_start <= col {
                best = Some(entry.span);
            }
        }

        best.unwrap_or_else(|| {
            // Use first entry on this line
            line_entries.first().map(|e| e.span).unwrap_or(self.fallback)
        })
    }

    /// Find span for a line (for compatibility with line-offset based lookup).
    fn span_for_line(&self, line: usize) -> Span {
        self.span_at(line, 1)
    }
}

/// Compiles a template from a TokenStream (the macro input).
/// Handles position parsing (Top, Above, Within, Below, Bottom).
pub fn compile_template_tokens(input: TokenStream) -> syn::Result<TokenStream> {
    let parsed = parse_position(input)?;

    // Build span map from token stream for error highlighting
    let span_map = SpanMap::from_token_stream(&parsed.body);

    // Use original source text if available (preserves formatting and line numbers).
    // Fall back to to_string() which may alter whitespace/line structure.
    let has_source_text = parsed.source_text.is_some();
    let (template_str, line_offset) = if let Some(ref source) = parsed.source_text {
        // source_text includes the braces, so strip them
        let trimmed = source.trim();
        let inner = if trimmed.starts_with('{') && trimmed.ends_with('}') {
            trimmed[1..trimmed.len() - 1].to_string()
        } else {
            trimmed.to_string()
        };
        // Line offset: brace_line - 1 because template line 1 maps to file line brace_line + 1
        // but we add line_offset + template_line, so offset = brace_line - 1 + 1 = brace_line...
        // Actually: template line 1 is after the opening brace newline, so it's brace_line + 1
        // absolute = template_line + offset, we want: 1 + offset = brace_line + 1
        // Therefore: offset = brace_line
        // But we're getting +1, so try: offset = brace_line - 1
        (inner, parsed.brace_line.saturating_sub(1))
    } else {
        // Fall back to stringifying tokens
        let first_token_line = parsed
            .body
            .clone()
            .into_iter()
            .next()
            .map(|t| t.span().start().line)
            .unwrap_or(1);
        (parsed.body.to_string(), first_token_line.saturating_sub(1))
    };

    #[cfg(debug_assertions)]
    if std::env::var("MF_DEBUG_LINE").is_ok() {
        eprintln!("[MF_DEBUG] Brace line: {}", parsed.brace_line);
        eprintln!("[MF_DEBUG] Line offset: {}", line_offset);
        eprintln!("[MF_DEBUG] Has source_text: {}", has_source_text);
    }

    let position = parsed.position;

    #[cfg(debug_assertions)]
    if std::env::var("MF_DEBUG_TEMPLATE").is_ok() {
        eprintln!(
            "[MF_DEBUG] Template string ({} chars): {:?}",
            template_str.len(),
            template_str
        );
    }

    // For Within position, wrap in dummy class
    let template_str = if position == Some("Within") {
        format!("class __MF_DUMMY__ {{ {} }}", template_str)
    } else {
        template_str
    };

    compile_template_with_spans(&template_str, position, line_offset, &span_map)
}

/// Result of parsing position keyword - includes original source if available.
struct ParsedInput {
    position: Option<&'static str>,
    body: TokenStream,
    brace_line: usize,
    /// Original source text if available (preserves formatting)
    source_text: Option<String>,
}

/// Parse position keyword from input if present.
fn parse_position(input: TokenStream) -> syn::Result<ParsedInput> {
    let mut iter = input.clone().into_iter().peekable();

    // Check if first token is a position keyword
    if let Some(proc_macro2::TokenTree::Ident(ident)) = iter.peek() {
        let pos = match ident.to_string().as_str() {
            "Top" => Some("Top"),
            "Above" => Some("Above"),
            "Within" => Some("Within"),
            "Below" => Some("Below"),
            "Bottom" => Some("Bottom"),
            _ => None,
        };

        if pos.is_some() {
            iter.next(); // Consume the position ident

            let remaining: TokenStream = iter.collect();
            let mut remaining_iter = remaining.into_iter();

            if let Some(proc_macro2::TokenTree::Group(group)) = remaining_iter.next()
                && group.delimiter() == proc_macro2::Delimiter::Brace
            {
                // Get the line of the opening brace
                let brace_line = group.span_open().start().line;
                // Try to get original source text (preserves formatting)
                let source_text = group.span().source_text();
                return Ok(ParsedInput {
                    position: pos,
                    body: group.stream(),
                    brace_line,
                    source_text,
                });
            }

            return Err(syn::Error::new_spanned(
                input,
                "expected `{` after position keyword (e.g., `ts_template!(Within { ... })`)",
            ));
        }
    }

    // No position keyword - get brace line from first group if present
    let (brace_line, source_text) = input
        .clone()
        .into_iter()
        .find_map(|t| {
            if let proc_macro2::TokenTree::Group(g) = t {
                Some((g.span_open().start().line, g.span().source_text()))
            } else {
                None
            }
        })
        .unwrap_or((0, None));

    Ok(ParsedInput {
        position: None,
        body: input,
        brace_line,
        source_text,
    })
}

/// Compiles a template string into Rust code that produces a TsStream.
/// `line_offset` is added to error line numbers to show absolute file positions.
pub fn compile_template(template: &str, position: Option<&str>, line_offset: usize) -> syn::Result<TokenStream> {
    // Create a dummy span map for tests/CLI usage
    let dummy_span_map = SpanMap {
        entries: Vec::new(),
        fallback: Span::call_site(),
    };
    compile_template_with_spans(template, position, line_offset, &dummy_span_map)
}

/// Compiles a template string with a SpanMap for precise error highlighting.
fn compile_template_with_spans(
    template: &str,
    position: Option<&str>,
    line_offset: usize,
    span_map: &SpanMap,
) -> syn::Result<TokenStream> {
    use crate::compiler::parser::SourceLocation;

    if template.trim().is_empty() {
        let insert_pos = position_to_tokens(position);
        return Ok(quote! {
            macroforge_ts::ts_syn::TsStream::with_insert_pos(String::new(), #insert_pos)
        });
    }

    // Create parser (may fail with LexError)
    let parser = Parser::try_new(template).map_err(|e| {
        // Convert byte position to line/column, then look up precise span
        let loc = SourceLocation::from_offset(template, e.position);
        let absolute_line = loc.line + line_offset;
        let span = span_map.span_at(absolute_line, loc.column);
        syn::Error::new(
            span,
            e.format_with_source_and_file(template, "template", line_offset),
        )
    })?;

    // Parse to IR (may fail with ParseError)
    let ir = parser.parse().map_err(|e| {
        // Convert byte position to line/column, then look up precise span
        let loc = SourceLocation::from_offset(template, e.position);
        let absolute_line = loc.line + line_offset;
        let span = span_map.span_at(absolute_line, loc.column);
        syn::Error::new(
            span,
            e.format_with_source_and_file(template, "template", line_offset),
        )
    })?;

    // Generate code that builds Vec<ModuleItem>
    let config = CodegenConfig::default();
    let stmts_code = Codegen::with_config(config)
        .generate(&ir)
        .map_err(|e| {
            // Use span from error if available, otherwise fallback
            let span = if let Some(ir_span) = e.span {
                span_map.span_at(
                    SourceLocation::from_offset(template, ir_span.start).line + line_offset,
                    SourceLocation::from_offset(template, ir_span.start).column,
                )
            } else {
                span_map.fallback
            };
            syn::Error::new(span, e.format_with_source_and_file(template, "template", line_offset))
        })?;

    // Wrap in TsStream construction
    let insert_pos = position_to_tokens(position);
    let is_within = position == Some("Within");

    if is_within {
        Ok(quote! {
            {
                let __stmts: Vec<macroforge_ts::swc_core::ecma::ast::ModuleItem> = #stmts_code;
                let __comments = macroforge_ts::swc_core::common::comments::SingleThreadedComments::default();
                let __full_source = macroforge_ts::ts_syn::emit_module_items(&__stmts, &__comments);

                // Extract body from __MF_DUMMY__ wrapper
                let __body_source = {
                    let marker = "class __MF_DUMMY__";
                    if let Some(pos) = __full_source.find(marker) {
                        let after = &__full_source[pos + marker.len()..];
                        let after_trimmed = after.trim_start();
                        if after_trimmed.starts_with('{') {
                            let brace_offset = after.len() - after_trimmed.len();
                            let after_brace = &after[brace_offset + 1..];
                            if let Some(end) = after_brace.rfind('}') {
                                after_brace[..end].trim().to_string()
                            } else {
                                after_brace.trim().to_string()
                            }
                        } else {
                            after.trim().to_string()
                        }
                    } else {
                        __full_source.clone()
                    }
                };

                let __source = format!("/* @macroforge:body */{}", __body_source);
                macroforge_ts::ts_syn::TsStream::with_insert_pos(__source, #insert_pos)
            }
        })
    } else {
        Ok(quote! {
            {
                let __stmts: Vec<macroforge_ts::swc_core::ecma::ast::ModuleItem> = #stmts_code;
                let __comments = macroforge_ts::swc_core::common::comments::SingleThreadedComments::default();
                let __source = macroforge_ts::ts_syn::emit_module_items(&__stmts, &__comments);
                macroforge_ts::ts_syn::TsStream::with_insert_pos(__source, #insert_pos)
            }
        })
    }
}

fn position_to_tokens(position: Option<&str>) -> TokenStream {
    match position {
        Some("Top") => quote! { macroforge_ts::ts_syn::InsertPos::Top },
        Some("Above") => quote! { macroforge_ts::ts_syn::InsertPos::Above },
        Some("Within") => quote! { macroforge_ts::ts_syn::InsertPos::Within },
        Some("Bottom") => quote! { macroforge_ts::ts_syn::InsertPos::Bottom },
        _ => quote! { macroforge_ts::ts_syn::InsertPos::Below },
    }
}
