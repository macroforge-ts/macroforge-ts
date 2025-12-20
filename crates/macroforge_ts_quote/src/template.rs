//! Rust-style templating for TypeScript code generation
//!
//! Provides a template syntax with interpolation and control flow:
//! - `@{expr}` - Interpolate expressions (calls `.to_string()`)
//! - `{| content |}` - Ident block: concatenates content without spaces (e.g., `{|get@{name}|}` → `getUser`)
//! - `{> "comment" <}` - Block comment: outputs `/* comment */` (string preserves whitespace)
//! - `{>> "doc" <<}` - Doc comment: outputs `/** doc */` (string preserves whitespace)
//! - `@@{` - Escape for literal `@{` (e.g., `"@@{foo}"` → `@{foo}`)
//! - `"string @{expr}"` - String interpolation (auto-detected)
//! - `"'^template ${expr}^'"` - JS backtick template literal (outputs `` `template ${expr}` ``)
//! - `{#if cond}...{/if}` - Conditional blocks
//! - `{#if let pattern = expr}...{/if}` - Pattern matching if-let blocks
//! - `{:else}` - Else clause
//! - `{:else if cond}` - Else-if clause
//! - `{#match expr}{:case pattern}...{/match}` - Match blocks with case arms
//! - `{#for item in list}...{/for}` - Iteration
//! - `{#while cond}...{/while}` - While loop
//! - `{#while let pattern = expr}...{/while}` - While-let pattern matching loop
//! - `{$let name = expr}` - Local constants
//! - `{$let mut name = expr}` - Mutable local binding
//! - `{$do expr}` - Execute side-effectful expression (discard result)
//! - `{$typescript stream}` - Inject a TsStream, preserving its source and runtime_patches (imports)
//!
//! Note: A single `@` not followed by `{` passes through unchanged (e.g., `email@domain.com`).

use proc_macro2::{Delimiter, Group, Span, TokenStream as TokenStream2, TokenTree};
use quote::{ToTokens, quote};
use std::iter::Peekable;

/// Helper struct to buffer string literals and merge them before emitting code.
///
/// This optimization reduces the number of `__out.push_str()` calls in the
/// generated code by concatenating adjacent string literals at compile time.
struct LiteralBuffer {
    tokens: TokenStream2,
    pending: String,
}

impl LiteralBuffer {
    fn new() -> Self {
        Self {
            tokens: TokenStream2::new(),
            pending: String::new(),
        }
    }

    fn push_str(&mut self, s: &str) {
        self.pending.push_str(s);
    }

