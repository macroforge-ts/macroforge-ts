//! A type parameter of a generic declaration, such as `T extends Shape = Box`.

use serde::{Deserialize, Serialize};

/// One type parameter as declared: `const T extends Shape = Box` is the name
/// `T`, the constraint `Shape`, the default `Box`, and `is_const`.
///
/// `in` and `out` variance annotations are not kept: they only describe the
/// declaring type, and TypeScript rejects them on a function's parameters,
/// which is where generated code redeclares them.
///
/// # Example
///
/// ```rust
/// use macroforge_ts_syn::abi::ir::TypeParamIR;
///
/// let param = TypeParamIR {
///     name: "T".to_string(),
///     constraint: Some("object".to_string()),
///     default: None,
///     is_const: false,
/// };
/// assert_eq!(param.declaration(), "T extends object");
/// assert_eq!(TypeParamIR::declare_all(&[param]), "<T extends object>");
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct TypeParamIR {
    /// The parameter's name, as type arguments refer to it.
    pub name: String,
    /// The type after `extends`, as written.
    pub constraint: Option<String>,
    /// The type after `=`, as written.
    pub default: Option<String>,
    /// Whether the parameter is declared `const`.
    pub is_const: bool,
}

impl TypeParamIR {
    /// A parameter with only a name, such as `T`.
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// The parameter as a generic function declares it:
    /// `const T extends Shape = Box`.
    pub fn declaration(&self) -> String {
        let mut declaration = String::new();
        if self.is_const {
            declaration.push_str("const ");
        }
        declaration.push_str(&self.name);
        if let Some(constraint) = &self.constraint {
            declaration.push_str(" extends ");
            declaration.push_str(constraint);
        }
        if let Some(default) = &self.default {
            declaration.push_str(" = ");
            declaration.push_str(default);
        }
        declaration
    }

    /// `<A, B extends C>` declaring every parameter, or empty for none.
    pub fn declare_all(params: &[Self]) -> String {
        if params.is_empty() {
            return String::new();
        }
        let declarations: Vec<String> = params.iter().map(Self::declaration).collect();
        format!("<{}>", declarations.join(", "))
    }

    /// `<A, B>` passing every parameter on as a type argument, or empty for
    /// none.
    pub fn apply_all(params: &[Self]) -> String {
        if params.is_empty() {
            return String::new();
        }
        let names: Vec<&str> = params.iter().map(|param| param.name.as_str()).collect();
        format!("<{}>", names.join(", "))
    }
}
