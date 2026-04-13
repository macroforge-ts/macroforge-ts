//! Orchestrates `@buildtime` discovery → sandbox invocation → patch emission.
//!
//! The pre-pass is pure: it owns no state beyond the sandbox it's given,
//! and it returns a [`PrepassOutput`] that the host splices into the
//! normal patch pipeline. Errors never propagate up through `anyhow` —
//! every failure becomes a [`Diagnostic`] attached to the offending
//! declaration, so a broken `@buildtime` block fails the user's build
//! with a useful message rather than crashing the compiler.

use std::path::{Path, PathBuf};

use oxc::ast::ast::Program;

use crate::host::buildtime::discovery::{BuildtimeDecl, BuildtimeKind, Visibility, discover};
use crate::host::buildtime::sandbox::{
    BuildtimeSandbox, SandboxError, SandboxOptions, SandboxValue,
};
use crate::host::buildtime::serialize::{SerializeError, value_to_ts_source};
use crate::host::patch_applicator::PatchApplicator;
use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, Patch};

/// Output of the buildtime pre-pass.
#[derive(Debug, Clone)]
pub struct PrepassOutput {
    /// Rewritten source, or `None` if no `@buildtime` declarations were
    /// found (or all of them produced errors that meant no patch was
    /// emitted). Callers feed this into the next pre-pass stage.
    pub rewritten: Option<String>,
    /// Absolute paths of every file the sandbox read during evaluation.
    /// Used by the host to wire up HMR watchers and invalidate the
    /// on-disk cache when any of them change.
    pub dependencies: Vec<PathBuf>,
    /// Diagnostics to surface to the user. Errors come from the sandbox
    /// (throw, timeout, capability-denied) or from the serializer
    /// (unrepresentable result).
    pub diagnostics: Vec<Diagnostic>,
}

impl PrepassOutput {
    /// True if no `@buildtime` declarations were found. The host uses
    /// this to skip the extra allocator scope that re-parses the
    /// (unchanged) source for the declarative pre-pass.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.rewritten.is_none() && self.diagnostics.is_empty()
    }
}

