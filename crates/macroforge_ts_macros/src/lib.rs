//! # Macroforge TypeScript Macros
//!
//! This crate provides procedural macros for generating TypeScript macro infrastructure
//! in the Macroforge ecosystem. It simplifies the creation of derive macros that can
//! transform TypeScript classes at compile time.
//!
//! ## Overview
//!
//! The primary macro provided is [`ts_macro_derive`], which transforms a Rust function
//! into a fully-fledged TypeScript macro that integrates with the Macroforge runtime.
//!
//! ## Example
//!
//! ```rust,ignore
//! use macroforge_ts_macros::ts_macro_derive;
//!
//! #[ts_macro_derive(Debug, description = "Generates debug formatting")]
//! fn debug_macro(input: TsStream) -> Result<TsStream, MacroError> {
//!     // Transform the input TypeScript class
//!     Ok(input)
//! }
//! ```
//!
//! This generates:
//! - A struct implementing the [`Macroforge`] trait
//! - A NAPI function for JavaScript interop
//! - Registration with the macro registry via `inventory`
//!
//! ## Architecture
//!
//! The generated code follows this pattern:
//!
//! 1. **Macro Struct**: A unit struct that implements the `Macroforge` trait
//! 2. **NAPI Bridge**: A function exposed to JavaScript that handles JSON serialization
//! 3. **Descriptor**: Static metadata about the macro for runtime discovery
//! 4. **Registration**: Automatic registration with the `inventory` crate

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Ident, ItemFn, LitStr, Result, parse::Parser, parse_macro_input, spanned::Spanned};

/// A procedural macro attribute that transforms a function into a TypeScript derive macro.
///
/// This attribute macro takes a function that processes TypeScript code and generates
/// all the necessary infrastructure for it to work as a Macroforge derive macro.
///
/// # Arguments
///
/// The macro accepts the following arguments:
///
/// - **name** (required, positional): The macro name as an identifier (e.g., `Debug`, `Clone`)
/// - **description** (optional): A string literal describing the macro's purpose
/// - **kind** (optional): The macro type - `"derive"` (default), `"attribute"`, or `"function"`
/// - **attributes** (optional): Decorator attributes that modify the macro's behavior
///
/// # Generated Code
///
/// For a function `debug_macro` with macro name `Debug`, this generates:
///
/// 1. **`Debug` struct**: Implements the `Macroforge` trait
/// 2. **`__ts_macro_run_debug` function**: NAPI-exported function for JS interop
/// 3. **`__TS_MACRO_DESCRIPTOR_DEBUG` static**: Metadata descriptor
/// 4. **Inventory registration**: Automatic discovery at runtime
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust,ignore
/// #[ts_macro_derive(Clone)]
/// fn clone_macro(input: TsStream) -> Result<TsStream, MacroError> {
///     // Implementation
///     Ok(input)
/// }
/// ```
///
/// ## With Description
///
/// ```rust,ignore
/// #[ts_macro_derive(Serialize, description = "Generates JSON serialization methods")]
/// fn serialize_macro(input: TsStream) -> Result<TsStream, MacroError> {
///     Ok(input)
/// }
/// ```
///
/// ## With Decorators
///
/// ```rust,ignore
/// #[ts_macro_derive(
///     Serde,
///     description = "Serialization with validation",
///     attributes((serde, "Configure serialization"), (validate, "Add validators"))
/// )]
/// fn serde_macro(input: TsStream) -> Result<TsStream, MacroError> {
///     Ok(input)
/// }
/// ```
///
/// ## Attribute Macro
///
/// ```rust,ignore
/// #[ts_macro_derive(Route, kind = "attribute")]
/// fn route_macro(input: TsStream) -> Result<TsStream, MacroError> {
///     Ok(input)
/// }
/// ```
///
/// # Panics
///
/// This macro will produce a compile error if:
/// - No macro name is provided
/// - The macro name is not a valid identifier
/// - An unknown option is specified
/// - The `kind` value is not one of the valid options
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
        /// Run this macro with the given context.
        ///
        /// This function is automatically generated and exposed to JavaScript via NAPI.
        /// It deserializes the macro context from JSON, executes the macro transformation,
        /// and serializes the result back to JSON for the TypeScript plugin.
        ///
        /// # Arguments
        ///
        /// * `context_json` - A JSON string containing the [`MacroContextIR`] with:
        ///   - `target_source`: The TypeScript source code to transform
        ///   - `file_name`: The source file path for error reporting
        ///   - Additional context metadata
        ///
        /// # Returns
        ///
        /// A JSON string containing the [`MacroResult`] with the transformed code
        /// or any diagnostic errors.
        ///
        /// # Errors
        ///
        /// Returns a NAPI error if:
        /// - The input JSON cannot be parsed
        /// - The `TsStream` cannot be created from the context
        /// - The result cannot be serialized to JSON
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

        /// Generated macro struct implementing the [`Macroforge`] trait.
        ///
        /// This struct is automatically created by the `#[ts_macro_derive]` attribute
        /// and provides the interface between the macro runtime and the implementation function.
        pub struct #struct_ident;

        impl macroforge_ts::host::Macroforge for #struct_ident {
            /// Returns the name of this macro as used in `@derive` decorators.
            fn name(&self) -> &str {
                #macro_name
            }

            /// Returns the kind of macro (Derive, Attribute, or Call).
            fn kind(&self) -> macroforge_ts::ts_syn::MacroKind {
                #kind_expr
            }

            /// Executes the macro transformation on the input TypeScript stream.
            ///
            /// This method delegates to the user-defined function and converts
            /// the result into a [`MacroResult`] for the runtime.
            fn run(&self, input: macroforge_ts::ts_syn::TsStream) -> macroforge_ts::ts_syn::MacroResult {
                match #fn_ident(input) {
                    Ok(stream) => macroforge_ts::ts_syn::TsStream::into_result(stream),
                    Err(err) => err.into(),
                }
            }

            /// Returns a human-readable description of what this macro does.
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

