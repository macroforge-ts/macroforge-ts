use std::fmt;

use anyhow::{Context, Error, bail};
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;
use quote::ToTokens;
use syn::{GenericArgument, PathArguments, Type, parse_quote};

use super::{ToCode, ctxt::Ctx};

pub struct BoxWrapper(Box<dyn ToCode>);

impl ToCode for BoxWrapper {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        self.0.to_code(cx)
    }
}

#[derive(Clone, Copy)]
enum OutputKind {
    Expr,
    Pat,
    Stmt,
    AssignTarget,
    ModuleItem,
    Program,
    TsType,
    PropOrSpread,
}

#[derive(Clone, Copy)]
enum PlaceholderMode {
    Bare,
    StringLiteral,
}

#[derive(Clone)]
struct Placeholder {
    start: usize,
    end: usize,
    name: String,
    mode: PlaceholderMode,
}

struct ParsedQuote {
    source: String,
    output: OutputKind,
    placeholders: Vec<Placeholder>,
}

struct ParsedBox<T>(T);

struct ParsedOption<T>(Option<T>);

impl<T> ToCode for ParsedBox<T>
where
    T: ToCode,
{
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let inner = self.0.to_code(cx);
        parse_quote!(Box::new(#inner))
    }
}

impl<T> ToCode for ParsedOption<T>
where
    T: ToCode,
{
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        match &self.0 {
            Some(inner) => {
                let inner = inner.to_code(cx);
                parse_quote!(Some(#inner))
            }
            None => parse_quote!(None),
        }
    }
}

impl ToCode for ParsedQuote {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        let source_expr = build_source_expr(&self.source, &self.placeholders, cx);
        let parser = self.output.parser_expr();
        let output_name = syn::LitStr::new(self.output.name(), proc_macro2::Span::call_site());
        parse_quote! {{
            let __mf_quote_source = #source_expr;
            #parser.unwrap_or_else(|err| {
                panic!(
                    "failed to parse Oxc quote as {}: {}\nsource:\n{}",
                    #output_name,
                    err,
                    __mf_quote_source
                )
            })
        }}
    }
}

impl OutputKind {
    fn name(self) -> &'static str {
        match self {
            Self::Expr => "Expr",
            Self::Pat => "Pat",
            Self::Stmt => "Stmt",
            Self::AssignTarget => "AssignTarget",
            Self::ModuleItem => "ModuleItem",
            Self::Program => "Program",
            Self::TsType => "TsType",
            Self::PropOrSpread => "PropOrSpread",
        }
    }

    fn parser_expr(self) -> syn::Expr {
        match self {
            Self::Expr => parse_quote!(macroforge_ts::ts_syn::parse_oxc_expr(&__mf_quote_source)),
            Self::Pat => {
                parse_quote!(macroforge_ts::ts_syn::parse_oxc_binding_pattern(
                    &__mf_quote_source
                ))
            }
            Self::Stmt => {
                parse_quote!(macroforge_ts::ts_syn::parse_oxc_statement(
                    &__mf_quote_source
                ))
            }
            Self::AssignTarget => parse_quote!(macroforge_ts::ts_syn::parse_oxc_assignment_target(
                &__mf_quote_source
            )),
            Self::ModuleItem => parse_quote!(macroforge_ts::ts_syn::parse_oxc_module_item(
                &__mf_quote_source
            )),
            Self::Program => {
                parse_quote!(macroforge_ts::ts_syn::parse_oxc_program(&__mf_quote_source))
            }
            Self::TsType => parse_quote!(macroforge_ts::ts_syn::parse_oxc_type(&__mf_quote_source)),
            Self::PropOrSpread => parse_quote!(macroforge_ts::ts_syn::parse_oxc_prop_or_spread(
                &__mf_quote_source
            )),
        }
    }
}

pub(crate) fn parse_input_type(input_str: &str, ty: &Type) -> Result<BoxWrapper, Error> {
    if let Some(ty) = extract_generic("Box", ty) {
        let node = parse_input_type(input_str, ty).context("failed to parse `T` in Box<T>")?;
        return Ok(BoxWrapper(Box::new(ParsedBox(node))));
    }

    if let Some(ty) = extract_generic("Option", ty) {
        if input_str.is_empty() {
            return Ok(BoxWrapper(Box::new(ParsedOption::<ParsedQuote>(None))));
        }

        let node = parse_input_type(input_str, ty).context("failed to parse `T` in Option<T>")?;
        return Ok(BoxWrapper(Box::new(ParsedOption(Some(node)))));
    }

    let output = output_kind(ty)?;
    validate_output(input_str, output)?;

    Ok(BoxWrapper(Box::new(ParsedQuote {
        source: input_str.to_owned(),
        output,
        placeholders: scan_placeholders(input_str),
    })))
}

