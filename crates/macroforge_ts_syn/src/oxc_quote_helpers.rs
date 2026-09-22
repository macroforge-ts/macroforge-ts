use oxc::allocator::Allocator;
use oxc::ast::ast::{
    AssignmentExpression, AssignmentTarget, BindingIdentifier, BindingPattern, Expression,
    FormalParameter, Function, IdentifierName, IdentifierReference, ObjectPropertyKind, Program,
    Statement, StringLiteral, TSType,
};
use oxc::codegen::{Codegen, Context, Gen};
use oxc::parser::Parser;
use oxc::span::SourceType;

use crate::TsSynError;

fn source_type() -> SourceType {
    SourceType::tsx()
}

// Every parser here takes the caller's arena. The returned node lives as long
// as that arena, which the caller frees by dropping it; the source is copied
// into the same arena because the node borrows from it.

/// The arena `ts_quote!` parses into, from either an owned `Allocator` or an
/// `&'a Allocator` the caller holds. `ts_quote!` always borrows its arena
/// argument, so a reference parameter keeps its own lifetime and the quoted
/// node can be returned to the caller.
pub trait QuoteArena<'a> {
    fn quote_arena(self) -> &'a Allocator;
}

impl<'a> QuoteArena<'a> for &'a Allocator {
    fn quote_arena(self) -> &'a Allocator {
        self
    }
}

impl<'a> QuoteArena<'a> for &&'a Allocator {
    fn quote_arena(self) -> &'a Allocator {
        let &arena = self;
        arena
    }
}

fn parse_program_in<'a>(allocator: &'a Allocator, source: &str) -> Result<Program<'a>, TsSynError> {
    let source = allocator.alloc_str(source);
    let parsed = Parser::new(allocator, source, source_type()).parse();
    if !parsed.diagnostics.is_empty() {
        return Err(TsSynError::Parse(format!(
            "Oxc parse errors: {}",
            parsed
                .diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.to_string())
                .collect::<Vec<_>>()
                .join("; ")
        )));
    }
    Ok(parsed.program)
}

fn codegen_node<T: Gen>(node: &T) -> String {
    let mut codegen = Codegen::new();
    node.print(&mut codegen, Context::default());
    codegen.into_source_text()
}

fn escape_string_literal(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 2);
    escaped.push('"');
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0C}' => escaped.push_str("\\f"),
            ch if ch.is_control() => {
                let code = ch as u32;
                escaped.push_str(&format!("\\u{:04X}", code));
            }
            _ => escaped.push(ch),
        }
    }
    escaped.push('"');
    escaped
}

pub fn parse_oxc_program<'a>(
    allocator: &'a Allocator,
    source: &str,
) -> Result<Program<'a>, TsSynError> {
    parse_program_in(allocator, source)
}

pub fn parse_oxc_expr<'a>(
    allocator: &'a Allocator,
    source: &str,
) -> Result<Expression<'a>, TsSynError> {
    let source = allocator.alloc_str(source);
    Parser::new(allocator, source, source_type())
        .parse_expression()
        .map_err(expression_parse_error)
}

fn expression_parse_error(errors: oxc::diagnostics::Diagnostics) -> TsSynError {
    TsSynError::Parse(
        errors
            .into_iter()
            .map(|diagnostic| diagnostic.to_string())
            .collect::<Vec<_>>()
            .join("; "),
    )
}

pub fn parse_oxc_statement<'a>(
    allocator: &'a Allocator,
    source: &str,
) -> Result<Statement<'a>, TsSynError> {
    let program = parse_program_in(allocator, source)?;
    program
        .body
        .into_iter()
        .next()
        .ok_or_else(|| TsSynError::Parse("no statement found".to_string()))
}

pub fn parse_oxc_module_item<'a>(
    allocator: &'a Allocator,
    source: &str,
) -> Result<Statement<'a>, TsSynError> {
    parse_oxc_statement(allocator, source)
}

