use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Ident, ItemFn, LitStr, Result, parse::Parser, parse_macro_input, spanned::Spanned};

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

    // Macro name is now required as first argument, convert to LitStr
    let macro_name = LitStr::new(&options.name.to_string(), options.name.span());
    let description = options
        .description
        .clone()
        .unwrap_or_else(|| LitStr::new("", Span::call_site()));

    // Derive package from CARGO_PKG_NAME
    let package_expr = quote! { env!("CARGO_PKG_NAME") };

    // Module is always resolved dynamically at runtime based on import source
    let module_expr = quote! { "__DYNAMIC_MODULE__" };

    // Default runtime to ["native"]
    let runtime_values = [LitStr::new("native", Span::call_site())];
    let runtime_exprs = runtime_values.iter().map(|lit| quote! { #lit });

    let kind_expr = options.kind.as_tokens();

    // Generate decorators from attributes list
    let decorator_exprs = options
        .attributes
        .iter()
        .map(|attr_name| generate_decorator_descriptor(attr_name, &package_expr));
    let decorator_stubs = options.attributes.iter().map(|attr_name| {
        generate_decorator_stub(attr_name, &struct_ident, options.description.as_ref())
    });

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

    let main_macro_stub_fn_ident = format_ident!(
        "__ts_macro_runtime_stub_{}",
        struct_ident.to_string().to_case(Case::Snake)
    );
    let features_args_type = features_args_type_literal();
    let main_macro_napi_stub = quote! {
        #[::napi_derive::napi(
            js_name = #macro_name,
            ts_return_type = "ClassDecorator",
            ts_args_type = #features_args_type
        )]
        pub fn #main_macro_stub_fn_ident() -> ::napi::Result<()> {
            // This stub function does nothing at runtime; its purpose is purely for TypeScript import resolution.
            Ok(())
        }
    };

    // Generate the runMacro NAPI function for this specific macro
    // Use the macro name (not the struct name) for consistent naming
    let run_macro_fn_ident = format_ident!(
        "__ts_macro_run_{}",
        options.name.to_string().to_case(Case::Snake)
    );
    let run_macro_js_name = format!("__macroforgeRun{}", options.name);
    let run_macro_js_name_lit = LitStr::new(&run_macro_js_name, Span::call_site());

    let run_macro_napi = quote! {
        /// Run this macro with the given context
        /// Called by the TS plugin to execute macro expansion
        #[::napi_derive::napi(js_name = #run_macro_js_name_lit)]
        pub fn #run_macro_fn_ident(context_json: String) -> ::napi::Result<String> {
            use ts_macro_host::Macroforge;

            // Parse the context from JSON
            let ctx: ts_macro_abi::MacroContextIR = serde_json::from_str(&context_json)
                .map_err(|e| ::napi::Error::new(::napi::Status::InvalidArg, format!("Invalid context JSON: {}", e)))?;

            // Create TsStream from context
            let input = ts_syn::TsStream::with_context(&ctx.target_source, &ctx.file_name, ctx.clone())
                .map_err(|e| ::napi::Error::new(::napi::Status::GenericFailure, format!("Failed to create TsStream: {:?}", e)))?;

            // Run the macro
            let macro_impl = #struct_ident;
            let result = macro_impl.run(input);

            // Serialize result to JSON
            serde_json::to_string(&result)
                .map_err(|e| ::napi::Error::new(::napi::Status::GenericFailure, format!("Failed to serialize result: {}", e)))
        }
    };

    let output = quote! {
        #function

        pub struct #struct_ident;

        impl ts_macro_host::Macroforge for #struct_ident {
            fn name(&self) -> &str {
                #macro_name
            }

            fn kind(&self) -> ts_macro_abi::MacroKind {
                #kind_expr
            }

            fn run(&self, input: ts_syn::TsStream) -> ts_macro_abi::MacroResult {
                match #fn_ident(input) {
                    Ok(stream) => ts_syn::TsStream::into_result(stream),
                    Err(err) => err.into(),
                }
            }

            fn description(&self) -> &str {
                #description
            }
        }

        #[allow(non_upper_case_globals)]
        const #ctor_ident: fn() -> std::sync::Arc<dyn ts_macro_host::Macroforge> = || {
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

        #(#decorator_stubs)*

        #main_macro_napi_stub

        #run_macro_napi
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
    if tokens.is_empty() {
        return Err(syn::Error::new(
            Span::call_site(),
            "ts_macro_derive requires a macro name as the first argument",
        ));
    }

    let mut opts = MacroOptions::default();
    let mut tokens_iter = tokens.into_iter().peekable();

    // First argument must be an identifier (the macro name)
    let first = tokens_iter
        .next()
        .ok_or_else(|| syn::Error::new(Span::call_site(), "expected macro name"))?;

    opts.name = match first {
        proc_macro2::TokenTree::Ident(ident) => ident,
        _ => {
            return Err(syn::Error::new(
                first.span(),
                "macro name must be an identifier",
            ));
        }
    };

    // Consume optional comma
    if let Some(proc_macro2::TokenTree::Punct(p)) = tokens_iter.peek()
        && p.as_char() == ','
    {
        tokens_iter.next();
    }

    // Collect remaining tokens for meta parsing
    let remaining: TokenStream2 = tokens_iter.collect();

    if !remaining.is_empty() {
        let parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("description") {
                opts.description = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("kind") {
                let lit: LitStr = meta.value()?.parse()?;
                opts.kind = MacroKindOption::from_lit(&lit)?;
            } else if meta.path.is_ident("attributes") {
                meta.parse_nested_meta(|attr_meta| {
                    if let Some(ident) = attr_meta.path.get_ident() {
                        opts.attributes.push(ident.clone());
                    } else {
                        return Err(syn::Error::new(
                            attr_meta.path.span(),
                            "attribute name must be an identifier",
                        ));
                    }
                    Ok(())
                })?;
            } else {
                return Err(syn::Error::new(
                    meta.path.span(),
                    "unknown ts_macro_derive option",
                ));
            }
            Ok(())
        });

        parser.parse2(remaining)?;
    }

    Ok(opts)
}

