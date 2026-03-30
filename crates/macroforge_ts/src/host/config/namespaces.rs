use swc_core::{
    common::{FileName, SourceMap, sync::Lrc},
    ecma::{
        ast::*,
        parser::{Lexer, Parser, StringInput, Syntax, TsSyntax},
    },
};

/// Extract namespace identifiers referenced in an expression string.
///
/// Parses the expression and finds all member expression roots that could be namespaces.
/// For example, `(v) => DateTime.formatIso(v)` would return `["DateTime"]`.
///
/// This is used to determine which namespaces need to be imported for foreign type
/// expressions to work at runtime.
pub fn extract_expression_namespaces(expr_str: &str) -> Vec<String> {
    use std::collections::HashSet;

    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        FileName::Custom("expr.ts".to_string()).into(),
        expr_str.to_string(),
    );

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: false,
            decorators: false,
            ..Default::default()
        }),
        EsVersion::latest(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let expr = match parser.parse_expr() {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    let mut namespaces = HashSet::new();
    collect_member_expression_roots(&expr, &mut namespaces);
    namespaces.into_iter().collect()
}

/// Recursively collect root identifiers from member expressions.
///
/// For `DateTime.formatIso(v)`, this extracts `DateTime`.
/// For `A.B.c()`, this extracts `A`.
fn collect_member_expression_roots(
    expr: &Expr,
    namespaces: &mut std::collections::HashSet<String>,
) {
    match expr {
        // Member expression: DateTime.formatIso
        Expr::Member(member) => {
            // Get the root of the member expression chain
            if let Some(root) = get_member_root(&member.obj) {
                namespaces.insert(root);
            }
            // Also check the object recursively for nested member expressions
            collect_member_expression_roots(&member.obj, namespaces);
        }
        // Call expression: DateTime.formatIso(v)
        Expr::Call(call) => {
            if let Callee::Expr(callee) = &call.callee {
                collect_member_expression_roots(callee, namespaces);
            }
            // Also check arguments
            for arg in &call.args {
                collect_member_expression_roots(&arg.expr, namespaces);
            }
        }
        // Arrow function: (v) => DateTime.formatIso(v)
        Expr::Arrow(arrow) => match &*arrow.body {
            BlockStmtOrExpr::Expr(e) => collect_member_expression_roots(e, namespaces),
            BlockStmtOrExpr::BlockStmt(block) => {
                for stmt in &block.stmts {
                    collect_statement_namespaces(stmt, namespaces);
                }
            }
        },
        // Function expression: function(v) { return DateTime.formatIso(v); }
        Expr::Fn(fn_expr) => {
            if let Some(body) = &fn_expr.function.body {
                for stmt in &body.stmts {
                    collect_statement_namespaces(stmt, namespaces);
                }
            }
        }
        // Parenthesized expression
        Expr::Paren(paren) => {
            collect_member_expression_roots(&paren.expr, namespaces);
        }
        // Binary expression
        Expr::Bin(bin) => {
            collect_member_expression_roots(&bin.left, namespaces);
            collect_member_expression_roots(&bin.right, namespaces);
        }
        // Conditional expression
        Expr::Cond(cond) => {
            collect_member_expression_roots(&cond.test, namespaces);
            collect_member_expression_roots(&cond.cons, namespaces);
            collect_member_expression_roots(&cond.alt, namespaces);
        }
        // New expression: new DateTime()
        Expr::New(new) => {
            collect_member_expression_roots(&new.callee, namespaces);
            if let Some(args) = &new.args {
                for arg in args {
                    collect_member_expression_roots(&arg.expr, namespaces);
                }
            }
        }
        // Array expression
        Expr::Array(arr) => {
            for elem in arr.elems.iter().flatten() {
                collect_member_expression_roots(&elem.expr, namespaces);
            }
        }
        // Object expression
        Expr::Object(obj) => {
            for prop in &obj.props {
                if let PropOrSpread::Prop(p) = prop
                    && let Prop::KeyValue(kv) = &**p
                {
                    collect_member_expression_roots(&kv.value, namespaces);
                }
            }
        }
        // Template literal
        Expr::Tpl(tpl) => {
            for expr in &tpl.exprs {
                collect_member_expression_roots(expr, namespaces);
            }
        }
        // Sequence expression
        Expr::Seq(seq) => {
            for expr in &seq.exprs {
                collect_member_expression_roots(expr, namespaces);
            }
        }
        _ => {}
    }
}

/// Collect namespaces from statements.
fn collect_statement_namespaces(stmt: &Stmt, namespaces: &mut std::collections::HashSet<String>) {
    match stmt {
        Stmt::Return(ret) => {
            if let Some(arg) = &ret.arg {
                collect_member_expression_roots(arg, namespaces);
            }
        }
        Stmt::Expr(expr) => {
            collect_member_expression_roots(&expr.expr, namespaces);
        }
        Stmt::If(if_stmt) => {
            collect_member_expression_roots(&if_stmt.test, namespaces);
            collect_statement_namespaces(&if_stmt.cons, namespaces);
            if let Some(alt) = &if_stmt.alt {
                collect_statement_namespaces(alt, namespaces);
            }
        }
        Stmt::Block(block) => {
            for s in &block.stmts {
                collect_statement_namespaces(s, namespaces);
            }
        }
        Stmt::Decl(Decl::Var(var)) => {
            for decl in &var.decls {
                if let Some(init) = &decl.init {
                    collect_member_expression_roots(init, namespaces);
                }
            }
        }
        _ => {}
    }
}

/// Get the root identifier of a member expression chain.
///
/// For `DateTime.formatIso`, returns `Some("DateTime")`.
/// For `a.b.c`, returns `Some("a")`.
fn get_member_root(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Ident(ident) => Some(ident.sym.to_string()),
        Expr::Member(member) => get_member_root(&member.obj),
        _ => None,
    }
}