fn output_kind(ty: &Type) -> Result<OutputKind, Error> {
    if let Type::Path(path) = ty
        && let Some(ident) = path.path.get_ident()
    {
        return match ident.to_string().as_str() {
            "Expr" => Ok(OutputKind::Expr),
            "Pat" => Ok(OutputKind::Pat),
            "Stmt" => Ok(OutputKind::Stmt),
            "AssignTarget" => Ok(OutputKind::AssignTarget),
            "ModuleItem" => Ok(OutputKind::ModuleItem),
            "Program" => Ok(OutputKind::Program),
            "TsType" => Ok(OutputKind::TsType),
            "PropOrSpread" => Ok(OutputKind::PropOrSpread),
            _ => bail!("Unknown quote type: {}", ty.to_token_stream()),
        };
    }

    bail!("Unknown quote type: {}", ty.to_token_stream())
}

fn validate_output(source: &str, output: OutputKind) -> Result<(), Error> {
    let allocator = Allocator::new();
    let source_type = SourceType::tsx();

    match output {
        OutputKind::Expr => Parser::new(&allocator, source, source_type)
            .parse_expression()
            .map(|_| ())
            .map_err(format_parse_errors)
            .context("failed to parse input as `Expr`"),
        OutputKind::Program => {
            let parsed = Parser::new(&allocator, source, source_type).parse();
            if parsed.errors.is_empty() {
                Ok(())
            } else {
                Err(format_parse_errors(parsed.errors))
                    .context("failed to parse input as `Program`")
            }
        }
        OutputKind::Stmt | OutputKind::ModuleItem => {
            let parsed = Parser::new(&allocator, source, source_type).parse();
            if !parsed.errors.is_empty() {
                return Err(format_parse_errors(parsed.errors))
                    .context(format!("failed to parse input as `{}`", output.name()));
            }
            if parsed.program.body.is_empty() {
                bail!(
                    "failed to parse input as `{}`: no statement found",
                    output.name()
                );
            }
            Ok(())
        }
        OutputKind::AssignTarget => {
            let wrapped = format!("({source}) = __macroforge_target;");
            let parsed = Parser::new(&allocator, &wrapped, source_type).parse();
            if !parsed.errors.is_empty() {
                return Err(format_parse_errors(parsed.errors))
                    .context("failed to parse input as `AssignTarget`");
            }
            Ok(())
        }
        OutputKind::Pat => {
            let wrapped = format!("function __macroforge__({source}) {{}}");
            let parsed = Parser::new(&allocator, &wrapped, source_type).parse();
            if !parsed.errors.is_empty() {
                return Err(format_parse_errors(parsed.errors))
                    .context("failed to parse input as `Pat`");
            }
            Ok(())
        }
        OutputKind::TsType => {
            let wrapped = format!("type __MacroforgeType = {source};");
            let parsed = Parser::new(&allocator, &wrapped, source_type).parse();
            if !parsed.errors.is_empty() {
                return Err(format_parse_errors(parsed.errors))
                    .context("failed to parse input as `TsType`");
            }
            Ok(())
        }
        OutputKind::PropOrSpread => {
            let wrapped = format!("({{ {source} }})");
            Parser::new(&allocator, &wrapped, source_type)
                .parse_expression()
                .map(|_| ())
                .map_err(format_parse_errors)
                .context("failed to parse input as `PropOrSpread`")
        }
    }
}

fn format_parse_errors(errors: Vec<impl fmt::Display>) -> Error {
    Error::msg(
        errors
            .into_iter()
            .map(|diagnostic| diagnostic.to_string())
            .collect::<Vec<_>>()
            .join("; "),
    )
}

