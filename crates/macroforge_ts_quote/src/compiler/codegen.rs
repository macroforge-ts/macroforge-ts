//! Code generation from IR to Rust TokenStream.
//!
//! This module generates Rust code that builds SWC AST at compile time.
//! It uses `macroforge_ts::macroforge_ts_quote::ts_quote!` for TypeScript parsing with native type support.
//!
//! ## Virtual Completion Strategy
//!
//! When control flow (`{#for}`, `{#if}`, etc.) appears inside a block (function body,
//! class body, etc.), the template gets split into incomplete chunks. For example:
//!
//! ```text
//! export function foo(): void {
//!     const x = 1;
//!     {#for item in items}
//!         console.log(item);
//!     {/for}
//!     return x;
//! }
//! ```
//!
//! This splits into:
//! 1. `export function foo(): void { const x = 1;` (incomplete - missing `}`)
//! 2. `console.log(item);` (the loop body)
//! 3. `return x; }` (incomplete - orphan `}`)
//!
//! To handle this, we use "virtual completion":
//! - Track brace depth to detect incomplete chunks
//! - Add virtual closing braces for SWC quote! validation
//! - Generate runtime code that builds proper statement lists
//! - Assemble the final AST by combining validated pieces

use super::ir::{Ir, IrNode};
use super::semantic::PlaceholderKind;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::cell::Cell;

/// A placeholder entry: (name, kind, rust_expr)
type PlaceholderEntry = (String, PlaceholderKind, String);

/// Debug logging for template transformations.
/// Writes .ts files to the logs directory showing both original and transformed templates.
#[cfg(debug_assertions)]
mod debug_log {
    use std::io::Write;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static LOG_COUNTER: AtomicUsize = AtomicUsize::new(0);

    /// Logs a template transformation to a .ts file for debugging.
    pub fn log_template_transform(
        original_template: &str,
        transformed_template: &str,
        parse_context: &str,
        brace_balance: &str,
        transform_type: &str, // e.g., "virtual_completion_closer", "balanced_split", etc.
        placeholders: &[(String, super::PlaceholderKind, String)],
    ) {
        if std::env::var("MF_DEBUG_CODEGEN").is_err() {
            return;
        }

        let count = LOG_COUNTER.fetch_add(1, Ordering::SeqCst);
        let logs_dir = format!("{}/logs", env!("CARGO_MANIFEST_DIR"));
        let _ = std::fs::create_dir_all(&logs_dir);

        let filename = format!("{}/transform_{:04}_{}.ts", logs_dir, count, transform_type);

        if let Ok(mut file) = std::fs::File::create(&filename) {
            let _ = writeln!(
                file,
                "// ============================================================"
            );
            let _ = writeln!(file, "// TEMPLATE TRANSFORMATION LOG #{}", count);
            let _ = writeln!(
                file,
                "// ============================================================"
            );
            let _ = writeln!(file, "// Transform Type: {}", transform_type);
            let _ = writeln!(file, "// Parse Context: {}", parse_context);
            let _ = writeln!(file, "// Brace Balance: {}", brace_balance);
            let _ = writeln!(file, "//");
            let _ = writeln!(file, "// Placeholders:");
            for (name, kind, expr) in placeholders {
                let _ = writeln!(file, "//   ${}: {:?} = {}", name, kind, expr);
            }
            let _ = writeln!(
                file,
                "// ============================================================"
            );
            let _ = writeln!(file);
            let _ = writeln!(file, "// ORIGINAL TEMPLATE:");
            let _ = writeln!(file, "// -------------------");
            for line in original_template.lines() {
                let _ = writeln!(file, "// {}", line);
            }
            let _ = writeln!(file);
            let _ = writeln!(file, "// TRANSFORMED (passed to ts_quote!):");
            let _ = writeln!(file, "// ------------------------------------");
            let _ = writeln!(file, "{}", transformed_template);
        }
    }
}

/// Parse context - determines what AST node type to parse as.
///
/// When template code is split by control flow, each chunk needs to know
/// what context it's in to parse correctly. For example, code inside an
/// object literal should parse as `PropOrSpread`, not `Stmt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParseContext {
    /// Top level module - expects ModuleItem (export, import, statements)
    #[default]
    Module,
    /// Function/method body - expects Stmt
    FunctionBody,
    /// Object literal `{ key: value }` - expects PropOrSpread
    ObjectLiteral,
    /// Array literal `[ elem, elem ]` - expects Expr (array element)
    ArrayLiteral,
    /// Class body - expects ClassMember
    ClassBody,
    /// Type object literal `{ prop: Type }` - expects TsPropertySignature
    TypeObjectLiteral,
}

// ============================================================================
// CONTEXT-FIRST ARCHITECTURE
// ============================================================================
//
// The core principle: Context is the primary organizing structure.
// Brace "balance" is implicit in the context stack depth.
//
// Every operation is either:
// 1. Push context (on `{` or `[`)
// 2. Pop context (on `}` or `]`)
// 3. Match on current context to decide handling
//
// ============================================================================

/// A span of template text with its context and depth information.
#[derive(Debug, Clone)]
struct ContextSpan {
    /// The text content of this span
    text: String,
    /// The context this span should be parsed in
    context: ParseContext,
    /// Depth relative to starting context:
    /// - negative = closer (we've popped contexts)
    /// - zero = same level as start
    /// - positive = opener (we've pushed contexts)
    start_depth: i32,
    /// Depth at end of this span
    end_depth: i32,
    /// Minimum depth reached during this span (for detecting middle spans)
    min_depth: i32,
    /// Byte offset in original template where this span starts
    start_offset: usize,
    /// Byte offset in original template where this span ends
    end_offset: usize,
    /// Actual count of opening braces `{` in this span (not limited by context stack)
    actual_opens: i32,
    /// Actual count of closing braces `}` in this span (not limited by context stack)
    actual_closes: i32,
}

impl ContextSpan {
    /// Returns true if this span closes contexts (ends shallower than it starts)
    fn is_closer(&self) -> bool {
        self.end_depth < self.start_depth
    }

    /// Returns true if this span opens contexts (ends deeper than it starts)
    fn is_opener(&self) -> bool {
        self.end_depth > self.start_depth
    }

    /// Returns true if this span is balanced (same depth at start and end)
    fn is_balanced(&self) -> bool {
        self.end_depth == self.start_depth
    }

    /// Returns true if this span is a "middle" span - closes then opens contexts,
    /// ending at the same depth but going below start during the span.
    /// Example: `} foo {` starts at 0, goes to -1, ends at 0.
    /// Middle spans are a subset of balanced spans that need special handling.
    fn is_middle(&self) -> bool {
        self.end_depth == self.start_depth && self.min_depth < self.start_depth
    }

    /// Number of contexts closed by this span
    fn closes_count(&self) -> i32 {
        (self.start_depth - self.end_depth).max(0)
    }

    /// Number of contexts opened by this span
    fn opens_count(&self) -> i32 {
        (self.end_depth - self.start_depth).max(0)
    }

    /// For middle spans, get the number of unmatched closes at the start.
    /// This is how far below the start depth we went.
    fn middle_closes_count(&self) -> i32 {
        (self.start_depth - self.min_depth).max(0)
    }

    /// For middle spans, get the number of unmatched opens at the end.
    /// This is how far we recovered from the minimum depth.
    fn middle_opens_count(&self) -> i32 {
        (self.end_depth - self.min_depth).max(0)
    }

    /// Actual brace imbalance (opens - closes).
    /// Positive = more opens than closes (opener)
    /// Negative = more closes than opens (closer)
    fn actual_brace_imbalance(&self) -> i32 {
        self.actual_opens - self.actual_closes
    }

    /// Number of actual closing braces that exceed opens (for virtual completion).
    /// This gives the real count needed for virtual completion, not limited by context stack.
    fn actual_excess_closes(&self) -> i32 {
        (self.actual_closes - self.actual_opens).max(0)
    }

    /// Number of actual opening braces that exceed closes (for virtual completion).
    fn actual_excess_opens(&self) -> i32 {
        (self.actual_opens - self.actual_closes).max(0)
    }
}

/// Context stack that tracks where we are in the template.
///
/// This is the PRIMARY data structure for template processing.
/// Brace balance is implicit in stack depth relative to starting depth.
#[derive(Debug, Clone)]
struct ContextStack {
    /// Stack of contexts, with Module at the bottom
    stack: Vec<ParseContext>,
    /// The depth when we started (to compute relative depth)
    starting_depth: usize,
}

impl ContextStack {
    /// Create a new context stack starting in the given context.
    fn new(starting_context: ParseContext) -> Self {
        let mut stack = vec![ParseContext::Module];
        if starting_context != ParseContext::Module {
            stack.push(starting_context);
        }
        let starting_depth = stack.len();
        Self {
            stack,
            starting_depth,
        }
    }

    /// Get the current (innermost) context.
    fn current(&self) -> ParseContext {
        *self.stack.last().unwrap_or(&ParseContext::Module)
    }

    /// Push a new context onto the stack.
    fn push(&mut self, ctx: ParseContext) {
        self.stack.push(ctx);
    }

    /// Pop a context from the stack. Returns the popped context.
    /// Never pops below Module level.
    fn pop(&mut self) -> Option<ParseContext> {
        if self.stack.len() > 1 {
            self.stack.pop()
        } else {
            None
        }
    }

    /// Depth relative to starting depth.
    /// - Negative = we've closed more than we've opened (closer)
    /// - Zero = same depth as start
    /// - Positive = we've opened more than we've closed (opener)
    fn relative_depth(&self) -> i32 {
        self.stack.len() as i32 - self.starting_depth as i32
    }

    /// Returns true if we're at module level (stack only has Module)
    fn at_module_level(&self) -> bool {
        self.stack.len() == 1
    }

    /// Scan a template and split it into context-aware spans.
    ///
    /// This is the core algorithm: we scan character by character,
    /// pushing/popping context on braces, and split whenever we
    /// transition to module level with module-level content following.
    fn scan_template(&mut self, template: &str) -> Vec<ContextSpan> {
        let mut spans = Vec::new();
        let mut current_span_start = 0;
        let mut current_span_start_depth = self.relative_depth();
        // Track the minimum depth reached during this span (for detecting middle spans)
        let mut current_span_min_depth = current_span_start_depth;
        // Track the actual context at the start of the current span
        let mut current_span_context = self.current();
        // Track actual brace counts for this span (not limited by context stack)
        let mut current_span_opens: i32 = 0;
        let mut current_span_closes: i32 = 0;

        let mut in_string = false;
        let mut string_char = '"';
        let mut escape_next = false;

        let chars: Vec<char> = template.chars().collect();
        let len = chars.len();
        let mut byte_pos = 0;
        let mut i = 0;

        while i < len {
            let ch = chars[i];
            let char_byte_len = ch.len_utf8();

            if escape_next {
                escape_next = false;
                byte_pos += char_byte_len;
                i += 1;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                byte_pos += char_byte_len;
                i += 1;
                continue;
            }

            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                byte_pos += char_byte_len;
                i += 1;
                continue;
            }

            match ch {
                '"' | '\'' | '`' => {
                    in_string = true;
                    string_char = ch;
                }
                '{' => {
                    // Track actual brace count
                    current_span_opens += 1;
                    // Classify what kind of context this brace opens
                    let new_context = Self::classify_open_brace(&chars, i);
                    self.push(new_context);
                }
                '}' => {
                    // Track actual brace count
                    current_span_closes += 1;
                    // Check if we're about to pop BELOW starting depth (to actual Module level)
                    let was_at_depth = self.relative_depth();
                    self.pop();
                    let now_at_depth = self.relative_depth();

                    // Track minimum depth for this span
                    if now_at_depth < current_span_min_depth {
                        current_span_min_depth = now_at_depth;
                    }

                    // Only split if we've gone BELOW starting depth (negative relative depth)
                    // This means we've closed more braces than we've opened relative to start
                    // AND we're now actually at module level in the stack
                    if now_at_depth < 0 && was_at_depth >= 0 && self.at_module_level() {
                        // Look ahead for module-level content
                        let remaining = &template[byte_pos + char_byte_len..];
                        if Self::has_module_level_content(remaining) {
                            // Split here! End current span after the `}`
                            let span_end = byte_pos + char_byte_len;
                            if span_end > current_span_start {
                                spans.push(ContextSpan {
                                    text: template[current_span_start..span_end].to_string(),
                                    context: current_span_context,
                                    start_depth: current_span_start_depth,
                                    end_depth: now_at_depth,
                                    min_depth: current_span_min_depth,
                                    start_offset: current_span_start,
                                    end_offset: span_end,
                                    actual_opens: current_span_opens,
                                    actual_closes: current_span_closes,
                                });
                            }
                            // Start new span at module level
                            current_span_start = span_end;
                            current_span_start_depth = now_at_depth;
                            current_span_min_depth = now_at_depth;
                            current_span_context = ParseContext::Module;
                            current_span_opens = 0;
                            current_span_closes = 0;
                        }
                    }
                }
                '[' => {
                    self.push(ParseContext::ArrayLiteral);
                }
                ']' => {
                    if self.current() == ParseContext::ArrayLiteral {
                        self.pop();
                    }
                }
                _ => {}
            }

            byte_pos += char_byte_len;
            i += 1;
        }

        // Push final span using the tracked context
        if byte_pos > current_span_start || spans.is_empty() {
            spans.push(ContextSpan {
                text: template[current_span_start..].to_string(),
                context: current_span_context,
                start_depth: current_span_start_depth,
                end_depth: self.relative_depth(),
                min_depth: current_span_min_depth,
                start_offset: current_span_start,
                end_offset: template.len(),
                actual_opens: current_span_opens,
                actual_closes: current_span_closes,
            });
        }

        spans
    }

    /// Classify what context an opening brace introduces.
    fn classify_open_brace(chars: &[char], brace_pos: usize) -> ParseContext {
        // Look backwards to determine context
        let before: String = chars[..brace_pos].iter().collect();
        let trimmed = before.trim_end();

        // Type object literal: `type X = {`, `export type X = {`
        // Must check BEFORE ObjectLiteral since both match `ends_with('=')`
        if trimmed.ends_with('=') && trimmed.contains("type ") {
            return ParseContext::TypeObjectLiteral;
        }

        // Interface body: `interface X {`, `export interface X {`
        if trimmed.contains("interface ") {
            // Check that we're not inside nested braces of the interface
            let after_interface = trimmed.rsplit("interface ").next().unwrap_or("");
            if !after_interface.contains('{') {
                return ParseContext::TypeObjectLiteral;
            }
        }

        // Object literal patterns: `= {`, `return {`, `: {`, `( {`, `[ {`, `, {`
        if trimmed.ends_with('=')
            || trimmed.ends_with("return")
            || trimmed.ends_with(':')
            || trimmed.ends_with('(')
            || trimmed.ends_with('[')
            || trimmed.ends_with(',')
            || trimmed.ends_with("=>")
        {
            return ParseContext::ObjectLiteral;
        }

        // Class body: `class X {`, `class X extends Y {`
        if let Some(class_pos) = trimmed.rfind("class ") {
            let after_class = &trimmed[class_pos..];
            if !after_class.contains('{') {
                return ParseContext::ClassBody;
            }
        }

        // Function body: `function`, `) {`, `=> {` (arrow already handled above for objects)
        if trimmed.ends_with(')') || trimmed.contains("function ") || trimmed.contains("function(")
        {
            return ParseContext::FunctionBody;
        }

        // Default to function body for blocks like `if {`, `else {`, `for {`
        ParseContext::FunctionBody
    }

    /// Check if the remaining text starts with module-level content.
    fn has_module_level_content(text: &str) -> bool {
        let trimmed = text.trim_start();
        trimmed.starts_with("export ")
            || trimmed.starts_with("import ")
            || trimmed.starts_with("function ")
            || trimmed.starts_with("class ")
            || trimmed.starts_with("interface ")
            || trimmed.starts_with("type ")
            || trimmed.starts_with("enum ")
            || trimmed.starts_with("const ")
            || trimmed.starts_with("let ")
            || trimmed.starts_with("var ")
            || trimmed.starts_with("declare ")
            || trimmed.starts_with("abstract ")
            || trimmed.starts_with("async function")
            || trimmed.starts_with("namespace ")
    }
}

