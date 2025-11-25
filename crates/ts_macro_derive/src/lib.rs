use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    Ident, ItemFn, LitStr, Result, meta, parse::Parser, parse_macro_input, spanned::Spanned,
};

#[proc_macro_attribute]
pub fn ts_macro_derive(attr: TokenStream, item: TokenStream) -> TokenStream {
    let options = match parse_macro_options(TokenStream2::from(attr)) {
        Ok(opts) => opts,
        Err(err) => return err.to_compile_error().into(),
    };

    let mut function = parse_macro_input!(item as ItemFn);
    function
        .attrs
        .retain(|attr| !attr.path().is_ident("ts_macro_derive"));

    let fn_ident = function.sig.ident.clone();
    let struct_ident = pascal_case_ident(&fn_ident);
    let macro_name = options
        .name
        .clone()
        .unwrap_or_else(|| LitStr::new(&struct_ident.to_string(), Span::call_site()));
    let description = options
        .description
        .clone()
        .unwrap_or_else(|| LitStr::new("", Span::call_site()));
    let package_expr = options
        .package
        .clone()
        .map(|lit| quote! { #lit })
        .unwrap_or_else(|| quote! { env!("CARGO_PKG_NAME") });
    let module_expr = options
        .module
        .clone()
        .map(|lit| quote! { #lit })
        .unwrap_or_else(|| quote! { "@macro/derive" });
    let runtime_values = if options.runtime.is_empty() {
        vec![LitStr::new("native", Span::call_site())]
    } else {
        options.runtime.clone()
    };
    let runtime_exprs = runtime_values.iter().map(|lit| quote! { #lit });
    let kind_expr = options.kind.as_tokens();
    let decorator_exprs = options
        .decorators
        .iter()
        .map(|decorator| decorator.to_tokens(&package_expr));

    let descriptor_ident = format_ident!(
        "__TS_MACRO_DESCRIPTOR_{}",
        struct_ident.to_string().to_uppercase()
    );
    let decorator_array_ident = format_ident!(
        "__TS_MACRO_DECORATORS_{}",
        struct_ident.to_string().to_uppercase()
    );
    let ctor_ident = format_ident!(
        "__ts_macro_ctor_{}",
        struct_ident.to_string().trim_start_matches("r#")
    );

    let output = quote! {
        #function

        pub struct #struct_ident;

        impl ts_macro_host::TsMacro for #struct_ident {
            fn name(&self) -> &str {
                #macro_name
            }

            fn kind(&self) -> ts_macro_abi::MacroKind {
                #kind_expr
            }

            fn run(&self, ctx: ts_macro_abi::MacroContextIR) -> ts_macro_abi::MacroResult {
                #fn_ident(ctx)
            }

            fn description(&self) -> &str {
                #description
            }
        }

        #[allow(non_upper_case_globals)]
        const #ctor_ident: fn() -> std::sync::Arc<dyn ts_macro_host::TsMacro> = || {
            std::sync::Arc::new(#struct_ident)
        };

        #[allow(non_upper_case_globals)]
        static #decorator_array_ident: &[ts_macro_host::derived::DecoratorDescriptor] = &[
            #(#decorator_exprs),*
        ];

        #[allow(non_upper_case_globals)]
        static #descriptor_ident: ts_macro_host::derived::DerivedMacroDescriptor =
            ts_macro_host::derived::DerivedMacroDescriptor {
                package: #package_expr,
                module: #module_expr,
                runtime: &[#(#runtime_exprs),*],
                name: #macro_name,
                kind: #kind_expr,
                description: #description,
                constructor: #ctor_ident,
                decorators: #decorator_array_ident,
            };

        inventory::submit! {
            ts_macro_host::derived::DerivedMacroRegistration {
                descriptor: &#descriptor_ident
            }
        }
    };

    output.into()
}

fn pascal_case_ident(ident: &Ident) -> Ident {
    let raw = ident.to_string();
    let trimmed = raw.trim_start_matches("r#");
    let pascal = trimmed.to_case(Case::Pascal);
    format_ident!("{}", pascal)
}

fn parse_macro_options(tokens: TokenStream2) -> Result<MacroOptions> {
    let mut opts = MacroOptions::default();
    if tokens.is_empty() {
        return Ok(opts);
    }

    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("package") {
            opts.package = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("module") {
            opts.module = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("name") {
            opts.name = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("description") {
            opts.description = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("runtime") {
            let expr: syn::ExprArray = meta.value()?.parse()?;
            let mut values = Vec::new();
            for element in expr.elems.iter() {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit),
                    ..
                }) = element
                {
                    values.push(lit.clone());
                } else {
                    return Err(syn::Error::new(
                        element.span(),
                        "runtime entries must be string literals",
                    ));
                }
            }
            opts.runtime = values;
        } else if meta.path.is_ident("kind") {
            let lit: LitStr = meta.value()?.parse()?;
            opts.kind = MacroKindOption::from_lit(&lit)?;
        } else if meta.path.is_ident("decorator") {
            let decorator = DecoratorOptions::parse(meta)?;
            opts.decorators.push(decorator);
        } else {
            return Err(syn::Error::new(
                meta.path.span(),
                "unknown ts_macro_derive option",
            ));
        }
        Ok(())
    });

    parser.parse2(tokens)?;
    Ok(opts)
}

struct MacroOptions {
    package: Option<LitStr>,
    module: Option<LitStr>,
    name: Option<LitStr>,
    description: Option<LitStr>,
    runtime: Vec<LitStr>,
    kind: MacroKindOption,
    decorators: Vec<DecoratorOptions>,
}

impl Default for MacroOptions {
    fn default() -> Self {
        MacroOptions {
            package: None,
            module: None,
            name: None,
            description: None,
            runtime: Vec::new(),
            kind: MacroKindOption::Derive,
            decorators: Vec::new(),
        }
    }
}

struct DecoratorOptions {
    module: Option<LitStr>,
    export: LitStr,
    kind: DecoratorKindOption,
    docs: Option<LitStr>,
}

impl DecoratorOptions {
    fn parse(meta: meta::ParseNestedMeta<'_>) -> Result<Self> {
        let mut module = None;
        let mut export = None;
        let mut kind = None;
        let mut docs = None;

        meta.parse_nested_meta(|decor_meta| {
            if decor_meta.path.is_ident("module") {
                module = Some(decor_meta.value()?.parse()?);
            } else if decor_meta.path.is_ident("name") {
                export = Some(decor_meta.value()?.parse()?);
            } else if decor_meta.path.is_ident("kind") {
                let lit: LitStr = decor_meta.value()?.parse()?;
                kind = Some(DecoratorKindOption::from_lit(&lit)?);
            } else if decor_meta.path.is_ident("docs") {
                docs = Some(decor_meta.value()?.parse()?);
            } else {
                return Err(syn::Error::new(
                    decor_meta.path.span(),
                    "unknown decorator option",
                ));
            }
            Ok(())
        })?;

        let export = export
            .ok_or_else(|| syn::Error::new(meta.path.span(), "decorator name is required"))?;

        Ok(DecoratorOptions {
            module,
            export,
            kind: kind.unwrap_or_default(),
            docs,
        })
    }

    fn to_tokens(&self, default_module: &TokenStream2) -> TokenStream2 {
        let module = self
            .module
            .clone()
            .map(|lit| quote! { #lit })
            .unwrap_or_else(|| default_module.clone());
        let export = &self.export;
        let kind = self.kind.as_tokens();
        let docs = self
            .docs
            .clone()
            .unwrap_or_else(|| LitStr::new("", Span::call_site()));

        quote! {
            ts_macro_host::derived::DecoratorDescriptor {
                module: #module,
                export: #export,
                kind: #kind,
                docs: #docs,
            }
        }
    }
}

#[derive(Clone, Default)]
enum MacroKindOption {
    #[default]
    Derive,
    Attribute,
    Call,
}

impl MacroKindOption {
    fn as_tokens(&self) -> TokenStream2 {
        match self {
            MacroKindOption::Derive => quote! { ts_macro_abi::MacroKind::Derive },
            MacroKindOption::Attribute => quote! { ts_macro_abi::MacroKind::Attribute },
            MacroKindOption::Call => quote! { ts_macro_abi::MacroKind::Call },
        }
    }

    fn from_lit(lit: &LitStr) -> Result<Self> {
        match lit.value().to_ascii_lowercase().as_str() {
            "derive" => Ok(MacroKindOption::Derive),
            "attribute" => Ok(MacroKindOption::Attribute),
            "function" | "call" => Ok(MacroKindOption::Call),
            _ => Err(syn::Error::new(
                lit.span(),
                "kind must be one of 'derive', 'attribute', or 'function'",
            )),
        }
    }
}

#[derive(Clone, Default)]
enum DecoratorKindOption {
    Class,
    #[default]
    Property,
    Method,
    Accessor,
    Parameter,
}

impl DecoratorKindOption {
    fn as_tokens(&self) -> TokenStream2 {
        match self {
            DecoratorKindOption::Class => quote! { ts_macro_host::derived::DecoratorKind::Class },
            DecoratorKindOption::Property => {
                quote! { ts_macro_host::derived::DecoratorKind::Property }
            }
            DecoratorKindOption::Method => quote! { ts_macro_host::derived::DecoratorKind::Method },
            DecoratorKindOption::Accessor => {
                quote! { ts_macro_host::derived::DecoratorKind::Accessor }
            }
            DecoratorKindOption::Parameter => {
                quote! { ts_macro_host::derived::DecoratorKind::Parameter }
            }
        }
    }

    fn from_lit(lit: &LitStr) -> Result<Self> {
        match lit.value().to_ascii_lowercase().as_str() {
            "class" => Ok(DecoratorKindOption::Class),
            "property" => Ok(DecoratorKindOption::Property),
            "method" => Ok(DecoratorKindOption::Method),
            "accessor" => Ok(DecoratorKindOption::Accessor),
            "parameter" => Ok(DecoratorKindOption::Parameter),
            other => Err(syn::Error::new(
                lit.span(),
                format!(
                    "decorator kind '{}' is invalid. expected class/property/method/accessor/parameter",
                    other
                ),
            )),
        }
    }
}