    fn flush(&mut self) {
        if !self.pending.is_empty() {
            let s = &self.pending;
            self.tokens.extend(quote! { __out.push_str(#s); });
            self.pending.clear();
        }
    }

    fn push_stream(&mut self, stream: TokenStream2) {
        self.flush();
        self.tokens.extend(stream);
    }

    fn into_stream(mut self) -> TokenStream2 {
        self.flush();
        self.tokens
    }
}

/// Creates a syntax error with contextual information about the template location.
///
/// This function constructs a [`syn::Error`] with an optional context string that
/// helps users identify where in their template the error occurred.
///
/// # Arguments
///
/// * `span` - The source span for error reporting
/// * `message` - The primary error message
/// * `context` - Optional context showing the problematic template code
///
/// # Returns
///
/// A [`syn::Error`] that can be converted to a compile error via `.to_compile_error()`.
///
/// # Example Output
///
/// ```text
/// error: Unclosed {#if} block: Missing {/if}
///   --> in: {#if condition}...
/// ```
fn template_error(span: Span, message: &str, context: Option<&str>) -> syn::Error {
    let full_message = if let Some(ctx) = context {
        format!("{}\n  --> in: {}", message, ctx)
    } else {
        message.to_string()
    };
    syn::Error::new(span, full_message)
}

/// Entry point for parsing a template token stream.
///
/// This function transforms template tokens into Rust code that builds a TypeScript
/// string at runtime. The generated code handles all interpolation, control flow,
/// and patch collection.
///
/// # Arguments
///
/// * `input` - The raw token stream from the template macro invocation
///
/// # Returns
///
/// On success, returns a [`TokenStream2`] containing Rust code that evaluates to
/// `(String, Vec<Patch>)` - the generated TypeScript source and any runtime patches
/// (imports, etc.) collected from `{$typescript}` injections.
///
/// # Errors
///
/// Returns a [`syn::Error`] if:
/// - A control flow block is unclosed (`{#if}` without `{/if}`)
/// - A terminator appears in an unexpected context
/// - String interpolation syntax is malformed
/// - An expression inside `@{}` cannot be parsed
///
/// # Generated Code Structure
///
/// ```ignore
/// {
///     let mut __out = String::new();
///     let mut __patches: Vec<Patch> = Vec::new();
///     // ... generated string-building code ...
///     (__out, __patches)
/// }
/// ```
pub fn parse_template(input: TokenStream2) -> syn::Result<TokenStream2> {
    // Parse the tokens into a Rust block that returns a String or a templating error
    let (body, _) = parse_fragment(&mut input.into_iter().peekable(), None)?;

    Ok(quote! {
        {
            let mut __out = String::new();
            let mut __patches: Vec<macroforge_ts::ts_syn::abi::Patch> = Vec::new();
            #body
            (__out, __patches)
        }
    })
}

/// Signals that cause the recursive parser to stop and return control.
///
/// When [`parse_fragment`] encounters one of these terminators while parsing,
/// it returns the accumulated output along with the terminator, allowing the
/// caller to handle the control flow appropriately.
///
/// # Variants
///
/// - `Else` - Encountered `{:else}`, signals end of if-true block
/// - `ElseIf(cond)` - Encountered `{:else if cond}`, signals chained conditional
/// - `EndIf` - Encountered `{/if}`, signals end of if block
/// - `EndFor` - Encountered `{/for}`, signals end of for loop
/// - `EndWhile` - Encountered `{/while}`, signals end of while loop
/// - `Case(pattern)` - Encountered `{:case pattern}`, signals new match arm
/// - `EndMatch` - Encountered `{/match}`, signals end of match block
#[derive(Debug, Clone)]
enum Terminator {
    /// `{:else}` - End of if-true block, start of else block
    Else,
    /// `{:else if condition}` - Chained conditional with new condition
    ElseIf(TokenStream2),
    /// `{/if}` - End of entire if/else-if/else chain
    EndIf,
    /// `{/for}` - End of for loop body
    EndFor,
    /// `{/while}` - End of while loop body
    EndWhile,
    /// `{:case pattern}` - Start of new match arm with pattern
    Case(TokenStream2),
    /// `{/match}` - End of entire match block
    EndMatch,
}

/// Classification of brace-delimited groups in the template.
///
/// When the parser encounters a `{ ... }` group, [`analyze_tag`] examines its
/// contents to determine whether it's a template control tag or regular
/// TypeScript code. This enum represents all possible classifications.
///
/// # Control Flow Tags
///
/// Tags starting with `#` introduce control structures:
/// - `{#if}`, `{#if let}` - Conditionals
/// - `{#for}` - Iteration
/// - `{#while}`, `{#while let}` - While loops
/// - `{#match}` - Pattern matching
///
/// Tags starting with `:` are continuations:
/// - `{:else}`, `{:else if}` - Conditional branches
/// - `{:case}` - Match arms
///
/// Tags starting with `/` close blocks:
/// - `{/if}`, `{/for}`, `{/while}`, `{/match}`
///
/// # Action Tags
///
/// Tags starting with `$` perform actions:
/// - `{$let}`, `{$let mut}` - Local bindings
/// - `{$do}` - Side effects
/// - `{$typescript}` - TsStream injection
///
/// # Special Syntax
///
/// - `{| ... |}` - Ident blocks (no-space concatenation)
/// - `{> "..." <}` - Block comments
/// - `{>> "..." <<}` - Doc comments
/// - `{ ... }` (no prefix) - Regular TypeScript blocks
enum TagType {
    /// `{#if condition}` - Start of conditional block
    If(TokenStream2),
    /// `{#if let pattern = expr}` - Pattern-matching conditional (pattern, expression)
    IfLet(TokenStream2, TokenStream2),
    /// `{#while condition}` - Start of while loop
    While(TokenStream2),
    /// `{#while let pattern = expr}` - Pattern-matching while loop (pattern, expression)
    WhileLet(TokenStream2, TokenStream2),
    /// `{#for item in collection}` - Start of for loop (item binding, collection expression)
    For(TokenStream2, TokenStream2),
    /// `{#match expr}` - Start of match block with expression to match
    Match(TokenStream2),
    /// `{:else}` - Else branch of conditional
    Else,
    /// `{:else if condition}` - Else-if branch with condition
    ElseIf(TokenStream2),
    /// `{:case pattern}` - Match arm with pattern
    Case(TokenStream2),
    /// `{/if}` - End of if/else-if/else block
    EndIf,
    /// `{/for}` - End of for loop
    EndFor,
    /// `{/while}` - End of while loop
    EndWhile,
    /// `{/match}` - End of match block
    EndMatch,
    /// `{$let name = expr}` - Immutable local binding
    Let(TokenStream2),
    /// `{$let mut name = expr}` - Mutable local binding
    LetMut(TokenStream2),
    /// `{$do expr}` - Execute expression for side effects (discard result)
    Do(TokenStream2),
    /// `{$typescript stream}` - Inject TsStream, preserving source and patches
    Typescript(TokenStream2),
    /// `{| content |}` - Identifier block with no internal spacing
    IdentBlock,
    /// `{> "comment" <}` - Block comment: outputs `/* comment */`
    BlockComment(String),
    /// `{>> "doc" <<}` - Doc comment: outputs `/** doc */`
    DocComment(String),
    /// Standard TypeScript/JavaScript block (no template syntax detected)
    Block,
}

/// Analyzes a brace-delimited group to determine its tag type.
///
/// This function examines the tokens inside a `{ ... }` group to classify
/// whether it's a template control tag (like `{#if}`, `{#for}`) or just
/// regular TypeScript code that should be passed through.
///
/// # Arguments
///
/// * `g` - The [`Group`] token to analyze (must have brace delimiter)
///
/// # Returns
///
/// A [`TagType`] classification indicating what kind of template construct
/// (if any) the group represents.
///
/// # Recognition Patterns
///
/// The function checks tokens in order of specificity:
///
/// 1. **Ident blocks**: `{| ... |}` - First and last tokens are `|`
/// 2. **Doc comments**: `{>> "..." <<}` - Starts with `>>`, ends with `<<`
/// 3. **Block comments**: `{> "..." <}` - Starts with `>`, ends with `<`
/// 4. **Control tags**: First token is `#`, `:`, `/`, or `$`
///    - `#if`, `#for`, `#while`, `#match` - Block openers
///    - `:else`, `:else if`, `:case` - Continuations
///    - `/if`, `/for`, `/while`, `/match` - Block closers
///    - `$let`, `$do`, `$typescript` - Actions
/// 5. **Plain blocks**: Anything else is treated as TypeScript code
///
/// # Example Classifications
///
/// ```text
/// {#if x > 0}     -> TagType::If(tokens for "x > 0")
/// {#for i in 0..5} -> TagType::For(tokens for "i", tokens for "0..5")
/// {:else}          -> TagType::Else
/// {/if}            -> TagType::EndIf
/// {$let x = 1}     -> TagType::Let(tokens for "x = 1")
/// {| foo @{bar} |} -> TagType::IdentBlock
/// { x: 1 }         -> TagType::Block
/// ```
fn analyze_tag(g: &Group) -> TagType {
    let tokens: Vec<TokenTree> = g.stream().into_iter().collect();

    // Check for {| ... |} ident block - must have at least | and |
    if tokens.len() >= 2
        && let (Some(TokenTree::Punct(first)), Some(TokenTree::Punct(last))) =
            (tokens.first(), tokens.last())
        && first.as_char() == '|'
        && last.as_char() == '|'
    {
        return TagType::IdentBlock;
    }

    // Check for {>> "string" <<} doc comment - must have >> string <<
    if tokens.len() >= 5
        && let (Some(TokenTree::Punct(p1)), Some(TokenTree::Punct(p2))) =
            (tokens.first(), tokens.get(1))
        && p1.as_char() == '>'
        && p2.as_char() == '>'
        && let (Some(TokenTree::Punct(p3)), Some(TokenTree::Punct(p4))) =
            (tokens.get(tokens.len() - 2), tokens.last())
        && p3.as_char() == '<'
        && p4.as_char() == '<'
    {
        // Extract the string literal in the middle
        if let Some(TokenTree::Literal(lit)) = tokens.get(2) {
            let content = extract_string_literal(lit);
            return TagType::DocComment(content);
        }
        // Fallback: join remaining tokens as string (for backwards compat)
        let content = tokens_to_spaced_string(&tokens[2..tokens.len() - 2]);
        return TagType::DocComment(content);
    }

    // Check for {> "string" <} block comment - must have > string <
    if tokens.len() >= 3
        && let (Some(TokenTree::Punct(first)), Some(TokenTree::Punct(last))) =
            (tokens.first(), tokens.last())
        && first.as_char() == '>'
        && last.as_char() == '<'
    {
        // Extract the string literal in the middle
        if let Some(TokenTree::Literal(lit)) = tokens.get(1) {
            let content = extract_string_literal(lit);
            return TagType::BlockComment(content);
        }
        // Fallback: join remaining tokens as string (for backwards compat)
        let content = tokens_to_spaced_string(&tokens[1..tokens.len() - 1]);
        return TagType::BlockComment(content);
    }

    if tokens.len() < 2 {
        return TagType::Block;
    }

    // Check for {# ...} tags
    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '#'
    {
        if i == "if" {
            // Check for {#if let pattern = expr}
            if let Some(TokenTree::Ident(let_kw)) = tokens.get(2)
                && let_kw == "let"
            {
                // Format: {#if let pattern = expr}
                // Split on "=" to separate pattern from expression
                let mut pattern = TokenStream2::new();
                let mut expr = TokenStream2::new();
                let mut seen_eq = false;

                for t in tokens.iter().skip(3) {
                    if let TokenTree::Punct(eq) = t
                        && eq.as_char() == '='
                        && !seen_eq
                    {
                        seen_eq = true;
                        continue;
                    }
                    if !seen_eq {
                        t.to_tokens(&mut pattern);
                    } else {
                        t.to_tokens(&mut expr);
                    }
                }
                return TagType::IfLet(pattern, expr);
            }

            // Format: {#if condition}
            let cond: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::If(cond);
        }

        if i == "match" {
            // Format: {#match expr}
            let expr: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Match(expr);
        }

        if i == "while" {
            // Check for {#while let pattern = expr}
            if let Some(TokenTree::Ident(let_kw)) = tokens.get(2)
                && let_kw == "let"
            {
                let mut pattern = TokenStream2::new();
                let mut expr = TokenStream2::new();
                let mut seen_eq = false;

                for t in tokens.iter().skip(3) {
                    if let TokenTree::Punct(eq) = t
                        && eq.as_char() == '='
                        && !seen_eq
                    {
                        seen_eq = true;
                        continue;
                    }
                    if !seen_eq {
                        t.to_tokens(&mut pattern);
                    } else {
                        t.to_tokens(&mut expr);
                    }
                }
                return TagType::WhileLet(pattern, expr);
            }

            // Simple {#while condition}
            let cond: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::While(cond);
        }

        if i == "for" {
            // Format: {#for item in collection}
            let mut item = TokenStream2::new();
            let mut list = TokenStream2::new();
            let mut seen_in = false;

            // Split on "in" keyword
            for t in tokens.iter().skip(2) {
                if let TokenTree::Ident(id) = t
                    && id == "in"
                    && !seen_in
                {
                    seen_in = true;
                    continue;
                }
                if !seen_in {
                    t.to_tokens(&mut item);
                } else {
                    t.to_tokens(&mut list);
                }
            }
            return TagType::For(item, list);
        }
    }

    // Check for {$ ...} tags (let, let mut, do, typescript)
    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '$'
    {
        if i == "let" {
            // Check for {$let mut name = expr}
            if let Some(TokenTree::Ident(mut_kw)) = tokens.get(2)
                && mut_kw == "mut"
            {
                let body: TokenStream2 =
                    tokens.iter().skip(3).map(|t| t.to_token_stream()).collect();
                return TagType::LetMut(body);
            }
            // Format: {$let name = expr}
            let body: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Let(body);
        }
        if i == "do" {
            // Format: {$do expr} - execute side-effectful expression
            let expr: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Do(expr);
        }
        if i == "typescript" {
            // Format: {$typescript stream_expr}
            let expr: TokenStream2 = tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Typescript(expr);
        }
    }

    // Check for {: ...} tags (else, else if, case)
    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == ':'
    {
        if i == "else" {
            // Check for {:else if condition}
            if let Some(TokenTree::Ident(next)) = tokens.get(2)
                && next == "if"
            {
                let cond: TokenStream2 =
                    tokens.iter().skip(3).map(|t| t.to_token_stream()).collect();
                return TagType::ElseIf(cond);
            }
            return TagType::Else;
        }

        if i == "case" {
            // Format: {:case pattern}
            let pattern: TokenStream2 =
                tokens.iter().skip(2).map(|t| t.to_token_stream()).collect();
            return TagType::Case(pattern);
        }
    }

    // Check for {/ ...} (End tags)
    if let (TokenTree::Punct(p), TokenTree::Ident(i)) = (&tokens[0], &tokens[1])
        && p.as_char() == '/'
    {
        if i == "if" {
            return TagType::EndIf;
        }
        if i == "for" {
            return TagType::EndFor;
        }
        if i == "while" {
            return TagType::EndWhile;
        }
        if i == "match" {
            return TagType::EndMatch;
        }
    }

    TagType::Block
}

/// Parses an if/else-if/else chain and generates the corresponding Rust code.
///
/// This function handles the complete parsing of a conditional block, including
/// any `{:else}` or `{:else if}` continuations. It recursively processes
/// else-if chains to support arbitrarily deep conditional nesting.
///
/// # Arguments
///
/// * `iter` - Mutable iterator over remaining template tokens
/// * `initial_cond` - The condition expression from `{#if condition}`
/// * `open_span` - Span of the opening `{#if}` tag for error reporting
///
/// # Returns
///
/// On success, returns Rust code that implements the conditional:
///
/// ```ignore
/// if condition {
///     // true block
/// } else if other_condition {
///     // else-if block
/// } else {
///     // else block
/// }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The `{#if}` block is never closed with `{/if}`
/// - A `{:else}` block is opened but never closed
///
/// # State Machine
///
/// ```text
/// {#if cond} -> parse true block -> {:else if} -> recurse
///                                -> {:else}    -> parse else block -> {/if}
///                                -> {/if}      -> done (no else)
/// ```
fn parse_if_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    initial_cond: TokenStream2,
    open_span: Span,
) -> syn::Result<TokenStream2> {
    // Parse the true block, stopping at {:else}, {:else if}, or {/if}
    let (true_block, terminator) = parse_fragment(
        iter,
        Some(&[
            Terminator::Else,
            Terminator::ElseIf(TokenStream2::new()),
            Terminator::EndIf,
        ]),
    )?;

    match terminator {
        Some(Terminator::EndIf) => {
            // Simple if without else
            Ok(quote! {
                if #initial_cond {
                    #true_block
                }
            })
        }
        Some(Terminator::Else) => {
            // if with else - parse else block until {/if}
            let (else_block, terminator) = parse_fragment(iter, Some(&[Terminator::EndIf]))?;
            if !matches!(terminator, Some(Terminator::EndIf)) {
                return Err(template_error(
                    open_span,
                    "Unclosed {:else} block: Missing {/if}",
                    Some("{:else}..."),
                ));
            }
            Ok(quote! {
                if #initial_cond {
                    #true_block
                } else {
                    #else_block
                }
            })
        }
        Some(Terminator::ElseIf(else_if_cond)) => {
            // if with else if - recursively parse the else-if chain
            // For the recursive call, we should ideally find the span of the else-if tag.
            // But keeping open_span is acceptable for now as it points to the start of the whole chain.
            let else_if_chain = parse_if_chain(iter, else_if_cond, open_span)?;
            Ok(quote! {
                if #initial_cond {
                    #true_block
                } else {
                    #else_if_chain
                }
            })
        }
        None => Err(template_error(
            open_span,
            "Unclosed {#if} block: Missing {/if}",
            Some("{#if condition}..."),
        )),
        _ => unreachable!(),
    }
}

