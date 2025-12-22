use macroforge_ts_quote::{above, below, body, signature, ts_quote, ts_template};
use proc_macro2::{Ident as PmIdent, Span as PmSpan, Spacing, TokenStream as TokenStream2, TokenTree};
use swc_core::common::{DUMMY_SP, SyntaxContext};
use swc_core::ecma::ast::{Decl, Expr, Ident as TsIdent, Stmt};

trait PascalCase {
    fn to_pascal_case(&self) -> String;
}

impl PascalCase for &str {
    fn to_pascal_case(&self) -> String {
        let mut chars = self.chars();
        let Some(first) = chars.next() else {
            return String::new();
        };
        let mut out = String::new();
        out.push(first.to_ascii_uppercase());
        out.push_str(chars.as_str());
        out
    }
}

#[test]
fn doc_ts_quote_simple_class() {
    let name = ident("MyClass");
    let stmt: Stmt = ts_quote!(class $(name) {} as Stmt);
    assert!(matches!(stmt, Stmt::Decl(Decl::Class(_))));
}

#[test]
fn doc_ts_template_for_fields() {
    let fields = vec!["name", "age"];
    let stream = ts_template! {
        {#for field in &fields}
            this.@{ident(field)} = @{expr_ident(field)};
        {/for}
    };
    let source = stream.source();
    assert!(source.contains("this.name = name"));
    assert!(source.contains("this.age = age"));
}

#[test]
fn doc_ts_quote_formatted_ident() {
    let field = "userName";
    let expr: Expr = ts_quote!(
        this.$(ident!("get{}", field.to_pascal_case()))()
    as Expr);
    assert!(matches!(expr, Expr::Call(_)));
}

#[test]
fn doc_ts_quote_object_literal() {
    let key = "status";
    let value = expr_ident("active");
    let expr: Expr = ts_quote!({ $(ident!("{}", key)): $(value: Expr) } as Expr);
    let is_object = matches!(expr, Expr::Object(_))
        || matches!(expr, Expr::Paren(paren) if matches!(*paren.expr, Expr::Object(_)));
    assert!(is_object);
}

#[test]
fn doc_quote_ident() {
    let ts = quote_ident("swc_core");
    assert_eq!(ts.to_string(), "swc_core");
}

#[test]
fn doc_quote_punct() {
    let ts = quote_punct("::");
    assert_eq!(ts.to_string(), "::");
}

#[test]
fn doc_ts_template_tojson() {
    let class_name = "User";
    let stream = ts_template! {
        @{expr_ident(class_name)}.prototype.toJSON = function() {
            return { ok: true };
        };
    };
    let source = stream.source();
    assert!(source.contains("User.prototype.toJSON"));
    assert!(source.contains("return {"));
    assert!(source.contains("ok: true"));
}

#[test]
fn doc_ts_template_backtick() {
    let tag = "div";
    let stream = ts_template! {
        const html = "'^<@{tag}>${content}</@{tag}>^'";
    };
    let source = stream.source();
    assert!(source.contains(r#"`<${"div"}>${content}</${"div"}>`"#));
}

#[test]
fn doc_above_macro() {
    let stream = above! {
        const lodash = "lodash";
    };
    let source = stream.source();
    assert!(source.starts_with("/* @macroforge:above */"));
    assert!(source.contains("const lodash = \"lodash\""));
}

#[test]
fn doc_below_macro() {
    let class_name = "User";
    let stream = below! {
        @{expr_ident(class_name)}.prototype.toJSON = function() {
            return { ...this };
        };
    };
    let source = stream.source();
    assert!(source.starts_with("/* @macroforge:below */"));
    assert!(source.contains("User.prototype.toJSON"));
}

#[test]
fn doc_body_macro() {
    let fields = vec!["name", "age"];
    let stream = body! {
        {#for field in &fields}
            this.@{ident(field)} = @{expr_ident(field)};
        {/for}
    };
    let source = stream.source();
    assert!(source.starts_with("/* @macroforge:body */"));
    assert!(source.contains("this.name = name"));
    assert!(source.contains("this.age = age"));
}

#[test]
fn doc_signature_macro() {
    let param_name = "context";
    let param_type = "RequestContext";
    let stream = signature! {
        @{ident(param_name)}: @{expr_ident(param_type)}
    };
    let source = stream.source();
    assert!(source.starts_with("/* @macroforge:signature */"));
    assert!(source.contains("context: RequestContext"));
}
fn quote_ident(s: &str) -> TokenStream2 {
    let ident = PmIdent::new(s, PmSpan::call_site());
    TokenStream2::from(TokenTree::Ident(ident))
}

fn quote_punct(s: &str) -> TokenStream2 {
    s.chars()
        .map(|c| TokenTree::Punct(proc_macro2::Punct::new(c, Spacing::Joint)))
        .collect()
}

fn ident(name: &str) -> TsIdent {
    TsIdent::new(name.into(), DUMMY_SP, SyntaxContext::empty())
}

fn expr_ident(name: &str) -> Expr {
    Expr::Ident(ident(name))
}
