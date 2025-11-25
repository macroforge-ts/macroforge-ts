//! `syn`-like derive input types for TypeScript macros.
//!
//! This module provides a `DeriveInput` type analogous to `syn::DeriveInput`,
//! making it easy to write derive macros with a familiar API.
//!
//! # Example
//! ```ignore
//! use ts_syn::{parse_ts_macro_input, DeriveInput, Data};
//!
//! #[ts_macro_derive(MyMacro)]
//! pub fn my_macro(input: TsStream) -> MacroResult {
//!     let input = parse_ts_macro_input!(input as DeriveInput);
//!
//!     match &input.data {
//!         Data::Class(class) => {
//!             // Handle class
//!         }
//!         Data::Enum(enum_) => {
//!             // Handle enum
//!         }
//!     }
//! }
//! ```

use ts_macro_abi::{
    ClassIR, DecoratorIR, EnumIR, EnumVariantIR, FieldIR, MacroContextIR, MethodSigIR, SpanIR,
    TargetIR,
};

use crate::TsSynError;

#[cfg(feature = "swc")]
use crate::TsStream;

/// The input to a derive macro, analogous to `syn::DeriveInput`.
///
/// This provides a unified representation of the different types that can
/// have derive macros applied to them in TypeScript.
#[derive(Debug, Clone)]
pub struct DeriveInput {
    /// The name of the type (class or enum name)
    pub ident: Ident,

    /// The span of the entire type definition
    pub span: SpanIR,

    /// Decorators on this type (excluding the derive decorator itself)
    pub attrs: Vec<Attribute>,

    /// The data within the type (class fields/methods or enum variants)
    pub data: Data,

    /// The macro context, providing spans and other metadata
    pub context: MacroContextIR,
}

/// A simple identifier, analogous to `syn::Ident`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
    name: String,
    span: SpanIR,
}

impl Ident {
    /// Create a new identifier
    pub fn new(name: impl Into<String>, span: SpanIR) -> Self {
        Self {
            name: name.into(),
            span,
        }
    }

    /// Get the identifier as a string slice
    pub fn as_str(&self) -> &str {
        &self.name
    }