impl ParseContext {
    /// Returns the ts_quote! type name for this context.
    fn quote_type(&self) -> &'static str {
        match self {
            ParseContext::Module => "ModuleItem",
            ParseContext::FunctionBody => "Stmt",
            ParseContext::ObjectLiteral => "PropOrSpread",
            ParseContext::ArrayLiteral => "Expr",
            ParseContext::ClassBody => "ModuleItem", // ClassMember wrapped in Stmt
            ParseContext::TypeObjectLiteral => "PropOrSpread", // Simplified
        }
    }

    /// Returns the output variable suffix for this context.
    fn output_suffix(&self) -> &'static str {
        match self {
            ParseContext::Module => "",
            ParseContext::FunctionBody => "",
            ParseContext::ObjectLiteral => "_props",
            ParseContext::ArrayLiteral => "_elems",
            ParseContext::ClassBody => "_members",
            ParseContext::TypeObjectLiteral => "_type_props",
        }
    }

    /// Returns true if this context produces ModuleItem directly.
    fn is_module_item(&self) -> bool {
        matches!(self, ParseContext::Module | ParseContext::ClassBody)
    }

    /// Returns true if this context produces statements.
    fn is_statement(&self) -> bool {
        matches!(self, ParseContext::FunctionBody)
    }

    /// Returns true if this context is a non-statement context that needs a collector.
    fn is_non_statement_context(&self) -> bool {
        matches!(
            self,
            ParseContext::ObjectLiteral
                | ParseContext::ArrayLiteral
                | ParseContext::ClassBody
                | ParseContext::TypeObjectLiteral
        )
    }

    /// Returns the collector type ID for this context.
    /// 0=ObjectLiteral, 1=ArrayLiteral, 2=ClassBody, 3=TypeObjectLiteral, _=Stmts
    fn collector_type(&self) -> u8 {
        match self {
            ParseContext::ObjectLiteral => 0,
            ParseContext::ArrayLiteral => 1,
            ParseContext::ClassBody => 2,
            ParseContext::TypeObjectLiteral => 3,
            ParseContext::Module | ParseContext::FunctionBody => 255,
        }
    }
}

/// Tracks context transitions in a template string.
///
/// Analyzes patterns like `return {`, `= [`, `class Foo {` to determine
/// what context each unclosed brace introduces.
#[derive(Debug, Clone)]
struct ContextAnalysis {
    /// The context that should be used for content after this chunk.
    /// This is the innermost unclosed context.
    inner_context: ParseContext,
    /// Stack of contexts introduced by unclosed braces in this chunk.
    /// Used to properly close contexts when matching `}` or `]` are found.
    context_stack: Vec<ParseContext>,
    /// Number of closes that escaped beyond the starting context.
    /// When this is > 0, we've returned to Module level.
    escaped_closes: i32,
}

impl Default for ContextAnalysis {
    fn default() -> Self {
        Self {
            inner_context: ParseContext::Module,
            context_stack: Vec::new(),
            escaped_closes: 0,
        }
    }
}

impl ContextAnalysis {
    /// Analyzes a template string to determine context transitions.
    ///
    /// Detects patterns like:
    /// - `return {` or `= {` → ObjectLiteral
    /// - `[` → ArrayLiteral
    /// - `function foo() {` or `=> {` → FunctionBody
    /// - `class Foo {` → ClassBody
    ///
    /// Also tracks when closes escape beyond the starting context,
    /// which means we've returned to Module level.
    fn analyze(template: &str, starting_context: ParseContext) -> Self {
        // Start with Module as the base, then push the starting context on top
        // This allows us to properly pop back to Module when closes escape
        let mut context_stack: Vec<ParseContext> = vec![ParseContext::Module];
        if starting_context != ParseContext::Module {
            context_stack.push(starting_context);
        }

        let mut in_string = false;
        let mut string_char = '"';
        let mut escape_next = false;
        let mut escaped_closes: i32 = 0;

        let chars: Vec<char> = template.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            let ch = chars[i];

            if escape_next {
                escape_next = false;
                i += 1;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                i += 1;
                continue;
            }

            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                i += 1;
                continue;
            }

            match ch {
                '"' | '\'' | '`' => {
                    in_string = true;
                    string_char = ch;
                }
                '{' => {
                    // Determine what kind of context this brace opens
                    let context = Self::classify_open_brace(&chars, i);
                    context_stack.push(context);
                }
                '}' => {
                    // Pop the context, but track if we're escaping beyond the starting context
                    if context_stack.len() > 1 {
                        context_stack.pop();
                    } else {
                        // We're trying to close beyond Module - this is an escaped close
                        escaped_closes += 1;
                    }
                }
                '[' => {
                    // Array literal context
                    context_stack.push(ParseContext::ArrayLiteral);
                }
                ']' => {
                    // Pop array context if that's what we're in
                    if context_stack.last() == Some(&ParseContext::ArrayLiteral) {
                        context_stack.pop();
                    }
                }
                _ => {}
            }
            i += 1;
        }

        let inner_context = *context_stack.last().unwrap_or(&ParseContext::Module);

        Self {
            inner_context,
            context_stack,
            escaped_closes,
        }
    }

    /// Finds where a template transitions to module level and has module-level content.
    ///
    /// This is used for templates that are "balanced" in brace count but transition
    /// from FunctionBody to Module level with subsequent module declarations.
    ///
    /// Returns (split_pos, before_split, after_split) if found.
    fn find_module_level_split(
        template: &str,
        starting_context: ParseContext,
    ) -> Option<(usize, String, String)> {
        // Only relevant when starting inside a function body
        if starting_context == ParseContext::Module {
            return None;
        }

        let mut in_string = false;
        let mut string_char = '"';
        let mut escape_next = false;
        // Track depth relative to starting context (1 = inside function, 0 = at module level)
        let mut depth: i32 = 1;
        let mut last_module_level_pos: Option<usize> = None;

        let chars: Vec<char> = template.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            let ch = chars[i];

            if escape_next {
                escape_next = false;
                i += 1;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                i += 1;
                continue;
            }

            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                i += 1;
                continue;
            }

            match ch {
                '"' | '\'' | '`' => {
                    in_string = true;
                    string_char = ch;
                }
                '{' => {
                    depth += 1;
                }
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        // We've reached module level - remember this position (after the `}`)
                        last_module_level_pos = Some(i + 1);
                    }
                }
                _ => {}
            }
            i += 1;
        }

        // If we reached module level at some point, check if there's module-level content after
        if let Some(pos) = last_module_level_pos {
            let remaining = &template[pos..];
            let remaining_trimmed = remaining.trim_start();

            // Check if remaining content is module-level
            let is_module_level = remaining_trimmed.starts_with("export ")
                || remaining_trimmed.starts_with("import ")
                || remaining_trimmed.starts_with("function ")
                || remaining_trimmed.starts_with("class ")
                || remaining_trimmed.starts_with("interface ")
                || remaining_trimmed.starts_with("type ")
                || remaining_trimmed.starts_with("enum ")
                || remaining_trimmed.starts_with("const ")
                || remaining_trimmed.starts_with("let ")
                || remaining_trimmed.starts_with("var ")
                || remaining_trimmed.starts_with("declare ")
                || remaining_trimmed.starts_with("abstract ")
                || remaining_trimmed.starts_with("async function")
                || remaining_trimmed.starts_with("namespace ");

            if is_module_level && !remaining_trimmed.is_empty() {
                return Some((
                    pos,
                    template[..pos].to_string(),
                    template[pos..].to_string(),
                ));
            }
        }

        None
    }

    /// Classifies what context an opening brace `{` introduces.
    ///
    /// Looks at preceding tokens to determine:
    /// - TypeObjectLiteral: after `type X =` or `interface X`
    /// - ObjectLiteral: after `return`, `=`, `(`, `:`, `,`
    /// - FunctionBody: after `)` with function signature, or `=>`
    /// - ClassBody: after `class Name` or `extends Name`
    fn classify_open_brace(chars: &[char], brace_pos: usize) -> ParseContext {
        // Look backwards to find what precedes the brace
        let before: String = chars[..brace_pos].iter().collect();
        let trimmed = before.trim_end();

        // Check for type literal: `type X = {` or `export type X = {`
        // Must check BEFORE ObjectLiteral since both match ends_with('=')
        if trimmed.ends_with('=') && trimmed.contains("type ") {
            return ParseContext::TypeObjectLiteral;
        }

        // Check for interface: `interface X {` or `export interface X {`
        if trimmed.contains("interface ") {
            let after_interface = trimmed.rsplit("interface ").next().unwrap_or("");
            if !after_interface.contains('{') {
                return ParseContext::TypeObjectLiteral;
            }
        }

        // Check for arrow function: `=> {`
        if trimmed.ends_with("=>") {
            return ParseContext::FunctionBody;
        }

        // Check for function/method: `) {` after a parameter list
        if trimmed.ends_with(')') {
            // Could be function, method, if, for, while, etc.
            // Look for function-like keywords
            let lower = trimmed.to_lowercase();
            if lower.contains("function ")
                || lower.contains("function(")
                || Self::looks_like_method_signature(trimmed)
            {
                return ParseContext::FunctionBody;
            }
            // Control flow statements also use FunctionBody context
            if lower.ends_with("if") || lower.ends_with("for") || lower.ends_with("while") {
                return ParseContext::FunctionBody;
            }
            // Default: assume it's a block statement
            return ParseContext::FunctionBody;
        }

        // Check for class: `class Name {` or `extends Name {`
        let words: Vec<&str> = trimmed.split_whitespace().collect();
        if words.len() >= 2 {
            let second_last = words.get(words.len().saturating_sub(2));
            if second_last == Some(&"class")
                || second_last == Some(&"extends")
                || second_last == Some(&"implements")
            {
                return ParseContext::ClassBody;
            }
        }

        // Check for object literal patterns
        let last_non_ws = trimmed.chars().last();
        match last_non_ws {
            // After these, `{` is an object literal
            Some('=' | ':' | ',' | '(' | '[' | '?') => ParseContext::ObjectLiteral,
            // `return {` is object literal
            _ if trimmed.ends_with("return") => ParseContext::ObjectLiteral,
            // `yield {` is object literal
            _ if trimmed.ends_with("yield") => ParseContext::ObjectLiteral,
            // Default: assume function body (block statement)
            _ => ParseContext::FunctionBody,
        }
    }

    /// Checks if the string looks like a method signature.
    fn looks_like_method_signature(s: &str) -> bool {
        // Simple heuristic: contains a method name followed by (
        // e.g., `foo()`, `getValue()`, `async process()`
        let trimmed = s.trim();
        if let Some(paren_pos) = trimmed.rfind('(') {
            let before_paren = &trimmed[..paren_pos];
            let last_word = before_paren.split_whitespace().last().unwrap_or("");
            // If last word before ( is an identifier, it's likely a method
            !last_word.is_empty() && last_word.chars().all(|c| c.is_alphanumeric() || c == '_')
        } else {
            false
        }
    }
}

/// Configuration for code generation.
#[derive(Debug, Clone)]
pub struct CodegenConfig {
    /// Variable name for the output accumulator (Vec<ModuleItem>).
    pub output_var: String,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self {
            output_var: "__mf_stmts".to_string(),
        }
    }
}

/// Brace balance information for a template chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BraceBalance {
    /// Net change in brace depth (positive = more opens, negative = more closes)
    net_change: i32,
    /// Number of unmatched opening braces
    unclosed_opens: i32,
    /// Number of unmatched closing braces
    unmatched_closes: i32,
}

impl BraceBalance {
    /// Analyzes a template string to determine brace balance.
    fn analyze(template: &str) -> Self {
        let mut depth: i32 = 0;
        let mut min_depth: i32 = 0;
        let mut in_string = false;
        let mut string_char = '"';
        let mut escape_next = false;

        for ch in template.chars() {
            if escape_next {
                escape_next = false;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                continue;
            }

            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                continue;
            }

            match ch {
                '"' | '\'' | '`' => {
                    in_string = true;
                    string_char = ch;
                }
                '{' => {
                    depth += 1;
                }
                '}' => {
                    depth -= 1;
                    if depth < min_depth {
                        min_depth = depth;
                    }
                }
                _ => {}
            }
        }

        // unclosed_opens = how many `{` don't have matching `}`
        // unmatched_closes = how many `}` don't have matching `{`
        let unmatched_closes = (-min_depth).max(0);
        let unclosed_opens = (depth - min_depth).max(0);

        Self {
            net_change: depth,
            unclosed_opens,
            unmatched_closes,
        }
    }

    /// Returns true if the template has balanced braces.
    fn is_balanced(&self) -> bool {
        self.net_change == 0 && self.unclosed_opens == 0 && self.unmatched_closes == 0
    }

    /// Finds the position where unmatched closes are consumed in the template.
    /// Returns `Some((split_pos, closer_part, remaining_part))` if the template can be
    /// split into a closer part (the closes) and a remaining balanced MODULE-LEVEL part.
    /// Returns `None` if there's no balanced module-level content after the closes.
    fn find_closer_split(template: &str, unmatched_closes: i32) -> Option<(usize, String, String)> {
        if unmatched_closes <= 0 {
            return None;
        }

        let mut depth: i32 = 0;
        let mut closes_seen: i32 = 0;
        let mut in_string = false;
        let mut string_char = '"';
        let mut escape_next = false;
        let mut split_pos: Option<usize> = None;

        let chars: Vec<char> = template.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            let ch = chars[i];

            if escape_next {
                escape_next = false;
                i += 1;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                i += 1;
                continue;
            }

            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                i += 1;
                continue;
            }

            match ch {
                '"' | '\'' | '`' => {
                    in_string = true;
                    string_char = ch;
                }
                '{' => {
                    depth += 1;
                }
                '}' => {
                    if depth > 0 {
                        depth -= 1;
                    } else {
                        closes_seen += 1;
                        if closes_seen >= unmatched_closes && split_pos.is_none() {
                            // Found where all unmatched closes are consumed
                            split_pos = Some(i + 1);
                        }
                    }
                }
                _ => {}
            }
            i += 1;
        }

        // Check if we found a split point and remaining content is balanced MODULE-LEVEL code
        if let Some(pos) = split_pos {
            let remaining = &template[pos..];
            let remaining_trimmed = remaining.trim_start();

            // Only split if the remaining content is MODULE-LEVEL (export/import/function/class/etc.)
            // NOT if it's statement-level like `else`, `catch`, operators, etc.
            let is_module_level = remaining_trimmed.starts_with("export ")
                || remaining_trimmed.starts_with("import ")
                || remaining_trimmed.starts_with("function ")
                || remaining_trimmed.starts_with("class ")
                || remaining_trimmed.starts_with("interface ")
                || remaining_trimmed.starts_with("type ")
                || remaining_trimmed.starts_with("enum ")
                || remaining_trimmed.starts_with("const ")
                || remaining_trimmed.starts_with("let ")
                || remaining_trimmed.starts_with("var ")
                || remaining_trimmed.starts_with("declare ")
                || remaining_trimmed.starts_with("abstract ");

            if !is_module_level {
                return None;
            }

            let remaining_balance = Self::analyze(remaining);

            // If remaining content is balanced, we can split here
            if remaining_balance.unmatched_closes == 0 && remaining_balance.unclosed_opens == 0 {
                let closer_part = template[..pos].to_string();
                let remaining_part = remaining.to_string();

                // Only split if there's actual content in the remaining part
                if !remaining_part.trim().is_empty() {
                    return Some((pos, closer_part, remaining_part));
                }
            }
        }

        None
    }
}

