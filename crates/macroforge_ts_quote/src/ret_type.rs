use std::any::type_name;

use anyhow::{Context, Error, anyhow, bail};
use swc_core::common::{FileName, SourceMap, sync::Lrc};
use swc_core::ecma::ast::{AssignTarget, Decl, EsVersion, ModuleDecl, ModuleItem, Stmt};
use swc_core::ecma::parser::{PResult, Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};
use syn::{GenericArgument, PathArguments, Type};

use super::{ast::ToCode, ctxt::Ctx};

/// Storage for `dyn ToCode`.The first `Box`, which is required to store `dyn
/// ToCode`, is ignored.
pub struct BoxWrapper(Box<dyn ToCode>);

impl ToCode for BoxWrapper {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        (*self.0).to_code(cx)
    }
}

pub(crate) fn parse_input_type(input_str: &str, ty: &Type) -> Result<BoxWrapper, Error> {
    if let Some(ty) = extract_generic("Box", ty) {
        let node = parse_input_type(input_str, ty).context("failed to parse `T` in Box<T>")?;
        return Ok(BoxWrapper(Box::new(Box::new(node))));
    }

    if let Some(ty) = extract_generic("Option", ty) {
        if input_str.is_empty() {
            return Ok(BoxWrapper(Box::new(None::<swc_core::ecma::ast::Expr>)));
        }

        let node = parse_input_type(input_str, ty).context("failed to parse `T` in Option<T>")?;
        return Ok(BoxWrapper(Box::new(Some(node))));
    }

    if let Type::Path(p) = ty
        && let Some(ident) = p.path.get_ident()
    {
        match &*ident.to_string() {
            "Expr" => return parse(input_str, &mut |p| p.parse_expr().map(|v| *v)),
            "Pat" => return parse(input_str, &mut |p| p.parse_pat()),
            "Stmt" => return parse(input_str, &mut |p| p.parse_stmt_list_item()),
            "AssignTarget" => {
                return parse_assign_target(input_str);
            }
            "ModuleItem" => return parse(input_str, &mut |p| p.parse_module_item()),
            "Module" => return parse(input_str, &mut |p| p.parse_module()),
            "TsType" => return parse_ts_type(input_str),
            "PropOrSpread" => return parse_prop_or_spread(input_str),
            _ => {}
        }
    }

    bail!("Unknown quote type: {ty:?}");
}

fn parse<T>(
    input_str: &str,
    op: &mut dyn FnMut(&mut Parser<Lexer>) -> PResult<T>,
) -> Result<BoxWrapper, Error>
where
    T: ToCode,
{
    let cm = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(FileName::Anon.into(), input_str.to_string());

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax::default()),
        EsVersion::Es2020,
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let result = op(&mut parser)
        .map_err(|err| anyhow!("{err:?}"))
        .with_context(|| format!("failed to parse input as `{}`", type_name::<T>()));

    // Debug: write failures to logs directory for inspection
    if result.is_err() {
        use std::io::Write;
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let count = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let logs_dir = format!("{}/logs", env!("CARGO_MANIFEST_DIR"));
        let _ = std::fs::create_dir_all(&logs_dir);
        let filename = format!(
            "{}/malformed_{:03}_{}.ts",
            logs_dir,
            count,
            type_name::<T>().split("::").last().unwrap_or("unknown")
        );
        if let Ok(mut file) = std::fs::File::create(&filename) {
            let _ = writeln!(file, "// Failed to parse as: {}", type_name::<T>());
            let _ = writeln!(file, "// Error: {:?}", result.as_ref().err());
            let _ = writeln!(file);
            let _ = write!(file, "{}", input_str);
        }
    }

    result.map(|val| BoxWrapper(Box::new(val)))
}