/// Parses an if-let/else chain and generates the corresponding Rust code.
///
/// This function handles pattern-matching conditionals that use the
/// `{#if let pattern = expr}` syntax. Unlike regular if chains, if-let
/// does not support `{:else if}` continuations (only `{:else}`).
///
/// # Arguments
///
/// * `iter` - Mutable iterator over remaining template tokens
/// * `pattern` - The destructuring pattern (e.g., `Some(value)`)
/// * `expr` - The expression to match against
/// * `open_span` - Span of the opening tag for error reporting
///
/// # Returns
///
/// On success, returns Rust code that implements the pattern match:
///
/// ```ignore
/// if let pattern = expr {
///     // body when pattern matches
/// } else {
///     // optional else body
/// }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The block is never closed with `{/if}`
/// - A `{:else}` block is opened but never closed
///
/// # Example
///
/// ```text
/// {#if let Some(name) = user.name}
///     Hello, @{name}!
/// {:else}
///     Hello, anonymous!
/// {/if}
/// ```
fn parse_if_let_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    pattern: TokenStream2,
    expr: TokenStream2,
    open_span: Span,
) -> syn::Result<TokenStream2> {
    // Parse the true block, stopping at {:else} or {/if}
    let (true_block, terminator) =
        parse_fragment(iter, Some(&[Terminator::Else, Terminator::EndIf]))?;

    match terminator {
        Some(Terminator::EndIf) => {
            // Simple if let without else
            Ok(quote! {
                if let #pattern = #expr {
                    #true_block
                }
            })
        }
        Some(Terminator::Else) => {
            // if let with else - parse else block until {/if}
            let (else_block, terminator) = parse_fragment(iter, Some(&[Terminator::EndIf]))?;
            if !matches!(terminator, Some(Terminator::EndIf)) {
                return Err(template_error(
                    open_span,
                    "Unclosed {:else} block in {#if let}: Missing {/if}",
                    Some("{#if let pattern = expr}{:else}..."),
                ));
            }
            Ok(quote! {
                if let #pattern = #expr {
                    #true_block
                } else {
                    #else_block
                }
            })
        }
        None => Err(template_error(
            open_span,
            "Unclosed {#if let} block: Missing {/if}",
            Some("{#if let pattern = expr}..."),
        )),
        _ => unreachable!(),
    }
}

