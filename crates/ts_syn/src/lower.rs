use ts_macro_abi::*;

use crate::TsSynError;

#[cfg(feature = "swc")]
use swc_core::common::{Span, Spanned};
#[cfg(feature = "swc")]
use swc_core::ecma::ast::*;
#[cfg(feature = "swc")]
use swc_core::ecma::visit::{Visit, VisitWith};

/// Lower a module into ClassIR list (derive targets).
#[cfg(feature = "swc")]
pub fn lower_classes(module: &Module, source: &str) -> Result<Vec<ClassIR>, TsSynError> {
    let mut v = ClassCollector {
        out: vec![],
        source,
    };
    module.visit_with(&mut v);
    Ok(v.out)
}

#[cfg(feature = "swc")]
struct ClassCollector<'a> {
    out: Vec<ClassIR>,
    source: &'a str,
}

#[cfg(feature = "swc")]
impl<'a> Visit for ClassCollector<'a> {
    fn visit_class_decl(&mut self, n: &ClassDecl) {
        let name = n.ident.sym.to_string();
        let span = swc_span_to_ir(n.class.span);

        let class_source = snippet(self.source, n.class.span);
        let body_span = if let (Some(open_brace), Some(close_brace)) =
            (class_source.find('{'), class_source.rfind('}'))
        {
            SpanIR::new(
                n.class.span.lo.0 + open_brace as u32,
                n.class.span.lo.0 + close_brace as u32 + 1,
            )
        } else {
            // fallback for classes without a body, though they can't have macros.
            span
        };

        let decorators = lower_decorators(&n.class.decorators, self.source);

        let (fields, methods) = lower_members(&n.class.body, self.source);

        self.out.push(ClassIR {
            name,
            span,
            body_span,
            is_abstract: n.class.is_abstract,
            type_params: vec![],
            heritage: vec![], // TODO: lower extends/implements
            decorators,
            decorators_ast: n.class.decorators.clone(),
            fields,
            methods,
            members: n.class.body.clone(),
        });
    }
}

#[cfg(feature = "swc")]
fn lower_members(body: &[ClassMember], source: &str) -> (Vec<FieldIR>, Vec<MethodSigIR>) {
    let mut fields = vec![];
    let mut methods = vec![];

    for m in body {
        match m {
            ClassMember::ClassProp(p) => {
                let name = match &p.key {
                    PropName::Ident(i) => i.sym.to_string(),
                    _ => continue,
                };

                let ts_type = p
                    .type_ann
                    .as_ref()
                    .map(|t| snippet(source, t.span()))
                    .unwrap_or_else(|| "any".into());

                fields.push(FieldIR {
                    name,
                    span: swc_span_to_ir(p.span),
                    ts_type,
                    type_ann: p.type_ann.as_ref().map(|ann| ann.type_ann.clone()),
                    optional: p.is_optional, // Changed from p.optional
                    readonly: p.readonly,
                    visibility: lower_visibility(p.accessibility),
                    decorators: lower_decorators(&p.decorators, source),
                    prop_ast: Some(p.clone()),
                });
            }
            ClassMember::Method(meth) => {
                let name = match &meth.key {
                    PropName::Ident(i) => i.sym.to_string(),
                    _ => continue,
                };

                let method_span = if let Some(body) = &meth.function.body {
                    Span::new(meth.span.lo, body.span.hi)
                } else {
                    meth.span
                };

                // Extract parameters and type parameters from method source
                let method_src = snippet(source, meth.span);
                let params_src = extract_params_from_source(&method_src);
                let type_params_src = extract_type_params_from_source(&method_src, &name);

                // Adjust span to find the actual start (handles modifiers like public, static, async)
                let adjusted_span = adjust_method_span(
                    source,
                    method_span,
                    &name,
                    meth.is_static,
                    meth.accessibility,
                );

                methods.push(MethodSigIR {
                    name,
                    span: swc_span_to_ir(adjusted_span),
                    type_params_src,
                    params_src,
                    return_type_src: meth
                        .function
                        .return_type
                        .as_ref()
                        .map(|t| snippet(source, t.span()).trim().to_string())
                        .unwrap_or_else(|| "void".into()),
                    is_static: meth.is_static,
                    is_async: meth.function.is_async,
                    visibility: lower_visibility(meth.accessibility),
                    decorators: lower_decorators(&meth.function.decorators, source),
                    member_ast: Some(MethodAstIR::Method(meth.clone())),
                });
            }
            ClassMember::Constructor(c) => {
                let constructor_span = if let Some(body) = &c.body {
                    Span::new(c.span.lo, body.span.hi)
                } else {
                    c.span
                };

                // Extract parameters from constructor source
                let constructor_src = snippet(source, c.span);
                let params_src = extract_params_from_source(&constructor_src);

                // Adjust span to find the actual start (handles modifiers like private, protected, public)
                let adjusted_span =
                    adjust_constructor_span(source, constructor_span, c.accessibility);

                methods.push(MethodSigIR {
                    name: "constructor".into(),
                    span: swc_span_to_ir(adjusted_span),
                    type_params_src: String::new(), // constructors don't have type parameters
                    params_src,
                    return_type_src: String::new(), // constructors don't have return types
                    is_static: false,
                    is_async: false, // constructors can't be async
                    visibility: lower_visibility(c.accessibility),
                    decorators: vec![], // Constructors don't have decorators
                    member_ast: Some(MethodAstIR::Constructor(c.clone())),
                });
            }
            _ => {}
        }
    }

    (fields, methods)
}

