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
        .map(|attr| generate_decorator_descriptor(attr, &package_expr));

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
        #[macroforge_ts::napi_derive::napi(js_name = #run_macro_js_name_lit)]
        pub fn #run_macro_fn_ident(context_json: String) -> macroforge_ts::napi::Result<String> {
            use macroforge_ts::host::Macroforge;

            // Parse the context from JSON
            let ctx: macroforge_ts::ts_syn::MacroContextIR = macroforge_ts::serde_json::from_str(&context_json)
                .map_err(|e| macroforge_ts::napi::Error::new(macroforge_ts::napi::Status::InvalidArg, format!("Invalid context JSON: {}", e)))?;

            // Create TsStream from context
            let input = macroforge_ts::ts_syn::TsStream::with_context(&ctx.target_source, &ctx.file_name, ctx.clone())
                .map_err(|e| macroforge_ts::napi::Error::new(macroforge_ts::napi::Status::GenericFailure, format!("Failed to create TsStream: {:?}", e)))?;

            // Run the macro
            let macro_impl = #struct_ident;
            let result = macro_impl.run(input);

            // Serialize result to JSON
            macroforge_ts::serde_json::to_string(&result)
                .map_err(|e| macroforge_ts::napi::Error::new(macroforge_ts::napi::Status::GenericFailure, format!("Failed to serialize result: {}", e)))
        }
    };

    let output = quote! {
        #function

        pub struct #struct_ident;

        impl macroforge_ts::host::Macroforge for #struct_ident {
            fn name(&self) -> &str {
                #macro_name
            }

            fn kind(&self) -> macroforge_ts::ts_syn::MacroKind {
                #kind_expr
            }

            fn run(&self, input: macroforge_ts::ts_syn::TsStream) -> macroforge_ts::ts_syn::MacroResult {
                match #fn_ident(input) {
                    Ok(stream) => macroforge_ts::ts_syn::TsStream::into_result(stream),
                    Err(err) => err.into(),
                }
            }

            fn description(&self) -> &str {
                #description
            }
        }

        #[allow(non_upper_case_globals)]
        const #ctor_ident: fn() -> std::sync::Arc<dyn macroforge_ts::host::Macroforge> = || {
            std::sync::Arc::new(#struct_ident)
        };

        #[allow(non_upper_case_globals)]
        static #decorator_array_ident: &[macroforge_ts::host::derived::DecoratorDescriptor] = &[
            #(#decorator_exprs),*
        ];

        #[allow(non_upper_case_globals)]
        static #descriptor_ident: macroforge_ts::host::derived::DerivedMacroDescriptor =
            macroforge_ts::host::derived::DerivedMacroDescriptor {
                package: #package_expr,
                module: #module_expr,
                runtime: &[#(#runtime_exprs),*],
                name: #macro_name,
                kind: #kind_expr,
                description: #description,
                constructor: #ctor_ident,
                decorators: #decorator_array_ident,
            };

        macroforge_ts::inventory::submit! {
            macroforge_ts::host::derived::DerivedMacroRegistration {
                descriptor: &#descriptor_ident
            }
        }

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
                // Parse attributes(...) which can contain:
                // - Simple identifiers: `serde`
                // - Tuples with docs: `(serde, "Configure serialization")`
                let content;
                syn::parenthesized!(content in meta.input);

                while !content.is_empty() {
                    // Check if it's a tuple (starts with parenthesis)
                    if content.peek(syn::token::Paren) {
                        // Parse (ident, "docs") tuple
                        let inner;
                        syn::parenthesized!(inner in content);
                        let ident: Ident = inner.parse()?;
                        inner.parse::<syn::Token![,]>()?;
                        let docs: LitStr = inner.parse()?;
                        opts.attributes.push(AttributeWithDoc::with_docs(ident, docs));
                    } else {
                        // Parse simple identifier
                        let ident: Ident = content.parse()?;
                        opts.attributes.push(AttributeWithDoc::new(ident));
                    }

                    // Consume optional comma between attributes
                    if content.peek(syn::Token![,]) {
                        content.parse::<syn::Token![,]>()?;
                    }
                }
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

/// An attribute with optional documentation
/// Supports both `attr_name` and `(attr_name, "documentation")` syntax
struct AttributeWithDoc {
    name: Ident,
    docs: LitStr,
}

impl AttributeWithDoc {
    fn new(name: Ident) -> Self {
        Self {
            docs: LitStr::new("", name.span()),
            name,
        }
    }

    fn with_docs(name: Ident, docs: LitStr) -> Self {
        Self { name, docs }
    }
}

struct MacroOptions {
    name: Ident,
    description: Option<LitStr>,
    kind: MacroKindOption,
    attributes: Vec<AttributeWithDoc>,
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

// Helper function to generate a decorator descriptor from an attribute with docs
fn generate_decorator_descriptor(attr: &AttributeWithDoc, package_expr: &TokenStream2) -> TokenStream2 {
    let attr_str = LitStr::new(&attr.name.to_string(), attr.name.span());
    let kind = quote! { macroforge_ts::host::derived::DecoratorKind::Property };
    let docs = &attr.docs;

    quote! {
        macroforge_ts::host::derived::DecoratorDescriptor {
            module: #package_expr,
            export: #attr_str,
            kind: #kind,
            docs: #docs,
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
            MacroKindOption::Derive => quote! { macroforge_ts::ts_syn::MacroKind::Derive },
            MacroKindOption::Attribute => quote! { macroforge_ts::ts_syn::MacroKind::Attribute },
            MacroKindOption::Call => quote! { macroforge_ts::ts_syn::MacroKind::Call },
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
