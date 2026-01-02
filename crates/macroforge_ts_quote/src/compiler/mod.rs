//! Compiler infrastructure for the template language.
//!
//! This module provides a clean compiler architecture:
//! - Lexer: Tokenizes input into a token stream (with normalization)
//! - Parser: Parses tokens directly into IR with inline placeholder classification
//! - Codegen: Generates Rust TokenStream output from IR

mod codegen;
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

/// Builds a span map from token positions to proc_macro2 Spans.
/// Maps line numbers to the spans of tokens on those lines for precise error highlighting.
struct SpanMap {
    /// Map from line number to a representative span on that line
    line_spans: std::collections::HashMap<usize, Span>,
    /// Fallback span covering the whole content
    fallback: Span,
}

impl SpanMap {
    /// Build a span map from a TokenStream.
    fn from_token_stream(stream: &TokenStream) -> Self {
        let mut line_spans = std::collections::HashMap::new();
        let fallback = stream
            .clone()
            .into_iter()
            .next()
            .map(|t| t.span())
            .unwrap_or_else(Span::call_site);

        Self::collect_line_spans(stream, &mut line_spans);

        Self { line_spans, fallback }
    }

    /// Recursively collect spans by line number
    fn collect_line_spans(stream: &TokenStream, line_spans: &mut std::collections::HashMap<usize, Span>) {
        for token in stream.clone() {
            let span = token.span();
            let line = span.start().line;
            // Only store the first span we see for each line
            line_spans.entry(line).or_insert(span);

            // Recurse into groups
            if let TokenTree::Group(group) = token {
                Self::collect_line_spans(&group.stream(), line_spans);
            }
        }
    }

    /// Find a span for the given absolute line number
    fn span_for_line(&self, line: usize) -> Span {
        self.line_spans.get(&line).copied().unwrap_or(self.fallback)
    }
}

/// Compiles a template from a TokenStream (the macro input).
/// Handles position parsing (Top, Above, Within, Below, Bottom).
pub fn compile_template_tokens(input: TokenStream) -> syn::Result<TokenStream> {
    let parsed = parse_position(input)?;

    // Build span map for precise error highlighting
    let span_map = SpanMap::from_token_stream(&parsed.body);

    // Use original source text if available (preserves formatting and line numbers).
    // Fall back to to_string() which may alter whitespace/line structure.
    let has_source_text = parsed.source_text.is_some();
    let (template_str, line_offset) = if let Some(source) = parsed.source_text {
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
        line_spans: std::collections::HashMap::new(),
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
        // Calculate the absolute line for this error
        let loc = SourceLocation::from_offset(template, e.position);
        let absolute_line = loc.line + line_offset;
        let span = span_map.span_for_line(absolute_line);
        syn::Error::new(
            span,
            e.format_with_source_and_file(template, "template", line_offset),
        )
    })?;

    // Parse to IR (may fail with ParseError)
    let ir = parser.parse().map_err(|e| {
        // Calculate the absolute line for this error
        let loc = SourceLocation::from_offset(template, e.position);
        let absolute_line = loc.line + line_offset;
        let span = span_map.span_for_line(absolute_line);
        syn::Error::new(
            span,
            e.format_with_source_and_file(template, "template", line_offset),
        )
    })?;

    // Generate code that builds Vec<ModuleItem>
    let config = CodegenConfig::default();
    let stmts_code = Codegen::with_config(config)
        .generate(&ir)
        .map_err(|e| syn::Error::new(span_map.fallback, e.to_string()))?;

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
