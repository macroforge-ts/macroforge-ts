//! The names of a type's generated functions, and its parameters.

use crate::ast::Ident;
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::TypeParamIR;
use crate::ts_syn::ts_ident;

use super::value_type::rendered;

use convert_case::{Case, Casing};

/// A type's name and parameters, which name its generated functions.
pub(crate) struct TypeNames {
    /// The type as written in messages, e.g. `Box`.
    pub type_name: String,
    /// The parameters as declared, e.g. `<T extends Shape>`, or empty.
    pub generic_decl: String,
    /// The parameters' names, e.g. `["T"]`.
    pub params: Vec<String>,
    /// The parameters applied as arguments, e.g. `<T>`, or empty.
    pub generic_args: String,
    /// The type applied to its parameters, e.g. `Box<T>`.
    pub full_type: String,
}

/// The key parameter of a `validateField`.
pub(crate) struct KeyParam {
    /// `K`, or the first of `K1`, `K2`, ... that no parameter of the type
    /// already names.
    pub name: Ident,
    /// `<T, K extends keyof Box<T> = keyof Box<T>>`: the type's parameters,
    /// then the key. The key takes a default so it may follow parameters that
    /// have one.
    pub decl: String,
}

impl TypeNames {
    pub fn new(type_name: &str, type_params: &[TypeParamIR]) -> Self {
        let generic_args = TypeParamIR::apply_all(type_params);
        Self {
            type_name: type_name.to_string(),
            full_type: format!("{type_name}{generic_args}"),
            generic_decl: TypeParamIR::declare_all(type_params),
            params: type_params.iter().map(|param| param.name.clone()).collect(),
            generic_args,
        }
    }

    /// `{camelName}{suffix}`, the name of one generated function.
    pub fn function(&self, suffix: &str) -> String {
        format!("{}{suffix}", self.type_name.to_case(Case::Camel))
    }

    /// `{camelName}{suffix}<T extends Shape>`, a generated function declaring
    /// the type's parameters.
    pub fn generic_function(&self, suffix: &str) -> Ident {
        ts_ident!(format!("{}{}", self.function(suffix), self.generic_decl))
    }

    /// `{name}<T extends Shape>`, a generated static method declaring the
    /// type's parameters: a static member cannot name the class's own.
    pub fn generic_method(&self, name: &str) -> Ident {
        ts_ident!(format!("{name}{}", self.generic_decl))
    }

    /// The key a `validateField` validates, declared after the type's own
    /// parameters.
    pub fn key_param(&self) -> KeyParam {
        let mut name = String::from("K");
        let mut suffix = 0;
        while self.params.contains(&name) {
            suffix += 1;
            name = format!("K{suffix}");
        }
        let full = &self.full_type;
        let key = name.as_str();
        let decl = match self.generic_decl.strip_suffix('>') {
            Some(params) => {
                rendered(ts_template! { @{params}, @{key} extends keyof @{full} = keyof @{full}> })
            }
            None => rendered(ts_template! { <@{key} extends keyof @{full}> }),
        };
        KeyParam {
            name: ts_ident!(name.as_str()),
            decl,
        }
    }

    /// `{camelName}{suffix}<T>`, a call to a generated function that passes
    /// the type's parameters on.
    pub fn generic_call(&self, suffix: &str) -> Ident {
        ts_ident!(format!("{}{}", self.function(suffix), self.generic_args))
    }

    /// The type applied to its parameters, as an identifier.
    pub fn full_type_ident(&self) -> Ident {
        ts_ident!(self.full_type.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::TypeNames;
    use crate::ts_syn::abi::ir::TypeParamIR;

    #[test]
    fn parameters_keep_their_declaration_and_apply_by_name() {
        let param = TypeParamIR {
            name: "T".to_string(),
            constraint: Some("Shape".to_string()),
            default: Some("Square".to_string()),
            is_const: true,
        };
        let names = TypeNames::new("Box", &[param]);
        assert_eq!(names.generic_decl, "<const T extends Shape = Square>");
        assert_eq!(names.params, ["T"]);
        assert_eq!(names.generic_args, "<T>");
        assert_eq!(names.full_type, "Box<T>");
    }

    #[test]
    fn the_key_parameter_avoids_the_types_own_names() {
        let params = ["K", "K1"].map(|name| TypeParamIR {
            name: name.to_string(),
            constraint: None,
            default: None,
            is_const: false,
        });
        let key = TypeNames::new("Keyed", &params).key_param();
        assert_eq!(key.name.to_string(), "K2");
        assert_eq!(
            key.decl,
            "<K, K1, K2 extends keyof Keyed<K, K1> = keyof Keyed<K, K1>>"
        );
        let key = TypeNames::new("Point", &[]).key_param();
        assert_eq!(key.name.to_string(), "K");
        assert_eq!(key.decl, "<K extends keyof Point>");
    }

    #[test]
    fn a_type_without_parameters_has_no_brackets() {
        let names = TypeNames::new("Point", &[]);
        assert_eq!(names.full_type, "Point");
        assert!(names.generic_decl.is_empty() && names.generic_args.is_empty());
    }
}