/// Converts a snake_case identifier to PascalCase.
///
/// This is used to derive struct names from function names. For example,
/// `debug_macro` becomes `DebugMacro`.
///
/// # Arguments
///
/// * `ident` - The identifier to convert
///
/// # Returns
///
/// A new identifier in PascalCase, preserving the original span.
///
/// # Examples
///
/// - `debug_macro` → `DebugMacro`
/// - `r#type` → `Type` (raw identifier prefix stripped)
/// - `serialize_json` → `SerializeJson`
fn pascal_case_ident(ident: &Ident) -> Ident {
    let raw = ident.to_string();
    let trimmed = raw.trim_start_matches("r#");
    let pascal = trimmed.to_case(Case::Pascal);
    format_ident!("{}", pascal)
}

/// Parses the attribute arguments into a [`MacroOptions`] struct.
///
/// This function handles the custom syntax of `#[ts_macro_derive(...)]`:
///
/// ```text
/// #[ts_macro_derive(MacroName, description = "...", kind = "...", attributes(...))]
/// ```
///
/// # Arguments
///
/// * `tokens` - The token stream from the attribute arguments
///
/// # Returns
///
/// - `Ok(MacroOptions)` with the parsed configuration
/// - `Err(syn::Error)` if parsing fails with a descriptive message
///
/// # Errors
///
/// Returns an error if:
/// - No macro name is provided (empty tokens)
/// - The first token is not an identifier
/// - An unknown option key is used
/// - Option values have incorrect types
///
/// # Syntax
///
/// ## Macro Name (Required)
///
/// The first argument must be a bare identifier representing the macro name:
///
/// ```rust,ignore
/// #[ts_macro_derive(Debug)]  // ✓ Valid
/// #[ts_macro_derive("Debug")]  // ✗ Invalid - must be identifier, not string
/// ```
///
/// ## Optional Arguments
///
/// After the name, comma-separated key-value pairs can be provided:
///
/// - `description = "..."` - A string describing the macro
/// - `kind = "derive"` - One of "derive", "attribute", or "function"/"call"
/// - `attributes(...)` - List of decorator attributes
///
/// ## Attributes Syntax
///
/// The `attributes` option supports two forms:
///
/// ```rust,ignore
/// // Simple identifier (no documentation)
/// attributes(serde, clone)
///
/// // Tuple with documentation
/// attributes((serde, "Configure serialization"), (validate, "Add validators"))
///
/// // Mixed
/// attributes(clone, (serde, "Serialization support"))
/// ```
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

    // Consume optional comma after the macro name
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
                        opts.attributes
                            .push(AttributeWithDoc::with_docs(ident, docs));
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

/// Represents a decorator attribute with optional documentation.
///
/// Decorators are field-level annotations that modify how the macro processes
/// individual class members. For example, `@serde.skip` might exclude a field
/// from serialization.
///
/// # Syntax
///
/// Supports two forms in the `attributes` option:
///
/// - Simple: `attr_name` - Creates an attribute with empty documentation
/// - Documented: `(attr_name, "Documentation string")` - Includes docs
///
/// # Examples
///
/// ```rust,ignore
/// // In macro definition
/// #[ts_macro_derive(Serde, attributes(
///     skip,                                    // Simple form
///     (rename, "Rename the field in output")   // With documentation
/// ))]
/// ```
struct AttributeWithDoc {
    /// The attribute identifier (e.g., `skip`, `rename`)
    name: Ident,
    /// Documentation string for the attribute, empty if not provided
    docs: LitStr,
}

impl AttributeWithDoc {
    /// Creates a new attribute with empty documentation.
    ///
    /// # Arguments
    ///
    /// * `name` - The attribute identifier
    ///
    /// # Returns
    ///
    /// An `AttributeWithDoc` with an empty documentation string.
    fn new(name: Ident) -> Self {
        Self {
            docs: LitStr::new("", name.span()),
            name,
        }
    }