pub fn parse_oxc_assignment_target<'a>(
    allocator: &'a Allocator,
    source: &str,
) -> Result<AssignmentTarget<'a>, TsSynError> {
    let statement = parse_oxc_statement(allocator, &format!("({source}) = __macroforge_target;"))?;
    let Statement::ExpressionStatement(expr_stmt) = statement else {
        return Err(TsSynError::Parse(
            "assignment target wrapper did not parse as an expression statement".to_string(),
        ));
    };
    let expr_stmt = expr_stmt.unbox();

    let Expression::AssignmentExpression(assignment) = expr_stmt.expression else {
        return Err(TsSynError::Parse(
            "assignment target wrapper did not parse as an assignment expression".to_string(),
        ));
    };
    let AssignmentExpression { left, .. } = assignment.unbox();
    Ok(left)
}

pub fn parse_oxc_binding_pattern<'a>(
    allocator: &'a Allocator,
    source: &str,
) -> Result<BindingPattern<'a>, TsSynError> {
    let statement = parse_oxc_statement(
        allocator,
        &format!("function __macroforge__({source}) {{}}"),
    )?;
    let Statement::FunctionDeclaration(function_decl) = statement else {
        return Err(TsSynError::Parse(
            "binding pattern wrapper did not parse as a function declaration".to_string(),
        ));
    };
    let function_decl = function_decl.unbox();

    let Function { params, .. } = function_decl;
    let params = params.unbox();
    let FormalParameter { pattern, .. } =
        params.items.into_iter().next().ok_or_else(|| {
            TsSynError::Parse("binding pattern wrapper had no parameter".to_string())
        })?;

    Ok(pattern)
}

pub fn parse_oxc_type<'a>(
    allocator: &'a Allocator,
    source: &str,
) -> Result<TSType<'a>, TsSynError> {
    let statement = parse_oxc_statement(allocator, &format!("type __MacroforgeType = {source};"))?;
    let Statement::TSTypeAliasDeclaration(type_alias) = statement else {
        return Err(TsSynError::Parse(
            "type wrapper did not parse as a type alias declaration".to_string(),
        ));
    };
    let type_alias = type_alias.unbox();

    Ok(type_alias.type_annotation)
}

pub fn parse_oxc_prop_or_spread<'a>(
    allocator: &'a Allocator,
    source: &str,
) -> Result<ObjectPropertyKind<'a>, TsSynError> {
    let expression = parse_oxc_expr(allocator, &format!("({{ {source} }})"))?;
    let object = match expression {
        Expression::ObjectExpression(object) => object,
        Expression::ParenthesizedExpression(paren) => {
            let inner = paren.unbox().expression;
            let Expression::ObjectExpression(object) = inner else {
                return Err(TsSynError::Parse(
                    "property wrapper did not parse as an object expression".to_string(),
                ));
            };
            object
        }
        _ => {
            return Err(TsSynError::Parse(
                "property wrapper did not parse as an object expression".to_string(),
            ));
        }
    };
    let object = object.unbox();

    object.properties.into_iter().next().ok_or_else(|| {
        TsSynError::Parse("property wrapper did not produce any object properties".to_string())
    })
}

pub fn oxc_stmt_to_string(stmt: &Statement<'_>) -> String {
    codegen_node(stmt)
}

pub fn oxc_binding_pattern_to_string(pattern: &BindingPattern<'_>) -> String {
    codegen_node(pattern)
}

pub fn oxc_assignment_target_to_string(target: &AssignmentTarget<'_>) -> String {
    codegen_node(target)
}

pub fn oxc_type_to_string(ty: &TSType<'_>) -> String {
    codegen_node(ty)
}

pub fn oxc_string_literal_to_string(lit: &StringLiteral<'_>) -> String {
    codegen_node(lit)
}

pub fn oxc_expr_to_string(expr: &Expression<'_>) -> String {
    let mut codegen = Codegen::new();
    codegen.print_expression(expr);
    codegen.into_source_text()
}