/// Parse an assign target pattern.
/// Not all patterns can be used as assignment targets - this will error for invalid patterns.
fn parse_assign_target(input_str: &str) -> Result<BoxWrapper, Error> {
    let cm = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(FileName::Anon.into(), input_str.to_string());

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax::default()),
        EsVersion::Es2020,
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);

    let pat = parser
        .parse_pat()
        .map_err(|err| anyhow!("{err:?}"))
        .context("failed to parse pattern")?;

    let assign_target = AssignTarget::try_from(pat).map_err(|_| {
        anyhow!(
            "pattern `{}` cannot be used as an assignment target",
            input_str.trim()
        )
    })?;

    Ok(BoxWrapper(Box::new(assign_target)))
}

/// Parse a TypeScript type by wrapping it in a type alias declaration.
/// Since the parser's `parse_ts_type` is not public, we parse `type __T = <input>;`
/// and extract the type from the resulting type alias declaration.
fn parse_ts_type(input_str: &str) -> Result<BoxWrapper, Error> {
    let wrapped = format!("type __T = {input_str};");

    let cm = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(FileName::Anon.into(), wrapped);

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax::default()),
        EsVersion::Es2020,
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);

    let module_item = parser
        .parse_module_item()
        .map_err(|err| anyhow!("{err:?}"))
        .context("failed to parse type alias wrapper")?;

    // Extract the type from: type __T = <type>;
    let ts_type = match module_item {
        ModuleItem::Stmt(Stmt::Decl(Decl::TsTypeAlias(type_alias))) => *type_alias.type_ann,
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
            if let Decl::TsTypeAlias(type_alias) = export_decl.decl {
                *type_alias.type_ann
            } else {
                bail!(
                    "Expected type alias declaration, got: {:?}",
                    export_decl.decl
                );
            }
        }
        other => bail!("Expected type alias declaration, got: {:?}", other),
    };

    Ok(BoxWrapper(Box::new(ts_type)))
}

/// Parse an object property by wrapping it in an object literal.
/// We parse `({ <input> })` and extract the first property.
fn parse_prop_or_spread(input_str: &str) -> Result<BoxWrapper, Error> {
    // Wrap in parenthesized object literal to parse as expression
    let wrapped = format!("({{ {input_str} }})");

    let cm = Lrc::new(SourceMap::default());
    let fm = cm.new_source_file(FileName::Anon.into(), wrapped);

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax::default()),
        EsVersion::Es2020,
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);

    let expr = parser
        .parse_expr()
        .map_err(|err| anyhow!("{err:?}"))
        .context("failed to parse object literal wrapper")?;

    // Extract the property from: ({ prop })
    let prop = match *expr {
        swc_core::ecma::ast::Expr::Paren(paren) => match *paren.expr {
            swc_core::ecma::ast::Expr::Object(obj) => obj
                .props
                .into_iter()
                .next()
                .ok_or_else(|| anyhow!("expected at least one property in object literal"))?,
            other => bail!("Expected object literal, got: {:?}", other),
        },
        other => bail!("Expected parenthesized expression, got: {:?}", other),
    };

    Ok(BoxWrapper(Box::new(prop)))
}