/// Run the pre-pass against `source`.
///
/// * `program` — already-parsed OXC AST. The host reuses its allocator.
/// * `source` — original TS source text, used for span-to-text lookups.
/// * `origin_path` — absolute file path; surfaced inside the sandbox
///   as `buildtime.location.file`.
/// * `sandbox` — backend implementation. Typically
///   [`crate::host::buildtime::default_backend`].
/// * `base_options` — capability/timeout/heap config, built from the
///   user's `macroforge.config.{js,ts}`. Each declaration gets a copy
///   with its `source_file` and `source_line` overridden to point at
///   the declaration.
pub fn run_prepass(
    program: &Program<'_>,
    source: &str,
    origin_path: &Path,
    sandbox: &dyn BuildtimeSandbox,
    base_options: &SandboxOptions,
) -> PrepassOutput {
    let decls = discover(program, source);
    if decls.is_empty() {
        return PrepassOutput {
            rewritten: None,
            dependencies: Vec::new(),
            diagnostics: Vec::new(),
        };
    }

    let mut patches: Vec<Patch> = Vec::with_capacity(decls.len());
    let mut dependencies: Vec<PathBuf> = Vec::new();
    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    // Build a same-file prelude: the source with all `@buildtime`
    // declarations stripped, so the sandbox can see pure helper
    // functions/constants the user defined alongside the buildtime
    // blocks. This gives Zig-comptime-like semantics where a @buildtime
    // block can call any pure function in scope.
    //
    // If the stripped source has impure top-level code, the prelude is
    // disabled and we emit a diagnostic — but evaluation still proceeds
    // with an empty prelude so files that don't rely on siblings work.
    let prelude = build_same_file_prelude(program, source, &decls);
    let prelude_for_sandbox = match &prelude {
        Ok(text) => text.as_str(),
        Err(diag) => {
            diagnostics.push(diag.clone());
            ""
        }
    };

    for decl in decls {
        let mut opts = base_options.clone();
        opts.source_file = origin_path.to_path_buf();
        // SpanIR is 1-based; the line/column helper expects a 0-based
        // byte offset into source.
        let (line, column) =
            byte_offset_to_line_col(source, decl.decl_span.start.saturating_sub(1));
        opts.source_line = line;
        opts.source_column = column;

        // Compose: prelude first, then the user's body. The prelude is
        // `const` / `function` / etc. declarations hoisted to the top;
        // the body is a `return (EXPR);` or statement list that
        // references those declarations.
        let raw_source = if prelude_for_sandbox.is_empty() {
            decl.body_source.clone()
        } else {
            format!("{}\n{}", prelude_for_sandbox, decl.body_source)
        };
        // Boa is a JS-only engine, so any TS syntax in the user's
        // body (e.g. `: number` annotations, `as Foo` casts inside
        // an IIFE) needs stripping before evaluation. We wrap the
        // body in a fresh function, run it through OXC's TS->JS
        // transformer, then unwrap.
        let effective_source = match strip_ts_from_body(&raw_source) {
            Ok(stripped) => stripped,
            Err(e) => {
                diagnostics.push(Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!(
                        "@buildtime `{}` body could not be lowered to JS: {}",
                        decl.name, e
                    ),
                    span: Some(decl.decl_span),
                    notes: vec![],
                    help: None,
                });
                continue;
            }
        };
        match sandbox.evaluate(&effective_source, origin_path, &opts) {
            Ok(result) => {
                dependencies.extend(result.dependencies);
                match build_patch(&decl, result.value) {
                    Ok(patch) => patches.push(patch),
                    Err(err) => diagnostics.push(serialize_error_to_diagnostic(&decl, err)),
                }
            }
            Err(err) => diagnostics.push(sandbox_error_to_diagnostic(
                &decl,
                err,
                origin_path,
                opts.source_line,
            )),
        }
    }

    let rewritten = if patches.is_empty() {
        None
    } else {
        match PatchApplicator::new(source, patches).apply() {
            Ok(code) => Some(code),
            Err(err) => {
                diagnostics.push(Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!("buildtime patch apply failed: {}", err),
                    span: None,
                    notes: vec![],
                    help: None,
                });
                None
            }
        }
    };

    // Deduplicate dependencies — multiple decls may read the same file.
    dependencies.sort();
    dependencies.dedup();

    PrepassOutput {
        rewritten,
        dependencies,
        diagnostics,
    }
}

fn build_patch(decl: &BuildtimeDecl, value: SandboxValue) -> Result<Patch, SerializeError> {
    let replacement = match (decl.kind, &value) {
        (BuildtimeKind::Tier1Const, _) => {
            let literal = value_to_ts_source(&value)?;
            format_const_decl(decl, &literal)
        }
        // Tier 2 — string result splices verbatim.
        (BuildtimeKind::Tier2Function, SandboxValue::String(text)) => text.clone(),
        // Tier 2 — non-string result degrades to a const declaration
        // named for the original function. This matches the spec:
        // `function f() { return 42; }` ≡ `const f = 42;`.
        (BuildtimeKind::Tier2Function, _) => {
            let literal = value_to_ts_source(&value)?;
            format_const_decl(decl, &literal)
        }
        // Tier 3 — the returned string IS the new type RHS.
        (BuildtimeKind::Tier3Type, SandboxValue::String(text)) => format_type_decl(decl, text),
        // Tier 3 with non-string return — error out, this is almost
        // always a user mistake (forgot to join(), returned a number
        // that can't be a type, etc.). SerializeError carries through
        // as a diagnostic.
        (BuildtimeKind::Tier3Type, _) => {
            return Err(SerializeError::NotRepresentable(
                "@buildtime type RHS must evaluate to a string of TS type syntax",
            ));
        }
    };
    Ok(Patch::ReplaceRaw {
        span: decl.decl_span,
        code: replacement,
        context: Some(format!("@buildtime:{}", decl.name)),
        source_macro: Some("buildtime".to_string()),
    })
}

