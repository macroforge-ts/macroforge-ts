use std::any::type_name;

use anyhow::{anyhow, bail, Context, Error};
use swc_core::common::{sync::Lrc, FileName, SourceMap};
use swc_core::ecma::ast::{AssignTarget, Decl, EsVersion, ModuleDecl, ModuleItem, PropOrSpread, Stmt};
use swc_core::ecma::parser::{lexer::Lexer, PResult, Parser, StringInput, Syntax, TsSyntax};
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
    op(&mut parser)
        .map_err(|err| anyhow!("{err:?}"))
        .with_context(|| format!("failed to parse input as `{}`", type_name::<T>()))
        .map(|val| BoxWrapper(Box::new(val)))
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
        ModuleItem::Stmt(Stmt::Decl(Decl::TsTypeAlias(type_alias))) => {
            *type_alias.type_ann
        }
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
            if let Decl::TsTypeAlias(type_alias) = export_decl.decl {
                *type_alias.type_ann
            } else {
                bail!("Expected type alias declaration, got: {:?}", export_decl.decl);
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
            swc_core::ecma::ast::Expr::Object(obj) => {
                obj.props.into_iter().next().ok_or_else(|| {
                    anyhow!("expected at least one property in object literal")
                })?
            }
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
