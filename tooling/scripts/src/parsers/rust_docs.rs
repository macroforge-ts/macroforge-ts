//! Rust documentation extraction using syn
//!
//! Extracts //! (module docs) and /// (item docs) from Rust source files.

use serde::{Deserialize, Serialize};
use syn::{Attribute, Item, Lit, Meta};

/// Item documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDoc {
    pub name: String,
    pub kind: String,
    pub signature: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<ParamDoc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returns: Option<ReturnDoc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamDoc {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnDoc {
    #[serde(rename = "type")]
    pub return_type: String,
    pub description: String,
}

/// Extract module-level documentation (//! comments) from source
pub fn extract_module_docs(source: &str) -> String {
    // First try to parse as a file and get inner attributes
    if let Ok(file) = syn::parse_file(source) {
        let doc = extract_doc_from_attrs(&file.attrs);
        if !doc.is_empty() {
            return doc;
        }
    }

    // Fallback: manual extraction for //! comments at the top of the file
    let mut doc_lines = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("//!") {
            let content = rest.strip_prefix(' ').unwrap_or(rest);
            doc_lines.push(content.to_string());
        } else if trimmed.is_empty() && !doc_lines.is_empty() {
            doc_lines.push(String::new());
        } else if !trimmed.starts_with("//") && !trimmed.is_empty() {
            break;
        }
    }

    // Trim trailing empty lines
    while doc_lines.last().map(|s| s.is_empty()).unwrap_or(false) {
        doc_lines.pop();
    }

    doc_lines.join("\n")
}

/// Extract documented items from Rust source using syn
pub fn extract_item_docs(source: &str) -> Vec<ItemDoc> {
    let file = match syn::parse_file(source) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let mut items = Vec::new();

    for item in &file.items {
        if let Some(doc_item) = extract_item_doc(item) {
            items.push(doc_item);
        }
    }

    items
}

/// Extract doc comments from attributes (both `///` and `#[doc = "..."]`)
fn extract_doc_from_attrs(attrs: &[Attribute]) -> String {
    let mut doc_lines = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("doc")
            && let Meta::NameValue(meta) = &attr.meta
            && let syn::Expr::Lit(expr_lit) = &meta.value
            && let Lit::Str(lit_str) = &expr_lit.lit
        {
            let content = lit_str.value();
            // Remove leading space that rustdoc adds
            let content = content.strip_prefix(' ').unwrap_or(&content);
            doc_lines.push(content.to_string());
        }
    }

    doc_lines.join("\n")
}

/// Parsed doc sections: (description, params, returns, examples)
type DocSections = (
    String,
    Option<Vec<ParamDoc>>,
    Option<ReturnDoc>,
    Option<Vec<String>>,
);

/// Split a doc string at the first `# Section` header.
/// Returns (description, params, returns, examples) parsed from the sections.
fn parse_doc_sections(doc: &str) -> DocSections {
    let mut description_lines = Vec::new();
    let mut current_section: Option<String> = None;
    let mut current_content = Vec::new();
    let mut params = Vec::new();
    let mut returns = None;
    let mut examples = Vec::new();

    let section_re = regex::Regex::new(r"^#\s+(.+)$").unwrap();
    let param_re = regex::Regex::new(r"^\*\s*`(\w+)`\s*-\s*(.+)").unwrap();

    for line in doc.lines() {
        if let Some(caps) = section_re.captures(line) {
            // Save previous section
            if let Some(ref section) = current_section {
                save_section(
                    section,
                    &current_content,
                    &param_re,
                    &mut params,
                    &mut returns,
                    &mut examples,
                );
            }
            current_section = Some(caps.get(1).unwrap().as_str().to_lowercase());
            current_content.clear();
        } else if current_section.is_some() {
            current_content.push(line.to_string());
        } else {
            description_lines.push(line.to_string());
        }
    }

    // Save last section
    if let Some(ref section) = current_section {
        save_section(
            section,
            &current_content,
            &param_re,
            &mut params,
            &mut returns,
            &mut examples,
        );
    }

    let description = description_lines.join("\n").trim().to_string();
    let params = if params.is_empty() {
        None
    } else {
        Some(params)
    };
    let examples = if examples.is_empty() {
        None
    } else {
        Some(examples)
    };

    (description, params, returns, examples)
}