/// A chunk of template content that can be processed together.
#[derive(Debug)]
enum Chunk<'a> {
    /// A parseable chunk of static text + placeholders.
    /// Can be compiled with macroforge_ts::macroforge_ts_quote::ts_quote!.
    Parseable {
        /// Template string with $placeholder markers.
        template: String,
        /// Placeholder info: (placeholder_name, kind, rust_expr)
        placeholders: Vec<(String, PlaceholderKind, String)>,
        /// Brace balance for virtual completion.
        brace_balance: BraceBalance,
        /// The parse context for this chunk (what AST type to parse as).
        parse_context: ParseContext,
        /// Context for content that follows (after unclosed braces).
        inner_context: ParseContext,
    },
    /// Control flow node that needs special handling.
    /// Includes the context that should be used for the loop/if body.
    ControlFlow {
        node: &'a IrNode,
        /// The context inherited from the preceding chunk.
        body_context: ParseContext,
    },
    /// Directive that generates Rust code directly.
    Directive {
        node: &'a IrNode,
        context: ParseContext,
    },
    /// String interpolation for template literals.
    StringInterp {
        quote_char: char,
        parts: &'a [IrNode],
    },
    /// A doc comment to add to pending comments.
    Comment { text: String },
}

/// Code generator from IR.
pub struct Codegen {
    config: CodegenConfig,
    /// Counter for generating unique placeholder names.
    placeholder_counter: Cell<usize>,
}

impl Codegen {
    /// Creates a new code generator with default config.
    pub fn new() -> Self {
        Self {
            config: CodegenConfig::default(),
            placeholder_counter: Cell::new(0),
        }
    }

    /// Creates a new code generator with the given config.
    pub fn with_config(config: CodegenConfig) -> Self {
        Self {
            config,
            placeholder_counter: Cell::new(0),
        }
    }

    /// Generates a unique placeholder name.
    fn next_placeholder_name(&self) -> String {
        let n = self.placeholder_counter.get();
        self.placeholder_counter.set(n + 1);
        format!("MfPh{}", n)
    }

    /// Detects if text contains class member syntax (method declarations, properties).
    ///
    /// Used to identify middle spans that transition through class body level,
    /// e.g., `} static foo() {` which ends one method and starts another.
    fn contains_class_member_syntax(text: &str) -> bool {
        Self::find_class_member_transition(text).is_some()
    }

    /// Finds the index of a `}` that is followed by a class member declaration,
    /// skipping whitespace and comments (for JSDoc between members).
    fn find_class_member_transition(text: &str) -> Option<usize> {
        let bytes = text.as_bytes();
        let len = bytes.len();
        let mut i = 0;

        while i < len {
            if bytes[i] == b'}' {
                let mut j = i + 1;
                loop {
                    while j < len && bytes[j].is_ascii_whitespace() {
                        j += 1;
                    }
                    if j + 1 < len && bytes[j] == b'/' && bytes[j + 1] == b'/' {
                        j += 2;
                        while j < len && bytes[j] != b'\n' {
                            j += 1;
                        }
                        continue;
                    }
                    if j + 1 < len && bytes[j] == b'/' && bytes[j + 1] == b'*' {
                        j += 2;
                        while j + 1 < len && !(bytes[j] == b'*' && bytes[j + 1] == b'/') {
                            j += 1;
                        }
                        if j + 1 < len {
                            j += 2;
                        }
                        continue;
                    }
                    break;
                }

                if j < len && Self::starts_with_class_member_token(&text[j..]) {
                    return Some(i);
                }
            }
            i += 1;
        }

        None
    }

    fn starts_with_class_member_token(text: &str) -> bool {
        let text = text.trim_start();
        if let Some(rest) = text.strip_prefix("async")
            && rest.trim_start().starts_with("function")
        {
            return false;
        }

        let keywords = [
            "static",
            "readonly",
            "public",
            "private",
            "protected",
            "abstract",
            "async",
            "get",
            "set",
            "constructor",
        ];

        for kw in keywords {
            if let Some(rest) = text.strip_prefix(kw) {
                let next = rest.as_bytes().first().copied();
                if next.is_none_or(|b| !Self::is_ident_char(b)) {
                    return true;
                }
            }
        }

        text.starts_with('@')
    }

    fn is_ident_char(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_' || b == b'$'
    }

    /// Checks if an opener text is for a type literal (type alias or interface).
    ///
    /// Patterns:
    /// - `export type X = {` or `type X = {` (with optional content after `{`)
    /// - `export interface X {` or `interface X {` (with optional content after `{`)
    fn is_type_literal_opener(text: &str) -> bool {
        // Find the first `{` in the text
        let chars: Vec<char> = text.chars().collect();
        let mut first_open_brace = None;
        let mut in_string = false;
        let mut string_char = '"';
        let mut escape_next = false;

        for (i, &ch) in chars.iter().enumerate() {
            if escape_next {
                escape_next = false;
                continue;
            }
            if ch == '\\' {
                escape_next = true;
                continue;
            }
            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                continue;
            }
            match ch {
                '"' | '\'' | '`' => {
                    in_string = true;
                    string_char = ch;
                }
                '{' => {
                    first_open_brace = Some(i);
                    break;
                }
                _ => {}
            }
        }

        let Some(brace_pos) = first_open_brace else {
            return false;
        };

        // Check what precedes the first `{`
        let before_brace: String = chars[..brace_pos].iter().collect();
        let trimmed = before_brace.trim_end();

        // Type alias: `type X =` or `export type X =`
        if trimmed.ends_with('=') && trimmed.contains("type ") {
            return true;
        }

        // Interface: `interface X` or `export interface X`
        if trimmed.contains("interface ") {
            // Make sure we're not inside a nested interface
            let after_interface = trimmed.rsplit("interface ").next().unwrap_or("");
            if !after_interface.contains('{') {
                return true;
            }
        }