pub trait ToOxcExprSource {
    fn to_oxc_expr_source(&self) -> String;
}

pub trait ToOxcIdentSource {
    fn to_oxc_ident_source(&self) -> String;
}

pub trait ToOxcPatSource {
    fn to_oxc_pat_source(&self) -> String;
}

pub trait ToOxcAssignTargetSource {
    fn to_oxc_assign_target_source(&self) -> String;
}

pub trait ToOxcTypeSource {
    fn to_oxc_type_source(&self) -> String;
}

pub trait ToOxcStringLiteralSource {
    fn to_oxc_string_literal_source(&self) -> String;
}

impl<T> ToOxcExprSource for &T
where
    T: ToOxcExprSource + ?Sized,
{
    fn to_oxc_expr_source(&self) -> String {
        (*self).to_oxc_expr_source()
    }
}

impl<T> ToOxcIdentSource for &T
where
    T: ToOxcIdentSource + ?Sized,
{
    fn to_oxc_ident_source(&self) -> String {
        (*self).to_oxc_ident_source()
    }
}

impl<T> ToOxcPatSource for &T
where
    T: ToOxcPatSource + ?Sized,
{
    fn to_oxc_pat_source(&self) -> String {
        (*self).to_oxc_pat_source()
    }
}

impl<T> ToOxcAssignTargetSource for &T
where
    T: ToOxcAssignTargetSource + ?Sized,
{
    fn to_oxc_assign_target_source(&self) -> String {
        (*self).to_oxc_assign_target_source()
    }
}

impl<T> ToOxcTypeSource for &T
where
    T: ToOxcTypeSource + ?Sized,
{
    fn to_oxc_type_source(&self) -> String {
        (*self).to_oxc_type_source()
    }
}

impl<T> ToOxcStringLiteralSource for &T
where
    T: ToOxcStringLiteralSource + ?Sized,
{
    fn to_oxc_string_literal_source(&self) -> String {
        (*self).to_oxc_string_literal_source()
    }
}

macro_rules! impl_str_conversions {
    ($ty:ty) => {
        impl ToOxcExprSource for $ty {
            fn to_oxc_expr_source(&self) -> String {
                self.to_string()
            }
        }

        impl ToOxcIdentSource for $ty {
            fn to_oxc_ident_source(&self) -> String {
                self.to_string()
            }
        }

        impl ToOxcPatSource for $ty {
            fn to_oxc_pat_source(&self) -> String {
                self.to_string()
            }
        }

        impl ToOxcAssignTargetSource for $ty {
            fn to_oxc_assign_target_source(&self) -> String {
                self.to_string()
            }
        }

        impl ToOxcTypeSource for $ty {
            fn to_oxc_type_source(&self) -> String {
                self.to_string()
            }
        }

        impl ToOxcStringLiteralSource for $ty {
            fn to_oxc_string_literal_source(&self) -> String {
                escape_string_literal(self)
            }
        }
    };
}

impl_str_conversions!(str);
impl_str_conversions!(String);

impl ToOxcExprSource for Expression<'_> {
    fn to_oxc_expr_source(&self) -> String {
        oxc_expr_to_string(self)
    }
}

impl ToOxcExprSource for IdentifierReference<'_> {
    fn to_oxc_expr_source(&self) -> String {
        self.name.to_string()
    }
}

impl ToOxcExprSource for IdentifierName<'_> {
    fn to_oxc_expr_source(&self) -> String {
        self.name.to_string()
    }
}

impl ToOxcExprSource for BindingIdentifier<'_> {
    fn to_oxc_expr_source(&self) -> String {
        self.name.to_string()
    }
}

impl ToOxcIdentSource for IdentifierReference<'_> {
    fn to_oxc_ident_source(&self) -> String {
        self.name.to_string()
    }
}