#[cfg(feature = "swc")]
fn lower_decorators(decs: &[Decorator], source: &str) -> Vec<DecoratorIR> {
    decs.iter()
        .filter_map(|d| {
            let span = adjust_decorator_span(d.span, source);
            let (name, args_src) = match &*d.expr {
                Expr::Ident(i) => (i.sym.to_string(), String::new()),
                Expr::Call(call) => {
                    let callee = match &call.callee {
                        Callee::Expr(e) => match &**e {
                            Expr::Ident(i) => i.sym.to_string(),
                            _ => return None,
                        },
                        _ => return None,
                    };
                    (callee, call_args_src(call, source))
                }
                _ => return None,
            };
            Some(DecoratorIR {
                name,
                args_src,
                span,
                node: Some(d.clone()),
            })
        })
        .collect()
}

fn adjust_decorator_span(span: Span, source: &str) -> SpanIR {
    let mut ir = swc_span_to_ir(span);
    let bytes = source.as_bytes();
    let mut start = ir.start as usize;
    let mut end = ir.end as usize;

    // Extend backward to include '@' symbol and any leading whitespace on the same line
    if start > 0 && bytes[start - 1] == b'@' {
        start -= 1;
        // Continue backward to include leading whitespace, but stop at newline
        while start > 0 && (bytes[start - 1] == b' ' || bytes[start - 1] == b'\t') {
            start -= 1;
        }
        ir.start = start as u32;
    }

    // Extend forward to include trailing newline and subsequent indentation
    if end < bytes.len() {
        // Skip trailing whitespace on the same line
        while end < bytes.len() && (bytes[end] == b' ' || bytes[end] == b'\t') {
            end += 1;
        }
        // If we hit a newline, include it and any leading whitespace on the next line
        if end < bytes.len() && bytes[end] == b'\n' {
            end += 1;
            while end < bytes.len() && (bytes[end] == b' ' || bytes[end] == b'\t') {
                end += 1;
            }
            ir.end = end as u32;
        }
    }

    ir
}

#[cfg(feature = "swc")]
fn call_args_src(call: &CallExpr, source: &str) -> String {
    if call.args.is_empty() {
        return String::new();
    }

    let call_src = snippet(source, call.span);
    if let (Some(open), Some(close)) = (call_src.find('('), call_src.rfind(')'))
        && open < close
    {
        return call_src[open + 1..close].trim().to_string();
    }
    String::new()
}

#[cfg(feature = "swc")]
fn lower_visibility(acc: Option<Accessibility>) -> Visibility {
    match acc {
        Some(Accessibility::Public) => Visibility::Public,
        Some(Accessibility::Protected) => Visibility::Protected,
        Some(Accessibility::Private) => Visibility::Private,
        None => Visibility::Public,
    }
}

#[cfg(feature = "swc")]
fn swc_span_to_ir(sp: Span) -> SpanIR {
    SpanIR::new(sp.lo.0, sp.hi.0)
}

#[cfg(feature = "swc")]
fn snippet(source: &str, sp: Span) -> String {
    if sp.is_dummy() {
        return String::new();
    }
    let lo = sp.lo.0 as usize;
    let hi = sp.hi.0 as usize;
    source.get(lo..hi).unwrap_or("").to_string()
}

#[cfg(feature = "swc")]
fn extract_params_from_source(method_src: &str) -> String {
    // Find the first '(' which starts the parameter list
    let Some(open) = method_src.find('(') else {
        return String::new();
    };

    // Now find the matching closing ')', accounting for nested parentheses
    let chars: Vec<char> = method_src.chars().collect();
    let mut depth = 0;
    let mut close_pos = None;

    for (i, &ch) in chars.iter().enumerate().skip(open) {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    close_pos = Some(i);
                    break;
                }
            }
            _ => {}
        }
    }

    if let Some(close) = close_pos
        && open < close
    {
        return chars[open + 1..close].iter().collect();
    }

    String::new()
}