/// Parses a while loop and generates the corresponding Rust code.
///
/// This function handles `{#while condition}...{/while}` blocks, generating
/// a Rust while loop that conditionally produces template output.
///
/// # Arguments
///
/// * `iter` - Mutable iterator over remaining template tokens
/// * `cond` - The loop condition expression
/// * `open_span` - Span of the opening tag for error reporting
///
/// # Returns
///
/// On success, returns Rust code:
///
/// ```ignore
/// while condition {
///     // loop body (string building code)
/// }
/// ```
///
/// # Errors
///
/// Returns an error if the block is never closed with `{/while}`.
///
/// # Example
///
/// ```text
/// {$let mut i = 0}
/// {#while i < 3}
///     Item @{i}
///     {$do i += 1}
/// {/while}
/// ```
fn parse_while_loop(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    cond: TokenStream2,
    open_span: Span,
) -> syn::Result<TokenStream2> {
    let (body, terminator) = parse_fragment(iter, Some(&[Terminator::EndWhile]))?;

    if !matches!(terminator, Some(Terminator::EndWhile)) {
        return Err(template_error(
            open_span,
            "Unclosed {#while} block: Missing {/while}",
            Some("{#while condition}..."),
        ));
    }

    Ok(quote! {
        while #cond {
            #body
        }
    })
}

/// Parses a while-let loop and generates the corresponding Rust code.
///
/// This function handles `{#while let pattern = expr}...{/while}` blocks,
/// generating a Rust while-let loop that iterates while a pattern matches.
///
/// # Arguments
///
/// * `iter` - Mutable iterator over remaining template tokens
/// * `pattern` - The destructuring pattern to match
/// * `expr` - The expression to match against each iteration
/// * `open_span` - Span of the opening tag for error reporting
///
/// # Returns
///
/// On success, returns Rust code:
///
/// ```ignore
/// while let pattern = expr {
///     // loop body (string building code)
/// }
/// ```
///
/// # Errors
///
/// Returns an error if the block is never closed with `{/while}`.
///
/// # Example
///
/// ```text
/// {#while let Some(item) = iter.next()}
///     Processing @{item}
/// {/while}
/// ```
fn parse_while_let_loop(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    pattern: TokenStream2,
    expr: TokenStream2,
    open_span: Span,
) -> syn::Result<TokenStream2> {
    let (body, terminator) = parse_fragment(iter, Some(&[Terminator::EndWhile]))?;

    if !matches!(terminator, Some(Terminator::EndWhile)) {
        return Err(template_error(
            open_span,
            "Unclosed {#while let} block: Missing {/while}",
            Some("{#while let pattern = expr}..."),
        ));
    }

    Ok(quote! {
        while let #pattern = #expr {
            #body
        }
    })
}

/// Parses a match expression with case arms and generates the corresponding Rust code.
///
/// This function handles `{#match expr}{:case pattern}...{/match}` blocks,
/// generating a Rust match expression with multiple arms. Each `{:case pattern}`
/// starts a new arm.
///
/// # Arguments
///
/// * `iter` - Mutable iterator over remaining template tokens
/// * `match_expr` - The expression being matched against
/// * `open_span` - Span of the opening tag for error reporting
///
/// # Returns
///
/// On success, returns Rust code:
///
/// ```ignore
/// match expr {
///     pattern1 => {
///         // body1 (string building code)
///     }
///     pattern2 => {
///         // body2 (string building code)
///     }
///     // ...
/// }
/// ```
///
/// # Errors
///
/// Returns an error if the block is never closed with `{/match}`.
///
/// # Example
///
/// ```text
/// {#match status}
///     {:case "active"}
///         Status: Active
///     {:case "pending"}
///         Status: Pending
///     {:case _}
///         Status: Unknown
/// {/match}
/// ```
///
/// # Note
///
/// Content before the first `{:case}` is ignored. Each arm's body consists
/// of all content between its `{:case}` tag and the next `{:case}` or `{/match}`.
fn parse_match_arms(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    match_expr: TokenStream2,
    open_span: Span,
) -> syn::Result<TokenStream2> {
    let mut arms = TokenStream2::new();
    let mut current_pattern: Option<TokenStream2> = None;

    loop {
        // Parse until we hit {:case} or {/match}
        let (body, terminator) = parse_fragment(
            iter,
            Some(&[Terminator::Case(TokenStream2::new()), Terminator::EndMatch]),
        )?;

        match terminator {
            Some(Terminator::Case(pattern)) => {
                // If we have a previous pattern, emit its arm with the body we just parsed
                if let Some(prev_pattern) = current_pattern.take() {
                    arms.extend(quote! {
                        #prev_pattern => {
                            #body
                        }
                    });
                }
                // Store this pattern for the next iteration
                current_pattern = Some(pattern);
            }
            Some(Terminator::EndMatch) => {
                // Emit the final arm if we have one
                if let Some(prev_pattern) = current_pattern.take() {
                    arms.extend(quote! {
                        #prev_pattern => {
                            #body
                        }
                    });
                }
                break;
            }
            None => {
                return Err(template_error(
                    open_span,
                    "Unclosed {#match} block: Missing {/match}",
                    Some("{#match expr}{:case pattern}...{/match}"),
                ));
            }
            _ => unreachable!(),
        }
    }

    Ok(quote! {
        match #match_expr {
            #arms
        }
    })
}

