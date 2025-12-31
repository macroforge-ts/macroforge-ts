use proc_macro2::Span;
use syn::{Expr, ExprStruct, FieldValue, Ident, Member, Token, punctuated::Punctuated};

/// A builder for constructing `syn::ExprStruct` expressions.
/// Used to generate Rust code that constructs SWC AST nodes.
pub(crate) struct Builder {
    type_name: syn::Ident,
    fields: Punctuated<FieldValue, Token![,]>,
}

impl Builder {
    pub fn new(ident: &str) -> Self {
        Self {
            type_name: Ident::new(ident, Span::call_site()),
            fields: Default::default(),
        }
    }

    pub fn add(&mut self, name: &str, value: Expr) {
        self.fields.push(FieldValue {
            attrs: Default::default(),
            member: Member::Named(Ident::new(name, Span::call_site())),
            colon_token: Some(Default::default()),
            expr: value,
        });
    }

    pub fn build(self) -> ExprStruct {
        let type_name = self.type_name;

        ExprStruct {
            attrs: Default::default(),
            brace_token: Default::default(),
            path: syn::parse_quote!(macroforge_ts::swc_core::ecma::ast::#type_name),
            fields: self.fields,
            dot2_token: Default::default(),
            rest: Default::default(),
            qself: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    /// Helper to create a simple expression for testing
    fn make_expr(s: &str) -> Expr {
        syn::parse_str(s).unwrap()
    }

    #[test]
    fn test_builder_new_creates_correct_ident() {
        let builder = Builder::new("Ident");
        assert_eq!(builder.type_name.to_string(), "Ident");
        assert!(builder.fields.is_empty());
    }

    #[test]
    fn test_builder_new_with_various_type_names() {
        let test_cases = ["Expr", "Stmt", "Pat", "Module", "TsType", "CallExpr"];
        for name in test_cases {
            let builder = Builder::new(name);
            assert_eq!(builder.type_name.to_string(), name);
        }
    }

    #[test]
    fn test_builder_add_single_field() {
        let mut builder = Builder::new("TestStruct");
        builder.add("field1", make_expr("42"));

        assert_eq!(builder.fields.len(), 1);
        let field = builder.fields.first().unwrap();
        if let Member::Named(ident) = &field.member {
            assert_eq!(ident.to_string(), "field1");
        } else {
            panic!("Expected named member");
        }
    }

    #[test]
    fn test_builder_add_multiple_fields() {
        let mut builder = Builder::new("TestStruct");
        builder.add("field1", make_expr("1"));
        builder.add("field2", make_expr("\"hello\""));
        builder.add("field3", make_expr("true"));

        assert_eq!(builder.fields.len(), 3);

        let field_names: Vec<String> = builder
            .fields
            .iter()
            .filter_map(|f| {
                if let Member::Named(ident) = &f.member {
                    Some(ident.to_string())
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(field_names, vec!["field1", "field2", "field3"]);
    }

    #[test]
    fn test_builder_add_complex_expressions() {
        let mut builder = Builder::new("ComplexStruct");
        builder.add("call", make_expr("foo()"));
        builder.add("binary", make_expr("a + b"));
        builder.add("member", make_expr("obj.field"));
        builder.add("closure", make_expr("|x| x + 1"));

        assert_eq!(builder.fields.len(), 4);
    }

    #[test]
    fn test_builder_build_creates_valid_expr_struct() {
        let mut builder = Builder::new("Ident");
        builder.add("sym", make_expr("sym_value"));
        builder.add("span", make_expr("DUMMY_SP"));

        let result = builder.build();

        // Verify the path contains the expected structure
        let path_str = result.path.to_token_stream().to_string();
        assert!(path_str.contains("macroforge_ts"));
        assert!(path_str.contains("swc_core"));
        assert!(path_str.contains("ecma"));
        assert!(path_str.contains("ast"));
        assert!(path_str.contains("Ident"));
    }

    #[test]
    fn test_builder_build_empty_fields() {
        let builder = Builder::new("EmptyStruct");
        let result = builder.build();

        assert!(result.fields.is_empty());
        assert!(result.attrs.is_empty());
        assert!(result.rest.is_none());
        assert!(result.qself.is_none());
    }

    #[test]
    fn test_builder_field_has_colon_token() {
        let mut builder = Builder::new("TestStruct");
        builder.add("field", make_expr("value"));

        let field = builder.fields.first().unwrap();
        assert!(field.colon_token.is_some());
    }

    #[test]
    fn test_builder_field_has_empty_attrs() {
        let mut builder = Builder::new("TestStruct");
        builder.add("field", make_expr("value"));

        let field = builder.fields.first().unwrap();
        assert!(field.attrs.is_empty());
    }

    #[test]
    fn test_builder_preserves_expression_value() {
        let mut builder = Builder::new("TestStruct");
        let expr = make_expr("complex_value.method().chain()");
        builder.add("field", expr.clone());

        let field = builder.fields.first().unwrap();
        let field_expr_str = field.expr.to_token_stream().to_string();
        let expected_str = expr.to_token_stream().to_string();
        assert_eq!(field_expr_str, expected_str);
    }

    #[test]
    fn test_builder_with_special_characters_in_name() {
        // Test with snake_case names
        let mut builder = Builder::new("TsTypeRef");
        builder.add("type_name", make_expr("name"));
        builder.add("type_args", make_expr("args"));

        assert_eq!(builder.fields.len(), 2);
    }

    #[test]
    fn test_builder_order_preserved() {
        let mut builder = Builder::new("OrderedStruct");
        builder.add("first", make_expr("1"));
        builder.add("second", make_expr("2"));
        builder.add("third", make_expr("3"));

        let field_names: Vec<String> = builder
            .fields
            .iter()
            .filter_map(|f| {
                if let Member::Named(ident) = &f.member {
                    Some(ident.to_string())
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(field_names, vec!["first", "second", "third"]);
    }
}
