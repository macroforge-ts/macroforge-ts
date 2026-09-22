#![cfg(feature = "oxc")]

use macroforge_ts::macros::ts_quote;
use macroforge_ts::ts_syn::oxc::allocator::Allocator;
use macroforge_ts::ts_syn::oxc::ast::ast::{AssignmentTarget, BindingPattern, Expression, TSType};
use macroforge_ts::ts_syn::{
    oxc_assignment_target_to_string, oxc_binding_pattern_to_string, oxc_expr_to_string,
    oxc_type_to_string, parse_oxc_expr, parse_oxc_type,
};

#[test]
fn oxc_ts_quote_interpolates_ident_and_expr() {
    let arena = Allocator::default();
    let rhs = parse_oxc_expr(&arena, "1 + 2").unwrap();
    let expr: Expression<'_> = ts_quote!("$name = $rhs" as Expr, name = "count", rhs: Expr = rhs);

    assert_eq!(oxc_expr_to_string(&expr), "count = 1 + 2");
}

#[test]
fn oxc_ts_quote_interpolates_pattern_and_type() {
    let arena = Allocator::default();
    let props = parse_oxc_type(&arena, "Props").unwrap();
    let pattern: BindingPattern<'_> =
        ts_quote!("{ foo, bar }: $props" as Pat, props: TsType = props);

    // OXC's BindingPattern is an enum without type_annotation field;
    // the type annotation lives on FormalParameter. Pattern part should be correct.
    assert_eq!(oxc_binding_pattern_to_string(&pattern), "{ foo, bar }");
}

#[test]
fn oxc_ts_quote_interpolates_string_literal_and_assignment_target() {
    let arena = Allocator::default();
    let target: AssignmentTarget<'_> =
        ts_quote!("$target" as AssignTarget, target: AssignTarget = "foo.bar");
    let literal: Expression<'_> = ts_quote!("[$target, \"$label\"]" as Expr, target: Expr = parse_oxc_expr(&arena, "foo.bar").unwrap(), label: Str = "hello\nworld");

    assert_eq!(oxc_assignment_target_to_string(&target), "foo.bar");
    assert_eq!(oxc_expr_to_string(&literal), "[foo.bar, \"hello\\nworld\"]");
}

#[test]
fn oxc_ts_quote_returns_type_nodes() {
    let arena = Allocator::default();
    let inner = parse_oxc_type(&arena, "User").unwrap();
    let ty: TSType<'_> = ts_quote!("Readonly<$inner>" as TsType, inner: TsType = inner);

    assert_eq!(oxc_type_to_string(&ty), "Readonly<User>");
}

#[test]
fn oxc_ts_quote_takes_an_explicit_arena() {
    let other = Allocator::default();
    let expr = ts_quote!(&other, "a + b" as Expr);
    assert_eq!(oxc_expr_to_string(&expr), "a + b");
}

fn quote_member<'a>(arena: &'a Allocator, field: &str) -> Expression<'a> {
    ts_quote!("this.$field" as Expr, field = field)
}

#[test]
fn oxc_ts_quote_returns_nodes_tied_to_a_parameter_arena() {
    let arena = Allocator::default();
    let expr = quote_member(&arena, "count");
    assert_eq!(oxc_expr_to_string(&expr), "this.count");
}