    /// Creates a new attribute with the specified documentation.
    ///
    /// # Arguments
    ///
    /// * `name` - The attribute identifier
    /// * `docs` - The documentation string literal
    ///
    /// # Returns
    ///
    /// An `AttributeWithDoc` with the provided documentation.
    fn with_docs(name: Ident, docs: LitStr) -> Self {
        Self { name, docs }
    }
}

/// Configuration options parsed from the `#[ts_macro_derive]` attribute.
///
/// This struct holds all the settings extracted from the macro attribute
/// and is used to generate the appropriate code.
///
/// # Fields
///
/// - `name`: The macro's public name (required)
/// - `description`: Human-readable description (optional)
/// - `kind`: The type of macro - derive, attribute, or function
/// - `attributes`: List of decorator attributes the macro supports
struct MacroOptions {
    /// The name of the macro as used in `@derive` decorators.
    ///
    /// This is the first positional argument and is required.
    /// Example: `Debug` in `@derive(Debug)`
    name: Ident,

    /// An optional human-readable description of the macro.
    ///
    /// This is displayed in documentation and error messages.
    description: Option<LitStr>,

    /// The kind of macro (derive, attribute, or call).
    ///
    /// Defaults to `Derive` if not specified.
    kind: MacroKindOption,

    /// List of decorator attributes that this macro recognizes.
    ///
    /// These are field-level annotations like `@serde.skip` or `@validate.email`.
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

/// Generates a `DecoratorDescriptor` token stream for a given attribute.
///
/// This function creates the static descriptor that describes a decorator
/// attribute, including its module, export name, kind, and documentation.
///
/// # Arguments
///
/// * `attr` - The attribute definition with name and optional docs
/// * `package_expr` - Token stream evaluating to the package name
///
/// # Returns
///
/// A token stream that constructs a `DecoratorDescriptor` at compile time.
///
/// # Generated Structure
///
/// ```rust,ignore
/// DecoratorDescriptor {
///     module: "package-name",
///     export: "attribute_name",
///     kind: DecoratorKind::Property,
///     docs: "Documentation string",
/// }
/// ```
fn generate_decorator_descriptor(
    attr: &AttributeWithDoc,
    package_expr: &TokenStream2,
) -> TokenStream2 {
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

/// Represents the different kinds of macros supported by Macroforge.
///
/// This enum maps to `MacroKind` in the runtime but is used at compile time
/// for code generation.
///
/// # Variants
///
/// - `Derive`: Class-level macros applied via `@derive(MacroName)`
/// - `Attribute`: Decorators that modify declarations (e.g., `@Route("/path")`)
/// - `Call`: Function-like macros invoked directly (e.g., `include!("file.ts")`)
#[derive(Clone, Default)]
enum MacroKindOption {
    /// Derive macros that generate code based on class structure.
    ///
    /// These are the most common type and are applied using `@derive`:
    ///
    /// ```typescript
    /// @derive(Debug, Clone)
    /// class User { ... }
    /// ```
    #[default]
    Derive,

    /// Attribute macros that transform decorated items.
    ///
    /// These work like TypeScript decorators and can modify
    /// the structure or behavior of the decorated element:
    ///
    /// ```typescript
    /// @Route("/api/users")
    /// class UserController { ... }
    /// ```
    Attribute,

    /// Function-like macros that are invoked with arguments.
    ///
    /// These macros are called like functions and can generate
    /// arbitrary code:
    ///
    /// ```typescript
    /// const config = include!("config.json");
    /// ```
    Call,
}

impl MacroKindOption {
    /// Converts this option to a token stream representing the runtime `MacroKind`.
    ///
    /// # Returns
    ///
    /// A token stream that evaluates to the corresponding `MacroKind` variant.
    fn as_tokens(&self) -> TokenStream2 {
        match self {
            MacroKindOption::Derive => quote! { macroforge_ts::ts_syn::MacroKind::Derive },
            MacroKindOption::Attribute => quote! { macroforge_ts::ts_syn::MacroKind::Attribute },
            MacroKindOption::Call => quote! { macroforge_ts::ts_syn::MacroKind::Call },
        }
    }

    /// Parses a string literal into a `MacroKindOption`.
    ///
    /// # Arguments
    ///
    /// * `lit` - A string literal containing the kind name
    ///
    /// # Returns
    ///
    /// - `Ok(MacroKindOption)` for valid kind names
    /// - `Err(syn::Error)` for invalid values
    ///
    /// # Valid Values
    ///
    /// - `"derive"` → `Derive`
    /// - `"attribute"` → `Attribute`
    /// - `"function"` or `"call"` → `Call`
    ///
    /// Matching is case-insensitive.
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
