use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Attribute, DeriveInput, Ident, LitStr, Result, parse_macro_input, spanned::Spanned};

#[proc_macro_derive(TsMacroDefinition, attributes(ts_macro))]
pub fn ts_macro_definition(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_ts_macro(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand_ts_macro(input: &DeriveInput) -> Result<TokenStream2> {
    let opts = MacroOptions::from_attrs(&input.attrs)?;
    let ident = &input.ident;
    let macro_name = opts
        .name
        .clone()
        .unwrap_or_else(|| default_macro_name(ident));
    let description = opts
        .description
        .clone()
        .unwrap_or_else(|| LitStr::new("", Span::call_site()));
    let package_expr = opts
        .package
        .clone()
        .map(|lit| quote! { #lit })
        .unwrap_or_else(|| quote! { env!("CARGO_PKG_NAME") });
    let module_expr = opts
        .module
        .clone()
        .map(|lit| quote! { #lit })
        .unwrap_or_else(|| quote! { "@macro/derive" });
    let runtime_values = if opts.runtime.is_empty() {
        vec![LitStr::new("native", Span::call_site())]
    } else {
        opts.runtime.clone()
    };
    let runtime_exprs = runtime_values.iter().map(|lit| quote! { #lit });
    let kind_expr = opts.kind.as_tokens();
    let entry_method = opts.entry_method.clone();
    let decorator_exprs = opts
        .decorators
        .iter()
        .map(|decorator| decorator.to_tokens(&package_expr));

    let constructor_name = format_ident!(
        "__ts_macro_ctor_{}",
        ident.to_string().trim_start_matches("r#")
    );
    let descriptor_name =
        format_ident!("__TS_MACRO_DESCRIPTOR_{}", ident.to_string().to_uppercase());
    let decorator_array_name =
        format_ident!("__TS_MACRO_DECORATORS_{}", ident.to_string().to_uppercase());

    Ok(quote! {
        impl ts_macro_host::TsMacro for #ident {
            fn name(&self) -> &str {
                #macro_name
            }

            fn kind(&self) -> ts_macro_abi::MacroKind {
                #kind_expr
            }

            fn run(&self, ctx: ts_macro_abi::MacroContextIR) -> ts_macro_abi::MacroResult {
                self.#entry_method(ctx)
            }

            fn description(&self) -> &str {
                #description
            }
        }

        #[allow(non_upper_case_globals)]
        #[allow(non_upper_case_globals)]
        const #constructor_name: fn() -> std::sync::Arc<dyn ts_macro_host::TsMacro> = || {
            std::sync::Arc::new(#ident)
        };

        #[allow(non_upper_case_globals)]
        static #decorator_array_name: &[ts_macro_host::derived::DecoratorDescriptor] = &[
            #(#decorator_exprs),*
        ];

        #[allow(non_upper_case_globals)]
        static #descriptor_name: ts_macro_host::derived::DerivedMacroDescriptor = ts_macro_host::derived::DerivedMacroDescriptor {
            package: #package_expr,
            module: #module_expr,
            runtime: &[#(#runtime_exprs),*],
            name: #macro_name,
            kind: #kind_expr,
            description: #description,
            constructor: #constructor_name,
            decorators: #decorator_array_name,
        };

        inventory::submit! {
            ts_macro_host::derived::DerivedMacroRegistration {
                descriptor: &#descriptor_name
            }
        }
    })
}

fn default_macro_name(ident: &Ident) -> LitStr {
    let name = ident.to_string();
    if let Some(stripped) = name.strip_suffix("Macro") {
        LitStr::new(stripped, ident.span())
    } else {
        LitStr::new(&name, ident.span())
    }
}

struct MacroOptions {
    package: Option<LitStr>,
    module: Option<LitStr>,
    name: Option<LitStr>,
    kind: MacroKindOption,
    description: Option<LitStr>,
    runtime: Vec<LitStr>,
    entry_method: Ident,
    decorators: Vec<DecoratorOptions>,
}

impl MacroOptions {
    fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut opts = MacroOptions::default();

        for attr in attrs.iter().filter(|attr| attr.path().is_ident("ts_macro")) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("package") {
                    opts.package = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("module") {
                    opts.module = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("name") {
                    opts.name = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("description") {
                    opts.description = Some(meta.value()?.parse()?);
                } else if meta.path.is_ident("entry") {
                    let lit: LitStr = meta.value()?.parse()?;
                    opts.entry_method = Ident::new(&lit.value(), lit.span());
                } else if meta.path.is_ident("runtime") {
                    let content: syn::ExprArray = meta.value()?.parse()?;
                    let mut values = Vec::new();
                    for expr in content.elems.iter() {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit),
                            ..
                        }) = expr
                        {
                            values.push(lit.clone());
                        } else {
                            return Err(syn::Error::new(
                                expr.span(),
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
                    return Err(syn::Error::new(meta.path.span(), "unknown ts_macro option"));
                }
                Ok(())
            })?;
        }

        Ok(opts)
    }
}

impl Default for MacroOptions {
    fn default() -> Self {
        MacroOptions {
            package: None,
            module: None,
            name: None,
            kind: MacroKindOption::default(),
            description: None,
            runtime: Vec::new(),
            entry_method: Ident::new("expand", Span::call_site()),
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
    fn parse(meta: syn::meta::ParseNestedMeta<'_>) -> Result<Self> {
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

        let kind = kind.unwrap_or(DecoratorKindOption::Property);

        Ok(DecoratorOptions {
            module,
            export,
            kind,
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