fn save_section(
    section: &str,
    content: &[String],
    param_re: &regex::Regex,
    params: &mut Vec<ParamDoc>,
    returns: &mut Option<ReturnDoc>,
    examples: &mut Vec<String>,
) {
    let text = content.join("\n").trim().to_string();
    match section {
        "arguments" | "parameters" | "params" => {
            for line in content {
                if let Some(caps) = param_re.captures(line.trim()) {
                    params.push(ParamDoc {
                        name: caps.get(1).unwrap().as_str().to_string(),
                        param_type: String::new(),
                        description: caps.get(2).unwrap().as_str().trim().to_string(),
                    });
                }
            }
        }
        "returns" | "return" => {
            if !text.is_empty() {
                *returns = Some(ReturnDoc {
                    return_type: String::new(),
                    description: text,
                });
            }
        }
        "example" | "examples" => {
            // Extract code blocks from the example section
            let code_re = regex::Regex::new(r"```(?:\w+)?\n([\s\S]*?)```").unwrap();
            for caps in code_re.captures_iter(&text) {
                examples.push(caps.get(1).unwrap().as_str().trim().to_string());
            }
        }
        _ => {} // Ignore other sections (Errors, Performance, etc.)
    }
}

/// Build an ItemDoc from raw doc string with section parsing
fn build_item_doc(name: String, kind: &str, signature: String, doc: String) -> ItemDoc {
    let (description, params, returns, examples) = parse_doc_sections(&doc);
    ItemDoc {
        name,
        kind: kind.to_string(),
        signature,
        description,
        params,
        returns,
        examples,
    }
}