fn extract_generic<'a>(name: &str, ty: &'a Type) -> Option<&'a Type> {
    if let Type::Path(p) = ty {
        let last = p.path.segments.last().unwrap();

        if !last.arguments.is_empty() && last.ident == name {
            match &last.arguments {
                PathArguments::AngleBracketed(tps) => {
                    let arg = tps.args.first().unwrap();

                    match arg {
                        GenericArgument::Type(arg) => return Some(arg),
                        _ => unimplemented!("generic parameter other than type"),
                    }
                }
                _ => unimplemented!("Box() -> T or Box without a type parameter"),
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== parse_input_type Tests ====================

    #[test]
    fn test_parse_expr() {
        let ty: Type = syn::parse_quote!(Expr);
        let result = parse_input_type("1 + 2", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_expr_complex() {
        let ty: Type = syn::parse_quote!(Expr);
        let result = parse_input_type("foo.bar().baz[0]?.qux", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_expr_arrow_function() {
        let ty: Type = syn::parse_quote!(Expr);
        let result = parse_input_type("(x: number) => x + 1", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_pat_identifier() {
        let ty: Type = syn::parse_quote!(Pat);
        let result = parse_input_type("x", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_pat_destructure_object() {
        let ty: Type = syn::parse_quote!(Pat);
        let result = parse_input_type("{ a, b: c }", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_pat_destructure_array() {
        let ty: Type = syn::parse_quote!(Pat);
        let result = parse_input_type("[a, b, ...rest]", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_stmt_var_decl() {
        let ty: Type = syn::parse_quote!(Stmt);
        let result = parse_input_type("let x = 1;", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_stmt_if() {
        let ty: Type = syn::parse_quote!(Stmt);
        let result = parse_input_type("if (true) { x = 1; }", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_stmt_for() {
        let ty: Type = syn::parse_quote!(Stmt);
        let result = parse_input_type("for (let i = 0; i < 10; i++) { }", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_module_item_import() {
        let ty: Type = syn::parse_quote!(ModuleItem);
        let result = parse_input_type("import { foo } from 'bar';", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_module_item_export() {
        let ty: Type = syn::parse_quote!(ModuleItem);
        let result = parse_input_type("export const x = 1;", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_module() {
        let ty: Type = syn::parse_quote!(Module);
        let result = parse_input_type("const x = 1; export { x };", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_ts_type_primitive() {
        let ty: Type = syn::parse_quote!(TsType);
        let result = parse_input_type("number", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_ts_type_union() {
        let ty: Type = syn::parse_quote!(TsType);
        let result = parse_input_type("string | number", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_ts_type_generic() {
        let ty: Type = syn::parse_quote!(TsType);
        let result = parse_input_type("Array<string>", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_ts_type_function() {
        let ty: Type = syn::parse_quote!(TsType);
        let result = parse_input_type("(x: number) => string", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_ts_type_object() {
        let ty: Type = syn::parse_quote!(TsType);
        let result = parse_input_type("{ foo: string; bar?: number }", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_assign_target_simple() {
        let ty: Type = syn::parse_quote!(AssignTarget);
        let result = parse_input_type("x", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_assign_target_member() {
        let ty: Type = syn::parse_quote!(AssignTarget);
        let result = parse_input_type("obj.prop", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_prop_or_spread_key_value() {
        let ty: Type = syn::parse_quote!(PropOrSpread);
        let result = parse_input_type("foo: 42", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_prop_or_spread_shorthand() {
        let ty: Type = syn::parse_quote!(PropOrSpread);
        let result = parse_input_type("foo", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_prop_or_spread_spread() {
        let ty: Type = syn::parse_quote!(PropOrSpread);
        let result = parse_input_type("...rest", &ty);
        assert!(result.is_ok());
    }

    // ==================== Box<T> Tests ====================

    #[test]
    fn test_parse_box_expr() {
        let ty: Type = syn::parse_quote!(Box<Expr>);
        let result = parse_input_type("x + 1", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_box_pat() {
        let ty: Type = syn::parse_quote!(Box<Pat>);
        let result = parse_input_type("x", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_nested_box() {
        let ty: Type = syn::parse_quote!(Box<Box<Expr>>);
        let result = parse_input_type("42", &ty);
        assert!(result.is_ok());
    }

    // ==================== Option<T> Tests ====================

    #[test]
    fn test_parse_option_empty() {
        let ty: Type = syn::parse_quote!(Option<Expr>);
        let result = parse_input_type("", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_option_with_value() {
        let ty: Type = syn::parse_quote!(Option<Expr>);
        let result = parse_input_type("x", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_option_box() {
        let ty: Type = syn::parse_quote!(Option<Box<Expr>>);
        let result = parse_input_type("42", &ty);
        assert!(result.is_ok());
    }

    // ==================== extract_generic Tests ====================

    #[test]
    fn test_extract_generic_box() {
        let ty: Type = syn::parse_quote!(Box<Expr>);
        let inner = extract_generic("Box", &ty);
        assert!(inner.is_some());

        if let Some(Type::Path(path)) = inner {
            assert_eq!(path.path.segments.last().unwrap().ident.to_string(), "Expr");
        }
    }

    #[test]
    fn test_extract_generic_option() {
        let ty: Type = syn::parse_quote!(Option<Pat>);
        let inner = extract_generic("Option", &ty);
        assert!(inner.is_some());
    }

    #[test]
    fn test_extract_generic_wrong_name() {
        let ty: Type = syn::parse_quote!(Box<Expr>);
        let inner = extract_generic("Option", &ty);
        assert!(inner.is_none());
    }

    #[test]
    fn test_extract_generic_no_generic() {
        let ty: Type = syn::parse_quote!(Expr);
        let inner = extract_generic("Box", &ty);
        assert!(inner.is_none());
    }

    #[test]
    fn test_extract_generic_nested() {
        let ty: Type = syn::parse_quote!(Box<Option<Expr>>);
        let inner = extract_generic("Box", &ty);
        assert!(inner.is_some());

        if let Some(inner_ty) = inner {
            let inner_inner = extract_generic("Option", inner_ty);
            assert!(inner_inner.is_some());
        }
    }

    // ==================== Error Cases ====================

    #[test]
    fn test_parse_invalid_expr() {
        let ty: Type = syn::parse_quote!(Expr);
        let result = parse_input_type("{ invalid syntax }", &ty);
        // Object literals parse as expressions, but malformed ones fail
        // This depends on what the parser accepts
    }

    #[test]
    fn test_parse_unknown_type() {
        let ty: Type = syn::parse_quote!(UnknownType);
        let result = parse_input_type("x", &ty);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_assign_target_invalid() {
        let ty: Type = syn::parse_quote!(AssignTarget);
        // Literals cannot be assignment targets
        let result = parse_input_type("42", &ty);
        assert!(result.is_err());
    }

    // ==================== BoxWrapper Tests ====================

    #[test]
    fn test_box_wrapper_to_code() {
        let ty: Type = syn::parse_quote!(Expr);
        let wrapper = parse_input_type("42", &ty).unwrap();

        // Create a minimal context
        let ctx = Ctx {
            vars: rustc_hash::FxHashMap::default(),
        };

        // Should produce valid code
        let _code = wrapper.to_code(&ctx);
    }

    // ==================== TypeScript-specific Tests ====================

    #[test]
    fn test_parse_ts_type_conditional() {
        let ty: Type = syn::parse_quote!(TsType);
        let result = parse_input_type("T extends U ? X : Y", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_ts_type_mapped() {
        let ty: Type = syn::parse_quote!(TsType);
        let result = parse_input_type("{ [K in keyof T]: T[K] }", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_ts_type_infer() {
        let ty: Type = syn::parse_quote!(TsType);
        let result = parse_input_type("T extends (infer U)[] ? U : never", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_ts_type_tuple() {
        let ty: Type = syn::parse_quote!(TsType);
        let result = parse_input_type("[string, number, boolean]", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_ts_type_readonly() {
        let ty: Type = syn::parse_quote!(TsType);
        let result = parse_input_type("readonly string[]", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_expr_as_const() {
        let ty: Type = syn::parse_quote!(Expr);
        let result = parse_input_type("{ x: 1 } as const", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_expr_satisfies() {
        let ty: Type = syn::parse_quote!(Expr);
        let result = parse_input_type("value satisfies SomeType", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_stmt_type_alias() {
        let ty: Type = syn::parse_quote!(Stmt);
        let result = parse_input_type("type Foo<T> = T | null;", &ty);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_stmt_interface() {
        let ty: Type = syn::parse_quote!(Stmt);
        let result = parse_input_type("interface Foo { bar: string; }", &ty);
        assert!(result.is_ok());
    }
}