/// Parses tokens without adding spaces between them.
///
/// This is a specialized parser for `{| ... |}` ident blocks where tokens
/// should be concatenated without any whitespace. This is essential for
/// building compound identifiers or strings from multiple parts.
///
/// # Arguments
///
/// * `iter` - Mutable iterator over the tokens inside the ident block
///
/// # Returns
///
/// Rust code that builds a string by concatenating all tokens without spaces.
///
/// # Differences from `parse_fragment`
///
/// - No automatic spacing between tokens
/// - No handling of control flow tags (they would break ident generation)
/// - Simplified interpolation handling (just `@{expr}`)
///
/// # Example
///
/// ```text
/// {| get @{field_name} |}
/// // Produces: "getfieldName" (if field_name = "fieldName")
/// // NOT: "get fieldName " (with spaces)
/// ```
fn parse_fragment_no_spacing(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
) -> syn::Result<TokenStream2> {
    let mut buffer = LiteralBuffer::new();

    while let Some(token) = iter.peek().cloned() {
        match &token {
            // Handle @{ expr } interpolation
            TokenTree::Punct(p) if p.as_char() == '@' => {
                iter.next(); // Consume '@'

                let is_group = matches!(iter.peek(), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace);

                if is_group {
                    if let Some(TokenTree::Group(g)) = iter.next() {
                        let content = g.stream();
                        buffer.push_stream(quote! {
                            __out.push_str(&#content.to_string());
                        });
                    }
                } else {
                    buffer.push_str("@");
                }
            }

            // Handle nested groups (but not ident blocks - those are already consumed)
            TokenTree::Group(g) => {
                iter.next();
                let (open, close) = match g.delimiter() {
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::Brace => ("{", "}"),
                    Delimiter::None => ("", ""),
                };
                buffer.push_str(open);
                let inner = parse_fragment_no_spacing(&mut g.stream().into_iter().peekable())?;
                buffer.push_stream(inner);
                buffer.push_str(close);
            }

            // All other tokens - just emit, no spacing
            _ => {
                let t = iter.next().unwrap();
                let s = t.to_string();
                buffer.push_str(&s);
                // NO space added - that's the point of ident blocks
            }
        }
    }

    Ok(buffer.into_stream())
}

/// Main recursive parser that processes template tokens and generates string-building code.
///
/// This is the core parsing function that walks through template tokens, generating
/// Rust code that builds the output string at runtime. It handles all template
/// syntax including interpolation, control flow, and special blocks.
///
/// # Arguments
///
/// * `iter` - Mutable iterator over template tokens to process
/// * `stop_at` - Optional list of terminators that should cause this function to return.
///   Used by control flow parsers to stop at `{:else}`, `{/if}`, etc.
///
/// # Returns
///
/// A tuple of:
/// 1. Rust code that builds the string (appends to `__out`)
/// 2. The terminator that caused parsing to stop, or `None` if EOF was reached
///
/// # Processing Cases
///
/// The function handles tokens in this order of priority:
///
/// 1. **Interpolation** (`@{expr}`) - Calls `expr.to_string()` and appends
/// 2. **Brace groups** (`{...}`) - Analyzed via [`analyze_tag`]:
///    - Control flow tags spawn sub-parsers
///    - Terminators cause early return
///    - Plain blocks are recursively processed
/// 3. **Other groups** (`(...)`, `[...]`) - Recursively processed
/// 4. **Backtick templates** (`"'^...^'"`) - Processed for `@{}` interpolation
/// 5. **String literals** - Checked for `@{}` interpolation
/// 6. **Plain tokens** - Appended with intelligent spacing
///
/// # Spacing Logic
///
/// The parser adds spaces between tokens following these rules:
/// - Space after identifiers (unless followed by punctuation or groups)
/// - No space after `.`, `!`, `(`, `[`, `{`, `<`, `>`, `@`, `$`
/// - No space before `)`, `]`, `}`, `>`, `.`, `,`, `;`
/// - Joint punctuation (like `::`) stays together
///
/// For explicit no-space concatenation, use `{| ... |}` ident blocks.
fn parse_fragment(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    stop_at: Option<&[Terminator]>,
) -> syn::Result<(TokenStream2, Option<Terminator>)> {
    let mut buffer = LiteralBuffer::new();

    while let Some(token) = iter.peek().cloned() {
        match &token {
            // Case 1: Interpolation @{ expr }
            TokenTree::Punct(p) if p.as_char() == '@' => {
                // Check if the NEXT token is a Group { ... }
                let p_clone = p.clone();
                iter.next(); // Consume '@'

                // Look ahead
                let is_group = matches!(iter.peek(), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace);

                if is_group {
                    // It IS interpolation: @{ expr }
                    if let Some(TokenTree::Group(g)) = iter.next() {
                        let content = g.stream();
                        buffer.push_stream(quote! {
                            __out.push_str(&#content.to_string());
                        });
                    }
                } else {
                    // It is just a literal '@'
                    let s = p_clone.to_string();
                    buffer.push_str(&s);
                }

                // Spacing logic after interpolation: always add space unless followed by punctuation
                // Use {| |} for explicit no-space concatenation
                let next = iter.peek();
                let next_char = match next {
                    Some(TokenTree::Punct(p)) => Some(p.as_char()),
                    _ => None,
                };

                let mut add_space = true;

                // No space at end of stream (group delimiter like ) will follow)
                if next.is_none() {
                    add_space = false;
                }

                // No space before punctuation
                if matches!(next_char, Some(c) if ".,;:?()[]{}<>!".contains(c)) {
                    add_space = false;
                }

                // No space before ( or [ groups (function calls, indexing)
                if let Some(TokenTree::Group(g)) = next {
                    match g.delimiter() {
                        Delimiter::Parenthesis | Delimiter::Bracket => add_space = false,
                        _ => {}
                    }
                }

                // No space if followed by @{ interpolation (for @{a}@{b} patterns)
                if let Some(TokenTree::Punct(p)) = next
                    && p.as_char() == '@'
                {
                    // Peek ahead to check if it's @{ (interpolation)
                    let mut peek_iter = iter.clone();
                    peek_iter.next(); // skip the @
                    if matches!(peek_iter.peek(), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace)
                    {
                        add_space = false;
                    }
                }

                if add_space {
                    buffer.push_str(" ");
                }
            }

            // Case 2: Groups { ... } - Could be Tag or Block
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => {
                let tag = analyze_tag(g);
                let span = g.span();

                match tag {
                    TagType::If(cond) => {
                        iter.next(); // Consume {#if}
                        buffer.push_stream(parse_if_chain(iter, cond, span)?);
                    }
                    TagType::IfLet(pattern, expr) => {
                        iter.next(); // Consume {#if let}
                        buffer.push_stream(parse_if_let_chain(iter, pattern, expr, span)?);
                    }
                    TagType::For(item, list) => {
                        iter.next(); // Consume {#for}

                        let (body, terminator) = parse_fragment(iter, Some(&[Terminator::EndFor]))?;
                        if !matches!(terminator, Some(Terminator::EndFor)) {
                            return Err(template_error(
                                span,
                                "Unclosed {#for} block: Missing {/for}",
                                Some("{#for item in collection}..."),
                            ));
                        }

                        buffer.push_stream(quote! {
                            for #item in #list {
                                #body
                            }
                        });
                    }
                    TagType::Match(expr) => {
                        iter.next(); // Consume {#match}
                        buffer.push_stream(parse_match_arms(iter, expr, span)?);
                    }
                    TagType::While(cond) => {
                        iter.next(); // Consume {#while}
                        buffer.push_stream(parse_while_loop(iter, cond, span)?);
                    }
                    TagType::WhileLet(pattern, expr) => {
                        iter.next(); // Consume {#while let}
                        buffer.push_stream(parse_while_let_loop(iter, pattern, expr, span)?);
                    }
                    TagType::Else => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::Else))
                        {
                            iter.next(); // Consume
                            return Ok((buffer.into_stream(), Some(Terminator::Else)));
                        }
                        return Err(template_error(
                            span,
                            "Unexpected {:else} - not inside an {#if} block",
                            None,
                        ));
                    }
                    TagType::ElseIf(cond) => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::ElseIf(_)))
                        {
                            iter.next(); // Consume
                            return Ok((buffer.into_stream(), Some(Terminator::ElseIf(cond))));
                        }
                        return Err(template_error(
                            span,
                            "Unexpected {:else if} - not inside an {#if} block",
                            None,
                        ));
                    }
                    TagType::EndIf => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndIf))
                        {
                            iter.next(); // Consume
                            return Ok((buffer.into_stream(), Some(Terminator::EndIf)));
                        }
                        return Err(template_error(
                            span,
                            "Unexpected {/if} - no matching {#if} block",
                            None,
                        ));
                    }
                    TagType::EndFor => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndFor))
                        {
                            iter.next(); // Consume
                            return Ok((buffer.into_stream(), Some(Terminator::EndFor)));
                        }
                        return Err(template_error(
                            span,
                            "Unexpected {/for} - no matching {#for} block",
                            None,
                        ));
                    }
                    TagType::EndWhile => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndWhile))
                        {
                            iter.next(); // Consume
                            return Ok((buffer.into_stream(), Some(Terminator::EndWhile)));
                        }
                        return Err(template_error(
                            span,
                            "Unexpected {/while} - no matching {#while} block",
                            None,
                        ));
                    }
                    TagType::Case(pattern) => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::Case(_)))
                        {
                            iter.next(); // Consume
                            return Ok((buffer.into_stream(), Some(Terminator::Case(pattern))));
                        }
                        return Err(template_error(
                            span,
                            "Unexpected {:case} - not inside a {#match} block",
                            None,
                        ));
                    }
                    TagType::EndMatch => {
                        if let Some(stops) = stop_at
                            && stops.iter().any(|s| matches!(s, Terminator::EndMatch))
                        {
                            iter.next(); // Consume
                            return Ok((buffer.into_stream(), Some(Terminator::EndMatch)));
                        }
                        return Err(template_error(
                            span,
                            "Unexpected {/match} - no matching {#match} block",
                            None,
                        ));
                    }
                    TagType::Let(body) => {
                        iter.next(); // Consume {$let ...}
                        buffer.push_stream(quote! {
                            let #body;
                        });
                    }
                    TagType::LetMut(body) => {
                        iter.next(); // Consume {$let mut ...}
                        buffer.push_stream(quote! {
                            let mut #body;
                        });
                    }
                    TagType::Do(expr) => {
                        iter.next(); // Consume {$do ...}
                        buffer.push_stream(quote! {
                            #expr;
                        });
                    }
                    TagType::Typescript(expr) => {
                        iter.next(); // Consume {$typescript ...}
                        buffer.push_stream(quote! {
                            {
                                let __ts_stream = #expr;
                                __out.push_str(__ts_stream.source());
                                __patches.extend(__ts_stream.runtime_patches);
                            }
                        });
                    }
                    TagType::IdentBlock => {
                        iter.next(); // Consume {| ... |}

                        // Get the content between the | markers
                        let inner_tokens: Vec<TokenTree> = g.stream().into_iter().collect();
                        // Skip first | and last |, extract content in between
                        if inner_tokens.len() >= 2 {
                            let content: TokenStream2 = inner_tokens[1..inner_tokens.len() - 1]
                                .iter()
                                .map(|t| t.to_token_stream())
                                .collect();

                            // Parse with no-spacing mode
                            let inner_output =
                                parse_fragment_no_spacing(&mut content.into_iter().peekable())?;
                            buffer.push_stream(inner_output);
                        }

                        // Spacing logic after ident block - same as @{} interpolation
                        // Add space unless followed by punctuation or function call
                        let next = iter.peek();
                        let next_char = match next {
                            Some(TokenTree::Punct(p)) => Some(p.as_char()),
                            _ => None,
                        };

                        let mut add_space = true;

                        // No space at end of stream
                        if next.is_none() {
                            add_space = false;
                        }

                        // No space before punctuation
                        if matches!(next_char, Some(c) if ".,;:?()[]{}<>!".contains(c)) {
                            add_space = false;
                        }

                        // No space before ( or [ groups (function calls, indexing)
                        if let Some(TokenTree::Group(g)) = next {
                            match g.delimiter() {
                                Delimiter::Parenthesis | Delimiter::Bracket => add_space = false,
                                _ => {}
                            }
                        }

                        // No space if followed by @{ interpolation (for {|a|}@{b} concatenation)
                        if let Some(TokenTree::Punct(p)) = next
                            && p.as_char() == '@'
                        {
                            // Peek ahead to check if it's @{ (interpolation)
                            let mut peek_iter = iter.clone();
                            peek_iter.next(); // skip the @
                            if matches!(peek_iter.peek(), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace)
                            {
                                add_space = false;
                            }
                        }

                        if add_space {
                            buffer.push_str(" ");
                        }
                    }
                    TagType::BlockComment(content) => {
                        iter.next(); // Consume {> "..." <}
                        buffer.push_str("/* ");
                        buffer.push_str(&content);
                        buffer.push_str(" */");
                    }
                    TagType::DocComment(content) => {
                        iter.next(); // Consume {>> "..." <<}
                        buffer.push_str("/** ");
                        buffer.push_str(&content);
                        buffer.push_str(" */");
                    }
                    TagType::Block => {
                        // Regular TS Block { ... }
                        // Recurse to allow macros inside standard TS objects
                        iter.next(); // Consume
                        let inner_stream = g.stream();

                        buffer.push_str("{");
                        let (inner_parsed, _) =
                            parse_fragment(&mut inner_stream.into_iter().peekable(), None)?;
                        buffer.push_stream(inner_parsed);
                        buffer.push_str("}");
                    }
                }
            }

            // Case 3: Other groups (parentheses, brackets)
            TokenTree::Group(g) => {
                iter.next();
                let (open, close) = match g.delimiter() {
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::Brace => ("{", "}"), // Shouldn't reach here
                    Delimiter::None => ("", ""),
                };

                buffer.push_str(open);
                let (inner_parsed, _) =
                    parse_fragment(&mut g.stream().into_iter().peekable(), None)?;
                buffer.push_stream(inner_parsed);
                buffer.push_str(close);
            }

            // Case 4a: Backtick template literals "'^...^'" -> `...`
            TokenTree::Literal(lit) if is_backtick_template(lit) => {
                iter.next(); // Consume
                let processed = process_backtick_template(lit)?;
                buffer.push_stream(processed);
                buffer.push_str(" ");
            }

            // Case 4b: String literals with interpolation
            TokenTree::Literal(lit) if is_string_literal(lit) => {
                iter.next(); // Consume
                let interpolated = interpolate_string_literal(lit)?;
                buffer.push_stream(interpolated);
                buffer.push_str(" ");
            }

            // Case 5: Plain Text
            _ => {
                let t = iter.next().unwrap();
                let s = t.to_string();

                // Analyze current token
                let is_ident = matches!(&t, TokenTree::Ident(_));
                let punct_char = if let TokenTree::Punct(p) = &t {
                    Some(p.as_char())
                } else {
                    None
                };
                let is_joint = if let TokenTree::Punct(p) = &t {
                    p.spacing() == proc_macro2::Spacing::Joint
                } else {
                    false
                };

                // Analyze next token
                let next = iter.peek();
                let next_char = match next {
                    Some(TokenTree::Punct(p)) => Some(p.as_char()),
                    _ => None,
                };

                // Emit token string
                buffer.push_str(&s);

                // Decide whether to append a space
                // Simplified: always add space unless followed by punctuation
                // Use {| |} for explicit no-space concatenation
                let mut add_space = true;

                // No space at end of stream (group delimiter like ) will follow)
                if next.is_none() || is_joint {
                    add_space = false;
                } else if is_ident {
                    // Identifiers need space, EXCEPT when followed by punctuation or groups
                    if matches!(next_char, Some(c) if ".,;:?()[]{}<>!".contains(c)) {
                        add_space = false;
                    } else if let Some(TokenTree::Group(g)) = next {
                        match g.delimiter() {
                            Delimiter::Parenthesis | Delimiter::Bracket => add_space = false,
                            _ => {}
                        }
                    }
                    // Always add space before @ interpolation (use {| |} for concatenation)
                } else if let Some(c) = punct_char {
                    // Punctuation specific rules
                    match c {
                        '.' => add_space = false,             // obj.prop
                        '!' => add_space = false,             // !unary or non-null!
                        '(' | '[' | '{' => add_space = false, // Openers: (expr)
                        '<' | '>' => add_space = false, // Generics: Type<T> or T>(...) (compact)
                        '@' => add_space = false,       // Decorator: @Dec
                        '$' => add_space = false,       // Svelte runes: $state, $derived
                        _ => {}
                    }

                    // Never add space if next is a closing delimiter or separator
                    if matches!(next_char, Some(nc) if ".,;)]}>".contains(nc)) {
                        add_space = false;
                    }
                } else {
                    // Groups/Literals
                    // Prevent space if next is punctuation like . , ; ) ] >
                    if matches!(next_char, Some(nc) if ".,;)]}>".contains(nc)) {
                        add_space = false;
                    }
                }

                if add_space {
                    buffer.push_str(" ");
                }
            }
        }
    }

    Ok((buffer.into_stream(), None))
}