fn build_source_expr(source: &str, placeholders: &[Placeholder], cx: &Ctx) -> syn::Expr {
    let mut stmts: Vec<syn::Stmt> = Vec::new();
    let capacity = source.len();
    stmts.push(parse_quote!(let mut __mf_quote_source = String::with_capacity(#capacity);));

    let mut cursor = 0;
    for placeholder in placeholders {
        if cursor < placeholder.start {
            let literal = syn::LitStr::new(
                &source[cursor..placeholder.start],
                proc_macro2::Span::call_site(),
            );
            stmts.push(parse_quote! {
                __mf_quote_source.push_str(#literal);
            });
        }

        if let Some(replacement) = placeholder_replacement_expr(placeholder, cx) {
            stmts.push(parse_quote! {
                __mf_quote_source.push_str(&(#replacement));
            });
        } else {
            let literal = syn::LitStr::new(
                &source[placeholder.start..placeholder.end],
                proc_macro2::Span::call_site(),
            );
            stmts.push(parse_quote! {
                __mf_quote_source.push_str(#literal);
            });
        }

        cursor = placeholder.end;
    }

    if cursor < source.len() {
        let literal = syn::LitStr::new(&source[cursor..], proc_macro2::Span::call_site());
        stmts.push(parse_quote! {
            __mf_quote_source.push_str(#literal);
        });
    }

    stmts.push(syn::Stmt::Expr(parse_quote!(__mf_quote_source), None));
    parse_quote!({ #(#stmts)* })
}

fn placeholder_replacement_expr(placeholder: &Placeholder, cx: &Ctx) -> Option<syn::Expr> {
    match placeholder.mode {
        PlaceholderMode::StringLiteral => cx.var(crate::ctxt::VarPos::Str, &placeholder.name).map(|var| {
            let expr = var.get_expr();
            parse_quote! {
                macroforge_ts::ts_syn::ToOxcStringLiteralSource::to_oxc_string_literal_source(&#expr)
            }
        }),
        PlaceholderMode::Bare => {
            if let Some(var) = cx.var(crate::ctxt::VarPos::Ident, &placeholder.name) {
                let expr = var.get_expr();
                return Some(parse_quote! {
                    macroforge_ts::ts_syn::ToOxcIdentSource::to_oxc_ident_source(&#expr)
                });
            }
            if let Some(var) = cx.var(crate::ctxt::VarPos::Expr, &placeholder.name) {
                let expr = var.get_expr();
                return Some(parse_quote! {
                    macroforge_ts::ts_syn::ToOxcExprSource::to_oxc_expr_source(&#expr)
                });
            }
            if let Some(var) = cx.var(crate::ctxt::VarPos::Pat, &placeholder.name) {
                let expr = var.get_expr();
                return Some(parse_quote! {
                    macroforge_ts::ts_syn::ToOxcPatSource::to_oxc_pat_source(&#expr)
                });
            }
            if let Some(var) = cx.var(crate::ctxt::VarPos::AssignTarget, &placeholder.name) {
                let expr = var.get_expr();
                return Some(parse_quote! {
                    macroforge_ts::ts_syn::ToOxcAssignTargetSource::to_oxc_assign_target_source(&#expr)
                });
            }
            if let Some(var) = cx.var(crate::ctxt::VarPos::TsType, &placeholder.name) {
                let expr = var.get_expr();
                return Some(parse_quote! {
                    macroforge_ts::ts_syn::ToOxcTypeSource::to_oxc_type_source(&#expr)
                });
            }
            None
        }
    }
}

fn scan_placeholders(source: &str) -> Vec<Placeholder> {
    let mut placeholders = Vec::new();
    scan_code(source, 0, source.len(), &mut placeholders);
    placeholders.sort_by_key(|placeholder| placeholder.start);
    placeholders
}

fn scan_code(
    source: &str,
    mut index: usize,
    end: usize,
    placeholders: &mut Vec<Placeholder>,
) -> usize {
    let bytes = source.as_bytes();
    while index < end {
        match bytes[index] {
            b'\'' | b'"' => {
                index = scan_string(source, index, bytes[index], placeholders);
            }
            b'`' => {
                index = scan_template(source, index, placeholders);
            }
            b'/' if index + 1 < end && bytes[index + 1] == b'/' => {
                index = skip_line_comment(source, index + 2);
            }
            b'/' if index + 1 < end && bytes[index + 1] == b'*' => {
                index = skip_block_comment(source, index + 2);
            }
            b'$' => {
                if let Some((name, end_index)) = parse_placeholder_name(source, index + 1) {
                    placeholders.push(Placeholder {
                        start: index,
                        end: end_index,
                        name,
                        mode: PlaceholderMode::Bare,
                    });
                    index = end_index;
                } else {
                    index += 1;
                }
            }
            _ => {
                index += 1;
            }
        }
    }
    index
}

fn scan_string(
    source: &str,
    start: usize,
    quote: u8,
    placeholders: &mut Vec<Placeholder>,
) -> usize {
    let bytes = source.as_bytes();
    let mut index = start + 1;
    let content_start = index;
    let mut escaped = false;

    while index < bytes.len() {
        let byte = bytes[index];
        if escaped {
            escaped = false;
            index += 1;
            continue;
        }
        if byte == b'\\' {
            escaped = true;
            index += 1;
            continue;
        }
        if byte == quote {
            let content = &source[content_start..index];
            if let Some(name) = content.strip_prefix('$')
                && is_valid_placeholder_name(name)
            {
                placeholders.push(Placeholder {
                    start,
                    end: index + 1,
                    name: name.to_owned(),
                    mode: PlaceholderMode::StringLiteral,
                });
            }
            return index + 1;
        }
        index += 1;
    }

    bytes.len()
}

fn scan_template(source: &str, mut index: usize, placeholders: &mut Vec<Placeholder>) -> usize {
    let bytes = source.as_bytes();
    index += 1;
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index += 2,
            b'`' => return index + 1,
            b'$' if index + 1 < bytes.len() && bytes[index + 1] == b'{' => {
                index = scan_template_expr(source, index + 2, placeholders);
                if index < bytes.len() && bytes[index] == b'}' {
                    index += 1;
                }
            }
            _ => index += 1,
        }
    }
    bytes.len()
}

fn scan_template_expr(
    source: &str,
    mut index: usize,
    placeholders: &mut Vec<Placeholder>,
) -> usize {
    let bytes = source.as_bytes();
    let mut depth = 1usize;

    while index < bytes.len() {
        match bytes[index] {
            b'\'' | b'"' => index = scan_string(source, index, bytes[index], placeholders),
            b'`' => index = scan_template(source, index, placeholders),
            b'/' if index + 1 < bytes.len() && bytes[index + 1] == b'/' => {
                index = skip_line_comment(source, index + 2);
            }
            b'/' if index + 1 < bytes.len() && bytes[index + 1] == b'*' => {
                index = skip_block_comment(source, index + 2);
            }
            b'{' => {
                depth += 1;
                index += 1;
            }
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return index;
                }
                index += 1;
            }
            b'$' => {
                if let Some((name, end_index)) = parse_placeholder_name(source, index + 1) {
                    placeholders.push(Placeholder {
                        start: index,
                        end: end_index,
                        name,
                        mode: PlaceholderMode::Bare,
                    });
                    index = end_index;
                } else {
                    index += 1;
                }
            }
            _ => index += 1,
        }
    }

    bytes.len()
}

fn skip_line_comment(source: &str, mut index: usize) -> usize {
    let bytes = source.as_bytes();
    while index < bytes.len() && bytes[index] != b'\n' {
        index += 1;
    }
    index
}

fn skip_block_comment(source: &str, mut index: usize) -> usize {
    let bytes = source.as_bytes();
    while index + 1 < bytes.len() {
        if bytes[index] == b'*' && bytes[index + 1] == b'/' {
            return index + 2;
        }
        index += 1;
    }
    bytes.len()
}

fn parse_placeholder_name(source: &str, start: usize) -> Option<(String, usize)> {
    let bytes = source.as_bytes();
    if start >= bytes.len() || !is_placeholder_start(bytes[start]) {
        return None;
    }

    let mut end = start + 1;
    while end < bytes.len() && is_placeholder_continue(bytes[end]) {
        end += 1;
    }

    Some((source[start..end].to_owned(), end))
}

fn is_valid_placeholder_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    !bytes.is_empty()
        && is_placeholder_start(bytes[0])
        && bytes[1..].iter().copied().all(is_placeholder_continue)
}

fn is_placeholder_start(byte: u8) -> bool {
    byte == b'_' || byte.is_ascii_alphabetic()
}

fn is_placeholder_continue(byte: u8) -> bool {
    byte == b'_' || byte.is_ascii_alphanumeric()
}

fn extract_generic<'a>(name: &str, ty: &'a Type) -> Option<&'a Type> {
    if let Type::Path(path) = ty {
        let last = path.path.segments.last().unwrap();
        if last.ident == name && !last.arguments.is_empty() {
            return match &last.arguments {
                PathArguments::AngleBracketed(args) => match args.args.first().unwrap() {
                    GenericArgument::Type(arg) => Some(arg),
                    _ => None,
                },
                _ => None,
            };
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_expr_output() {
        let ty: Type = syn::parse_quote!(Expr);
        assert!(parse_input_type("$value + 1", &ty).is_ok());
    }

    #[test]
    fn parses_pat_output() {
        let ty: Type = syn::parse_quote!(Pat);
        assert!(parse_input_type("{ foo, bar }: Props", &ty).is_ok());
    }

    #[test]
    fn parses_assign_target_output() {
        let ty: Type = syn::parse_quote!(AssignTarget);
        assert!(parse_input_type("foo.bar", &ty).is_ok());
    }

    #[test]
    fn parses_prop_or_spread_output() {
        let ty: Type = syn::parse_quote!(PropOrSpread);
        assert!(parse_input_type("foo: bar", &ty).is_ok());
    }

    #[test]
    fn scans_bare_and_string_placeholders() {
        let placeholders = scan_placeholders(r#"fn($arg, "$label", `${$expr}`)"#);
        assert_eq!(placeholders.len(), 3);
        assert_eq!(placeholders[0].name, "arg");
        assert!(matches!(placeholders[0].mode, PlaceholderMode::Bare));
        assert_eq!(placeholders[1].name, "label");
        assert!(matches!(
            placeholders[1].mode,
            PlaceholderMode::StringLiteral
        ));
        assert_eq!(placeholders[2].name, "expr");
    }

    #[test]
    fn skips_comments() {
        let placeholders = scan_placeholders("// $ignored\n$value /* $also_ignored */");
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0].name, "value");
    }
}