struct MacroOptions {
    name: Ident,
    description: Option<LitStr>,
    kind: MacroKindOption,
    attributes: Vec<Ident>,
}

impl Default for MacroOptions {
    fn default() -> Self {
        MacroOptions {
            name: Ident::new("Unknown", Span::call_site()),
            description: None,
            kind: MacroKindOption::Derive,
            attributes: Vec::new(),
        }
    }
}

fn features_args_type_literal() -> LitStr {
    // Allow strings, decorators, closures, or option objects (for field-level configs)
    LitStr::new(
        "...features: Array<string | ClassDecorator | PropertyDecorator | ((...args:\n  any[]) => unknown) | Record<string, unknown>>",
        Span::call_site(),
    )
}

// Helper function to generate a decorator descriptor from an attribute identifier
fn generate_decorator_descriptor(attr_name: &Ident, package_expr: &TokenStream2) -> TokenStream2 {
    let attr_str = LitStr::new(&attr_name.to_string(), attr_name.span());
    let kind = quote! { ts_macro_host::derived::DecoratorKind::Property };
    let docs = LitStr::new("", Span::call_site());

    quote! {
        ts_macro_host::derived::DecoratorDescriptor {
            module: #package_expr,
            export: #attr_str,
            kind: #kind,
            docs: #docs,
        }
    }
}

// Helper function to generate a napi stub for an attribute decorator
fn generate_decorator_stub(
    attr_name: &Ident,
    owner_ident: &Ident,
    description: Option<&LitStr>,
) -> TokenStream2 {
    let owner_snake = owner_ident.to_string().to_case(Case::Snake);
    let decorator_snake = attr_name.to_string().to_case(Case::Snake);
    let fn_ident = format_ident!("__ts_macro_stub_{}_{}", owner_snake, decorator_snake);
    let js_name = LitStr::new(&attr_name.to_string(), attr_name.span());

    let doc_comment = description.map(|desc| {
        let desc_str = desc.value();
        quote! {
            #[doc = #desc_str]
        }
    });

    let decorator_args_type = features_args_type_literal();

    quote! {
        #doc_comment
        #[::napi_derive::napi(
            js_name = #js_name,
            ts_args_type = #decorator_args_type,
            ts_return_type = "PropertyDecorator"
        )]
        pub fn #fn_ident() -> ::napi::Result<()> {
            Ok(())
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