/// Converts a slice of tokens to a space-separated string.
///
/// This is a simple utility for converting token sequences to strings
/// when the exact formatting doesn't matter (e.g., for fallback content
/// in comment blocks).
///
/// # Arguments
///
/// * `tokens` - Slice of tokens to convert
///
/// # Returns
///
/// A string with each token separated by a single space.
///
/// # Example
///
/// ```ignore
/// let tokens = vec![ident("hello"), punct(','), ident("world")];
/// assert_eq!(tokens_to_spaced_string(&tokens), "hello , world");
/// ```
fn tokens_to_spaced_string(tokens: &[TokenTree]) -> String {
    let mut result = String::new();
    for (i, token) in tokens.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push_str(&token.to_string());
    }
    result
}

/// Extracts the inner content from a string literal, removing quotes and unescaping.
///
/// This function handles multiple string literal formats:
/// - Regular strings: `"hello"` → `hello`
/// - Raw strings: `r"hello"` → `hello`
/// - Raw strings with hashes: `r#"hello"#` → `hello`
///
/// # Arguments
///
/// * `lit` - The string literal token to extract from
///
/// # Returns
///
/// The unquoted and unescaped string content.
///
/// # Processing
///
/// For regular strings, escape sequences are processed via [`unescape_string`]:
/// - `\n` → newline
/// - `\t` → tab
/// - `\"` → quote
/// - etc.
///
/// Raw strings are returned verbatim (no escape processing needed).
fn extract_string_literal(lit: &proc_macro2::Literal) -> String {
    let s = lit.to_string();
    // Handle regular strings "..."
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        // Unescape the string content
        let inner = &s[1..s.len() - 1];
        return unescape_string(inner);
    }
    // Handle raw strings r"..." or r#"..."#
    if s.starts_with("r\"") && s.ends_with('"') {
        return s[2..s.len() - 1].to_string();
    }
    if s.starts_with("r#\"") && s.ends_with("\"#") {
        return s[3..s.len() - 2].to_string();
    }
    // Fallback: return as-is
    s
}