/// Extract documentation from a single syn Item
fn extract_item_doc(item: &Item) -> Option<ItemDoc> {
    match item {
        Item::Fn(item_fn) => {
            let doc = extract_doc_from_attrs(&item_fn.attrs);
            if !is_public(&item_fn.vis) {
                return None;
            }

            let name = item_fn.sig.ident.to_string();
            let signature = format_fn_signature(&item_fn.sig, &item_fn.vis);

            Some(build_item_doc(name, "function", signature, doc))
        }
        Item::Struct(item_struct) => {
            let doc = extract_doc_from_attrs(&item_struct.attrs);
            if !is_public(&item_struct.vis) {
                return None;
            }

            let name = item_struct.ident.to_string();
            let generics = if item_struct.generics.params.is_empty() {
                String::new()
            } else {
                format!(
                    "<{}>",
                    item_struct
                        .generics
                        .params
                        .iter()
                        .map(|p| quote::quote!(#p).to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            let signature = format!("pub struct {}{}", name, generics);

            Some(build_item_doc(name, "struct", signature, doc))
        }
        Item::Enum(item_enum) => {
            let doc = extract_doc_from_attrs(&item_enum.attrs);
            if !is_public(&item_enum.vis) {
                return None;
            }

            let name = item_enum.ident.to_string();
            let signature = format!("pub enum {}", name);

            Some(build_item_doc(name, "enum", signature, doc))
        }
        Item::Type(item_type) => {
            let doc = extract_doc_from_attrs(&item_type.attrs);
            if !is_public(&item_type.vis) {
                return None;
            }

            let name = item_type.ident.to_string();
            let signature = format!("pub type {} = ...", name);

            Some(build_item_doc(name, "type", signature, doc))
        }
        Item::Trait(item_trait) => {
            let doc = extract_doc_from_attrs(&item_trait.attrs);
            if !is_public(&item_trait.vis) {
                return None;
            }

            let name = item_trait.ident.to_string();
            let signature = format!("pub trait {}", name);

            Some(build_item_doc(name, "trait", signature, doc))
        }
        _ => None,
    }
}

/// Check if an item has public visibility
fn is_public(vis: &syn::Visibility) -> bool {
    matches!(vis, syn::Visibility::Public(_))
}

/// Format a function signature for display
fn format_fn_signature(sig: &syn::Signature, vis: &syn::Visibility) -> String {
    let vis_str = if is_public(vis) { "pub " } else { "" };
    let async_str = if sig.asyncness.is_some() {
        "async "
    } else {
        ""
    };
    let fn_name = &sig.ident;

    let generics = if sig.generics.params.is_empty() {
        String::new()
    } else {
        format!(
            "<{}>",
            sig.generics
                .params
                .iter()
                .map(|p| quote::quote!(#p).to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    let inputs: Vec<String> = sig
        .inputs
        .iter()
        .map(|arg| quote::quote!(#arg).to_string())
        .collect();

    let output = match &sig.output {
        syn::ReturnType::Default => String::new(),
        syn::ReturnType::Type(_, ty) => format!(" -> {}", quote::quote!(#ty)),
    };

    format!(
        "{}{}fn {}{}({}){}",
        vis_str,
        async_str,
        fn_name,
        generics,
        inputs.join(", "),
        output
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_module_docs_basic() {
        let source = r#"
//! This is a module
//! with multiple lines
//! of documentation.

pub fn main() {}
"#;
        let result = extract_module_docs(source);
        assert_eq!(
            result,
            "This is a module\nwith multiple lines\nof documentation."
        );
    }

    #[test]
    fn test_extract_module_docs_with_blank_lines() {
        let source = r#"
//! First paragraph
//!
//! Second paragraph
//! continues here

pub fn main() {}
"#;
        let result = extract_module_docs(source);
        assert_eq!(
            result,
            "First paragraph\n\nSecond paragraph\ncontinues here"
        );
    }

    #[test]
    fn test_extract_module_docs_no_space_after_marker() {
        let source = r#"
//!Module without space
//!Second line

pub fn main() {}
"#;
        let result = extract_module_docs(source);
        assert_eq!(result, "Module without space\nSecond line");
    }

    #[test]
    fn test_extract_module_docs_empty() {
        let source = r#"
pub fn main() {}
"#;
        let result = extract_module_docs(source);
        assert_eq!(result, "");
    }

    #[test]
    fn test_extract_module_docs_stops_at_code() {
        let source = r#"
//! Module docs
//! More docs

use std::io;

pub fn main() {}
"#;
        let result = extract_module_docs(source);
        assert_eq!(result, "Module docs\nMore docs");
    }

    #[test]
    fn test_extract_module_docs_with_regular_comments() {
        let source = r#"
//! Module docs
// Regular comment should not be included
//! More module docs

pub fn main() {}
"#;
        let result = extract_module_docs(source);
        // Regular comments break the sequence, so we only get the first part
        // But the function continues collecting if there's a blank line
        assert_eq!(result, "Module docs\nMore module docs");
    }

    #[test]
    fn test_extract_item_docs_public_function() {
        let source = r#"
/// Adds two numbers together
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "add");
        assert_eq!(items[0].kind, "function");
        assert_eq!(items[0].signature, "pub fn add(a : i32, b : i32) -> i32");
        assert_eq!(items[0].description, "Adds two numbers together");
    }

    #[test]
    fn test_extract_item_docs_private_function_skipped() {
        let source = r#"
/// This is private
fn private_fn() {}

/// This is public
pub fn public_fn() {}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "public_fn");
    }

    #[test]
    fn test_extract_item_docs_async_function() {
        let source = r#"
/// Asynchronously fetches data
pub async fn fetch_data() -> Result<String, Error> {
    Ok("data".to_string())
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "fetch_data");
        assert!(items[0].signature.contains("async"));
        assert!(items[0].signature.contains("pub async fn fetch_data"));
    }

    #[test]
    fn test_extract_item_docs_function_with_generics() {
        let source = r#"
/// Generic function
pub fn process<T: Clone, U>(data: T, other: U) -> T {
    data.clone()
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "process");
        assert!(items[0].signature.contains("<T : Clone, U>"));
        assert_eq!(items[0].description, "Generic function");
    }

    #[test]
    fn test_extract_item_docs_public_struct() {
        let source = r#"
/// A point in 2D space
pub struct Point {
    pub x: f64,
    pub y: f64,
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Point");
        assert_eq!(items[0].kind, "struct");
        assert_eq!(items[0].signature, "pub struct Point");
        assert_eq!(items[0].description, "A point in 2D space");
    }

    #[test]
    fn test_extract_item_docs_struct_with_generics() {
        let source = r#"
/// Generic container
pub struct Container<T, E> {
    value: T,
    error: E,
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Container");
        assert_eq!(items[0].signature, "pub struct Container<T, E>");
    }

    #[test]
    fn test_extract_item_docs_private_struct_skipped() {
        let source = r#"
/// Private struct
struct Private {
    data: i32,
}

/// Public struct
pub struct Public {
    data: i32,
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Public");
    }

    #[test]
    fn test_extract_item_docs_public_enum() {
        let source = r#"
/// Status codes
pub enum Status {
    Ok,
    Error,
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Status");
        assert_eq!(items[0].kind, "enum");
        assert_eq!(items[0].signature, "pub enum Status");
        assert_eq!(items[0].description, "Status codes");
    }

    #[test]
    fn test_extract_item_docs_private_enum_skipped() {
        let source = r#"
/// Private enum
enum Private {
    A, B,
}

/// Public enum
pub enum Public {
    C, D,
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Public");
    }

    #[test]
    fn test_extract_item_docs_public_trait() {
        let source = r#"
/// A trait for converting values
pub trait Convert {
    fn convert(&self) -> String;
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Convert");
        assert_eq!(items[0].kind, "trait");
        assert_eq!(items[0].signature, "pub trait Convert");
        assert_eq!(items[0].description, "A trait for converting values");
    }

    #[test]
    fn test_extract_item_docs_private_trait_skipped() {
        let source = r#"
/// Private trait
trait Private {
    fn do_something(&self);
}

/// Public trait
pub trait Public {
    fn do_something(&self);
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Public");
    }

    #[test]
    fn test_extract_item_docs_type_alias() {
        let source = r#"
/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Result");
        assert_eq!(items[0].kind, "type");
        assert_eq!(items[0].signature, "pub type Result = ...");
        assert_eq!(items[0].description, "Result type alias");
    }

    #[test]
    fn test_extract_item_docs_multiple_items() {
        let source = r#"
/// First function
pub fn first() {}

/// A struct
pub struct MyStruct;

/// An enum
pub enum MyEnum {
    A, B,
}

/// A trait
pub trait MyTrait {}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 4);
        assert_eq!(items[0].name, "first");
        assert_eq!(items[1].name, "MyStruct");
        assert_eq!(items[2].name, "MyEnum");
        assert_eq!(items[3].name, "MyTrait");
    }

    #[test]
    fn test_extract_item_docs_no_docs() {
        let source = r#"
pub fn undocumented() {}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].description, "");
    }

    #[test]
    fn test_extract_item_docs_multiline_docs() {
        let source = r#"
/// First line of documentation
/// Second line of documentation
/// Third line of documentation
pub fn documented() {}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].description,
            "First line of documentation\nSecond line of documentation\nThird line of documentation"
        );
    }

    #[test]
    fn test_extract_item_docs_function_no_return_type() {
        let source = r#"
/// Does something
pub fn do_something() {
    println!("something");
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].signature, "pub fn do_something()");
        assert!(!items[0].signature.contains("->"));
    }

    #[test]
    fn test_extract_item_docs_function_with_self() {
        let source = r#"
impl MyStruct {
    /// Method with self
    pub fn method(&self) -> i32 {
        42
    }
}
"#;
        let items = extract_item_docs(source);
        // Methods inside impl blocks are not extracted as top-level items
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn test_extract_item_docs_invalid_syntax() {
        let source = "this is not valid rust code {{{";
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn test_extract_item_docs_complex_generics() {
        let source = r#"
/// Complex generic function
pub fn complex<'a, T: Clone + Debug, U: Into<String>>(
    data: &'a T,
    converter: U,
) -> Result<String, Error> {
    Ok(converter.into())
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "complex");
        assert!(items[0].signature.contains("<"));
        assert!(items[0].signature.contains("-> Result < String , Error >"));
    }

    #[test]
    fn test_extract_item_docs_doc_attribute_format() {
        let source = r#"
#[doc = "Documentation via attribute"]
pub fn with_attribute() {}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].description, "Documentation via attribute");
    }

    #[test]
    fn test_extract_doc_from_attrs_mixed_formats() {
        let source = r#"
/// Line 1
#[doc = "Line 2"]
/// Line 3
pub fn mixed() {}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].description, "Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_extract_item_docs_function_with_where_clause() {
        let source = r#"
/// Function with where clause
pub fn with_where<T>(data: T) -> T
where
    T: Clone + Send,
{
    data.clone()
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "with_where");
        // Where clause is part of generics in signature
        assert!(items[0].signature.contains("with_where"));
    }

    #[test]
    fn test_is_public() {
        let public_source = "pub fn test() {}";
        let private_source = "fn test() {}";

        let pub_file = syn::parse_file(public_source).unwrap();
        let priv_file = syn::parse_file(private_source).unwrap();

        if let Item::Fn(pub_fn) = &pub_file.items[0] {
            assert!(is_public(&pub_fn.vis));
        }

        if let Item::Fn(priv_fn) = &priv_file.items[0] {
            assert!(!is_public(&priv_fn.vis));
        }
    }

    #[test]
    fn test_extract_module_docs_with_trailing_whitespace() {
        let source = r#"
//! Module docs
//!
//!

pub fn main() {}
"#;
        let result = extract_module_docs(source);
        // syn::parse_file preserves the empty doc lines as attributes
        // Trailing empty doc comments become part of the result
        assert_eq!(result, "Module docs\n\n");
    }

    #[test]
    fn test_extract_item_docs_empty_source() {
        let source = "";
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn test_extract_module_docs_only_whitespace() {
        let source = "   \n  \n  \n";
        let result = extract_module_docs(source);
        assert_eq!(result, "");
    }

    #[test]
    fn test_signature_formatting_async_with_generics_and_return() {
        let source = r#"
/// Complex async function
pub async fn fetch<T: Deserialize>(url: &str) -> Result<T, Error> {
    unimplemented!()
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert!(items[0].signature.contains("pub async fn"));
        assert!(items[0].signature.contains("fetch"));
        assert!(items[0].signature.contains("<T : Deserialize>"));
        assert!(items[0].signature.contains("-> Result < T , Error >"));
    }

    #[test]
    fn test_extract_item_docs_struct_with_lifetime_generics() {
        let source = r#"
/// Struct with lifetime
pub struct Borrowed<'a, T> {
    data: &'a T,
}
"#;
        let items = extract_item_docs(source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Borrowed");
        assert!(items[0].signature.contains("<'a, T>"));
    }

    #[test]
    fn test_extract_module_docs_complex_realistic() {
        let source = r#"
//! # My Module
//!
//! This module provides utilities for processing data.
//!
//! ## Features
//!
//! - Fast processing
//! - Memory efficient
//! - Easy to use
//!
//! ## Example
//!
//! ```rust
//! use my_module::process;
//! let result = process("data");
//! ```

use std::io;

pub fn process(data: &str) -> String {
    data.to_string()
}
"#;
        let result = extract_module_docs(source);
        assert!(result.contains("# My Module"));
        assert!(result.contains("This module provides utilities"));
        assert!(result.contains("## Features"));
        assert!(result.contains("```rust"));
        assert!(!result.contains("use std::io"));
    }
}