impl ToOxcIdentSource for IdentifierName<'_> {
    fn to_oxc_ident_source(&self) -> String {
        self.name.to_string()
    }
}

impl ToOxcIdentSource for BindingIdentifier<'_> {
    fn to_oxc_ident_source(&self) -> String {
        self.name.to_string()
    }
}

impl ToOxcPatSource for BindingPattern<'_> {
    fn to_oxc_pat_source(&self) -> String {
        oxc_binding_pattern_to_string(self)
    }
}

impl ToOxcPatSource for BindingIdentifier<'_> {
    fn to_oxc_pat_source(&self) -> String {
        self.name.to_string()
    }
}

impl ToOxcAssignTargetSource for AssignmentTarget<'_> {
    fn to_oxc_assign_target_source(&self) -> String {
        oxc_assignment_target_to_string(self)
    }
}

impl ToOxcAssignTargetSource for IdentifierReference<'_> {
    fn to_oxc_assign_target_source(&self) -> String {
        self.name.to_string()
    }
}

impl ToOxcAssignTargetSource for BindingIdentifier<'_> {
    fn to_oxc_assign_target_source(&self) -> String {
        self.name.to_string()
    }
}

impl ToOxcTypeSource for TSType<'_> {
    fn to_oxc_type_source(&self) -> String {
        oxc_type_to_string(self)
    }
}

impl ToOxcTypeSource for IdentifierReference<'_> {
    fn to_oxc_type_source(&self) -> String {
        self.name.to_string()
    }
}

impl ToOxcTypeSource for IdentifierName<'_> {
    fn to_oxc_type_source(&self) -> String {
        self.name.to_string()
    }
}

impl ToOxcTypeSource for BindingIdentifier<'_> {
    fn to_oxc_type_source(&self) -> String {
        self.name.to_string()
    }
}

impl ToOxcStringLiteralSource for StringLiteral<'_> {
    fn to_oxc_string_literal_source(&self) -> String {
        oxc_string_literal_to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_expression() {
        let allocator = Allocator::new();
        let expr = parse_oxc_expr(&allocator, "foo?.bar ?? baz").unwrap();
        let source = oxc_expr_to_string(&expr);
        assert!(source.contains("foo"));
        assert!(source.contains("baz"));
    }

    #[test]
    fn parses_statement() {
        let allocator = Allocator::new();
        let stmt = parse_oxc_statement(&allocator, "export class Foo {}").unwrap();
        let source = oxc_stmt_to_string(&stmt);
        assert!(source.contains("class Foo"));
    }

    #[test]
    fn parses_binding_pattern() {
        let allocator = Allocator::new();
        let pattern = parse_oxc_binding_pattern(&allocator, "{ foo, bar }: Props").unwrap();
        let source = oxc_binding_pattern_to_string(&pattern);
        assert!(source.contains("foo"));
        assert!(source.contains("bar"));
    }

    #[test]
    fn parses_assignment_target() {
        let allocator = Allocator::new();
        let target = parse_oxc_assignment_target(&allocator, "foo.bar").unwrap();
        let source = oxc_assignment_target_to_string(&target);
        assert!(source.contains("foo.bar"));
    }

    #[test]
    fn parses_type() {
        let allocator = Allocator::new();
        let ty = parse_oxc_type(&allocator, "Record<string, number>").unwrap();
        let source = oxc_type_to_string(&ty);
        assert!(source.contains("Record"));
        assert!(source.contains("number"));
    }

    #[test]
    fn parses_prop_or_spread() {
        let allocator = Allocator::new();
        let prop = parse_oxc_prop_or_spread(&allocator, "foo: bar").unwrap();
        let source = codegen_node(&prop);
        assert!(source.contains("foo"));
        assert!(source.contains("bar"));
    }

    #[test]
    fn escapes_string_literals() {
        let escaped = "hello\n\"world\"".to_oxc_string_literal_source();
        assert_eq!(escaped, "\"hello\\n\\\"world\\\"\"");
    }
}