        false
    }

    /// Count actual `{` and `}` braces in a string, ignoring strings/escapes.
    fn count_actual_braces(text: &str) -> (i32, i32) {
        let mut opens = 0;
        let mut closes = 0;
        let mut in_string = false;
        let mut string_char = '"';
        let mut escape_next = false;

        for ch in text.chars() {
            if escape_next {
                escape_next = false;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                continue;
            }

            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                continue;
            }

            match ch {
                '"' | '\'' | '`' => {
                    in_string = true;
                    string_char = ch;
                }
                '{' => opens += 1,
                '}' => closes += 1,
                _ => {}
            }
        }

        (opens, closes)
    }

    /// Find the byte index of the first actual `}` brace (ignores strings/escapes).
    fn find_first_actual_closing_brace(text: &str) -> Option<usize> {
        let mut in_string = false;
        let mut string_char = '"';
        let mut escape_next = false;

        for (idx, ch) in text.char_indices() {
            if escape_next {
                escape_next = false;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                continue;
            }

            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                continue;
            }

            match ch {
                '"' | '\'' | '`' => {
                    in_string = true;
                    string_char = ch;
                }
                '}' => return Some(idx),
                _ => {}
            }
        }

        None
    }

    fn starts_with_module_decl(text: &str) -> bool {
        text.starts_with("export async function ")
            || text.starts_with("export function ")
            || text.starts_with("export default function ")
            || text.starts_with("export abstract class ")
            || text.starts_with("export class ")
            || text.starts_with("export interface ")
            || text.starts_with("export type ")
            || text.starts_with("export enum ")
            || text.starts_with("export namespace ")
            || text.starts_with("export declare ")
            || text.starts_with("async function ")
            || text.starts_with("function ")
            || text.starts_with("abstract class ")
            || text.starts_with("class ")
            || text.starts_with("interface ")
            || text.starts_with("type ")
            || text.starts_with("enum ")
            || text.starts_with("namespace ")
            || text.starts_with("declare ")
    }

    fn find_module_level_opener_split(text: &str) -> Option<usize> {
        let mut depth: i32 = 0;
        let mut in_string = false;
        let mut string_char = '"';
        let mut escape_next = false;
        let mut last_pos: Option<usize> = None;

        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();
        let mut byte_pos = 0;
        let mut i = 0;

        while i < len {
            let ch = chars[i];
            let char_byte_len = ch.len_utf8();

            if escape_next {
                escape_next = false;
                byte_pos += char_byte_len;
                i += 1;
                continue;
            }

            if ch == '\\' {
                escape_next = true;
                byte_pos += char_byte_len;
                i += 1;
                continue;
            }

            if in_string {
                if ch == string_char {
                    in_string = false;
                }
                byte_pos += char_byte_len;
                i += 1;
                continue;
            }

            match ch {
                '"' | '\'' | '`' => {
                    in_string = true;
                    string_char = ch;
                }
                '{' => {
                    depth += 1;
                }
                '}' => {
                    if depth > 0 {
                        depth -= 1;
                    }
                }
                _ => {}
            }

            if depth == 0 && !ch.is_whitespace() {
                let remaining = &text[byte_pos..];
                if Self::starts_with_module_decl(remaining) {
                    let before = text[..byte_pos].trim_end();
                    let preceded_by_export = before.ends_with("export");
                    if preceded_by_export {
                        byte_pos += char_byte_len;
                        i += 1;
                        continue;
                    }
                    last_pos = Some(byte_pos);
                }
            }

            byte_pos += char_byte_len;
            i += 1;
        }

        if depth > 0 { last_pos } else { None }
    }

    fn split_placeholders_by_pos(
        &self,
        template: &str,
        placeholders: &[PlaceholderEntry],
        split_pos: usize,
    ) -> (Vec<PlaceholderEntry>, Vec<PlaceholderEntry>) {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for (name, kind, expr) in placeholders {
            let marker = format!("${}", name);
            if let Some(pos) = Self::find_placeholder_pos(template, &marker) {
                if pos < split_pos {
                    left.push((name.clone(), *kind, expr.clone()));
                } else {
                    right.push((name.clone(), *kind, expr.clone()));
                }
            } else {
                right.push((name.clone(), *kind, expr.clone()));
            }
        }

        (left, right)
    }

    fn find_placeholder_pos(text: &str, marker: &str) -> Option<usize> {
        let mut offset = 0;
        let mut remaining = text;
        while let Some(pos) = remaining.find(marker) {
            let absolute = offset + pos;
            let after_idx = absolute + marker.len();
            let after = text.as_bytes().get(after_idx).copied();
            // Check if the character after the marker is NOT a continuation of the placeholder name.
            // Placeholder names are like $MfPh0, $MfPh12, etc. - they end with digits.
            // After the placeholder, valid terminators are:
            // - End of string
            // - $ (start of next placeholder)
            // - Any non-digit (since placeholder names end in digits, a letter means end)
            // The only invalid case is another digit (e.g., $MfPh1 followed by 0 = $MfPh10)
            if after.is_none_or(|b| !b.is_ascii_digit()) {
                return Some(absolute);
            }
            offset = after_idx;
            remaining = &text[offset..];
        }
        None
    }

    fn is_standalone_stmt_placeholder(
        &self,
        text: &str,
        placeholders: &[(String, PlaceholderKind, String)],
    ) -> Option<String> {
        if placeholders.len() != 1 {
            return None;
        }
        let (name, kind, _) = &placeholders[0];
        if *kind != PlaceholderKind::Stmt {
            return None;
        }

        let mut trimmed = text.trim();
        if let Some(stripped) = trimmed.strip_suffix(';') {
            trimmed = stripped.trim();
        }
        let marker = format!("${}", name);
        if trimmed == marker {
            Some(name.clone())
        } else {
            None
        }
    }

    /// Clone placeholder bindings so they can be passed to multiple ts_quote! calls.
    fn clone_quote_bindings(
        &self,
        quote_bindings: &[TokenStream],
    ) -> (Vec<TokenStream>, Vec<TokenStream>) {
        let mut clone_stmts = Vec::new();
        let mut clone_bindings = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for binding in quote_bindings {
            let binding_str = binding.to_string();
            let Some((name_part, rest)) = binding_str.split_once(':') else {
                continue;
            };
            let name = name_part.trim();
            if name.is_empty() || !seen.insert(name.to_string()) {
                continue;
            }
            let ty_part = rest.split('=').next().unwrap_or("").trim();
            if ty_part.is_empty() {
                continue;
            }

            let clone_name = format!("{}_mf_clone", name);
            let ph_ident = format_ident!("{}", name);
            let clone_ident = format_ident!("{}", clone_name);
            let ty_tokens: TokenStream = ty_part.parse().unwrap_or_else(|_| quote! { _ });

            clone_stmts.push(quote! { let #clone_ident = #ph_ident.clone(); });
            clone_bindings.push(quote! { #ph_ident: #ty_tokens = #clone_ident });
        }

        (clone_stmts, clone_bindings)
    }

    /// Generates Rust TokenStream from IR.
    ///
    /// The generated code builds `Vec<ModuleItem>` at compile time using
    /// `macroforge_ts::macroforge_ts_quote::ts_quote!` for static TypeScript and ToTs* traits for placeholders.
    pub fn generate(&self, ir: &Ir) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let body = self.generate_nodes(&ir.nodes);

        quote! {
            {
                let mut #output_var: Vec<macroforge_ts::swc_core::ecma::ast::ModuleItem> = Vec::new();
                #body
                #output_var
            }
        }
    }

    /// Generates code for a sequence of IR nodes.
    fn generate_nodes(&self, nodes: &[IrNode]) -> TokenStream {
        let chunks = self.chunk_nodes(nodes);

        // Check if we have any unbalanced chunks that need opener/closer tracking
        let has_unbalanced = chunks.iter().any(|c| match c {
            Chunk::Parseable {
                template,
                brace_balance,
                parse_context,
                ..
            } => {
                !brace_balance.is_balanced()
                    || self.parseable_needs_opener_stack(template, *parse_context)
            }
            Chunk::ControlFlow { node, body_context } => {
                self.controlflow_needs_outer_opener_stack(node, *body_context)
            }
            _ => false,
        });

        // Check if any opener has non-statement inner context (needs context collector)
        let needs_context_collector = chunks.iter().any(|c| match c {
            Chunk::Parseable {
                brace_balance,
                inner_context,
                ..
            } => {
                brace_balance.unclosed_opens > 0
                    && matches!(
                        inner_context,
                        ParseContext::ObjectLiteral
                            | ParseContext::ArrayLiteral
                            | ParseContext::ClassBody
                            | ParseContext::TypeObjectLiteral
                    )
            }
            _ => false,
        });

        let stmts: Vec<TokenStream> = chunks.iter().map(|c| self.generate_chunk(c)).collect();

        match (has_unbalanced, needs_context_collector) {
            (true, true) => {
                // Need both opener stack and context collector
                quote! {
                    let mut __mf_opener_stack: Vec<usize> = Vec::new();
                    let mut __mf_context_collector = macroforge_ts::macroforge_ts_syn::__internal::ContextCollector::for_object();
                    #(#stmts)*
                }
            }
            (true, false) => {
                // Only need opener stack (statement-level virtual completion)
                quote! {
                    let mut __mf_opener_stack: Vec<usize> = Vec::new();
                    #(#stmts)*
                }
            }
            _ => {
                quote! { #(#stmts)* }
            }
        }
    }

    fn parseable_needs_opener_stack(&self, template: &str, parse_context: ParseContext) -> bool {
        match parse_context {
            ParseContext::Module => Self::starts_with_class_member_token(template.trim_start()),
            ParseContext::FunctionBody | ParseContext::ClassBody => {
                Self::contains_class_member_syntax(template)
            }
            _ => false,
        }
    }

    fn controlflow_needs_outer_opener_stack(
        &self,
        node: &IrNode,
        body_context: ParseContext,
    ) -> bool {
        match node {
            IrNode::If {
                then_body,
                else_if_branches,
                else_body,
                ..
            } => {
                self.nodes_need_outer_opener_stack(then_body, body_context)
                    || else_if_branches
                        .iter()
                        .any(|(_, body)| self.nodes_need_outer_opener_stack(body, body_context))
                    || else_body
                        .as_ref()
                        .is_some_and(|body| self.nodes_need_outer_opener_stack(body, body_context))
            }
            IrNode::For { body, .. } | IrNode::While { body, .. } => {
                self.nodes_need_outer_opener_stack(body, body_context)
            }
            IrNode::Match { arms, .. } => arms
                .iter()
                .any(|(_, _, body)| self.nodes_need_outer_opener_stack(body, body_context)),
            _ => false,
        }
    }

    fn nodes_need_outer_opener_stack(&self, nodes: &[IrNode], context: ParseContext) -> bool {
        let saved_counter = self.placeholder_counter.get();
        let chunks = self.chunk_nodes_with_context(nodes, context);
        self.placeholder_counter.set(saved_counter);

        let has_unbalanced = chunks.iter().any(|c| match c {
            Chunk::Parseable { brace_balance, .. } => !brace_balance.is_balanced(),
            _ => false,
        });
        if has_unbalanced {
            return false;
        }

        for chunk in &chunks {
            match chunk {
                Chunk::Parseable {
                    template,
                    parse_context,
                    ..
                } => {
                    if self.parseable_needs_opener_stack(template, *parse_context) {
                        return true;
                    }
                }
                Chunk::ControlFlow { node, body_context } => {
                    if self.controlflow_needs_outer_opener_stack(node, *body_context) {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
    }

    /// Groups consecutive nodes into chunks that can be processed together.
    ///
    /// Tracks parse context through the chunks so that content inside object literals,
    /// arrays, etc. is parsed with the correct AST node type.
    fn chunk_nodes<'a>(&self, nodes: &'a [IrNode]) -> Vec<Chunk<'a>> {
        self.chunk_nodes_with_context(nodes, ParseContext::Module)
    }

    /// Groups nodes into chunks with explicit starting context.
    fn chunk_nodes_with_context<'a>(
        &self,
        nodes: &'a [IrNode],
        starting_context: ParseContext,
    ) -> Vec<Chunk<'a>> {
        let mut chunks = Vec::new();
        let mut current_template = String::new();
        let mut current_placeholders: Vec<(String, PlaceholderKind, String)> = Vec::new();
        let mut current_context = starting_context;
        // Track pending Ident placeholder for merging adjacent ones
        let mut pending_ident_exprs: Vec<String> = Vec::new();

        // Helper to flush pending ident expressions into a single placeholder
        let flush_pending_idents = |template: &mut String,
                                    placeholders: &mut Vec<(String, PlaceholderKind, String)>,
                                    pending: &mut Vec<String>,
                                    counter: &Cell<usize>| {
            if pending.is_empty() {
                return;
            }
            let ph_name = {
                let n = counter.get();
                counter.set(n + 1);
                format!("MfPh{}", n)
            };
            template.push('$');
            template.push_str(&ph_name);

            // Generate ident concatenation expression
            let ident_expr = if pending.len() == 1 {
                // Single ident - just use it directly
                pending[0].clone()
            } else {
                // Multiple idents - concatenate their symbols and create a new Ident
                // Build: { let mut __s = String::new(); __s.push_str(&a.sym); __s.push_str(&b.sym); Ident::new_no_ctxt(__s.into(), DUMMY_SP) }
                let push_stmts = pending
                        .iter()
                        .map(|e| format!("__mf_s.push_str(macroforge_ts::macroforge_ts_syn::ToTsIdent::to_ts_ident(&{}).sym.as_ref());", e))
                        .collect::<Vec<_>>()
                        .join(" ");
                format!(
                    "{{ let mut __mf_s = String::new(); {} macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(__mf_s.into(), macroforge_ts::swc_core::common::DUMMY_SP) }}",
                    push_stmts
                )
            };
            placeholders.push((ph_name, PlaceholderKind::Ident, ident_expr));
            pending.clear();
        };

        for node in nodes {
            match node {
                IrNode::Text(text) => {
                    // Flush any pending idents before adding text
                    flush_pending_idents(
                        &mut current_template,
                        &mut current_placeholders,
                        &mut pending_ident_exprs,
                        &self.placeholder_counter,
                    );
                    current_template.push_str(text);
                }

                IrNode::Placeholder { kind, rust_expr } => {
                    if *kind == PlaceholderKind::Ident {
                        // Accumulate Ident placeholders for potential merging
                        pending_ident_exprs.push(rust_expr.clone());
                    } else {
                        // Flush any pending idents first
                        flush_pending_idents(
                            &mut current_template,
                            &mut current_placeholders,
                            &mut pending_ident_exprs,
                            &self.placeholder_counter,
                        );
                        // Then emit this non-Ident placeholder
                        let ph_name = self.next_placeholder_name();
                        current_template.push('$');
                        current_template.push_str(&ph_name);
                        current_placeholders.push((ph_name, *kind, rust_expr.clone()));
                    }
                }

                IrNode::If { .. }
                | IrNode::For { .. }
                | IrNode::While { .. }
                | IrNode::Match { .. } => {
                    // Flush any pending idents first
                    flush_pending_idents(
                        &mut current_template,
                        &mut current_placeholders,
                        &mut pending_ident_exprs,
                        &self.placeholder_counter,
                    );
                    // Flush pending template and get the inner context
                    let inner_ctx = self.flush_parseable_with_context(
                        &mut chunks,
                        &mut current_template,
                        &mut current_placeholders,
                        current_context,
                    );
                    // Control flow body uses the inner context from the preceding chunk
                    chunks.push(Chunk::ControlFlow {
                        node,
                        body_context: inner_ctx,
                    });
                    // After control flow, we're back to the same context level
                    current_context = inner_ctx;
                }

                IrNode::Let { .. } | IrNode::Do { .. } => {
                    flush_pending_idents(
                        &mut current_template,
                        &mut current_placeholders,
                        &mut pending_ident_exprs,
                        &self.placeholder_counter,
                    );
                    let inner_ctx = self.flush_parseable_with_context(
                        &mut chunks,
                        &mut current_template,
                        &mut current_placeholders,
                        current_context,
                    );
                    chunks.push(Chunk::Directive {
                        node,
                        context: inner_ctx,
                    });
                    current_context = inner_ctx;
                }

                IrNode::TypeScript { stream: _ } => {
                    // {$typescript stream} - always flush and add as directive chunk.
                    // The directive will be handled by generate_typescript_with_context
                    // which properly parses the stream and adds statements/patches.
                    flush_pending_idents(
                        &mut current_template,
                        &mut current_placeholders,
                        &mut pending_ident_exprs,
                        &self.placeholder_counter,
                    );
                    let inner_ctx = self.flush_parseable_with_context(
                        &mut chunks,
                        &mut current_template,
                        &mut current_placeholders,
                        current_context,
                    );
                    chunks.push(Chunk::Directive {
                        node,
                        context: inner_ctx,
                    });
                    current_context = inner_ctx;
                }

                IrNode::IdentBlock { parts } => {
                    // Flush any pending idents first
                    flush_pending_idents(
                        &mut current_template,
                        &mut current_placeholders,
                        &mut pending_ident_exprs,
                        &self.placeholder_counter,
                    );
                    // Treat ident block as an Ident placeholder embedded in the template
                    let ph_name = self.next_placeholder_name();
                    current_template.push('$');
                    current_template.push_str(&ph_name);

                    // Generate the Rust expression that builds the identifier
                    let ident_builder = self.generate_ident_builder_expr(parts);
                    current_placeholders.push((ph_name, PlaceholderKind::Ident, ident_builder));
                }

                IrNode::StringInterp { quote: q, parts } => {
                    // Flush any pending idents first
                    flush_pending_idents(
                        &mut current_template,
                        &mut current_placeholders,
                        &mut pending_ident_exprs,
                        &self.placeholder_counter,
                    );
                    // Check if this string actually contains interpolations
                    let has_interpolations = parts
                        .iter()
                        .any(|p| matches!(p, IrNode::Placeholder { .. }));

                    if has_interpolations {
                        // Check if this is a simple string with just one placeholder and no other text
                        // e.g., "@{class_name}" should become a string literal, not a template literal
                        let is_simple_placeholder =
                            parts.len() == 1 && matches!(&parts[0], IrNode::Placeholder { .. });

                        if is_simple_placeholder {
                            // Single placeholder - use Expr placeholder which will be converted to string literal
                            // The placeholder value should be a string that becomes a Lit::Str
                            if let IrNode::Placeholder { kind: _, rust_expr } = &parts[0] {
                                let ph_name = self.next_placeholder_name();
                                // Use a special marker that creates a string literal from the expression
                                // We'll use Expr kind and wrap the expression to produce a string literal
                                current_template.push('$');
                                current_template.push_str(&ph_name);
                                // Push as Expr placeholder - the expression should evaluate to a string
                                // and ToTsExpr for String creates Lit::Str
                                current_placeholders.push((
                                    ph_name,
                                    PlaceholderKind::Expr,
                                    rust_expr.clone(),
                                ));
                            }
                        } else {
                            // Multiple parts - convert to inline template literal with placeholders
                            // This keeps the string inline in the template rather than splitting
                            // the expression into separate chunks
                            current_template.push('`');
                            for part in parts {
                                match part {
                                    IrNode::Text(text) => {
                                        // Escape backticks and ${} in static parts
                                        let escaped =
                                            text.replace('`', "\\`").replace("${", "\\${");
                                        current_template.push_str(&escaped);
                                    }
                                    IrNode::Placeholder { kind, rust_expr } => {
                                        // Add placeholder inline using template literal syntax
                                        let ph_name = self.next_placeholder_name();
                                        current_template.push_str("${$");
                                        current_template.push_str(&ph_name);
                                        current_template.push('}');
                                        current_placeholders.push((
                                            ph_name,
                                            *kind,
                                            rust_expr.clone(),
                                        ));
                                    }
                                    _ => {}
                                }
                            }
                            current_template.push('`');
                        }
                    } else {
                        // No interpolations - just append as static text with quotes
                        current_template.push(*q);
                        for part in parts {
                            if let IrNode::Text(text) = part {
                                current_template.push_str(text);
                            }
                        }
                        current_template.push(*q);
                    }
                }

                IrNode::Comment { text } => {
                    // Flush any pending idents first
                    flush_pending_idents(
                        &mut current_template,
                        &mut current_placeholders,
                        &mut pending_ident_exprs,
                        &self.placeholder_counter,
                    );
                    // Flush any pending template before the comment
                    let inner_ctx = self.flush_parseable_with_context(
                        &mut chunks,
                        &mut current_template,
                        &mut current_placeholders,
                        current_context,
                    );
                    current_context = inner_ctx;

                    // Add a comment chunk
                    chunks.push(Chunk::Comment { text: text.clone() });
                }
            }
        }

        // Flush any remaining pending idents
        flush_pending_idents(
            &mut current_template,
            &mut current_placeholders,
            &mut pending_ident_exprs,
            &self.placeholder_counter,
        );

        // Flush remaining
        self.flush_parseable_with_context(
            &mut chunks,
            &mut current_template,
            &mut current_placeholders,
            current_context,
        );

        chunks
    }

    /// Flushes pending template content into a Parseable chunk with context tracking.
    ///
    /// Analyzes the template to determine context transitions and returns the inner
    /// context that should be used for subsequent chunks.
    fn flush_parseable_with_context(
        &self,
        chunks: &mut Vec<Chunk>,
        current_template: &mut String,
        current_placeholders: &mut Vec<(String, PlaceholderKind, String)>,
        current_context: ParseContext,
    ) -> ParseContext {
        let template = std::mem::take(current_template);
        let placeholders = std::mem::take(current_placeholders);

        if template.trim().is_empty() && placeholders.is_empty() {
            return current_context;
        }

        let brace_balance = BraceBalance::analyze(&template);
        let context_analysis = ContextAnalysis::analyze(&template, current_context);

        chunks.push(Chunk::Parseable {
            template,
            placeholders,
            brace_balance,
            parse_context: current_context,
            inner_context: context_analysis.inner_context,
        });

        context_analysis.inner_context
    }

    /// Generates code for a single chunk.
    fn generate_chunk(&self, chunk: &Chunk) -> TokenStream {
        match chunk {
            Chunk::Parseable {
                template,
                placeholders,
                brace_balance,
                parse_context,
                inner_context: _,
            } => self.generate_parseable_chunk(
                template,
                placeholders,
                *brace_balance,
                *parse_context,
            ),

            Chunk::ControlFlow { node, body_context } => {
                self.generate_control_flow_with_context(node, *body_context)
            }

            Chunk::Directive { node, context } => {
                self.generate_directive_with_context(node, *context)
            }

            Chunk::StringInterp { quote_char, parts } => {
                self.generate_string_interp(*quote_char, parts)
            }

            Chunk::Comment { text } => {
                // Emit code to add the comment to __pending_comments
                let comment_text = format!("* {} ", text.trim());
                quote! {
                    __pending_comments.push(macroforge_ts::swc_core::common::comments::Comment {
                        kind: macroforge_ts::swc_core::common::comments::CommentKind::Block,
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        text: #comment_text.into(),
                    });
                }
            }
        }
    }

    /// Generates code for a parseable chunk using context-first architecture.
    ///
    /// ## Context-First Design
    ///
    /// 1. First, scan template with ContextStack to split at module-level transitions
    /// 2. For each span, match on context to determine handling
    /// 3. Depth info (from span) determines opener/closer/balanced
    ///
    /// This replaces the old brace-balance-first approach.
    fn generate_parseable_chunk(
        &self,
        template: &str,
        placeholders: &[(String, PlaceholderKind, String)],
        _brace_balance: BraceBalance, // TODO: Remove once refactor is complete
        parse_context: ParseContext,
    ) -> TokenStream {
        // Skip empty templates
        if template.trim().is_empty() && placeholders.is_empty() {
            return quote! {};
        }

        if parse_context == ParseContext::Module
            && _brace_balance.unclosed_opens > 0
            && let Some(split_pos) = Self::find_module_level_opener_split(template)
            && split_pos > 0
            && split_pos < template.len()
        {
            let (left_placeholders, right_placeholders) =
                self.split_placeholders_by_pos(template, placeholders, split_pos);
            let left = &template[..split_pos];
            let right = &template[split_pos..];
            let left_balance = BraceBalance::analyze(left);
            let right_balance = BraceBalance::analyze(right);

            let left_code = self.generate_parseable_chunk(
                left,
                &left_placeholders,
                left_balance,
                parse_context,
            );
            let right_code = self.generate_parseable_chunk(
                right,
                &right_placeholders,
                right_balance,
                parse_context,
            );

            return quote! {
                #left_code
                #right_code
            };
        }

        // STEP 1: Use ContextStack to scan and split at module-level transitions
        let mut ctx_stack = ContextStack::new(parse_context);
        let spans = ctx_stack.scan_template(template);

        if std::env::var("MF_DEBUG_SPANS").is_ok() {
            eprintln!(
                "[MF_DEBUG_SPANS] template context={:?}, spans={}",
                parse_context,
                spans.len()
            );
            for (i, span) in spans.iter().enumerate() {
                eprintln!(
                    "[MF_DEBUG_SPANS]   Span {}: context={:?}, depth={}→{}, text={:?}",
                    i,
                    span.context,
                    span.start_depth,
                    span.end_depth,
                    if span.text.len() > 120 {
                        format!("{}...", &span.text[..120])
                    } else {
                        span.text.clone()
                    }
                );
            }
        }

        #[cfg(debug_assertions)]
        if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
            eprintln!(
                "[MF_DEBUG_CODEGEN] Context-first scan: {} spans from template (context={:?})",
                spans.len(),
                parse_context
            );
            for (i, span) in spans.iter().enumerate() {
                eprintln!(
                    "[MF_DEBUG_CODEGEN]   Span {}: context={:?}, depth={}→{}, text={:?}",
                    i,
                    span.context,
                    span.start_depth,
                    span.end_depth,
                    if span.text.len() > 60 {
                        format!("{}...", &span.text[..60])
                    } else {
                        span.text.clone()
                    }
                );
            }
        }

        // STEP 2: Generate code for each span
        let mut code_parts: Vec<TokenStream> = Vec::new();

        for span in &spans {
            // Split placeholders for this span based on byte offsets
            let span_placeholders: Vec<(String, PlaceholderKind, String)> = placeholders
                .iter()
                .filter(|(name, _, _)| {
                    let marker = format!("${}", name);
                    if let Some(pos) = Self::find_placeholder_pos(template, &marker) {
                        pos >= span.start_offset && pos < span.end_offset
                    } else {
                        false
                    }
                })
                .cloned()
                .collect();

            let span_code = self.generate_span(span, &span_placeholders);
            code_parts.push(span_code);
        }

        quote! {
            #(#code_parts)*
        }
    }

    /// Generates code for a single context span.
    ///
    /// Match on context, then on depth to determine handling:
    /// - Module context: emit as ModuleItem or inject as stream
    /// - FunctionBody/etc: check depth for opener/closer/balanced
    fn generate_span(
        &self,
        span: &ContextSpan,
        placeholders: &[(String, PlaceholderKind, String)],
    ) -> TokenStream {
        if span.text.trim().is_empty() && placeholders.is_empty() {
            return quote! {};
        }

        // Generate placeholder bindings
        let (binding_stmts, quote_bindings) = self.generate_placeholder_bindings(placeholders);

        if matches!(
            span.context,
            ParseContext::Module | ParseContext::FunctionBody
        ) && span.is_balanced()
            && let Some(stmt_name) = self.is_standalone_stmt_placeholder(&span.text, placeholders)
        {
            let output_var = format_ident!("{}", self.config.output_var);
            let stmt_ident = format_ident!("{}", stmt_name);
            return quote! {
                {
                    #(#binding_stmts)*
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(#stmt_ident));
                }
            };
        }

        // MATCH ON CONTEXT - this is the primary dispatch
        match span.context {
            ParseContext::Module => {
                if Self::starts_with_class_member_token(span.text.trim_start()) {
                    self.generate_class_middle_span(
                        &span.text,
                        &binding_stmts,
                        &quote_bindings,
                        span,
                    )
                } else if span.is_balanced() {
                    // Module-level content - inject as raw source stream
                    // This handles multiple declarations correctly
                    self.generate_module_span(&span.text, placeholders)
                } else if span.is_closer() {
                    self.generate_closer_span(&span.text, &binding_stmts, &quote_bindings, span)
                } else if span.is_opener() {
                    // Check if this opener is for a type literal (type X = { or interface X {)
                    // Type literals with control flow need raw source emission to maintain
                    // proper ordering with for-loop content
                    if Self::is_type_literal_opener(&span.text) {
                        self.generate_raw_source_span(
                            &span.text,
                            &binding_stmts,
                            &quote_bindings,
                            span,
                        )
                    } else {
                        self.generate_opener_span(&span.text, &binding_stmts, &quote_bindings, span)
                    }
                } else {
                    self.generate_middle_span(&span.text, &binding_stmts, &quote_bindings, span)
                }
            }

            ParseContext::FunctionBody => {
                // Check span type: spans with class members get special handling (raw source),
                // otherwise use standard balanced/closer/opener logic
                //
                // Also use raw source for:
                // - Closers that close multiple levels (going past FunctionBody to Module)
                //   These can't be properly tracked in the opener stack because the opener
                //   might have been in Module context (which uses raw source, no stack push)
                let multi_level_closer = span.is_closer() && span.actual_excess_closes() > 1;
                if Self::contains_class_member_syntax(&span.text) {
                    // Class member transitions like `} static foo() {` need special handling
                    self.generate_class_middle_span(
                        &span.text,
                        &binding_stmts,
                        &quote_bindings,
                        span,
                    )
                } else if multi_level_closer {
                    if let Some(split_at) = Self::find_first_actual_closing_brace(&span.text) {
                        let (before, after) = span.text.split_at(split_at + 1);
                        let (before_opens, before_closes) = Self::count_actual_braces(before);
                        let before_excess_closes = (before_closes - before_opens).max(0) as usize;
                        let closer_code = self.generate_closer_span_with_count(
                            before,
                            &quote_bindings,
                            before_excess_closes,
                        );
                        let after_code = if after.trim().is_empty() {
                            quote! {}
                        } else {
                            self.build_raw_source_emit(after, &quote_bindings)
                        };

                        quote! {
                            {
                                #(#binding_stmts)*
                                #closer_code
                                #after_code
                            }
                        }
                    } else {
                        self.generate_raw_source_span(
                            &span.text,
                            &binding_stmts,
                            &quote_bindings,
                            span,
                        )
                    }
                } else if span.is_balanced() {
                    // Balanced: same depth at start and end (includes regular middle spans like `} else {`)
                    self.generate_balanced_stmt_items_span(
                        &span.text,
                        &binding_stmts,
                        &quote_bindings,
                    )
                } else if span.is_closer() {
                    // Closer: closes exactly one context
                    self.generate_closer_span(&span.text, &binding_stmts, &quote_bindings, span)
                } else {
                    // Opener: opens more contexts than it closes
                    self.generate_opener_span(&span.text, &binding_stmts, &quote_bindings, span)
                }
            }

            ParseContext::ObjectLiteral => {
                // Object literal - collect properties
                self.generate_object_span(&span.text, &binding_stmts, &quote_bindings, span)
            }

            ParseContext::ArrayLiteral => {
                // Array literal - collect elements
                self.generate_array_span(&span.text, &binding_stmts, &quote_bindings, span)
            }

            ParseContext::ClassBody => {
                // Class body - collect class members
                if span.is_balanced() {
                    // Balanced - collect class members as a single wrapped class
                    self.generate_class_body_span(&span.text, &binding_stmts, &quote_bindings)
                } else if Self::contains_class_member_syntax(&span.text) {
                    // Content contains class member transitions (e.g., `} static foo() { ... }`)
                    // Use generate_class_middle_span which handles stack updates properly
                    self.generate_class_middle_span(
                        &span.text,
                        &binding_stmts,
                        &quote_bindings,
                        span,
                    )
                } else if span.is_closer() {
                    // Closer in class body context
                    self.generate_closer_span(&span.text, &binding_stmts, &quote_bindings, span)
                } else {
                    // Opener in class body context: treat as class member start
                    self.generate_class_middle_span(
                        &span.text,
                        &binding_stmts,
                        &quote_bindings,
                        span,
                    )
                }
            }

            ParseContext::TypeObjectLiteral => {
                // Type object literal - uses type-specific handling
                self.generate_type_object_span(&span.text, &binding_stmts, &quote_bindings, span)
            }
        }
    }

    /// Generate code for a module-level span (inject as raw source).
    fn generate_module_span(
        &self,
        text: &str,
        placeholders: &[(String, PlaceholderKind, String)],
    ) -> TokenStream {
        if text.trim().is_empty() {
            return quote! {};
        }

        let output_var = format_ident!("{}", self.config.output_var);

        // TsStream placeholders use marker-based insertion:
        // - Markers like /*__MF_MARKER_MfPh0__*/ stay in the template as comments
        // - Bindings push to __injected_streams with the marker
        // - lib.rs replaces markers with stream content at runtime
        // No special handling needed here - just generate normal bindings

        let (binding_stmts, quote_bindings) = self.generate_placeholder_bindings(placeholders);
        let template_lit = syn::LitStr::new(text, proc_macro2::Span::call_site());
        let quote_call = if quote_bindings.is_empty() {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#template_lit as Module) }
        } else {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#template_lit as Module, #(#quote_bindings),*) }
        };

        quote! {
            {
                #(#binding_stmts)*
                let __mf_module: macroforge_ts::swc_core::ecma::ast::Module = #quote_call;
                #output_var.extend(__mf_module.body);
            }
        }
    }

    /// Generate code for an opener span (virtually complete with closing braces).
    fn generate_opener_span(
        &self,
        text: &str,
        binding_stmts: &[TokenStream],
        quote_bindings: &[TokenStream],
        span: &ContextSpan,
    ) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        // Use actual brace counts for virtual completion
        let actual_opens = span.actual_excess_opens() as usize;
        let virtual_closes = "}".repeat(actual_opens);
        let completed = format!("{}{}", text, virtual_closes);
        let completed_lit = syn::LitStr::new(&completed, proc_macro2::Span::call_site());

        #[cfg(debug_assertions)]
        if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
            eprintln!(
                "[MF_DEBUG_CODEGEN] Opener span: {:?} (actual_opens={})",
                completed, actual_opens
            );
        }

        let quote_call = if quote_bindings.is_empty() {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem) }
        } else {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem, #(#quote_bindings),*) }
        };

        quote! {
            {
                #(#binding_stmts)*
                let __mf_item: macroforge_ts::swc_core::ecma::ast::ModuleItem = #quote_call;
                let __mf_idx = macroforge_ts::macroforge_ts_syn::__internal::push_opener(
                    __mf_item,
                    &mut #output_var,
                    #actual_opens,
                );
                __mf_opener_stack.push(__mf_idx);
            }
        }
    }

    /// Generate code for a closer span (virtually complete with opening function).
    fn generate_closer_span(
        &self,
        text: &str,
        binding_stmts: &[TokenStream],
        quote_bindings: &[TokenStream],
        span: &ContextSpan,
    ) -> TokenStream {
        let actual_closes = span.actual_excess_closes() as usize;
        let closer_code = self.generate_closer_span_with_count(text, quote_bindings, actual_closes);
        quote! {
            {
                #(#binding_stmts)*
                #closer_code
            }
        }
    }

    fn generate_closer_span_with_count(
        &self,
        text: &str,
        quote_bindings: &[TokenStream],
        actual_closes: usize,
    ) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let extra_opens = "{".repeat(actual_closes.saturating_sub(1));
        let completed = format!("function __mf_virtual() {{ {}{}", extra_opens, text);
        let completed_lit = syn::LitStr::new(&completed, proc_macro2::Span::call_site());

        #[cfg(debug_assertions)]
        if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
            eprintln!(
                "[MF_DEBUG_CODEGEN] Closer span: {:?} (actual_closes={})",
                completed, actual_closes
            );
        }

        let quote_call = if quote_bindings.is_empty() {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem) }
        } else {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem, #(#quote_bindings),*) }
        };

        quote! {
            let __mf_item: macroforge_ts::swc_core::ecma::ast::ModuleItem = #quote_call;
            let __mf_opener_idx = __mf_opener_stack.pop()
                .expect("Virtual completion: no matching opener for closer");
            macroforge_ts::macroforge_ts_syn::__internal::finalize_closer(
                __mf_item,
                &mut #output_var,
                __mf_opener_idx,
            );
        }
    }

    /// Generate code for a balanced statement span.
    fn generate_balanced_stmt_span(
        &self,
        text: &str,
        binding_stmts: &[TokenStream],
        quote_bindings: &[TokenStream],
    ) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let template_lit = syn::LitStr::new(text, proc_macro2::Span::call_site());

        let quote_call = if quote_bindings.is_empty() {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#template_lit as Stmt) }
        } else {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#template_lit as Stmt, #(#quote_bindings),*) }
        };

        quote! {
            {
                #(#binding_stmts)*
                #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(#quote_call));
            }
        }
    }

    fn generate_balanced_stmt_items_span(
        &self,
        text: &str,
        binding_stmts: &[TokenStream],
        quote_bindings: &[TokenStream],
    ) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        if text.trim().is_empty() {
            return quote! {};
        }

        let wrapped = format!("function __mf_virtual() {{ {} }}", text);
        let wrapped_lit = syn::LitStr::new(&wrapped, proc_macro2::Span::call_site());

        let quote_call = if quote_bindings.is_empty() {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#wrapped_lit as ModuleItem) }
        } else {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#wrapped_lit as ModuleItem, #(#quote_bindings),*) }
        };

        quote! {
            {
                #(#binding_stmts)*
                let __mf_item: macroforge_ts::swc_core::ecma::ast::ModuleItem = #quote_call;
                let __mf_stmts =
                    macroforge_ts::macroforge_ts_syn::__internal::extract_function_body_statements(&__mf_item);
                for __mf_stmt in __mf_stmts {
                    #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(__mf_stmt));
                }
            }
        }
    }

    /// Generate code for a middle span (both opens and closes).
    fn generate_middle_span(
        &self,
        text: &str,
        binding_stmts: &[TokenStream],
        quote_bindings: &[TokenStream],
        span: &ContextSpan,
    ) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        // Use actual brace counts for virtual completion
        let actual_opens = span.actual_opens as usize;
        let actual_closes = span.actual_closes as usize;

        let virtual_opens = "{".repeat(actual_closes);
        let virtual_closes = "}".repeat(actual_opens);
        let completed = format!(
            "function __mf_virtual() {{ {}{}{} }}",
            virtual_opens, text, virtual_closes
        );
        let completed_lit = syn::LitStr::new(&completed, proc_macro2::Span::call_site());

        #[cfg(debug_assertions)]
        if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
            eprintln!(
                "[MF_DEBUG_CODEGEN] Middle span: {:?} (opens={}, closes={})",
                completed, actual_opens, actual_closes
            );
        }

        let quote_call = if quote_bindings.is_empty() {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem) }
        } else {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem, #(#quote_bindings),*) }
        };

        quote! {
            {
                #(#binding_stmts)*
                let __mf_item: macroforge_ts::swc_core::ecma::ast::ModuleItem = #quote_call;
                let __mf_opener_idx = *__mf_opener_stack.last()
                    .expect("Virtual completion: no matching opener for middle chunk");
                macroforge_ts::macroforge_ts_syn::__internal::push_middle(
                    __mf_item,
                    &mut #output_var,
                    __mf_opener_idx,
                );
            }
        }
    }

    /// Generate code for an object literal span.
    fn generate_object_span(
        &self,
        text: &str,
        binding_stmts: &[TokenStream],
        quote_bindings: &[TokenStream],
        span: &ContextSpan,
    ) -> TokenStream {
        if span.is_balanced() {
            // Balanced - collect property
            let wrapped = format!("({{ {} }})", text);
            let wrapped_lit = syn::LitStr::new(&wrapped, proc_macro2::Span::call_site());

            let quote_call = if quote_bindings.is_empty() {
                quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#wrapped_lit as Expr) }
            } else {
                quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#wrapped_lit as Expr, #(#quote_bindings),*) }
            };

            quote! {
                {
                    #(#binding_stmts)*
                    let __mf_wrapped_expr: macroforge_ts::swc_core::ecma::ast::Expr = #quote_call;
                    if let Some(__mf_prop) = macroforge_ts::macroforge_ts_syn::__internal::extract_prop_from_wrapped_expr(&__mf_wrapped_expr) {
                        macroforge_ts::macroforge_ts_syn::__internal::push_object_prop(&mut __mf_context_collector, __mf_prop);
                    }
                }
            }
        } else if span.is_closer() {
            // Closer in object context - use raw source emission because object literal
            // closers often contain type assertions (e.g., `} as SomeType;`) that can't
            // be wrapped in a block statement
            self.generate_raw_source_span(text, binding_stmts, quote_bindings, span)
        } else {
            // Opener in object context
            self.generate_opener_span(text, binding_stmts, quote_bindings, span)
        }
    }

    /// Generate code for an array literal span.
    fn generate_array_span(
        &self,
        text: &str,
        binding_stmts: &[TokenStream],
        quote_bindings: &[TokenStream],
        span: &ContextSpan,
    ) -> TokenStream {
        if span.is_balanced() {
            // Balanced - collect element
            let wrapped = format!("[{}]", text);
            let wrapped_lit = syn::LitStr::new(&wrapped, proc_macro2::Span::call_site());

            let quote_call = if quote_bindings.is_empty() {
                quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#wrapped_lit as Expr) }
            } else {
                quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#wrapped_lit as Expr, #(#quote_bindings),*) }
            };

            quote! {
                {
                    #(#binding_stmts)*
                    let __mf_wrapped_expr: macroforge_ts::swc_core::ecma::ast::Expr = #quote_call;
                    if let Some(__mf_elem) = macroforge_ts::macroforge_ts_syn::__internal::extract_elem_from_wrapped_expr(&__mf_wrapped_expr) {
                        macroforge_ts::macroforge_ts_syn::__internal::push_array_elem(&mut __mf_context_collector, __mf_elem);
                    }
                }
            }
        } else if span.is_closer() {
            // Closer in array context - use raw source emission similar to object literal
            self.generate_raw_source_span(text, binding_stmts, quote_bindings, span)
        } else {
            self.generate_opener_span(text, binding_stmts, quote_bindings, span)
        }
    }

    /// Generate code for a type object literal span (type literal members).
    ///
    /// Unlike object literals, type literals have property signatures with type annotations
    /// like `{ name: string; age: number }`.
    ///
    /// For type literals, we use raw source emission to properly handle control flow
    /// (for-loops, if-blocks) inside the type body. The AST-based collector approach
    /// doesn't integrate well with the opener/closer virtual completion system.
    fn generate_type_object_span(
        &self,
        text: &str,
        binding_stmts: &[TokenStream],
        quote_bindings: &[TokenStream],
        span: &ContextSpan,
    ) -> TokenStream {
        // Always use raw source emission for type literals.
        // This allows control flow (for-loops) inside type bodies to work correctly
        // by directly emitting source text that flows naturally with opener/closer.
        self.generate_raw_source_span(text, binding_stmts, quote_bindings, span)
    }

    /// Generate code for a class body span.
    ///
    /// Wraps the content in a dummy class for parsing, then extracts the class members.
    fn generate_class_body_span(
        &self,
        text: &str,
        binding_stmts: &[TokenStream],
        quote_bindings: &[TokenStream],
    ) -> TokenStream {
        // Wrap in a dummy class so it parses as valid TypeScript
        let wrapped = format!("class __MF_DUMMY__ {{ {} }}", text);
        let wrapped_lit = syn::LitStr::new(&wrapped, proc_macro2::Span::call_site());

        let quote_call = if quote_bindings.is_empty() {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#wrapped_lit as ModuleItem) }
        } else {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#wrapped_lit as ModuleItem, #(#quote_bindings),*) }
        };

        quote! {
            {
                #(#binding_stmts)*
                let __mf_wrapped_item: macroforge_ts::swc_core::ecma::ast::ModuleItem = #quote_call;
                for __mf_member in macroforge_ts::macroforge_ts_syn::__internal::extract_class_members_from_wrapped(&__mf_wrapped_item) {
                    macroforge_ts::macroforge_ts_syn::__internal::push_class_member(&mut __mf_context_collector, __mf_member);
                }
            }
        }
    }

    /// Generate code for a class body middle span (closes and opens contexts).
    ///
    /// For content like `} static foo() {` that ends one method and starts another.
    /// This transitions between FunctionBody and ClassBody contexts, so we:
    /// - finalize the previous method using a virtual closer
    /// - parse the new class member by wrapping it in a dummy class
    /// - merge the new member into the existing class AST
    /// - push a new opener if the new method body is still open
    fn generate_class_middle_span(
        &self,
        text: &str,
        binding_stmts: &[TokenStream],
        quote_bindings: &[TokenStream],
        span: &ContextSpan,
    ) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let _ = span;
        let split_at = Self::find_class_member_transition(text);
        let (before, after, has_transition) = match split_at {
            Some(idx) => {
                let (before, after) = text.split_at(idx + 1);
                (before, after, true)
            }
            None => ("", text, false),
        };

        let (before_opens, before_closes) = if has_transition {
            Self::count_actual_braces(before)
        } else {
            (0, 0)
        };
        let before_excess_closes = (before_closes - before_opens).max(0) as usize;
        let after_trimmed = after.trim_start();
        let (after_opens, after_closes) = Self::count_actual_braces(after_trimmed);
        let after_excess_opens = (after_opens - after_closes).max(0) as usize;
        let virtual_closes = "}".repeat(after_excess_opens);

        #[cfg(debug_assertions)]
        if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
            eprintln!(
                "[MF_DEBUG_CODEGEN] Class middle span: closes={} opens={}, after={:?}",
                span.middle_closes_count(),
                span.middle_opens_count(),
                if after_trimmed.len() > 60 {
                    format!("{}...", &after_trimmed[..60])
                } else {
                    after_trimmed.to_string()
                }
            );
        }

        if !has_transition {
            if after_trimmed.is_empty() {
                return quote! { #(#binding_stmts)* };
            }

            let wrapped = format!(
                "class __MF_DUMMY__ {{ {}{} }}",
                after_trimmed, virtual_closes
            );
            let wrapped_lit = syn::LitStr::new(&wrapped, proc_macro2::Span::call_site());

            let quote_call = if quote_bindings.is_empty() {
                quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#wrapped_lit as ModuleItem) }
            } else {
                quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#wrapped_lit as ModuleItem, #(#quote_bindings),*) }
            };

            let should_push = after_excess_opens > 0;

            return quote! {
                {
                    #(#binding_stmts)*
                    let __mf_wrapped_item: macroforge_ts::swc_core::ecma::ast::ModuleItem = #quote_call;
                    let __mf_members =
                        macroforge_ts::macroforge_ts_syn::__internal::extract_class_members_from_wrapped(&__mf_wrapped_item);
                    let __mf_last_idx = #output_var.len().checked_sub(1);
                    let __mf_has_last_class = __mf_last_idx.map_or(false, |idx| {
                        matches!(
                            #output_var.get(idx),
                            Some(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                                macroforge_ts::swc_core::ecma::ast::Stmt::Decl(
                                    macroforge_ts::swc_core::ecma::ast::Decl::Class(_)
                                )
                            ))
                                | Some(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                                    macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDecl(
                                        macroforge_ts::swc_core::ecma::ast::ExportDecl {
                                            decl: macroforge_ts::swc_core::ecma::ast::Decl::Class(_),
                                            ..
                                        }
                                    )
                                ))
                                | Some(macroforge_ts::swc_core::ecma::ast::ModuleItem::ModuleDecl(
                                    macroforge_ts::swc_core::ecma::ast::ModuleDecl::ExportDefaultDecl(
                                        macroforge_ts::swc_core::ecma::ast::ExportDefaultDecl {
                                            decl: macroforge_ts::swc_core::ecma::ast::DefaultDecl::Class(_),
                                            ..
                                        }
                                    )
                                ))
                        )
                    });

                    if let Some(&__mf_opener_idx) = __mf_opener_stack.last() {
                        if let Some(__mf_opener) = #output_var.get_mut(__mf_opener_idx) {
                            macroforge_ts::macroforge_ts_syn::__internal::merge_class_members(__mf_opener, __mf_members);
                        }

                        if #should_push {
                            __mf_opener_stack.push(__mf_opener_idx);
                        }
                    } else if __mf_has_last_class {
                        let __mf_opener_idx = __mf_last_idx.expect("last class index missing");
                        if let Some(__mf_opener) = #output_var.get_mut(__mf_opener_idx) {
                            macroforge_ts::macroforge_ts_syn::__internal::merge_class_members(__mf_opener, __mf_members);
                        }

                        if #should_push {
                            __mf_opener_stack.push(__mf_opener_idx);
                        }
                    } else {
                        let __mf_opener_idx = #output_var.len();
                        #output_var.push(__mf_wrapped_item);

                        if #should_push {
                            __mf_opener_stack.push(__mf_opener_idx);
                        }
                    }
                }
            };
        }

        let (clone_stmts, clone_bindings) = self.clone_quote_bindings(quote_bindings);

        let closer_lit = if before_excess_closes > 0 {
            let extra_opens = "{".repeat(before_excess_closes.saturating_sub(1));
            let completed = format!("function __mf_virtual() {{ {}{}", extra_opens, before);
            syn::LitStr::new(&completed, proc_macro2::Span::call_site())
        } else {
            syn::LitStr::new("function __mf_virtual() {}", proc_macro2::Span::call_site())
        };

        let closer_call = if quote_bindings.is_empty() {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#closer_lit as ModuleItem) }
        } else {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#closer_lit as ModuleItem, #(#quote_bindings),*) }
        };

        if after_trimmed.is_empty() {
            return quote! {
                {
                    #(#binding_stmts)*
                    #(#clone_stmts)*
                    let __mf_closer_item: macroforge_ts::swc_core::ecma::ast::ModuleItem = #closer_call;
                    let __mf_opener_idx = __mf_opener_stack.pop()
                        .expect("Virtual completion: no matching opener for class member transition");
                    macroforge_ts::macroforge_ts_syn::__internal::finalize_closer(
                        __mf_closer_item,
                        &mut #output_var,
                        __mf_opener_idx,
                    );
                }
            };
        }

        let wrapped = format!(
            "class __MF_DUMMY__ {{ {}{} }}",
            after_trimmed, virtual_closes
        );
        let wrapped_lit = syn::LitStr::new(&wrapped, proc_macro2::Span::call_site());

        let quote_call = if !clone_bindings.is_empty() {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#wrapped_lit as ModuleItem, #(#clone_bindings),*) }
        } else if quote_bindings.is_empty() {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#wrapped_lit as ModuleItem) }
        } else {
            quote! { macroforge_ts::macroforge_ts_quote::ts_quote!(#wrapped_lit as ModuleItem, #(#quote_bindings),*) }
        };

        let should_push = after_excess_opens > 0;

        quote! {
            {
                #(#binding_stmts)*
                #(#clone_stmts)*
                let __mf_closer_item: macroforge_ts::swc_core::ecma::ast::ModuleItem = #closer_call;
                let __mf_opener_idx = __mf_opener_stack.pop()
                    .expect("Virtual completion: no matching opener for class member transition");
                macroforge_ts::macroforge_ts_syn::__internal::finalize_closer(
                    __mf_closer_item,
                    &mut #output_var,
                    __mf_opener_idx,
                );

                let __mf_wrapped_item: macroforge_ts::swc_core::ecma::ast::ModuleItem = #quote_call;
                let __mf_members = macroforge_ts::macroforge_ts_syn::__internal::extract_class_members_from_wrapped(&__mf_wrapped_item);
                if let Some(__mf_opener) = #output_var.get_mut(__mf_opener_idx) {
                    macroforge_ts::macroforge_ts_syn::__internal::merge_class_members(__mf_opener, __mf_members);
                }

                if #should_push {
                    __mf_opener_stack.push(__mf_opener_idx);
                }
            }
        }
    }

    /// Generate code for a span that should be emitted as raw source.
    ///
    /// Used for complex closers (like ObjectLiteral closers with type assertions)
    /// that can't be wrapped in a function block for virtual completion.
    ///
    /// Raw source spans bypass the virtual completion system entirely - they don't
    /// interact with the opener/closer stack. The text is injected directly as source.
    fn generate_raw_source_span(
        &self,
        text: &str,
        binding_stmts: &[TokenStream],
        quote_bindings: &[TokenStream],
        _span: &ContextSpan,
    ) -> TokenStream {
        #[cfg(debug_assertions)]
        if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
            eprintln!("[MF_DEBUG_CODEGEN] Raw source span: {:?}", text);
        }

        let emit_code = self.build_raw_source_emit(text, quote_bindings);

        // Raw source bypasses virtual completion - no opener/closer stack operations
        quote! {
            {
                #(#binding_stmts)*
                #emit_code
            }
        }
    }

    fn build_raw_source_emit(&self, text: &str, quote_bindings: &[TokenStream]) -> TokenStream {
        let mut format_str = text.to_string();
        let mut format_args: Vec<TokenStream> = Vec::new();
        let mut tokens: Vec<String> = Vec::new();

        for (idx, binding) in quote_bindings.iter().enumerate() {
            let binding_str = binding.to_string();
            let Some((name_part, rest)) = binding_str.split_once(':') else {
                continue;
            };
            let name = name_part.trim();
            if name.is_empty() {
                continue;
            }
            let kind = rest.split('=').next().unwrap_or("").trim();
            let marker = format!("${}", name);
            if let Some(pos) = Self::find_placeholder_pos(&format_str, &marker) {
                let token = format!("__MF_FMT_{}__", idx);
                format_str.replace_range(pos..pos + marker.len(), &token);
                tokens.push(token);

                let ph_ident = format_ident!("{}", name);
                let arg = match kind {
                    "Ident" => quote! { #ph_ident.sym.as_str() },
                    "Expr" => quote! { macroforge_ts::ts_syn::emit_expr(&#ph_ident) },
                    "Stmt" => quote! { macroforge_ts::ts_syn::emit_stmt(&#ph_ident) },
                    "TsType" => quote! { macroforge_ts::ts_syn::emit_ts_type(&#ph_ident) },
                    _ => quote! { macroforge_ts::ts_syn::emit_ts_type(&#ph_ident) },
                };
                format_args.push(arg);
            }
        }

        format_str = format_str.replace('{', "{{").replace('}', "}}");
        for token in tokens {
            format_str = format_str.replace(&token, "{}");
        }

        if format_args.is_empty() {
            let text_lit = syn::LitStr::new(text, proc_macro2::Span::call_site());
            quote! {
                __injected_streams.push(
                    macroforge_ts::ts_syn::TsStream::from_string(#text_lit.to_string())
                );
            }
        } else {
            let format_lit = syn::LitStr::new(&format_str, proc_macro2::Span::call_site());
            quote! {
                let __formatted = format!(#format_lit, #(#format_args),*);
                __injected_streams.push(
                    macroforge_ts::ts_syn::TsStream::from_string(__formatted)
                );
            }
        }
    }

    /// Generates placeholder binding statements and quote bindings.
    ///
    /// The quote bindings include type annotations because `ts_quote!` requires
    /// them to know what AST node type to expect in each position.
    ///
    /// Returns:
    /// - binding_stmts: Let statements that convert Rust values to SWC AST nodes
    /// - quote_bindings: Bindings for ts_quote! macro
    fn generate_placeholder_bindings(
        &self,
        placeholders: &[(String, PlaceholderKind, String)],
    ) -> (Vec<TokenStream>, Vec<TokenStream>) {
        let mut binding_stmts = Vec::new();
        let mut quote_bindings = Vec::new();

        for (ph_name, kind, rust_expr) in placeholders {
            let ph_ident = format_ident!("{}", ph_name);
            let expr: TokenStream = rust_expr.parse().unwrap_or_else(|_| {
                let ident = format_ident!("{}", rust_expr);
                quote! { #ident }
            });

            // ts_quote! supports Ident, Expr, Pat, and TsType placeholder types natively.
            match kind {
                PlaceholderKind::Expr => {
                    // Clone the expression to allow reuse if the same variable appears multiple times
                    binding_stmts.push(quote! {
                        let #ph_ident: macroforge_ts::swc_core::ecma::ast::Expr = macroforge_ts::macroforge_ts_syn::ToTsExpr::to_ts_expr((#expr).clone());
                    });
                    quote_bindings.push(quote! { #ph_ident: Expr = #ph_ident });
                }
                PlaceholderKind::Type => {
                    // ts_quote! natively supports TsType placeholders
                    binding_stmts.push(quote! {
                        let #ph_ident: macroforge_ts::swc_core::ecma::ast::TsType = macroforge_ts::macroforge_ts_syn::ToTsType::to_ts_type(&#expr);
                    });
                    quote_bindings.push(quote! { #ph_ident: TsType = #ph_ident });
                }
                PlaceholderKind::Ident => {
                    binding_stmts.push(quote! {
                        let #ph_ident: macroforge_ts::swc_core::ecma::ast::Ident = macroforge_ts::macroforge_ts_syn::ToTsIdent::to_ts_ident(&#expr);
                    });
                    quote_bindings.push(quote! { #ph_ident: Ident = #ph_ident });
                }
                PlaceholderKind::Stmt => {
                    // ts_quote! supports Stmt directly
                    binding_stmts.push(quote! {
                        let #ph_ident: macroforge_ts::swc_core::ecma::ast::Stmt = macroforge_ts::macroforge_ts_syn::ToTsStmt::to_ts_stmt(#expr);
                    });
                    quote_bindings.push(quote! { #ph_ident: Stmt = #ph_ident });
                }
            }
        }

        (binding_stmts, quote_bindings)
    }

    /// Generates code for a chunk with unbalanced braces using virtual completion.
    ///
    /// This handles the case where control flow has split a template into incomplete
    /// chunks. We virtually complete the template for validation, then extract
    /// just the statements we need at runtime.
    ///
    /// The generated code uses a stack (`__mf_opener_stack`) to track opener indices.
    /// Openers push their index, closers pop and finalize, middles peek the current opener.
    fn generate_virtually_completed_chunk(
        &self,
        template: &str,
        binding_stmts: &[TokenStream],
        quote_bindings: &[TokenStream],
        brace_balance: BraceBalance,
        _is_module_decl: bool,
        _parse_context: ParseContext,
    ) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);

        // Determine the virtual completion strategy
        if brace_balance.unclosed_opens > 0 && brace_balance.unmatched_closes == 0 {
            // Case: Template opens blocks but doesn't close them
            // e.g., "export function foo(): void { const x = 1;"
            // Strategy: Add virtual closing braces, parse as ModuleItem, push and track index

            let virtual_closes = "}".repeat(brace_balance.unclosed_opens as usize);
            let completed_template = format!("{}{}", template, virtual_closes);

            #[cfg(debug_assertions)]
            {
                if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
                    eprintln!(
                        "[MF_DEBUG_CODEGEN] Virtually completed opener template: {:?}",
                        completed_template
                    );
                }
                debug_log::log_template_transform(
                    template,
                    &completed_template,
                    &format!("{:?}", _parse_context),
                    &format!("{:?}", brace_balance),
                    "virtual_opener",
                    &[], // placeholders already converted to bindings
                );
            }

            let completed_lit =
                syn::LitStr::new(&completed_template, proc_macro2::Span::call_site());

            let quote_call = if quote_bindings.is_empty() {
                quote! {
                    macroforge_ts::macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem)
                }
            } else {
                quote! {
                    macroforge_ts::macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem, #(#quote_bindings),*)
                }
            };

            let unclosed = brace_balance.unclosed_opens as usize;

            // Push the opener and track its index on the stack
            quote! {
                {
                    #(#binding_stmts)*
                    // Parse the virtually completed template
                    let __mf_item: macroforge_ts::swc_core::ecma::ast::ModuleItem = #quote_call;

                    // Push the opener and record its index for later finalization
                    let __mf_idx = macroforge_ts::macroforge_ts_syn::__internal::push_opener(
                        __mf_item,
                        &mut #output_var,
                        #unclosed,
                    );
                    __mf_opener_stack.push(__mf_idx);
                }
            }
        } else if brace_balance.unmatched_closes > 0 && brace_balance.unclosed_opens == 0 {
            // Case: Template closes blocks without opening them
            // e.g., "return x; }" or "} as Type; }"
            // Strategy: Wrap in virtual function opening, parse, finalize the opener
            //
            // NOTE: The split logic for balanced content after closes is handled in
            // generate_parseable_chunk before calling this function.
            //
            // Context-aware: If we're in ObjectLiteral context, the first `}` closes
            // an object literal, so we need to open with `return { __dummy: 0` not just `{`

            let completed_template = match _parse_context {
                ParseContext::ObjectLiteral => {
                    // First close is an object literal, subsequent closes are blocks/function
                    // For 2 closes: `} as X; }` → object close + function close
                    // Virtual: `function __mf_virtual() { return { __mf_dummy: 0 } as X; }`
                    //
                    // For n>2 closes, add extra block opens before the return
                    let extra_opens = if brace_balance.unmatched_closes > 2 {
                        "{".repeat((brace_balance.unmatched_closes - 2) as usize)
                    } else {
                        String::new()
                    };
                    format!(
                        "function __mf_virtual() {{ {}return {{ __mf_dummy: 0 {}",
                        extra_opens, template
                    )
                }
                ParseContext::ArrayLiteral => {
                    // First close is an array literal
                    let extra_opens = if brace_balance.unmatched_closes > 2 {
                        "{".repeat((brace_balance.unmatched_closes - 2) as usize)
                    } else {
                        String::new()
                    };
                    format!(
                        "function __mf_virtual() {{ {}return [ 0 {}",
                        extra_opens, template
                    )
                }
                _ => {
                    // Standard block context
                    let extra_opens =
                        "{".repeat((brace_balance.unmatched_closes - 1).max(0) as usize);
                    format!("function __mf_virtual() {{ {}{}", extra_opens, template)
                }
            };

            #[cfg(debug_assertions)]
            {
                if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
                    eprintln!(
                        "[MF_DEBUG_CODEGEN] Virtually completed closer template (context={:?}): {:?}",
                        _parse_context, completed_template
                    );
                }
                debug_log::log_template_transform(
                    template,
                    &completed_template,
                    &format!("{:?}", _parse_context),
                    &format!("{:?}", brace_balance),
                    "virtual_closer",
                    &[], // placeholders already converted to bindings
                );
            }

            let completed_lit =
                syn::LitStr::new(&completed_template, proc_macro2::Span::call_site());

            let quote_call = if quote_bindings.is_empty() {
                quote! {
                    macroforge_ts::macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem)
                }
            } else {
                quote! {
                    macroforge_ts::macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem, #(#quote_bindings),*)
                }
            };

            // Pop the opener index and finalize, merging any collected items
            let merge_code = match _parse_context {
                ParseContext::ObjectLiteral => {
                    quote! {
                        // Merge collected object properties into the opener
                        if let macroforge_ts::macroforge_ts_syn::__internal::ContextCollector::ObjectProps(props) =
                            std::mem::replace(&mut __mf_context_collector, macroforge_ts::macroforge_ts_syn::__internal::ContextCollector::for_object())
                        {
                            if let Some(opener) = #output_var.get_mut(__mf_opener_idx) {
                                macroforge_ts::macroforge_ts_syn::__internal::merge_object_props(opener, props);
                            }
                        }
                    }
                }
                ParseContext::ArrayLiteral => {
                    quote! {
                        // Merge collected array elements into the opener
                        if let macroforge_ts::macroforge_ts_syn::__internal::ContextCollector::ArrayElems(elems) =
                            std::mem::replace(&mut __mf_context_collector, macroforge_ts::macroforge_ts_syn::__internal::ContextCollector::for_array())
                        {
                            if let Some(opener) = #output_var.get_mut(__mf_opener_idx) {
                                macroforge_ts::macroforge_ts_syn::__internal::merge_array_elems(opener, elems);
                            }
                        }
                    }
                }
                ParseContext::ClassBody => {
                    quote! {
                        // Merge collected class members into the opener
                        if let macroforge_ts::macroforge_ts_syn::__internal::ContextCollector::ClassMembers(members) =
                            std::mem::replace(&mut __mf_context_collector, macroforge_ts::macroforge_ts_syn::__internal::ContextCollector::for_class())
                        {
                            if let Some(opener) = #output_var.get_mut(__mf_opener_idx) {
                                macroforge_ts::macroforge_ts_syn::__internal::merge_class_members(opener, members);
                            }
                        }
                    }
                }
                ParseContext::TypeObjectLiteral => {
                    quote! {
                        // Merge collected type properties into the opener
                        if let macroforge_ts::macroforge_ts_syn::__internal::ContextCollector::TypeProps(props) =
                            std::mem::replace(&mut __mf_context_collector, macroforge_ts::macroforge_ts_syn::__internal::ContextCollector::for_type_object())
                        {
                            if let Some(opener) = #output_var.get_mut(__mf_opener_idx) {
                                macroforge_ts::macroforge_ts_syn::__internal::merge_type_props(opener, props);
                            }
                        }
                    }
                }
                _ => quote! {},
            };

            quote! {
                {
                    #(#binding_stmts)*
                    // Parse the virtually completed template
                    let __mf_item: macroforge_ts::swc_core::ecma::ast::ModuleItem = #quote_call;

                    // Pop the opener index and finalize with accumulated statements
                    let __mf_opener_idx = __mf_opener_stack.pop()
                        .expect("Virtual completion: no matching opener for closer");

                    // Merge collected items (if any) into the opener
                    #merge_code

                    macroforge_ts::macroforge_ts_syn::__internal::finalize_closer(
                        __mf_item,
                        &mut #output_var,
                        __mf_opener_idx,
                    );
                }
            }
        } else {
            // Case: Template has both unclosed opens AND unmatched closes
            // This is a middle chunk inside a block
            // e.g., "} else { const y = 2;" (closes one, opens one)
            // Strategy: Wrap for validation, collect statements to current opener
            //
            // NOTE: Split logic for module-level content is handled in
            // generate_parseable_chunk before calling this function.
            //
            // Context-aware for ObjectLiteral: opening close is object literal

            let (virtual_opens, virtual_closes) = match _parse_context {
                ParseContext::ObjectLiteral => {
                    // First close is object literal
                    let extra_opens = if brace_balance.unmatched_closes > 1 {
                        "{".repeat((brace_balance.unmatched_closes - 1) as usize)
                    } else {
                        String::new()
                    };
                    let opens = format!("return {{ __mf_dummy: 0 {}", extra_opens);
                    let closes = "}".repeat(brace_balance.unclosed_opens as usize);
                    (opens, closes)
                }
                ParseContext::ArrayLiteral => {
                    let extra_opens = if brace_balance.unmatched_closes > 1 {
                        "{".repeat((brace_balance.unmatched_closes - 1) as usize)
                    } else {
                        String::new()
                    };
                    let opens = format!("return [ 0 {}", extra_opens);
                    let closes = "}".repeat(brace_balance.unclosed_opens as usize);
                    (opens, closes)
                }
                _ => {
                    let opens = "{".repeat(brace_balance.unmatched_closes as usize);
                    let closes = "}".repeat(brace_balance.unclosed_opens as usize);
                    (opens, closes)
                }
            };

            let completed_template = format!(
                "function __mf_virtual() {{ {}{}{} }}",
                virtual_opens, template, virtual_closes
            );

            #[cfg(debug_assertions)]
            {
                if std::env::var("MF_DEBUG_CODEGEN").is_ok() {
                    eprintln!(
                        "[MF_DEBUG_CODEGEN] Virtually completed middle template (context={:?}): {:?}",
                        _parse_context, completed_template
                    );
                }
                debug_log::log_template_transform(
                    template,
                    &completed_template,
                    &format!("{:?}", _parse_context),
                    &format!("{:?}", brace_balance),
                    "virtual_middle",
                    &[], // placeholders already converted to bindings
                );
            }

            let completed_lit =
                syn::LitStr::new(&completed_template, proc_macro2::Span::call_site());

            let quote_call = if quote_bindings.is_empty() {
                quote! {
                    macroforge_ts::macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem)
                }
            } else {
                quote! {
                    macroforge_ts::macroforge_ts_quote::ts_quote!(#completed_lit as ModuleItem, #(#quote_bindings),*)
                }
            };

            // For middle chunks, peek the opener index (don't pop - we're still in the same block)
            quote! {
                {
                    #(#binding_stmts)*
                    // Parse the virtually completed template for validation
                    let __mf_item: macroforge_ts::swc_core::ecma::ast::ModuleItem = #quote_call;

                    // Get the current opener index (peek, not pop)
                    let __mf_opener_idx = *__mf_opener_stack.last()
                        .expect("Virtual completion: no matching opener for middle chunk");
                    macroforge_ts::macroforge_ts_syn::__internal::push_middle(
                        __mf_item,
                        &mut #output_var,
                        __mf_opener_idx,
                    );
                }
            }
        }
    }

    /// Generates code for control flow nodes.
    fn generate_control_flow(&self, node: &IrNode) -> TokenStream {
        self.generate_control_flow_with_context(node, ParseContext::Module)
    }

    /// Generates code for control flow nodes with a specific body context.
    ///
    /// The body_context determines how the body content should be parsed:
    /// - ObjectLiteral: body generates PropOrSpread items
    /// - ArrayLiteral: body generates Expr items
    /// - FunctionBody/Module: body generates Stmt/ModuleItem
    fn generate_control_flow_with_context(
        &self,
        node: &IrNode,
        body_context: ParseContext,
    ) -> TokenStream {
        match node {
            IrNode::If {
                condition,
                then_body,
                else_if_branches,
                else_body,
            } => self.generate_if_with_context(
                condition,
                then_body,
                else_if_branches,
                else_body,
                body_context,
            ),

            IrNode::For {
                pattern,
                iterator,
                body,
            } => self.generate_for_with_context(pattern, iterator, body, body_context),

            IrNode::While { condition, body } => {
                self.generate_while_with_context(condition, body, body_context)
            }

            IrNode::Match { expr, arms } => {
                self.generate_match_with_context(expr, arms, body_context)
            }

            _ => quote! {},
        }
    }

    /// Generates code for directive nodes with context awareness.
    fn generate_directive_with_context(&self, node: &IrNode, context: ParseContext) -> TokenStream {
        match node {
            IrNode::Let {
                name,
                mutable,
                type_hint,
                value,
            } => self.generate_let(name, *mutable, type_hint.as_deref(), value),
            IrNode::Do { code } => self.generate_do(code),
            IrNode::TypeScript { stream } => self.generate_typescript_with_context(stream, context),
            _ => quote! {},
        }
    }

    /// Generates code for an if statement.
    fn generate_if(
        &self,
        condition: &str,
        then_body: &[IrNode],
        else_if_branches: &[(String, Vec<IrNode>)],
        else_body: &Option<Vec<IrNode>>,
    ) -> TokenStream {
        let cond: TokenStream = condition.parse().unwrap_or_else(|_| quote! { true });
        let then_code = self.generate_nodes(then_body);

        let else_if_code: Vec<TokenStream> = else_if_branches
            .iter()
            .map(|(cond, body)| {
                let c: TokenStream = cond.parse().unwrap_or_else(|_| quote! { true });
                let b = self.generate_nodes(body);
                quote! { else if #c { #b } }
            })
            .collect();

        let else_code = else_body.as_ref().map(|body| {
            let b = self.generate_nodes(body);
            quote! { else { #b } }
        });

        quote! {
            if #cond {
                #then_code
            }
            #(#else_if_code)*
            #else_code
        }
    }

    /// Generates code for a for loop.
    fn generate_for(&self, pattern: &str, iterator: &str, body: &[IrNode]) -> TokenStream {
        let pat: TokenStream = pattern.parse().unwrap_or_else(|_| quote! { _ });
        let iter: TokenStream = iterator
            .parse()
            .unwrap_or_else(|_| quote! { std::iter::empty::<()>() });
        let body_code = self.generate_nodes(body);

        quote! {
            for #pat in #iter {
                #body_code
            }
        }
    }

    /// Generates code for a while loop.
    fn generate_while(&self, condition: &str, body: &[IrNode]) -> TokenStream {
        let cond: TokenStream = condition.parse().unwrap_or_else(|_| quote! { false });
        let body_code = self.generate_nodes(body);

        quote! {
            while #cond {
                #body_code
            }
        }
    }

    /// Generates code for a match expression.
    fn generate_match(
        &self,
        expr: &str,
        arms: &[(String, Option<String>, Vec<IrNode>)],
    ) -> TokenStream {
        self.generate_match_with_context(expr, arms, ParseContext::Module)
    }

    /// Generates code for an if statement with a specific body context.
    fn generate_if_with_context(
        &self,
        condition: &str,
        then_body: &[IrNode],
        else_if_branches: &[(String, Vec<IrNode>)],
        else_body: &Option<Vec<IrNode>>,
        body_context: ParseContext,
    ) -> TokenStream {
        let cond: TokenStream = condition.parse().unwrap_or_else(|_| quote! { true });
        let then_code = self.generate_nodes_with_context(then_body, body_context);

        let else_if_code: Vec<TokenStream> = else_if_branches
            .iter()
            .map(|(cond, body)| {
                let c: TokenStream = cond.parse().unwrap_or_else(|_| quote! { true });
                let b = self.generate_nodes_with_context(body, body_context);
                quote! { else if #c { #b } }
            })
            .collect();

        let else_code = else_body.as_ref().map(|body| {
            let b = self.generate_nodes_with_context(body, body_context);
            quote! { else { #b } }
        });

        quote! {
            if #cond {
                #then_code
            }
            #(#else_if_code)*
            #else_code
        }
    }

    /// Generates code for a for loop with a specific body context.
    fn generate_for_with_context(
        &self,
        pattern: &str,
        iterator: &str,
        body: &[IrNode],
        body_context: ParseContext,
    ) -> TokenStream {
        let pat: TokenStream = pattern.parse().unwrap_or_else(|_| quote! { _ });
        let iter: TokenStream = iterator
            .parse()
            .unwrap_or_else(|_| quote! { std::iter::empty::<()>() });
        let body_code = self.generate_nodes_with_context(body, body_context);

        quote! {
            for #pat in #iter {
                #body_code
            }
        }
    }

    /// Generates code for a while loop with a specific body context.
    fn generate_while_with_context(
        &self,
        condition: &str,
        body: &[IrNode],
        body_context: ParseContext,
    ) -> TokenStream {
        let cond: TokenStream = condition.parse().unwrap_or_else(|_| quote! { false });
        let body_code = self.generate_nodes_with_context(body, body_context);

        quote! {
            while #cond {
                #body_code
            }
        }
    }

    /// Generates code for a match expression with a specific body context.
    fn generate_match_with_context(
        &self,
        expr: &str,
        arms: &[(String, Option<String>, Vec<IrNode>)],
        body_context: ParseContext,
    ) -> TokenStream {
        let e: TokenStream = expr.parse().unwrap_or_else(|_| quote! { () });

        let arm_code: Vec<TokenStream> = arms
            .iter()
            .map(|(pattern, guard, body)| {
                let pat: TokenStream = pattern.parse().unwrap_or_else(|_| quote! { _ });
                let body_code = self.generate_nodes_with_context(body, body_context);

                if let Some(g) = guard {
                    let guard_expr: TokenStream = g.parse().unwrap_or_else(|_| quote! { true });
                    quote! { #pat if #guard_expr => { #body_code } }
                } else {
                    quote! { #pat => { #body_code } }
                }
            })
            .collect();

        quote! {
            match #e {
                #(#arm_code)*
            }
        }
    }

    /// Generates code for a sequence of IR nodes with a specific starting context.
    fn generate_nodes_with_context(&self, nodes: &[IrNode], context: ParseContext) -> TokenStream {
        let chunks = self.chunk_nodes_with_context(nodes, context);

        // Check if we have any unbalanced chunks that need opener/closer tracking
        let has_unbalanced = chunks.iter().any(|c| match c {
            Chunk::Parseable { brace_balance, .. } => !brace_balance.is_balanced(),
            _ => false,
        });

        // Check if any opener has non-statement inner context (needs context collector)
        let needs_context_collector = chunks.iter().any(|c| match c {
            Chunk::Parseable {
                brace_balance,
                inner_context,
                ..
            } => {
                brace_balance.unclosed_opens > 0
                    && matches!(
                        inner_context,
                        ParseContext::ObjectLiteral
                            | ParseContext::ArrayLiteral
                            | ParseContext::ClassBody
                            | ParseContext::TypeObjectLiteral
                    )
            }
            _ => false,
        });

        let stmts: Vec<TokenStream> = chunks.iter().map(|c| self.generate_chunk(c)).collect();

        match (has_unbalanced, needs_context_collector) {
            (true, true) => {
                // Need both opener stack and context collector
                quote! {
                    let mut __mf_opener_stack: Vec<usize> = Vec::new();
                    let mut __mf_context_collector = macroforge_ts::macroforge_ts_syn::__internal::ContextCollector::for_object();
                    #(#stmts)*
                }
            }
            (true, false) => {
                // Only need opener stack (statement-level virtual completion)
                quote! {
                    let mut __mf_opener_stack: Vec<usize> = Vec::new();
                    #(#stmts)*
                }
            }
            _ => {
                // No unbalanced chunks - just generate the statements
                // But we might still need the collector for balanced non-statement chunks
                if context.is_non_statement_context() {
                    let collector_type = context.collector_type();
                    quote! {
                        let mut __mf_context_collector = macroforge_ts::macroforge_ts_syn::__internal::ContextCollector::new(#collector_type);
                        #(#stmts)*
                    }
                } else {
                    quote! { #(#stmts)* }
                }
            }
        }
    }

    /// Generates a Rust expression string that builds an identifier from parts.
    ///
    /// This is used to embed ident blocks as placeholders in macroforge_ts::macroforge_ts_quote::ts_quote!.
    fn generate_ident_builder_expr(&self, parts: &[IrNode]) -> String {
        let mut expr_parts = Vec::new();

        for part in parts {
            match part {
                IrNode::Text(text) => {
                    // Static text part
                    let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");
                    expr_parts.push(format!("\"{}\".to_string()", escaped));
                }
                IrNode::Placeholder { rust_expr, .. } => {
                    // Dynamic part - convert to string
                    expr_parts.push(format!("({}).to_string()", rust_expr));
                }
                _ => {}
            }
        }

        if expr_parts.is_empty() {
            // Empty ident block - just return empty string builder
            return "{ let s = String::new(); macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(s.into(), macroforge_ts::swc_core::common::DUMMY_SP) }".to_string();
        }

        // Generate expression that concatenates all parts and creates an Ident
        format!(
            "{{ let mut __s = String::new(); {} macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(__s.into(), macroforge_ts::swc_core::common::DUMMY_SP) }}",
            expr_parts
                .iter()
                .map(|p| format!("__s.push_str(&{});", p))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    /// Generates code for an identifier block.
    ///
    /// DEPRECATED: Ident blocks are now handled as placeholders in chunk_nodes.
    /// This function is kept for backwards compatibility but should not be called.
    #[allow(dead_code)]
    fn generate_ident_block(&self, parts: &[IrNode]) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);

        let part_stmts: Vec<TokenStream> = parts
            .iter()
            .filter_map(|p| match p {
                IrNode::Text(text) => Some(quote! { __ident_parts.push_str(#text); }),
                IrNode::Placeholder { rust_expr, .. } => {
                    let expr: TokenStream = rust_expr.parse().unwrap_or_else(|_| {
                        let ident = format_ident!("{}", rust_expr);
                        quote! { #ident }
                    });
                    Some(quote! { __ident_parts.push_str(&(#expr).to_string()); })
                }
                _ => None,
            })
            .collect();

        // Build the identifier and create an expression statement with it
        quote! {
            {
                let mut __ident_parts = String::new();
                #(#part_stmts)*
                let __ident = macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                    __ident_parts.into(),
                    macroforge_ts::swc_core::common::DUMMY_SP
                );
                #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                    macroforge_ts::swc_core::ecma::ast::Stmt::Expr(macroforge_ts::swc_core::ecma::ast::ExprStmt {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Ident(__ident)),
                    })
                ));
            }
        }
    }

    /// Generates code for string interpolation (template literals).
    fn generate_string_interp(&self, quote_char: char, parts: &[IrNode]) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);

        // Build quasis (static parts) and expressions
        let mut quasis = Vec::new();
        let mut exprs = Vec::new();
        let mut current_quasi = String::new();

        for part in parts {
            match part {
                IrNode::Text(text) => {
                    current_quasi.push_str(text);
                }
                IrNode::Placeholder { rust_expr, .. } => {
                    // Flush current quasi
                    let quasi_text = std::mem::take(&mut current_quasi);
                    quasis.push(quote! {
                        macroforge_ts::swc_core::ecma::ast::TplElement {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            tail: false,
                            cooked: Some(#quasi_text.into()),
                            raw: #quasi_text.into(),
                        }
                    });

                    let expr: TokenStream = rust_expr.parse().unwrap_or_else(|_| {
                        let ident = format_ident!("{}", rust_expr);
                        quote! { #ident }
                    });
                    exprs.push(quote! {
                        Box::new(macroforge_ts::macroforge_ts_syn::ToTsExpr::to_ts_expr(#expr))
                    });
                }
                _ => {}
            }
        }

        // Final quasi (tail)
        let final_quasi = current_quasi;
        quasis.push(quote! {
            macroforge_ts::swc_core::ecma::ast::TplElement {
                span: macroforge_ts::swc_core::common::DUMMY_SP,
                tail: true,
                cooked: Some(#final_quasi.into()),
                raw: #final_quasi.into(),
            }
        });

        let _ = quote_char; // We build a template literal regardless of quote char

        quote! {
            #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(
                macroforge_ts::swc_core::ecma::ast::Stmt::Expr(macroforge_ts::swc_core::ecma::ast::ExprStmt {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    expr: Box::new(macroforge_ts::swc_core::ecma::ast::Expr::Tpl(macroforge_ts::swc_core::ecma::ast::Tpl {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        exprs: vec![#(#exprs),*],
                        quasis: vec![#(#quasis),*],
                    })),
                })
            ));
        }
    }

    /// Generates code for a let directive.
    fn generate_let(
        &self,
        name: &str,
        mutable: bool,
        type_hint: Option<&str>,
        value: &str,
    ) -> TokenStream {
        let ident = format_ident!("{}", name);
        let val: TokenStream = value.parse().unwrap_or_else(|_| quote! { () });

        let mut_token = if mutable {
            quote! { mut }
        } else {
            quote! {}
        };

        let type_annotation = if let Some(ty) = type_hint {
            let ty_tokens: TokenStream = ty.parse().unwrap_or_else(|_| quote! { _ });
            quote! { : #ty_tokens }
        } else {
            quote! {}
        };

        quote! {
            let #mut_token #ident #type_annotation = #val;
        }
    }

    /// Generates code for a do directive.
    fn generate_do(&self, code: &str) -> TokenStream {
        let c: TokenStream = code.parse().unwrap_or_else(|_| quote! { () });
        quote! {
            #c;
        }
    }

    /// Generates code for a typescript directive with context awareness.
    fn generate_typescript_with_context(&self, stream: &str, context: ParseContext) -> TokenStream {
        let output_var = format_ident!("{}", self.config.output_var);
        let s: TokenStream = stream.parse().unwrap_or_else(|_| quote! { () });

        match context {
            ParseContext::Module => {
                quote! {
                    {
                        let __mf_stream: macroforge_ts::ts_syn::TsStream = #s;
                        let __mf_source = __mf_stream.source();
                        let __mf_insert_pos = __mf_stream.insert_pos;
                        if __mf_insert_pos == macroforge_ts::ts_syn::abi::InsertPos::Within
                            || __mf_source.contains("/* @macroforge:")
                        {
                            __injected_streams.push(__mf_stream);
                        } else {
                            let __mf_items =
                                macroforge_ts::macroforge_ts_syn::parse_ts_to_module_items(__mf_source);
                            if __mf_items.is_empty() && !__mf_source.trim().is_empty() {
                                __injected_streams.push(__mf_stream);
                            } else {
                                #output_var.extend(__mf_items);
                                __patches.extend(__mf_stream.runtime_patches);
                            }
                        }
                    }
                }
            }
            ParseContext::FunctionBody => {
                quote! {
                    {
                        let __mf_stream: macroforge_ts::ts_syn::TsStream = #s;
                        let __mf_items =
                            macroforge_ts::macroforge_ts_syn::parse_ts_to_module_items(__mf_stream.source());
                        #[cfg(debug_assertions)]
                        if std::env::var("MF_DEBUG_TS_DIRECTIVE").is_ok() {
                            let __mf_src = __mf_stream.source();
                            let __mf_preview_len = __mf_src.len().min(160);
                            eprintln!(
                                "[MF_DEBUG_TS_DIRECTIVE] fn_body items={} preview={:?}",
                                __mf_items.len(),
                                &__mf_src[..__mf_preview_len]
                            );
                        }
                        for __mf_item in __mf_items {
                            if let macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(__mf_stmt) = __mf_item {
                                #output_var.push(macroforge_ts::swc_core::ecma::ast::ModuleItem::Stmt(__mf_stmt));
                            }
                        }
                        __patches.extend(__mf_stream.runtime_patches);
                    }
                }
            }
            _ => {
                // Fallback: collect for merging at output.
                quote! {
                    // {$typescript} injects a TsStream - collect for merging at output
                    __injected_streams.push(#s);
                }
            }
        }
    }
}

impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::ir::lower;
    use crate::compiler::parser::Parser;
    use crate::compiler::semantic::analyze;
    use crate::compiler::syntax::SyntaxNode;

    fn compile_template(input: &str) -> TokenStream {
        let parser = Parser::new(input);
        let green = parser.parse();
        let root = SyntaxNode::new_root(green);
        let analysis = analyze(&root);
        let ir = lower(&root, analysis);
        Codegen::new().generate(&ir)
    }

    #[test]
    fn test_codegen_simple_text() {
        let code = compile_template("const x = 1;");
        let code_str = code.to_string();
        // Should use macroforge_ts::macroforge_ts_quote::ts_quote! for static text
        assert!(
            code_str.contains("macroforge_ts_quote :: ts_quote !"),
            "Generated code: {}",
            code_str
        );
    }

    #[test]
    fn test_codegen_interpolation() {
        let code = compile_template("const x = @{value};");
        let code_str = code.to_string();
        // Should contain ToTsExpr trait call
        assert!(
            code_str.contains("ToTsExpr") || code_str.contains("to_ts_expr"),
            "Generated code: {}",
            code_str
        );
    }

    #[test]
    fn test_codegen_type_placeholder() {
        let code = compile_template("const x: @{MyType} = 1;");
        let code_str = code.to_string();
        // Should contain ToTsType trait call for creating the type
        assert!(
            code_str.contains("ToTsType") || code_str.contains("to_ts_type"),
            "Expected ToTsType for type placeholder. Generated code: {}",
            code_str
        );
        // ts_quote! handles TsType placeholders natively via $name: TsType = expr
        assert!(
            code_str.contains("TsType ="),
            "Expected native TsType placeholder binding. Generated code: {}",
            code_str
        );
    }

    #[test]
    fn test_codegen_for_loop() {
        let code = compile_template("{#for item in items}@{item}{/for}");
        let code_str = code.to_string();
        // Should contain a for loop
        assert!(code_str.contains("for"), "Generated code: {}", code_str);
        assert!(code_str.contains("in"), "Generated code: {}", code_str);
    }

    #[test]
    fn test_codegen_if_block() {
        let code = compile_template("{#if cond}yes{/if}");
        let code_str = code.to_string();
        // Should contain an if statement
        assert!(code_str.contains("if"), "Generated code: {}", code_str);
    }

    #[test]
    fn test_codegen_generates_vec_module_item() {
        let code = compile_template("const x = 1;");
        let code_str = code.to_string();
        // Should generate Vec<ModuleItem>
        assert!(
            code_str.contains("Vec < macroforge_ts :: swc_core :: ecma :: ast :: ModuleItem >"),
            "Generated code: {}",
            code_str
        );
    }

    #[test]
    fn test_codegen_function_with_doc_attribute_tokenstream_format() {
        // This is how doc comments appear after going through Rust's TokenStream:
        // /** Doc */ becomes # [doc = "Doc"]
        let code = compile_template(
            r#"# [doc = "Doc comment"] export function @{fn_name}(value: @{type_param}): string { return @{body_expr}; }"#,
        );
        let code_str = code.to_string();
        eprintln!("Generated code:\n{}", code_str);

        // fn_name should be treated as Ident (ToTsIdent)
        assert!(
            code_str.contains("to_ts_ident"),
            "fn_name should use ToTsIdent. Generated code:\n{}",
            code_str
        );

        // type_param should be treated as Type (ToTsType)
        assert!(
            code_str.contains("to_ts_type"),
            "type_param should use ToTsType. Generated code:\n{}",
            code_str
        );

        // body_expr should be treated as Expr (ToTsExpr)
        assert!(
            code_str.contains("to_ts_expr"),
            "body_expr should use ToTsExpr. Generated code:\n{}",
            code_str
        );
    }
}