/// Unescapes common escape sequences in a string.
///
/// This function processes backslash escape sequences commonly found in
/// string literals, converting them to their actual character representations.
///
/// # Arguments
///
/// * `s` - The string with potential escape sequences
///
/// # Returns
///
/// The string with escape sequences replaced by their actual characters.
///
/// # Supported Escapes
///
/// | Escape | Result |
/// |--------|--------|
/// | `\n` | Newline |
/// | `\r` | Carriage return |
/// | `\t` | Tab |
/// | `\\` | Backslash |
/// | `\"` | Double quote |
/// | `\'` | Single quote |
///
/// Unknown escape sequences are preserved as-is (e.g., `\x` becomes `\x`).
fn unescape_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Checks if a literal token represents a string literal.
///
/// This function identifies various string literal formats in Rust:
/// - Double-quoted strings: `"hello"`
/// - Single-quoted strings: `'c'` (char literals, treated as strings here)
/// - Raw strings: `r"hello"` or `r#"hello"#`
///
/// # Arguments
///
/// * `lit` - The literal token to check
///
/// # Returns
///
/// `true` if the literal is a string-like literal that may contain
/// interpolation patterns (`@{expr}`), `false` otherwise.
fn is_string_literal(lit: &proc_macro2::Literal) -> bool {
    let s = lit.to_string();
    s.starts_with('"') || s.starts_with('\'') || s.starts_with("r\"") || s.starts_with("r#")
}

/// Checks if a literal is a backtick template literal marker.
///
/// The syntax `"'^...^'"` is used to generate JavaScript template literals
/// with backticks. This allows embedding JS `${...}` interpolation while
/// also supporting Rust `@{...}` interpolation.
///
/// # Arguments
///
/// * `lit` - The literal token to check
///
/// # Returns
///
/// `true` if the literal matches the backtick template pattern:
/// - `"'^content^'"` - Regular string with markers
/// - `r"'^content^'"` - Raw string with markers
/// - `r#"'^content^'"#` - Raw string with hashes
///
/// # Example
///
/// ```text
/// "'^Hello ${name}^'"   -> `Hello ${name}`
/// "'^<@{tag}>^'"        -> `<div>` (if tag = "div")
/// ```
fn is_backtick_template(lit: &proc_macro2::Literal) -> bool {
    let s = lit.to_string();
    // Check for "'^...^'" pattern (the outer quotes are part of the Rust string)
    if s.starts_with("\"'^") && s.ends_with("^'\"") && s.len() >= 6 {
        return true;
    }
    // Also support raw strings: r"'^...^'" or r#"'^...^'"#
    if s.starts_with("r\"'^") && s.ends_with("^'\"") {
        return true;
    }
    if s.starts_with("r#\"'^") && s.ends_with("^'\"#") {
        return true;
    }
    false
}