#[cfg(feature = "swc")]
fn extract_type_params_from_source(method_src: &str, method_name: &str) -> String {
    // Find the method name in the source
    let Some(name_pos) = method_src.find(method_name) else {
        return String::new();
    };

    // Look for '<' after the method name
    let after_name = &method_src[name_pos + method_name.len()..];
    let chars: Vec<char> = after_name.chars().collect();

    // Skip whitespace
    let mut i = 0;
    while i < chars.len() && chars[i].is_whitespace() {
        i += 1;
    }

    // Check if we have a '<'
    if i >= chars.len() || chars[i] != '<' {
        return String::new();
    }

    let start = i;
    i += 1; // skip the '<'
    let mut depth = 1;

    // Find matching '>'
    while i < chars.len() && depth > 0 {
        match chars[i] {
            '<' => depth += 1,
            '>' => depth -= 1,
            _ => {}
        }
        i += 1;
    }

    if depth == 0 {
        return chars[start..i].iter().collect();
    }

    String::new()
}

#[cfg(feature = "swc")]
fn adjust_constructor_span(source: &str, span: Span, accessibility: Option<Accessibility>) -> Span {
    let search_start = span.lo.0 as usize;
    let bytes = source.as_bytes();

    // Build list of possible keywords
    let mut keywords = vec!["constructor"];
    match accessibility {
        Some(Accessibility::Public) => keywords.insert(0, "public"),
        Some(Accessibility::Protected) => keywords.insert(0, "protected"),
        Some(Accessibility::Private) => keywords.insert(0, "private"),
        None => {}
    }

    // Search backwards for the earliest matching keyword
    let mut earliest_start = search_start;

    for keyword in &keywords {
        let keyword_bytes = keyword.as_bytes();
        let search_region_start = search_start.saturating_sub(keyword.len() + 20);
        let search_region_end = (search_start + keyword.len()).min(source.len());

        if search_region_start >= search_region_end {
            continue;
        }

        let search_region = &bytes[search_region_start..search_region_end];

        for i in 0..search_region.len() {
            if i + keyword_bytes.len() <= search_region.len() {
                let candidate = &search_region[i..i + keyword_bytes.len()];
                if candidate == keyword_bytes {
                    let abs_pos = search_region_start + i;
                    if abs_pos < earliest_start {
                        let is_word_boundary_before = abs_pos == 0
                            || !bytes[abs_pos - 1].is_ascii_alphanumeric()
                                && bytes[abs_pos - 1] != b'_';
                        let is_word_boundary_after = abs_pos + keyword_bytes.len() >= bytes.len()
                            || !bytes[abs_pos + keyword_bytes.len()].is_ascii_alphanumeric()
                                && bytes[abs_pos + keyword_bytes.len()] != b'_';

                        if is_word_boundary_before && is_word_boundary_after {
                            earliest_start = abs_pos;
                        }
                    }
                }
            }
        }
    }

    Span::new(swc_core::common::BytePos(earliest_start as u32), span.hi)
}

#[cfg(feature = "swc")]
fn adjust_method_span(
    source: &str,
    span: Span,
    method_name: &str,
    is_static: bool,
    accessibility: Option<Accessibility>,
) -> Span {
    let search_start = span.lo.0 as usize;
    let bytes = source.as_bytes();

    // Build list of possible keywords that might precede the method name
    let mut keywords = vec![method_name];
    if is_static {
        keywords.insert(0, "static");
    }
    match accessibility {
        Some(Accessibility::Public) => keywords.insert(0, "public"),
        Some(Accessibility::Protected) => keywords.insert(0, "protected"),
        Some(Accessibility::Private) => keywords.insert(0, "private"),
        None => {}
    }
    // Also check for async keyword
    keywords.push("async");

    // Search backwards for the earliest matching keyword
    let mut earliest_start = search_start;

    for keyword in &keywords {
        let keyword_bytes = keyword.as_bytes();
        let search_region_start = search_start.saturating_sub(keyword.len() + 20); // Extra buffer for whitespace
        let search_region_end = (search_start + keyword.len()).min(source.len());

        if search_region_start >= search_region_end {
            continue;
        }

        let search_region = &bytes[search_region_start..search_region_end];

        // Find all occurrences of this keyword
        for i in 0..search_region.len() {
            if i + keyword_bytes.len() <= search_region.len() {
                let candidate = &search_region[i..i + keyword_bytes.len()];
                if candidate == keyword_bytes {
                    let abs_pos = search_region_start + i;
                    if abs_pos < earliest_start {
                        // Verify it's a whole word (not part of a larger identifier)
                        let is_word_boundary_before = abs_pos == 0
                            || !bytes[abs_pos - 1].is_ascii_alphanumeric()
                                && bytes[abs_pos - 1] != b'_';
                        let is_word_boundary_after = abs_pos + keyword_bytes.len() >= bytes.len()
                            || !bytes[abs_pos + keyword_bytes.len()].is_ascii_alphanumeric()
                                && bytes[abs_pos + keyword_bytes.len()] != b'_';

                        if is_word_boundary_before && is_word_boundary_after {
                            earliest_start = abs_pos;
                        }
                    }
                }
            }
        }
    }

    Span::new(swc_core::common::BytePos(earliest_start as u32), span.hi)
}