fn format_type_decl(decl: &BuildtimeDecl, type_text: &str) -> String {
    match decl.visibility {
        Visibility::Export => format!("export type {} = {};", decl.name, type_text),
        Visibility::Private => format!("type {} = {};", decl.name, type_text),
    }
}

fn format_const_decl(decl: &BuildtimeDecl, literal: &str) -> String {
    match decl.visibility {
        Visibility::Export => format!("export const {} = {};", decl.name, literal),
        Visibility::Private => format!("const {} = {};", decl.name, literal),
    }
}

fn sandbox_error_to_diagnostic(
    decl: &BuildtimeDecl,
    err: SandboxError,
    origin_path: &Path,
    decl_line: u32,
) -> Diagnostic {
    let (message, notes, help) = match &err {
        SandboxError::Threw { message, stack } => {
            // Always include the user's source attribution in the
            // notes, even when the JS engine doesn't populate `e.stack`
            // (Boa, currently). The user needs to know which file the
            // throw originated from to debug it.
            let mut notes = vec![format!(
                "thrown from {}:{} (inside @buildtime `{}`)",
                origin_path.display(),
                decl_line,
                decl.name
            )];
            if !stack.is_empty() {
                notes.push(format!(
                    "stack (mapped back to {}):\n{}",
                    origin_path.display(),
                    remap_stack(stack, origin_path, decl_line)
                ));
            }
            (
                format!("@buildtime evaluation threw: {}", message),
                notes,
                Some(format!(
                    "the @buildtime `{}` declaration ran to completion but threw the above error",
                    decl.name
                )),
            )
        }
        SandboxError::Timeout { .. } => (
            format!("@buildtime evaluation for `{}` exceeded the timeout", decl.name),
            vec![],
            Some("increase `buildtime.capabilities.timeout` in macroforge.config.js or simplify the expression".to_string()),
        ),
        SandboxError::OutOfMemory { limit } => (
            format!(
                "@buildtime evaluation for `{}` exceeded the heap limit of {} bytes",
                decl.name, limit
            ),
            vec![],
            Some("increase `buildtime.capabilities.maxHeap` in macroforge.config.js".to_string()),
        ),
        SandboxError::UnauthorizedRead { path } => (
            format!(
                "@buildtime `{}` tried to read `{}` but filesystem reads are not permitted for that path",
                decl.name,
                path.display()
            ),
            vec![],
            Some("add the path to `buildtime.capabilities.filesystem.read` in macroforge.config.js".to_string()),
        ),
        SandboxError::UnauthorizedWrite { path } => (
            format!(
                "@buildtime `{}` tried to write `{}` but filesystem writes are not permitted for that path",
                decl.name,
                path.display()
            ),
            vec![],
            Some("add the path to `buildtime.capabilities.filesystem.write` in macroforge.config.js".to_string()),
        ),
        SandboxError::UnauthorizedEnv { var } => (
            format!(
                "@buildtime `{}` tried to read env var `{}` which is not in the capability allowlist",
                decl.name, var
            ),
            vec![],
            Some("add the variable name to `buildtime.capabilities.env` in macroforge.config.js".to_string()),
        ),
        SandboxError::UnauthorizedNetwork { url } => (
            format!(
                "@buildtime `{}` tried to make a network request to {} but the network capability is off",
                decl.name, url
            ),
            vec![],
            Some("set `buildtime.capabilities.network = true` in macroforge.config.js (and accept non-deterministic builds)".to_string()),
        ),
        SandboxError::UnserializableResult { kind } => (
            format!(
                "@buildtime `{}` returned a {} which has no TypeScript literal representation",
                decl.name, kind
            ),
            vec!["@buildtime results must be null, boolean, number, bigint, string, array, or plain object".to_string()],
            None,
        ),
        SandboxError::Backend(msg) => (
            format!("@buildtime backend error for `{}`: {}", decl.name, msg),
            vec![],
            None,
        ),
    };
    Diagnostic {
        level: DiagnosticLevel::Error,
        message,
        span: Some(decl.decl_span),
        notes,
        help,
    }
}

