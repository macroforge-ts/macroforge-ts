#[cfg(test)]
mod tests {
    #[cfg(feature = "swc")]
    #[test]
    fn test_quote_stmt() {
        use crate::quote;
        use crate::swc_core::common::DUMMY_SP;
        use crate::swc_core::ecma::ast::{Ident, Stmt};

        let name = Ident::new_no_ctxt("myVar".into(), DUMMY_SP);
        let _stmt: Stmt = quote!("const $name = 42;" as Stmt, name = name);
    }

    #[cfg(feature = "swc")]
    #[test]
    fn test_ident_macro() {
        use crate::ident;
        let id = ident!("testIdent");
        assert_eq!(id.sym.as_ref(), "testIdent");
    }

    #[cfg(feature = "swc")]
    #[test]
    fn test_obj_lit_empty() {
        use crate::obj_lit;
        use crate::swc_core::ecma::ast::Expr;

        let obj: Expr = obj_lit!();
        match obj {
            Expr::Object(obj_lit) => {
                assert_eq!(obj_lit.props.len(), 0);
            }
            _ => panic!("Expected object literal"),
        }
    }

    #[cfg(feature = "swc")]
    #[test]
    fn test_obj_lit_static() {
        use crate::swc_core::ecma::ast::Expr;
        use crate::{ident, obj_lit};

        let name_expr = Expr::Ident(ident!("name"));
        let age_expr = Expr::Lit(crate::swc_core::ecma::ast::Lit::Num(
            crate::swc_core::ecma::ast::Number {
                span: crate::swc_core::common::DUMMY_SP,
                value: 42.0,
                raw: None,
            },
        ));

        let obj: Expr = obj_lit!(
            "name" => name_expr,
            "age" => age_expr
        );

        match obj {
            Expr::Object(obj_lit) => {
                assert_eq!(obj_lit.props.len(), 2);
            }
            _ => panic!("Expected object literal"),
        }
    }

    #[cfg(feature = "swc")]
    #[test]
    fn test_array_lit() {
        use crate::swc_core::ecma::ast::Expr;
        use crate::{array_lit, ident};

        let elem1 = Expr::Ident(ident!("foo"));
        let elem2 = Expr::Ident(ident!("bar"));

        let arr: Expr = array_lit![elem1, elem2];

        match arr {
            Expr::Array(arr_lit) => {
                assert_eq!(arr_lit.elems.len(), 2);
            }
            _ => panic!("Expected array literal"),
        }
    }

    #[cfg(feature = "swc")]
    #[test]
    fn test_ts_quote_implicit_binding() {
        use crate::swc_core::ecma::ast::Stmt;
        use crate::{ident, ts_quote};

        let class_ident = ident!("MyClass");
        let method_ident = ident!("myMethod");

        // Test implicit bindings
        let _stmt: Stmt = ts_quote!(
            "$class.prototype.$method = function() {};" as Stmt,
            class_ident,
            method_ident
        );
    }

    #[cfg(feature = "swc")]
    #[test]
    fn test_ts_quote_explicit_binding() {
        use crate::swc_core::ecma::ast::Stmt;
        use crate::{ident, ts_quote};

        let my_class = ident!("User");
        let my_method = ident!("toString");

        // Test explicit bindings
        let _stmt: Stmt = ts_quote!(
            "$class.prototype.$method = function() {};" as Stmt,
            class = my_class,
            method = my_method
        );
    }

    #[cfg(feature = "swc")]
    #[test]
    fn test_ts_quote_mixed_bindings() {
        use crate::swc_core::common::DUMMY_SP;
        use crate::swc_core::ecma::ast::{Expr, Lit, Number, Stmt};
        use crate::{ident, ts_quote};

        let class_ident = ident!("Counter");
        let value_expr = Expr::Lit(Lit::Num(Number {
            span: DUMMY_SP,
            value: 42.0,
            raw: None,
        }));

        // Mix implicit and explicit typed bindings
        let _stmt: Stmt = ts_quote!(
            "$class.prototype.value = $val;" as Stmt,
            class_ident,           // implicit
            val: Expr = value_expr // explicit typed
        );
    }

    #[cfg(feature = "swc")]
    #[test]
    fn test_ts_quote_typed_implicit() {
        use crate::swc_core::ecma::ast::Stmt;
        use crate::{ident, ts_quote};

        let name = ident!("myVar");

        // Test typed implicit binding
        let _stmt: Stmt = ts_quote!(
            "const $name = 42;" as Stmt,
            name: Ident
        );
    }

    #[cfg(feature = "swc")]
    #[test]
    fn test_ts_quote_no_trailing_comma() {
        use crate::swc_core::ecma::ast::Stmt;
        use crate::{ident, ts_quote};

        let class_ident = ident!("Test");

        // Test without trailing comma
        let _stmt: Stmt = ts_quote!("class $class {}" as Stmt, class_ident);
    }
}