    /// Get the span of this identifier
    pub fn span(&self) -> SpanIR {
        self.span
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

/// An attribute (decorator), analogous to `syn::Attribute`
#[derive(Debug, Clone)]
pub struct Attribute {
    /// The decorator IR
    pub inner: DecoratorIR,
}

impl Attribute {
    /// Get the attribute/decorator name
    pub fn name(&self) -> &str {
        &self.inner.name
    }

    /// Get the raw arguments string
    pub fn args(&self) -> &str {
        &self.inner.args_src
    }

    /// Get the span
    pub fn span(&self) -> SpanIR {
        self.inner.span
    }
}

/// The data within a derive input, analogous to `syn::Data`
#[derive(Debug, Clone)]
pub enum Data {
    /// A TypeScript class
    Class(DataClass),
    /// A TypeScript enum
    Enum(DataEnum),
}

/// Data for a class, analogous to `syn::DataStruct`
#[derive(Debug, Clone)]
pub struct DataClass {
    /// The class IR with full details
    pub inner: ClassIR,
}

impl DataClass {
    /// Get the fields of the class
    pub fn fields(&self) -> &[FieldIR] {
        &self.inner.fields
    }

    /// Get the methods of the class
    pub fn methods(&self) -> &[MethodSigIR] {
        &self.inner.methods
    }

    /// Get the class body span (for inserting code)
    pub fn body_span(&self) -> SpanIR {
        self.inner.body_span
    }

    /// Check if the class is abstract
    pub fn is_abstract(&self) -> bool {
        self.inner.is_abstract
    }

    /// Get type parameters
    pub fn type_params(&self) -> &[String] {
        &self.inner.type_params
    }

    /// Get heritage clauses (extends/implements)
    pub fn heritage(&self) -> &[String] {
        &self.inner.heritage
    }

    /// Iterate over field names
    pub fn field_names(&self) -> impl Iterator<Item = &str> {
        self.inner.fields.iter().map(|f| f.name.as_str())
    }

    /// Get a field by name
    pub fn field(&self, name: &str) -> Option<&FieldIR> {
        self.inner.fields.iter().find(|f| f.name == name)
    }

    /// Get a method by name
    pub fn method(&self, name: &str) -> Option<&MethodSigIR> {
        self.inner.methods.iter().find(|m| m.name == name)
    }
}

/// Data for an enum, analogous to `syn::DataEnum`
#[derive(Debug, Clone)]
pub struct DataEnum {
    /// The enum IR with full details
    pub inner: EnumIR,
}

impl DataEnum {
    /// Get the variants of the enum
    pub fn variants(&self) -> &[EnumVariantIR] {
        &self.inner.variants
    }

    /// Iterate over variant names
    pub fn variant_names(&self) -> impl Iterator<Item = &str> {
        self.inner.variants.iter().map(|v| v.name.as_str())
    }

    /// Get a variant by name
    pub fn variant(&self, name: &str) -> Option<&EnumVariantIR> {
        self.inner.variants.iter().find(|v| v.name == name)
    }
}

impl DeriveInput {
    /// Create a DeriveInput from a MacroContextIR
    pub fn from_context(ctx: MacroContextIR) -> Result<Self, TsSynError> {
        let (ident, span, attrs, data) = match &ctx.target {
            TargetIR::Class(class) => {
                let ident = Ident::new(&class.name, class.span);
                let attrs = class
                    .decorators
                    .iter()
                    .filter(|d| d.name != "Derive") // Filter out the derive decorator itself
                    .cloned()
                    .map(|d| Attribute { inner: d })
                    .collect();
                let data = Data::Class(DataClass {
                    inner: class.clone(),
                });
                (ident, class.span, attrs, data)
            }
            TargetIR::Enum(enum_) => {
                let ident = Ident::new(&enum_.name, enum_.span);
                let attrs = enum_
                    .decorators
                    .iter()
                    .filter(|d| d.name != "Derive")
                    .cloned()
                    .map(|d| Attribute { inner: d })
                    .collect();
                let data = Data::Enum(DataEnum {
                    inner: enum_.clone(),
                });
                (ident, enum_.span, attrs, data)
            }
            TargetIR::Interface => {
                return Err(TsSynError::Unsupported(
                    "Interface derive macros not yet supported".into(),
                ))
            }
            TargetIR::Function => {
                return Err(TsSynError::Unsupported(
                    "Function derive macros not yet supported".into(),
                ))
            }
            TargetIR::Other => {
                return Err(TsSynError::Unsupported(
                    "Unknown target type for derive macro".into(),
                ))
            }
        };

        Ok(Self {
            ident,
            span,
            attrs,
            data,
            context: ctx,
        })
    }

    /// Get the name of the type as a string
    pub fn name(&self) -> &str {
        self.ident.as_str()
    }

    /// Get the class data, if this is a class
    pub fn as_class(&self) -> Option<&DataClass> {
        match &self.data {
            Data::Class(c) => Some(c),
            _ => None,
        }
    }

    /// Get the enum data, if this is an enum
    pub fn as_enum(&self) -> Option<&DataEnum> {
        match &self.data {
            Data::Enum(e) => Some(e),
            _ => None,
        }
    }

    /// Get the decorator span (for deletion/replacement)
    pub fn decorator_span(&self) -> SpanIR {
        self.context.decorator_span
    }

    /// Get the target span (for inserting after)
    pub fn target_span(&self) -> SpanIR {
        self.context.target_span
    }

    /// Get the class body span for inserting type signatures
    /// Returns None if this is not a class
    pub fn body_span(&self) -> Option<SpanIR> {
        match &self.data {
            Data::Class(c) => Some(c.body_span()),
            _ => None,
        }
    }
}

#[cfg(feature = "swc")]
impl crate::ParseTs for DeriveInput {
    fn parse(input: &mut TsStream) -> Result<Self, TsSynError> {
        let ctx = input
            .context()
            .ok_or_else(|| TsSynError::Parse("No macro context available".into()))?
            .clone();

        Self::from_context(ctx)
    }
}

/// Parse a `TsStream` into a `DeriveInput`, returning early with an error `MacroResult` on failure.
///
/// This macro is analogous to `syn::parse_macro_input!` and provides ergonomic error handling
/// for derive macros.
///
/// # Example
/// ```ignore
/// use ts_syn::{parse_ts_macro_input, DeriveInput};
/// use ts_macro_abi::MacroResult;
///
/// #[ts_macro_derive(MyMacro)]
/// pub fn my_macro(mut input: TsStream) -> MacroResult {
///     let input = parse_ts_macro_input!(input as DeriveInput);
///
///     // input is now a DeriveInput
///     let name = input.name();
///     // ...
/// }
/// ```
///
/// # Variants
/// - `parse_ts_macro_input!(stream as DeriveInput)` - Parse as DeriveInput
/// - `parse_ts_macro_input!(stream)` - Same as above (DeriveInput is the default)
#[macro_export]
macro_rules! parse_ts_macro_input {
    ($input:ident as $ty:ty) => {
        match <$ty as $crate::ParseTs>::parse(&mut $input) {
            Ok(parsed) => parsed,
            Err(e) => {
                return ::ts_macro_abi::MacroResult {
                    diagnostics: vec![::ts_macro_abi::Diagnostic {
                        level: ::ts_macro_abi::DiagnosticLevel::Error,
                        message: format!("Failed to parse input: {}", e),
                        span: None,
                        notes: vec![],
                        help: None,
                    }],
                    ..Default::default()
                };
            }
        }
    };
    ($input:ident) => {
        $crate::parse_ts_macro_input!($input as $crate::DeriveInput)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts_macro_abi::{MacroKind, SpanIR};

    fn make_test_class_context() -> MacroContextIR {
        MacroContextIR {
            abi_version: 1,
            macro_kind: MacroKind::Derive,
            macro_name: "Debug".into(),
            module_path: "@test/macro".into(),
            decorator_span: SpanIR::new(0, 10),
            target_span: SpanIR::new(11, 100),
            file_name: "test.ts".into(),
            target: TargetIR::Class(ClassIR {
                name: "User".into(),
                span: SpanIR::new(11, 100),
                body_span: SpanIR::new(20, 99),
                is_abstract: false,
                type_params: vec![],
                heritage: vec![],
                decorators: vec![],
                fields: vec![
                    FieldIR {
                        name: "id".into(),
                        span: SpanIR::new(25, 35),
                        ts_type: "number".into(),
                        optional: false,
                        readonly: false,
                        visibility: ts_macro_abi::Visibility::Public,
                        decorators: vec![],
                    },
                    FieldIR {
                        name: "name".into(),
                        span: SpanIR::new(40, 55),
                        ts_type: "string".into(),
                        optional: false,
                        readonly: false,
                        visibility: ts_macro_abi::Visibility::Public,
                        decorators: vec![],
                    },
                ],
                methods: vec![],
            }),
            target_source: "class User { id: number; name: string; }".into(),
        }
    }

    #[test]
    fn test_derive_input_from_class_context() {
        let ctx = make_test_class_context();
        let input = DeriveInput::from_context(ctx).expect("should parse");

        assert_eq!(input.name(), "User");
        assert!(input.as_class().is_some());
        assert!(input.as_enum().is_none());

        let class = input.as_class().unwrap();
        assert_eq!(class.fields().len(), 2);
        assert!(class.field("id").is_some());
        assert!(class.field("name").is_some());
        assert!(class.field("nonexistent").is_none());

        let field_names: Vec<_> = class.field_names().collect();
        assert_eq!(field_names, vec!["id", "name"]);
    }

    #[test]
    fn test_derive_input_from_enum_context() {
        let ctx = MacroContextIR {
            abi_version: 1,
            macro_kind: MacroKind::Derive,
            macro_name: "Debug".into(),
            module_path: "@test/macro".into(),
            decorator_span: SpanIR::new(0, 10),
            target_span: SpanIR::new(11, 100),
            file_name: "test.ts".into(),
            target: TargetIR::Enum(EnumIR {
                name: "Status".into(),
                span: SpanIR::new(11, 100),
                decorators: vec![],
                variants: vec![
                    EnumVariantIR {
                        name: "Active".into(),
                        span: SpanIR::new(20, 30),
                    },
                    EnumVariantIR {
                        name: "Inactive".into(),
                        span: SpanIR::new(35, 45),
                    },
                ],
            }),
            target_source: "enum Status { Active, Inactive }".into(),
        };

        let input = DeriveInput::from_context(ctx).expect("should parse");

        assert_eq!(input.name(), "Status");
        assert!(input.as_enum().is_some());
        assert!(input.as_class().is_none());

        let enum_ = input.as_enum().unwrap();
        assert_eq!(enum_.variants().len(), 2);
        assert!(enum_.variant("Active").is_some());
        assert!(enum_.variant("Inactive").is_some());

        let variant_names: Vec<_> = enum_.variant_names().collect();
        assert_eq!(variant_names, vec!["Active", "Inactive"]);
    }

    #[test]
    fn test_ident_display() {
        let ident = Ident::new("MyClass", SpanIR::new(0, 7));
        assert_eq!(format!("{}", ident), "MyClass");
        assert_eq!(ident.as_str(), "MyClass");
        assert_eq!(ident.as_ref(), "MyClass");
    }
}