/// Processes a backtick template literal and generates string-building code.
///
/// This function transforms the `"'^...^'"` syntax into JavaScript template
/// literals (backticks). It handles both JavaScript `${...}` interpolation
/// (passed through) and Rust `@{expr}` interpolation (evaluated at runtime).
///
/// # Arguments
///
/// * `lit` - The literal token containing the backtick template
///
/// # Returns
///
/// Rust code that builds the template literal string, wrapped in backticks.
///
/// # Processing
///
/// 1. Extracts content between `'^` and `^'` markers
/// 2. Outputs opening backtick
/// 3. Scans for `@{...}` patterns and generates interpolation code
/// 4. Handles `@@` escape for literal `@`
/// 5. Passes through `${...}` as-is for JS runtime interpolation
/// 6. Outputs closing backtick
///
/// # Errors
///
/// Returns an error if:
/// - Control flow tags (`{#...}`, `{/...}`, `{:...}`) are found inside
/// - `@{...}` interpolation is unclosed
/// - Expression inside `@{...}` cannot be parsed
///
/// # Example
///
/// ```text
/// Input:  "'^<@{tag}>${content}</@{tag}>^'"
/// Output: `<div>${content}</div>` (if tag = "div")
/// ```
fn process_backtick_template(lit: &proc_macro2::Literal) -> syn::Result<TokenStream2> {
    let raw = lit.to_string();
    let span = lit.span();

    // Extract content between '^...^' markers
    let content = if raw.starts_with("\"'^") && raw.ends_with("^'\"") {
        &raw[3..raw.len() - 3]
    } else if raw.starts_with("r\"'^") && raw.ends_with("^'\"") {
        &raw[4..raw.len() - 3]
    } else if raw.starts_with("r#\"'^") && raw.ends_with("^'\"#") {
        &raw[5..raw.len() - 4]
    } else {
        return Ok(quote! { __out.push_str(#raw); });
    };

    // Check for common mistakes: control flow tags inside template strings
    if content.contains("{#") || content.contains("{/") || content.contains("{:") {
        return Err(template_error(
            span,
            "Template control flow tags cannot be used inside backtick template literals",
            Some(&format!(
                "\"'^{}...^'\"",
                content.chars().take(40).collect::<String>()
            )),
        ));
    }

    // Check if there are any @{} interpolations or @@ escapes
    if !content.contains('@') {
        // No @ at all, output the backtick string as-is
        // The content may contain ${} for JS interpolation, which passes through
        let mut buffer = LiteralBuffer::new();
        buffer.push_str("`");
        buffer.push_str(content);
        buffer.push_str("`");
        return Ok(buffer.into_stream());
    }

    // Handle @{} Rust interpolations and @@ escapes within the backtick template
    let mut buffer = LiteralBuffer::new();
    buffer.push_str("`");

    let mut chars = content.chars().peekable();
    let mut current_literal = String::new();
    let mut char_pos = 0usize;

    while let Some(c) = chars.next() {
        char_pos += 1;
        if c == '@' {
            match chars.peek() {
                Some(&'@') => {
                    // @@ -> literal @
                    chars.next(); // Consume second @
                    char_pos += 1;
                    current_literal.push('@');
                }
                Some(&'{') => {
                    // @{ -> interpolation
                    // Flush current literal
                    if !current_literal.is_empty() {
                        buffer.push_str(&current_literal);
                        current_literal.clear();
                    }

                    chars.next(); // Consume '{'
                    char_pos += 1;
                    let expr_start_pos = char_pos;

                    // Collect expression until matching '}'
                    let mut expr_str = String::new();
                    let mut brace_depth = 1;

                    for ec in chars.by_ref() {
                        char_pos += 1;
                        if ec == '{' {
                            brace_depth += 1;
                            expr_str.push(ec);
                        } else if ec == '}' {
                            brace_depth -= 1;
                            if brace_depth == 0 {
                                break;
                            }
                            expr_str.push(ec);
                        } else {
                            expr_str.push(ec);
                        }
                    }

                    // Check for unclosed brace
                    if brace_depth != 0 {
                        return Err(template_error(
                            span,
                            &format!(
                                "Unclosed @{{}} interpolation at position {}",
                                expr_start_pos
                            ),
                            Some(&format!("@{{{}", expr_str)),
                        ));
                    }

                    // Parse the expression and generate interpolation code
                    match syn::parse_str::<syn::Expr>(&expr_str) {
                        Ok(expr) => {
                            buffer.push_stream(quote! {
                                __out.push_str(&#expr.to_string());
                            });
                        }
                        Err(parse_err) => {
                            return Err(template_error(
                                span,
                                &format!(
                                    "Invalid Rust expression in backtick template interpolation: {}",
                                    parse_err
                                ),
                                Some(&format!("@{{{}}}", expr_str)),
                            ));
                        }
                    }
                }
                _ => {
                    // Just a literal @
                    current_literal.push('@');
                }
            }
        } else {
            current_literal.push(c);
        }
    }

    // Flush remaining literal
    if !current_literal.is_empty() {
        buffer.push_str(&current_literal);
    }

    buffer.push_str("`");
    Ok(buffer.into_stream())
}

/// Processes a string literal and handles `@{expr}` interpolations inside it.
///
/// This function allows Rust expressions to be interpolated within string
/// literals in the template. It generates code that builds the string at
/// runtime, inserting evaluated expressions where `@{...}` patterns appear.
///
/// # Arguments
///
/// * `lit` - The string literal token to process
///
/// # Returns
///
/// Rust code that builds the string with interpolated values.
///
/// # Processing
///
/// 1. Determines quote character (`"` or `'`) and extracts content
/// 2. If no `@` is present, outputs the literal unchanged
/// 3. Otherwise, scans for `@{...}` patterns:
///    - `@{expr}` → evaluates `expr.to_string()` and inserts result
///    - `@@` → outputs literal `@` (escape sequence)
///    - Lone `@` → outputs literal `@`
/// 4. Preserves escape sequences (`\n`, `\t`, etc.) by passing through
///
/// # Errors
///
/// Returns an error if:
/// - Control flow tags are found inside the string (not supported)
/// - `@{...}` interpolation is unclosed
/// - Expression inside `@{...}` cannot be parsed
///
/// # Example
///
/// ```text
/// Input:  "Hello @{name}, you have @{count} messages"
/// Output: "Hello Alice, you have 5 messages" (at runtime)
/// ```
fn interpolate_string_literal(lit: &proc_macro2::Literal) -> syn::Result<TokenStream2> {
    let raw = lit.to_string();
    let span = lit.span();

    // Determine quote character and extract content
    let (quote_char, content) = if raw.starts_with('"') {
        ('"', &raw[1..raw.len() - 1])
    } else if raw.starts_with('\'') {
        ('\'', &raw[1..raw.len() - 1])
    } else if raw.starts_with("r\"") {
        // Raw string r"..."
        ('"', &raw[2..raw.len() - 1])
    } else if raw.starts_with("r#") {
        // Raw string r#"..."# - find the actual content
        let hash_count = raw[1..].chars().take_while(|&c| c == '#').count();
        let start = 2 + hash_count; // r + # + "
        let end = raw.len() - 1 - hash_count; // " + #
        ('"', &raw[start..end])
    } else {
        // Not a string we recognize, just output as-is
        return Ok(quote! { __out.push_str(#raw); });
    };

    // Check if there are any interpolations or escapes
    if !content.contains('@') {
        // No @ at all, output the string as-is
        return Ok(quote! { __out.push_str(#raw); });
    }

    // Check for common mistakes: control flow tags inside strings
    if content.contains("{#") || content.contains("{/") || content.contains("{:") {
        return Err(template_error(
            span,
            "Template control flow tags cannot be used inside string literals",
            Some(&format!(
                "\"{}...\"",
                content.chars().take(40).collect::<String>()
            )),
        ));
    }

    // Parse and interpolate
    let mut buffer = LiteralBuffer::new();
    let quote_str = quote_char.to_string();
    buffer.push_str(&quote_str);

    let mut chars = content.chars().peekable();
    let mut current_literal = String::new();
    let mut char_pos = 0usize;

    while let Some(c) = chars.next() {
        char_pos += 1;
        if c == '@' {
            match chars.peek() {
                Some(&'@') => {
                    // @@ -> literal @
                    chars.next(); // Consume second @
                    char_pos += 1;
                    current_literal.push('@');
                }
                Some(&'{') => {
                    // @{ -> interpolation
                    // Flush current literal
                    if !current_literal.is_empty() {
                        buffer.push_str(&current_literal);
                        current_literal.clear();
                    }

                    chars.next(); // Consume '{'
                    char_pos += 1;
                    let expr_start_pos = char_pos;

                    // Collect expression until matching '}'
                    let mut expr_str = String::new();
                    let mut brace_depth = 1;

                    for ec in chars.by_ref() {
                        char_pos += 1;
                        if ec == '{' {
                            brace_depth += 1;
                            expr_str.push(ec);
                        } else if ec == '}' {
                            brace_depth -= 1;
                            if brace_depth == 0 {
                                break;
                            }
                            expr_str.push(ec);
                        } else {
                            expr_str.push(ec);
                        }
                    }

                    // Check for unclosed brace
                    if brace_depth != 0 {
                        return Err(template_error(
                            span,
                            &format!(
                                "Unclosed @{{}} interpolation at position {}",
                                expr_start_pos
                            ),
                            Some(&format!("@{{{}", expr_str)),
                        ));
                    }

                    // Parse the expression and generate interpolation code
                    match syn::parse_str::<syn::Expr>(&expr_str) {
                        Ok(expr) => {
                            buffer.push_stream(quote! {
                                __out.push_str(&#expr.to_string());
                            });
                        }
                        Err(parse_err) => {
                            return Err(template_error(
                                span,
                                &format!(
                                    "Invalid Rust expression in string interpolation: {}",
                                    parse_err
                                ),
                                Some(&format!("@{{{}}}", expr_str)),
                            ));
                        }
                    }
                }
                _ => {
                    // Just a literal @
                    current_literal.push('@');
                }
            }
        } else if c == '\\' {
            // Handle escape sequences - pass through as-is
            current_literal.push(c);
            if chars.peek().is_some() {
                current_literal.push(chars.next().unwrap());
                char_pos += 1;
            }
        } else {
            current_literal.push(c);
        }
    }

    // Flush remaining literal
    if !current_literal.is_empty() {
        buffer.push_str(&current_literal);
    }

    buffer.push_str(&quote_str);

    Ok(buffer.into_stream())
}