#[cfg(not(feature = "swc"))]
pub fn lower_classes(_module: &(), _source: &str) -> Result<Vec<ClassIR>, TsSynError> {
    Err(TsSynError::Unsupported("swc feature disabled".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "swc")]
    use swc_core::common::{FileName, GLOBALS, Globals, SourceMap, sync::Lrc};
    #[cfg(feature = "swc")]
    use swc_core::ecma::parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

    #[cfg(feature = "swc")]
    #[test]
    fn test_regular_method_params() {
        GLOBALS.set(&Globals::new(), || {
            let source =
                "class User { getName(prefix: string, suffix: string): string { return ''; } }";
            let module = parse_module(source);
            let classes = lower_classes(&module, source).expect("lowering to succeed");
            let class = classes.first().expect("class");
            let method = class
                .methods
                .iter()
                .find(|m| m.name == "getName")
                .expect("getName method");

            // The params_src should include the commas and spacing
            assert!(method.params_src.contains("prefix: string"));
            assert!(method.params_src.contains("suffix: string"));
        });
    }

    #[cfg(feature = "swc")]
    #[test]
    fn test_method_with_modifiers() {
        GLOBALS.set(&Globals::new(), || {
            let source =
                "class User { public static async getUser(): Promise<User> { return null!; } }";
            let module = parse_module(source);
            let classes = lower_classes(&module, source).expect("lowering to succeed");
            let class = classes.first().expect("class");
            let method = class.methods.first().expect("method");

            // Check if the span starts at the correct position
            assert!(
                source[method.span.start as usize..].starts_with("public")
                    || source[method.span.start as usize..].starts_with("getUser"),
                "Method span should start at modifier or method name, got: {:?}",
                &source[method.span.start as usize..method.span.start as usize + 10]
            );
        });
    }

    #[cfg(feature = "swc")]
    fn parse_module(source: &str) -> Module {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(
            FileName::Custom("test.ts".into()).into(),
            source.to_string(),
        );
        let lexer = Lexer::new(
            Syntax::Typescript(TsSyntax {
                tsx: false,
                decorators: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );
        let mut parser = Parser::new_from(lexer);
        parser.parse_module().expect("module to parse")
    }

    #[cfg(feature = "swc")]
    #[test]
    fn lowers_decorator_arguments_for_fields() {
        GLOBALS.set(&Globals::new(), || {
            let source = r#"
            class User {
                @Debug({ rename: "identifier", skip: false })
                id: string;
            }
            "#;
            let module = parse_module(source);
            let classes = lower_classes(&module, source).expect("lowering to succeed");
            let first = classes.first().expect("class");
            let field = first.fields.first().expect("field");
            let decorator = field.decorators.first().expect("decorator");
            assert_eq!(decorator.name, "Debug");
            assert_eq!(
                decorator.args_src.trim(),
                r#"{ rename: "identifier", skip: false }"#
            );
        });
    }

    #[cfg(feature = "swc")]
    #[test]
    fn class_decorator_span_captures_at_symbol() {
        GLOBALS.set(&Globals::new(), || {
            let source = r#"
            @Derive("Debug")
            class User {}
            "#;
            let module = parse_module(source);
            let classes = lower_classes(&module, source).expect("lowering to succeed");
            let class = classes.first().expect("class");
            let decorator = class.decorators.first().expect("decorator");
            let snippet =
                &source.as_bytes()[decorator.span.start as usize..decorator.span.end as usize];
            let snippet_str = std::str::from_utf8(snippet).unwrap();

            // The span now includes leading whitespace for clean deletion
            assert!(
                snippet_str.contains("@Derive"),
                "decorator span should include '@Derive', got {:?}",
                snippet_str
            );

            // Verify it includes the trailing newline and next line's indentation
            assert!(
                snippet_str.ends_with('\n'),
                "decorator span should include trailing newline for clean deletion"
            );
        });
    }
}