fn serialize_error_to_diagnostic(decl: &BuildtimeDecl, err: SerializeError) -> Diagnostic {
    Diagnostic {
        level: DiagnosticLevel::Error,
        message: format!(
            "@buildtime `{}` produced a value that couldn't be serialized: {}",
            decl.name, err
        ),
        span: Some(decl.decl_span),
        notes: vec![],
        help: None,
    }
}

/// Convert a byte offset into a 1-based `(line, column)` pair for the
/// given source. Used to stamp the declaration's position onto
/// `SandboxOptions` for `buildtime.location.{line,column}`.
fn byte_offset_to_line_col(source: &str, offset: u32) -> (u32, u32) {
    let offset = (offset as usize).min(source.len());
    let mut line = 1u32;
    let mut column = 1u32;
    for (i, ch) in source.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

/// Re-export so callers don't need to pull in `ts_syn::abi::patch`.
///
/// A shim so external users can pattern-match the prepass output
/// without depending on the internal patch types.
pub use crate::ts_syn::abi::patch::Patch as PrepassPatch;

/// Build a prelude string that the sandbox will evaluate before the
/// user's `@buildtime` body. The prelude contains every pure top-level
/// `const` or `function` (and class) declaration from the source,
/// *transpiled to plain JS* so the sandbox (which doesn't understand
/// TS syntax) can parse it.
///
/// Returns:
/// - `Ok(String)` — the prelude text to prepend to the body.
/// - `Err(Diagnostic)` — the source has impure top-level code that
///   makes it unsafe to include in the prelude. Evaluation still
///   proceeds without the prelude (so @buildtime blocks that only
///   use their own body still work).
fn build_same_file_prelude(
    program: &Program<'_>,
    source: &str,
    decls: &[BuildtimeDecl],
) -> Result<String, Diagnostic> {
    use oxc::ast::ast::{Declaration, Statement};

    // First pass: scan for impurity so we can early-exit with a warning.
    let buildtime_spans: Vec<(u32, u32)> = decls
        .iter()
        .map(|d| (d.decl_span.start, d.decl_span.end))
        .collect();
    // A statement counts as "buildtime" when its span lies *inside*
    // any of the buildtime decl spans. Using containment rather than
    // equality is necessary because BuildtimeDecl.decl_span is
    // extended backward to swallow the leading `/** @buildtime */`
    // JSDoc, so it strictly contains the AST node's own span.
    let is_buildtime = |start: u32, end: u32| -> bool {
        buildtime_spans
            .iter()
            .any(|(s, e)| *s <= start && end <= *e)
    };

    let mut impure_diag: Option<Diagnostic> = None;
    for stmt in &program.body {
        match stmt {
            Statement::VariableDeclaration(var) => {
                if is_buildtime(var.span.start, var.span.end) {
                    continue;
                }
                if let Some(reason) = impure_init_reason(var) {
                    impure_diag.get_or_insert_with(|| {
                        prelude_warning(reason, var.span.start, var.span.end)
                    });
                }
            }
            Statement::FunctionDeclaration(_)
            | Statement::ClassDeclaration(_)
            | Statement::TSTypeAliasDeclaration(_)
            | Statement::TSInterfaceDeclaration(_)
            | Statement::TSEnumDeclaration(_)
            | Statement::TSModuleDeclaration(_)
            | Statement::ImportDeclaration(_)
            | Statement::ExportAllDeclaration(_)
            | Statement::ExportDefaultDeclaration(_) => {}
            Statement::ExportNamedDeclaration(export) => {
                if is_buildtime(export.span.start, export.span.end) {
                    continue;
                }
                if let Some(Declaration::VariableDeclaration(var)) = &export.declaration {
                    if let Some(reason) = impure_init_reason(var) {
                        impure_diag.get_or_insert_with(|| {
                            prelude_warning(reason, var.span.start, var.span.end)
                        });
                    }
                }
            }
            other => {
                if impure_diag.is_none() {
                    use oxc::span::GetSpan;
                    let sp = other.span();
                    impure_diag = Some(prelude_warning(
                        "unsupported top-level statement",
                        sp.start,
                        sp.end,
                    ));
                }
            }
        }
    }

    if let Some(diag) = impure_diag {
        return Err(diag);
    }

    // Second pass: collect the source text of each pure declaration we
    // want to include, per kind. Classes and TS-only declarations are
    // excluded because they can contain type annotations OXC codegen
    // doesn't strip — we'd end up passing invalid JS to the sandbox.
    // `const` initializers go through a JS-only check too: if the
    // initializer text contains `as`, `<T>`, `satisfies`, or a colon
    // outside a string, we skip that declaration.
    let mut prelude = String::new();
    for stmt in &program.body {
        match stmt {
            Statement::VariableDeclaration(var) => {
                if is_buildtime(var.span.start, var.span.end) {
                    continue;
                }
                let text = &source[var.span.start as usize..var.span.end as usize];
                if looks_ts_only(text) {
                    continue;
                }
                prelude.push_str(text);
                prelude.push('\n');
            }
            Statement::FunctionDeclaration(func) => {
                let text = &source[func.span.start as usize..func.span.end as usize];
                if looks_ts_only(text) {
                    continue;
                }
                prelude.push_str(text);
                prelude.push('\n');
            }
            Statement::ExportNamedDeclaration(export) => {
                if is_buildtime(export.span.start, export.span.end) {
                    continue;
                }
                let Some(declaration) = &export.declaration else {
                    continue;
                };
                match declaration {
                    Declaration::VariableDeclaration(var) => {
                        let text = &source[var.span.start as usize..var.span.end as usize];
                        if looks_ts_only(text) {
                            continue;
                        }
                        prelude.push_str(text);
                        prelude.push('\n');
                    }
                    Declaration::FunctionDeclaration(func) => {
                        let text = &source[func.span.start as usize..func.span.end as usize];
                        if looks_ts_only(text) {
                            continue;
                        }
                        prelude.push_str(text);
                        prelude.push('\n');
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Ok(prelude)
}

/// Conservative TS-vs-JS check: if the source has a TS-only
/// construct we can't reliably strip (type annotations, generics in
/// type position, `as`/`satisfies`), skip the declaration from the
/// prelude rather than risk a parse error in the sandbox.
///
/// False positives are fine — they just mean the user can't reference
/// that helper from `@buildtime`. The user can work around by moving
/// the helper to a plain-JS `.buildtime.ts` file in a follow-up.
fn looks_ts_only(text: &str) -> bool {
    // Strip strings + regex literals so colons / keywords inside them
    // don't trigger false positives.
    let stripped = strip_string_contents(text);
    if stripped.contains(" as ") || stripped.contains("<") && stripped.contains(">") {
        // `<T>` or `as X` — almost always a type annotation in a
        // top-level position.
        // `<T>` also matches JSX, but JSX at top level of a const
        // initializer is rare and still JS-safe — false positive is
        // acceptable.
    }
    // The main signal: a colon in a parameter list (`(x: T)`) or
    // in a variable declarator (`const x: T = ...`). We scan the text
    // for a colon that isn't inside a string literal and isn't part
    // of an object-property shorthand.
    contains_type_colon(&stripped)
}

/// Replace the contents of every string / template / regex literal in
/// `src` with spaces so token scans don't misfire on characters inside
/// them. Keeps the quote marks + backticks so the scanner can still
/// track opening/closing.
fn strip_string_contents(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut chars = src.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' | '\'' | '`' => {
                out.push(c);
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next == '\\' {
                        // Skip the escaped char.
                        if let Some(_escaped) = chars.next() {
                            out.push(' ');
                        }
                        out.push(' ');
                    } else if next == c {
                        out.push(c);
                        break;
                    } else {
                        out.push(' ');
                    }
                }
            }
            _ => out.push(c),
        }
    }
    out
}

/// True if `src` contains a `:` in a position consistent with a TS
/// type annotation: after `)`, `]`, or an identifier, and not followed
/// by `=` (which would make it `:=` — a shorthand property with
/// default — those are rare and we tolerate false positives).
fn contains_type_colon(src: &str) -> bool {
    let bytes = src.as_bytes();
    let mut paren_depth = 0i32;
    let mut brace_depth = 0i32;
    let mut bracket_depth = 0i32;
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' => paren_depth += 1,
            b')' => paren_depth -= 1,
            b'{' => brace_depth += 1,
            b'}' => brace_depth -= 1,
            b'[' => bracket_depth += 1,
            b']' => bracket_depth -= 1,
            b':' => {
                // Skip `::`, `?:`, and object-literal keys (`:` after
                // an identifier inside an object brace context).
                if i + 1 < bytes.len() && bytes[i + 1] == b':' {
                    continue;
                }
                if i > 0 && bytes[i - 1] == b'?' {
                    continue;
                }
                // Signal: `:` inside parens or in a decl RHS (outside
                // any braces / brackets) is a type annotation.
                if paren_depth > 0 || (brace_depth == 0 && bracket_depth == 0) {
                    // Still need to exclude object-literal shorthand
                    // like `{ a: 1 }` — which has brace_depth > 0 at
                    // the colon. We've already required brace_depth == 0
                    // for the non-paren case.
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn impure_init_reason(var: &oxc::ast::ast::VariableDeclaration<'_>) -> Option<&'static str> {
    for declarator in &var.declarations {
        if let Some(init) = &declarator.init {
            if let Some(reason) = crate::host::buildtime::purity::expression_side_effect(init) {
                return Some(reason);
            }
        }
    }
    None
}

fn prelude_warning(reason: &str, start: u32, end: u32) -> Diagnostic {
    use crate::ts_syn::abi::SpanIR;
    Diagnostic {
        level: DiagnosticLevel::Warning,
        message: format!(
            "@buildtime same-file prelude disabled: {}",
            reason
        ),
        span: Some(SpanIR { start, end }),
        notes: vec![
            "only pure top-level code (const/function/class, with pure initializers) can be referenced from @buildtime bodies".to_string(),
        ],
        help: Some(
            "move the impure code into a separate .buildtime.ts file or remove its side effects"
                .to_string(),
        ),
    }
}

/// Strip TypeScript syntax from a body source so the JS-only sandbox
/// can parse it. Wraps the body in a synthetic function (to make it a
/// valid program), runs OXC's TS→JS transformer, then unwraps the
/// function back out.
///
/// Errors are returned as a string for the caller to attach to a
/// Diagnostic.
fn strip_ts_from_body(body: &str) -> Result<String, String> {
    use oxc::allocator::Allocator;
    use oxc::codegen::Codegen;
    use oxc::parser::Parser as OxcParser;
    use oxc::semantic::SemanticBuilder;
    use oxc::span::SourceType;
    use oxc::transformer::{TransformOptions, Transformer};

    // Wrap as async so `await` in the user body parses.
    let wrapped = format!("async function __mf_body() {{\n{}\n}}", body);
    let allocator = Allocator::default();
    let parsed = OxcParser::new(&allocator, &wrapped, SourceType::ts()).parse();
    if !parsed.errors.is_empty() {
        return Err(format!(
            "parse error while stripping TS: {}",
            parsed
                .errors
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join("; ")
        ));
    }
    let mut program = parsed.program;
    let semantic = SemanticBuilder::new().build(&program);
    let scoping = semantic.semantic.into_scoping();
    let options = TransformOptions::default();
    let ret = Transformer::new(
        &allocator,
        std::path::Path::new("<buildtime-body>"),
        &options,
    )
    .build_with_scoping(scoping, &mut program);
    if !ret.errors.is_empty() {
        return Err(format!(
            "TS strip failed: {}",
            ret.errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("; ")
        ));
    }
    let printed = Codegen::new().build(&program).code;
    // Pull the body back out from `function __mf_body() { ... }`.
    let open = printed.find('{').ok_or_else(|| {
        format!("ts-strip: no `{{` in output: {printed}")
    })?;
    let close = printed.rfind('}').ok_or_else(|| {
        format!("ts-strip: no `}}` in output: {printed}")
    })?;
    if close <= open + 1 {
        return Ok(String::new());
    }
    Ok(printed[open + 1..close].trim_matches('\n').to_string())
}

/// Number of preamble lines the sandbox prepends to the user's body
/// in its synthetic wrapper (see `wrap_source` in backends/boa.rs).
/// A frame in the sandbox at line `N` corresponds to user-body line
/// `N - WRAPPER_PREAMBLE_LINES`.
const WRAPPER_PREAMBLE_LINES: u32 = 5;

/// Rewrite `eval_script:LINE:COL` stack frames so they point at the user's
/// source file with an approximate user-source line number. The mapping
/// is:
///
/// ```text
///   user_line ≈ (sandbox_line - WRAPPER_PREAMBLE_LINES) + (decl_line - 1)
/// ```
///
/// It's approximate because for Tier 2 functions we strip the outer
/// braces but preserve inner newlines, which adds a line of offset the
/// formula doesn't account for. Off-by-one is acceptable when the
/// alternative is a sandbox-internal `eval_script:5:10` with no
/// connection to the user's file.
fn remap_stack(stack: &str, origin_path: &Path, decl_line: u32) -> String {
    let origin_display = origin_path.display().to_string();
    let mut out = String::with_capacity(stack.len());
    for line in stack.lines() {
        out.push_str(&remap_stack_line(line, &origin_display, decl_line));
        out.push('\n');
    }
    // Drop the trailing newline we just added.
    if out.ends_with('\n') {
        out.pop();
    }
    out
}

fn remap_stack_line(line: &str, origin_display: &str, decl_line: u32) -> String {
    // Match `eval_script:LINE:COL` or `<eval>:LINE:COL` anywhere in the
    // line and rewrite. A frame without a `:LINE:COL` match is passed
    // through unchanged.
    let patterns = ["eval_script:", "<eval>:", "<anonymous>:", "<buildtime>:"];
    for pat in &patterns {
        if let Some(idx) = line.find(pat) {
            let after = &line[idx + pat.len()..];
            if let Some((ln_str, col_rest)) = split_line_col(after) {
                if let Ok(sandbox_line) = ln_str.parse::<i64>() {
                    let user_line = remap_line(sandbox_line, decl_line);
                    let prefix = &line[..idx];
                    return format!("{}{}:{}:{}", prefix, origin_display, user_line, col_rest);
                }
            }
        }
    }
    line.to_string()
}

/// Parse `LINE:COL<rest>` from the start of `s`. Returns
/// `("LINE", ":COL<rest>")` if successful.
fn split_line_col(s: &str) -> Option<(&str, &str)> {
    let first_colon = s.find(':')?;
    let line_part = &s[..first_colon];
    if line_part.is_empty() || !line_part.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some((line_part, &s[first_colon..]))
}

fn remap_line(sandbox_line: i64, decl_line: u32) -> u32 {
    let adjusted = sandbox_line - WRAPPER_PREAMBLE_LINES as i64 + decl_line as i64 - 1;
    adjusted.max(1) as u32
}
