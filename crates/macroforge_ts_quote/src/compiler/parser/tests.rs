use super::*;

fn parse(input: &str) -> Ir {
    Parser::try_new(input)
        .expect("Lexer error in test")
        .parse()
        .expect("Parse error in test")
}

     /// Helper to find all placeholders in the IR (recursively traverses all node types)
     fn find_placeholders(ir: &Ir) -> Vec<(PlaceholderKind, String)> {
         fn collect_node(node: &IrNode, result: &mut Vec<(PlaceholderKind, String)>) {
             match node {
                 IrNode::Placeholder { kind, expr } => {
                     result.push((*kind, expr.to_string()));
                 }
                 IrNode::If { then_body, else_if_branches, else_body, .. } => {
                     collect_nodes(then_body, result);
                     for (_, body) in else_if_branches {
                         collect_nodes(body, result);
                     }
                     if let Some(body) = else_body {
                         collect_nodes(body, result);
                     }
                 }
                 IrNode::For { body, .. } | IrNode::While { body, .. } => {
                     collect_nodes(body, result);
                 }
                 IrNode::ForInStmt { left, right, body } | IrNode::ForOfStmt { left, right, body, .. } => {
                     collect_node(left, result);
                     collect_node(right, result);
                     collect_node(body, result);
                 }
                 IrNode::Match { arms, .. } => {
                     for arm in arms {
                         collect_nodes(&arm.body, result);
                     }
                 }
                 IrNode::IdentBlock { parts } | IrNode::StringInterp { parts, .. } => {
                     collect_nodes(parts, result);
                 }
                 // Declarations
                 IrNode::FnDecl { name, type_params, params, return_type, body, .. } => {
                     collect_node(name, result);
                     if let Some(tp) = type_params {
                         collect_node(tp, result);
                     }
                     collect_nodes(params, result);
                     if let Some(rt) = return_type {
                         collect_node(rt, result);
                     }
                     if let Some(b) = body {
                         collect_node(b, result);
                     }
                 }
                 IrNode::ClassDecl { name, type_params, extends, implements, body, .. } => {
                     collect_node(name, result);
                     if let Some(tp) = type_params {
                         collect_node(tp, result);
                     }
                     if let Some(ext) = extends {
                         collect_node(ext, result);
                     }
                     collect_nodes(implements, result);
                     collect_nodes(body, result);
                 }
                 IrNode::InterfaceDecl { name, type_params, extends, body, .. } => {
                     collect_node(name, result);
                     if let Some(tp) = type_params {
                         collect_node(tp, result);
                     }
                     collect_nodes(extends, result);
                     collect_nodes(body, result);
                 }
                 IrNode::TypeAliasDecl { name, type_params, type_ann, .. } => {
                     collect_node(name, result);
                     if let Some(tp) = type_params {
                         collect_node(tp, result);
                     }
                     collect_node(type_ann, result);
                 }
                 IrNode::VarDecl { decls, .. } => {
                     for decl in decls {
                         collect_node(&decl.name, result);
                         if let Some(ta) = &decl.type_ann {
                             collect_node(ta, result);
                         }
                         if let Some(init) = &decl.init {
                             collect_node(init, result);
                         }
                     }
                 }
                 // Class members
                 IrNode::Constructor { params, body, .. } => {
                     collect_nodes(params, result);
                     if let Some(b) = body {
                         collect_node(b, result);
                     }
                 }
                 IrNode::Method { name, type_params, params, return_type, body, .. } => {
                     collect_node(name, result);
                     if let Some(tp) = type_params {
                         collect_node(tp, result);
                     }
                     collect_nodes(params, result);
                     if let Some(rt) = return_type {
                         collect_node(rt, result);
                     }
                     if let Some(b) = body {
                         collect_node(b, result);
                     }
                 }
                 IrNode::ClassProp { name, type_ann, value, .. } => {
                     collect_node(name, result);
                     if let Some(ta) = type_ann {
                         collect_node(ta, result);
                     }
                     if let Some(v) = value {
                         collect_node(v, result);
                     }
                 }
                 // Statements
                 IrNode::BlockStmt { stmts } => {
                     collect_nodes(stmts, result);
                 }
                 IrNode::ExprStmt { expr } => {
                     collect_node(expr, result);
                 }
                 IrNode::ReturnStmt { arg } => {
                     if let Some(a) = arg {
                         collect_node(a, result);
                     }
                 }
                 IrNode::ThrowStmt { arg } => {
                     collect_node(arg, result);
                 }
                 IrNode::TsIfStmt { test, cons, alt } => {
                     collect_node(test, result);
                     collect_node(cons, result);
                     if let Some(a) = alt {
                         collect_node(a, result);
                     }
                 }
                 // Parameters
                 IrNode::Param { decorators, pat } => {
                     collect_nodes(decorators, result);
                     collect_node(pat, result);
                 }
                 IrNode::BindingIdent { name, type_ann, .. } => {
                     collect_node(name, result);
                     if let Some(ta) = type_ann {
                         collect_node(ta, result);
                     }
                 }
                 IrNode::RestPat { arg, type_ann } => {
                     collect_node(arg, result);
                     if let Some(ta) = type_ann {
                         collect_node(ta, result);
                     }
                 }
                 IrNode::AssignPat { left, right } => {
                     collect_node(left, result);
                     collect_node(right, result);
                 }
                 // Types
                 IrNode::TypeAnnotation { type_ann } => {
                     collect_node(type_ann, result);
                 }
                 IrNode::TypeParams { params } => {
                     collect_nodes(params, result);
                 }
                 // Interface members
                 IrNode::PropSignature { name, type_ann, .. } => {
                     collect_node(name, result);
                     if let Some(ta) = type_ann {
                         collect_node(ta, result);
                     }
                 }
                 IrNode::MethodSignature { name, type_params, params, return_type, .. } => {
                     collect_node(name, result);
                     if let Some(tp) = type_params {
                         collect_node(tp, result);
                     }
                     collect_nodes(params, result);
                     if let Some(rt) = return_type {
                         collect_node(rt, result);
                     }
                 }
                 // Wrapper
                 IrNode::Documented { inner, .. } => {
                     collect_node(inner, result);
                 }
                // Expressions
                IrNode::CondExpr { test, consequent, alternate } => {
                    collect_node(test, result);
                    collect_node(consequent, result);
                    collect_node(alternate, result);
                }
                IrNode::BinExpr { left, right, .. } => {
                    collect_node(left, result);
                    collect_node(right, result);
                }
                IrNode::AssignExpr { left, right, .. } => {
                    collect_node(left, result);
                    collect_node(right, result);
                }
                IrNode::CallExpr { callee, args, .. } => {
                    collect_node(callee, result);
                    collect_nodes(args, result);
                }
                IrNode::MemberExpr { obj, prop, .. } => {
                    collect_node(obj, result);
                    collect_node(prop, result);
                }
                IrNode::NewExpr { callee, args, .. } => {
                    collect_node(callee, result);
                    collect_nodes(args, result);
                }
                IrNode::UnaryExpr { arg, .. } | IrNode::UpdateExpr { arg, .. } | IrNode::AwaitExpr { arg } => {
                    collect_node(arg, result);
                }
                IrNode::SeqExpr { exprs } => {
                    collect_nodes(exprs, result);
                }
                IrNode::ParenExpr { expr } => {
                    collect_node(expr, result);
                }
                IrNode::YieldExpr { arg, .. } => {
                    if let Some(a) = arg {
                        collect_node(a, result);
                    }
                }
                IrNode::ArrowExpr { params, body, .. } => {
                    collect_nodes(params, result);
                    collect_node(body, result);
                }
                IrNode::FnExpr { params, body, .. } => {
                    collect_nodes(params, result);
                    if let Some(b) = body {
                        collect_node(b, result);
                    }
                }
                IrNode::ClassExpr { extends, body, .. } => {
                    if let Some(e) = extends {
                        collect_node(e, result);
                    }
                    collect_nodes(body, result);
                }
                IrNode::ObjectLit { props } => {
                    collect_nodes(props, result);
                }
                IrNode::ArrayLit { elems } => {
                    collect_nodes(elems, result);
                }
                IrNode::KeyValueProp { key, value } => {
                    collect_node(key, result);
                    collect_node(value, result);
                }
                IrNode::ShorthandProp { key } => {
                    collect_node(key, result);
                }
                IrNode::MethodProp { name, type_params, params, return_type, body, .. } => {
                    collect_node(name, result);
                    if let Some(tp) = type_params {
                        collect_node(tp, result);
                    }
                    collect_nodes(params, result);
                    if let Some(rt) = return_type {
                        collect_node(rt, result);
                    }
                    collect_node(body, result);
                }
                IrNode::GetterProp { name, type_ann, body, .. } => {
                    collect_node(name, result);
                    if let Some(ta) = type_ann {
                        collect_node(ta, result);
                    }
                    collect_node(body, result);
                }
                IrNode::SetterProp { name, param, body, .. } => {
                    collect_node(name, result);
                    collect_node(param, result);
                    collect_node(body, result);
                }
                IrNode::SpreadElement { expr } => {
                    collect_node(expr, result);
                }
                IrNode::TplLit { exprs, .. } => {
                    collect_nodes(exprs, result);
                }
                IrNode::TaggedTpl { tag, tpl, .. } => {
                    collect_node(tag, result);
                    collect_node(tpl, result);
                }
                IrNode::TsAsExpr { expr, type_ann } | IrNode::TsSatisfiesExpr { expr, type_ann } => {
                    collect_node(expr, result);
                    collect_node(type_ann, result);
                }
                IrNode::TsNonNullExpr { expr } | IrNode::TsConstAssertion { expr } => {
                    collect_node(expr, result);
                }
                IrNode::TsInstantiation { expr, type_args } => {
                    collect_node(expr, result);
                    collect_node(type_args, result);
                }
                IrNode::OptChainExpr { base, expr } => {
                    collect_node(base, result);
                    collect_node(expr, result);
                }
                _ => {}
             }
         }
         fn collect_nodes(nodes: &[IrNode], result: &mut Vec<(PlaceholderKind, String)>) {
             for node in nodes {
                 collect_node(node, result);
             }
         }
         let mut result = Vec::new();
         collect_nodes(&ir.nodes, &mut result);
         result
     }

     // =========================================================================
     // Basic parsing tests
     // =========================================================================

     #[test]
     fn test_simple_text() {
         let ir = parse("hello world");
         assert_eq!(ir.nodes.len(), 1);
         match &ir.nodes[0] {
             IrNode::Raw(text) => assert!(text.contains("hello")),
             _ => panic!("Expected Raw"),
         }
     }

     #[test]
     fn test_empty_input() {
         let ir = parse("");
         assert!(ir.nodes.is_empty());
     }

     #[test]
     fn test_whitespace_only() {
         let ir = parse("   \n\t  ");
         // Should produce text node(s) with whitespace
         assert!(!ir.nodes.is_empty());
     }

     // =========================================================================
     // Placeholder classification tests
     // =========================================================================

     #[test]
     fn test_expr_placeholder_default() {
         let ir = parse("@{expr}");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 1);
         assert_eq!(placeholders[0], (PlaceholderKind::Expr, "expr".to_string()));
     }

     #[test]
     fn test_expr_placeholder_in_assignment() {
         let ir = parse("const x = @{value}");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 1);
         assert_eq!(placeholders[0].0, PlaceholderKind::Expr);
     }

     #[test]
     fn test_type_annotation_const() {
         let ir = parse("const x: @{T} = 1");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 1);
         assert_eq!(placeholders[0], (PlaceholderKind::Type, "T".to_string()));
     }

     #[test]
     fn test_type_annotation_let() {
         let ir = parse("let x: @{MyType} = value");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 1);
         assert_eq!(placeholders[0].0, PlaceholderKind::Type);
     }

     #[test]
     fn test_type_annotation_function_param() {
         let ir = parse("function foo(x: @{T}) {}");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 1);
         assert_eq!(placeholders[0].0, PlaceholderKind::Type);
     }

     #[test]
     fn test_type_annotation_function_return() {
         let ir = parse("function foo(): @{ReturnType} {}");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 1);
         assert_eq!(placeholders[0].0, PlaceholderKind::Type);
     }

     #[test]
     fn test_type_assertion_as() {
         let ir = parse("x as @{T}");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 1);
         assert_eq!(placeholders[0].0, PlaceholderKind::Type);
     }

     #[test]
     fn test_type_assertion_satisfies() {
         let ir = parse("x satisfies @{T}");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 1);
         assert_eq!(placeholders[0].0, PlaceholderKind::Type);
     }

     #[test]
     fn test_ternary_colon_not_type() {
         // In ternary, the : is not a type annotation
         let ir = parse("cond ? @{a} : @{b}");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 2);
         // Both should be Expr, not Type
         assert_eq!(placeholders[0].0, PlaceholderKind::Expr);
         assert_eq!(placeholders[1].0, PlaceholderKind::Expr);
     }

     #[test]
     fn test_ternary_with_type_before() {
         // Type annotation followed by ternary
         let ir = parse("const x: @{T} = cond ? @{a} : @{b}");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 3);
         assert_eq!(placeholders[0].0, PlaceholderKind::Type); // T
         assert_eq!(placeholders[1].0, PlaceholderKind::Expr); // a
         assert_eq!(placeholders[2].0, PlaceholderKind::Expr); // b
     }

     #[test]
     fn test_multiple_placeholders_mixed() {
         let ir = parse("const x: @{T} = @{v}");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 2);
         assert_eq!(placeholders[0].0, PlaceholderKind::Type);
         assert_eq!(placeholders[1].0, PlaceholderKind::Expr);
     }

     // =========================================================================
     // Control flow tests
     // =========================================================================

     #[test]
     fn test_if_simple() {
         let ir = parse("{#if cond}body{/if}");
         assert_eq!(ir.nodes.len(), 1);
         match &ir.nodes[0] {
             IrNode::If { condition, then_body, else_if_branches, else_body } => {
                 assert_eq!(condition.to_string(), "cond");
                 assert!(!then_body.is_empty());
                 assert!(else_if_branches.is_empty());
                 assert!(else_body.is_none());
             }
             _ => panic!("Expected If"),
         }
     }

     #[test]
     fn test_if_else() {
         let ir = parse("{#if cond}yes{:else}no{/if}");
         match &ir.nodes[0] {
             IrNode::If { condition, then_body, else_body, .. } => {
                 assert_eq!(condition.to_string(), "cond");
                 assert!(!then_body.is_empty());
                 assert!(else_body.is_some());
             }
             _ => panic!("Expected If"),
         }
     }

     #[test]
     fn test_if_else_if_else() {
         let ir = parse("{#if a}1{:else if b}2{:else if c}3{:else}4{/if}");
         match &ir.nodes[0] {
             IrNode::If { condition, else_if_branches, else_body, .. } => {
                 assert_eq!(condition.to_string(), "a");
                 assert_eq!(else_if_branches.len(), 2);
                 assert_eq!(else_if_branches[0].0.to_string(), "b");
                 assert_eq!(else_if_branches[1].0.to_string(), "c");
                 assert!(else_body.is_some());
             }
             _ => panic!("Expected If"),
         }
     }

     #[test]
     fn test_for_loop() {
         let ir = parse("{#for item in items}@{item}{/for}");
         assert_eq!(ir.nodes.len(), 1);
         match &ir.nodes[0] {
             IrNode::For { pattern, iterator, body } => {
                 assert_eq!(pattern.to_string(), "item");
                 assert_eq!(iterator.to_string(), "items");
                 assert!(!body.is_empty());
             }
             _ => panic!("Expected For"),
         }
     }

     #[test]
     fn test_for_with_tuple_pattern() {
         let ir = parse("{#for (key, value) in map}@{key}: @{value}{/for}");
         match &ir.nodes[0] {
             IrNode::For { pattern, iterator, .. } => {
                 let pat_str = pattern.to_string();
                 assert!(pat_str.contains("key"));
                 assert!(pat_str.contains("value"));
                 assert_eq!(iterator.to_string(), "map");
             }
             _ => panic!("Expected For"),
         }
     }

     #[test]
     fn test_while_loop() {
         let ir = parse("{#while cond}body{/while}");
         assert_eq!(ir.nodes.len(), 1);
         match &ir.nodes[0] {
             IrNode::While { condition, body } => {
                 assert_eq!(condition.to_string(), "cond");
                 assert!(!body.is_empty());
             }
             _ => panic!("Expected While"),
         }
     }

     #[test]
     fn test_match_block() {
         let ir = parse("{#match expr}{:case Some(x)}found{:case None}empty{/match}");
         assert_eq!(ir.nodes.len(), 1);
         match &ir.nodes[0] {
             IrNode::Match { expr, arms } => {
                 assert_eq!(expr.to_string(), "expr");
                 assert_eq!(arms.len(), 2);
                 assert!(arms[0].pattern.to_string().contains("Some"));
                 assert!(arms[1].pattern.to_string().contains("None"));
             }
             _ => panic!("Expected Match"),
         }
     }

     #[test]
     fn test_nested_control_flow() {
         let ir = parse("{#if outer}{#for x in xs}@{x}{/for}{/if}");
         match &ir.nodes[0] {
             IrNode::If { then_body, .. } => {
                 assert_eq!(then_body.len(), 1);
                 assert!(matches!(&then_body[0], IrNode::For { .. }));
             }
             _ => panic!("Expected If"),
         }
     }

     // =========================================================================
     // Directive tests
     // =========================================================================

     #[test]
     fn test_let_directive() {
         let ir = parse("{$let x = 1}");
         assert_eq!(ir.nodes.len(), 1);
         match &ir.nodes[0] {
             IrNode::Let { pattern, mutable, value, .. } => {
                 assert_eq!(pattern.to_string(), "x");
                 assert!(!mutable);
                 assert_eq!(value.to_string(), "1");
             }
             _ => panic!("Expected Let"),
         }
     }

     #[test]
     fn test_let_mut_directive() {
         let ir = parse("{$let mut count = 0}");
         match &ir.nodes[0] {
             IrNode::Let { pattern, mutable, .. } => {
                 assert_eq!(pattern.to_string(), "count");
                 assert!(*mutable);
             }
             _ => panic!("Expected Let"),
         }
     }

     #[test]
     fn test_do_directive() {
         let ir = parse("{$do println!(\"test\")}");
         assert_eq!(ir.nodes.len(), 1);
         match &ir.nodes[0] {
             IrNode::Do { code } => {
                 assert!(code.to_string().contains("println"));
             }
             _ => panic!("Expected Do"),
         }
     }

     // =========================================================================
     // Edge cases
     // =========================================================================

     #[test]
     fn test_adjacent_placeholders() {
         let ir = parse("@{a}@{b}@{c}");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 3);
     }

    #[test]
    fn test_interpolated_ident_extreme() {
        // Extreme case: @{prefix}Middle@{mid}Between@{suffix}End
        // This should create an IdentBlock with 6 parts:
        // [Placeholder(prefix), Raw("Middle"), Placeholder(mid), Raw("Between"), Placeholder(suffix), Raw("End")]
        let ir = parse("const x = @{prefix}Middle@{mid}Between@{suffix}End");

        // Find all placeholders - should be 3
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 3, "Expected 3 placeholders, got {}: {:?}", placeholders.len(), placeholders);

        // All should be Ident kind (for identifier concatenation)
        for (i, (kind, _)) in placeholders.iter().enumerate() {
            assert_eq!(*kind, PlaceholderKind::Ident, "Placeholder {} should be Ident kind", i);
        }

        // Verify the expression names
        assert!(placeholders[0].1.contains("prefix"), "First placeholder should be 'prefix'");
        assert!(placeholders[1].1.contains("mid"), "Second placeholder should be 'mid'");
        assert!(placeholders[2].1.contains("suffix"), "Third placeholder should be 'suffix'");
    }

    #[test]
    fn test_interpolated_ident_with_suffix_only() {
        // Simple case: @{name}Suffix
        let ir = parse("const x = @{name}Suffix");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0].0, PlaceholderKind::Ident);
        assert!(placeholders[0].1.contains("name"));
    }

    #[test]
    fn test_interpolated_ident_multiple_adjacent() {
        // Adjacent interpolations: @{a}@{b}@{c}Suffix
        let ir = parse("const x = @{a}@{b}@{c}Suffix");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 3);
        // All should be Ident kind
        for (kind, _) in &placeholders {
            assert_eq!(*kind, PlaceholderKind::Ident);
        }
    }

     #[test]
     fn test_placeholder_in_string() {
         // String interpolation using template literal
         let ir = parse("`hello @{name}`");
         assert_eq!(ir.nodes.len(), 1);
         match &ir.nodes[0] {
             IrNode::TplLit { quasis, exprs } => {
                 // Template literal with "hello " and "" quasis, with placeholder expression
                 assert!(!quasis.is_empty(), "Expected quasis");
                 assert!(!exprs.is_empty(), "Expected expressions (placeholder)");
             }
             _ => panic!("Expected TplLit, got {:?}", ir.nodes[0]),
         }
     }

     #[test]
     fn test_complex_rust_expr() {
         let ir = parse("@{vec![1, 2, 3].iter().map(|x| x * 2).collect::<Vec<_>>()}");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 1);
         // TokenStream.to_string() may format differently, check for key parts
         let expr_str = &placeholders[0].1;
         assert!(expr_str.contains("iter"), "Expected 'iter' in: {}", expr_str);
         assert!(expr_str.contains("map"), "Expected 'map' in: {}", expr_str);
         assert!(expr_str.contains("collect"), "Expected 'collect' in: {}", expr_str);
     }

     #[test]
     fn test_placeholder_with_generics() {
         let ir = parse("const x: @{HashMap<String, i32>} = map");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 1);
         assert_eq!(placeholders[0].0, PlaceholderKind::Type);
     }

     #[test]
     fn test_semicolon_lexer() {
         // First verify lexer handles semicolons correctly
         use crate::compiler::lexer::Lexer;
         let tokens = Lexer::new("x; y").tokenize().expect("lexer should succeed");
         let has_semi = tokens.iter().any(|t| t.kind == SyntaxKind::Semicolon);
         assert!(has_semi, "Expected Semicolon token, got: {:?}", tokens.iter().map(|t| (t.kind, &t.text)).collect::<Vec<_>>());
     }

     #[test]
     fn test_semicolon_simple() {
         // Simple semicolon test - should not hang
         let ir = parse("x; y");
         assert!(!ir.nodes.is_empty());
     }

     #[test]
     fn test_semicolon_ends_type_context() {
         // After semicolon, we should be back in expression context
         let ir = parse("const x: @{T} = 1; @{expr}");
         let placeholders = find_placeholders(&ir);
         assert_eq!(placeholders.len(), 2);
         assert_eq!(placeholders[0].0, PlaceholderKind::Type);
         assert_eq!(placeholders[1].0, PlaceholderKind::Expr);
     }

     // =========================================================================
     // Lexer integration tests
     // =========================================================================

     #[test]
     fn test_lexer_produces_colon() {
         use crate::compiler::lexer::Lexer;
         let tokens = Lexer::new("const x: T").tokenize().expect("lexer should succeed");
         let has_colon = tokens.iter().any(|t| t.kind == SyntaxKind::Colon);
         assert!(has_colon, "Expected Colon token");
     }

     #[test]
     fn test_lexer_produces_keywords() {
         use crate::compiler::lexer::Lexer;
         let tokens = Lexer::new("const let function class interface type").tokenize().expect("lexer should succeed");
         let kinds: Vec<_> = tokens.iter().map(|t| t.kind).collect();
         assert!(kinds.contains(&SyntaxKind::ConstKw));
         assert!(kinds.contains(&SyntaxKind::LetKw));
         assert!(kinds.contains(&SyntaxKind::FunctionKw));
         assert!(kinds.contains(&SyntaxKind::ClassKw));
         assert!(kinds.contains(&SyntaxKind::InterfaceKw));
         assert!(kinds.contains(&SyntaxKind::TypeKw));
     }

     #[test]
     fn test_debug_parser_output() {
         // Debug test to see what the parser produces for a template with control flow
         let input = r#"export function @{fn_name}() {
             {#for field in fields}
             console.log(@{field});
             {/for}
         }"#;
         let ir = parse(input);

         eprintln!("=== IR Nodes ===");
         for (i, node) in ir.nodes.iter().enumerate() {
             eprintln!("Node {}: {:?}", i, node);
         }

         // Helper to recursively find For nodes
         fn has_for_node(node: &IrNode) -> bool {
             match node {
                 IrNode::For { .. } => true,
                 IrNode::FnDecl { body, .. } => body.as_ref().map(|b| has_for_node(b)).unwrap_or(false),
                 IrNode::BlockStmt { stmts } => stmts.iter().any(has_for_node),
                 _ => false,
             }
         }

         // Verify for loop is parsed correctly (now inside FnDecl.body)
         let has_for = ir.nodes.iter().any(|n| has_for_node(n));
         assert!(has_for, "Expected For node in IR (may be nested inside FnDecl)");

         // Verify raw nodes don't contain control flow markers
         fn check_no_control_flow_in_raw(node: &IrNode) {
             match node {
                 IrNode::Raw(text) => {
                     assert!(!text.contains("{#for"), "Raw node should not contain {{#for: {}", text);
                     assert!(!text.contains("{/for"), "Raw node should not contain {{/for: {}", text);
                 }
                 IrNode::FnDecl { body, .. } => {
                     if let Some(b) = body {
                         check_no_control_flow_in_raw(b);
                     }
                 }
                 IrNode::BlockStmt { stmts } => {
                     for stmt in stmts {
                         check_no_control_flow_in_raw(stmt);
                     }
                 }
                 _ => {}
             }
         }
         for node in &ir.nodes {
             check_no_control_flow_in_raw(node);
         }
     }

    // =========================================================================
    // For-in / For-of loop tests (Phase 2)
    // =========================================================================

    #[test]
    fn test_for_in_stmt() {
        let ir = parse("for (const key in obj) { console.log(key); }");
        // For-in should be parsed as ForInStmt
        let has_for_in = ir.nodes.iter().any(|n| matches!(n, IrNode::ForInStmt { .. }));
        assert!(has_for_in, "Expected ForInStmt in IR");
    }

    #[test]
    fn test_for_of_stmt() {
        let ir = parse("for (const item of items) { console.log(item); }");
        // For-of should be parsed as ForOfStmt
        let has_for_of = ir.nodes.iter().any(|n| matches!(n, IrNode::ForOfStmt { await_: false, .. }));
        assert!(has_for_of, "Expected ForOfStmt in IR");
    }

    #[test]
    fn test_for_await_of_stmt() {
        let ir = parse("for await (const chunk of stream) { process(chunk); }");
        // For-await-of should be parsed as ForOfStmt with await_: true
        let has_for_await_of = ir.nodes.iter().any(|n| matches!(n, IrNode::ForOfStmt { await_: true, .. }));
        assert!(has_for_await_of, "Expected ForOfStmt with await in IR");
    }

    #[test]
    fn test_for_in_with_destructuring() {
        let ir = parse("for (const [key, value] in obj) { }");
        let has_for_in = ir.nodes.iter().any(|n| matches!(n, IrNode::ForInStmt { .. }));
        assert!(has_for_in, "Expected ForInStmt with destructuring in IR");
    }

    #[test]
    fn test_for_of_with_placeholder() {
        let ir = parse("for (const @{name} of @{items}) { @{body} }");
        let placeholders = find_placeholders(&ir);
        assert!(placeholders.len() >= 2, "Expected placeholders in for-of loop");
    }

    // =========================================================================
    // IndexSignature tests (Phase 9)
    // =========================================================================

    #[test]
    fn test_index_signature_basic() {
        let ir = parse("interface Dict { [key: string]: unknown; }");
        // Find IndexSignature in interface body
        fn find_index_signature(node: &IrNode) -> bool {
            match node {
                IrNode::InterfaceDecl { body, .. } => {
                    body.iter().any(|m| matches!(m, IrNode::IndexSignature { .. }))
                }
                _ => false,
            }
        }
        let has_index_sig = ir.nodes.iter().any(find_index_signature);
        assert!(has_index_sig, "Expected IndexSignature in interface");
    }

    #[test]
    fn test_index_signature_readonly() {
        let ir = parse("interface ReadonlyDict { readonly [key: string]: number; }");
        fn find_readonly_index_signature(node: &IrNode) -> bool {
            match node {
                IrNode::InterfaceDecl { body, .. } => {
                    body.iter().any(|m| matches!(m, IrNode::IndexSignature { readonly: true, .. }))
                }
                _ => false,
            }
        }
        let has_readonly = ir.nodes.iter().any(find_readonly_index_signature);
        assert!(has_readonly, "Expected readonly IndexSignature in interface");
    }

    #[test]
    fn test_interface_with_multiple_members() {
        let ir = parse("interface MyInterface { name: string; [key: string]: unknown; getValue(): number; }");
        fn count_interface_members(node: &IrNode) -> usize {
            match node {
                IrNode::InterfaceDecl { body, .. } => body.len(),
                _ => 0,
            }
        }
        let member_count: usize = ir.nodes.iter().map(count_interface_members).sum();
        assert!(member_count >= 3, "Expected at least 3 members in interface, got {}", member_count);
    }

    // =========================================================================
    // Destructuring pattern tests (Phase 2/7)
    // =========================================================================

    #[test]
    fn test_array_pattern_for_of() {
        let ir = parse("for (const [a, b] of pairs) { }");
        // Should parse array destructuring in for-of
        let has_for_of = ir.nodes.iter().any(|n| matches!(n, IrNode::ForOfStmt { .. }));
        assert!(has_for_of, "Expected ForOfStmt with array pattern");
    }

    #[test]
    fn test_object_pattern_for_of() {
        let ir = parse("for (const { name, value } of entries) { }");
        // Should parse object destructuring in for-of
        let has_for_of = ir.nodes.iter().any(|n| matches!(n, IrNode::ForOfStmt { .. }));
        assert!(has_for_of, "Expected ForOfStmt with object pattern");
    }

    #[test]
    fn test_rest_pattern_in_array() {
        let ir = parse("for (const [first, ...rest] of arrays) { }");
        let has_for_of = ir.nodes.iter().any(|n| matches!(n, IrNode::ForOfStmt { .. }));
        assert!(has_for_of, "Expected ForOfStmt with rest pattern");
    }

    // =========================================================================
    // Expression placeholder tests for new expression types
    // =========================================================================

    #[test]
    fn test_await_expr_placeholder() {
        let ir = parse("const result = await @{promise}");
        let placeholders = find_placeholders(&ir);
        assert!(!placeholders.is_empty(), "Expected placeholder in await expr");
    }

    #[test]
    fn test_as_expr_type_classification() {
        let ir = parse("const x = value as @{MyType}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0].0, PlaceholderKind::Type, "Placeholder after 'as' should be Type");
    }

    #[test]
    fn test_satisfies_expr_type_classification() {
        let ir = parse("const x = value satisfies @{MyType}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0].0, PlaceholderKind::Type, "Placeholder after 'satisfies' should be Type");
    }

    #[test]
    fn test_non_null_assertion() {
        // Non-null assertion: value!
        let ir = parse("const x = value!.prop");
        assert!(!ir.nodes.is_empty(), "Should parse non-null assertion");
    }

    // =========================================================================
    // Type parsing tests (Phase 8)
    // =========================================================================

    #[test]
    fn test_intersection_type() {
        let ir = parse("type Combined = @{A} & @{B}");
        let placeholders = find_placeholders(&ir);
        // Should find placeholders in type alias
        assert!(placeholders.len() >= 2, "Expected at least 2 placeholders, got {}", placeholders.len());
    }

    #[test]
    fn test_tuple_type() {
        let ir = parse("type Pair = [@{First}, @{Second}]");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 2);
    }

    #[test]
    fn test_keyof_type() {
        let ir = parse("type Keys = keyof @{T}");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0].0, PlaceholderKind::Type);
    }

    #[test]
    fn test_typeof_type() {
        let ir = parse("type TypeOfX = typeof x");
        // Should parse typeof as a type
        assert!(!ir.nodes.is_empty());
    }

    #[test]
    fn test_mapped_type() {
        let ir = parse("type Readonly<T> = { readonly [K in keyof T]: T[K] }");
        // Should parse mapped type syntax
        assert!(!ir.nodes.is_empty());
    }

    // =========================================================================
    // Object literal property tests (Phase 6)
    // =========================================================================

    #[test]
    fn test_getter_in_object() {
        let ir = parse("const obj = { get @{name}() { return value; } }");
        let placeholders = find_placeholders(&ir);
        assert!(!placeholders.is_empty(), "Expected placeholder in getter name");
    }

    #[test]
    fn test_setter_in_object() {
        let ir = parse("const obj = { set @{name}(value) { } }");
        let placeholders = find_placeholders(&ir);
        assert!(!placeholders.is_empty(), "Expected placeholder in setter name");
    }

    #[test]
    fn test_method_in_object() {
        let ir = parse("const obj = { @{name}() { return 42; } }");
        let placeholders = find_placeholders(&ir);
        assert!(!placeholders.is_empty(), "Expected placeholder in method name");
    }

    // =========================================================================
    // Complex expression tests (Phase 5)
    // =========================================================================

    #[test]
    fn test_paren_expr() {
        let ir = parse("const x = (@{a} + @{b})");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 2);
    }

    #[test]
    fn test_seq_expr() {
        let ir = parse("const x = (@{a}, @{b}, @{c})");
        let placeholders = find_placeholders(&ir);
        assert_eq!(placeholders.len(), 3);
    }

    #[test]
    fn test_tagged_template() {
        let ir = parse("const x = @{tag}`hello ${@{name}}`");
        let placeholders = find_placeholders(&ir);
        assert!(placeholders.len() >= 2, "Expected placeholders in tagged template");
    }

    #[test]
    fn test_class_expression() {
        let ir = parse("const MyClass = class { constructor() {} }");
        // Should parse class expression
        assert!(!ir.nodes.is_empty());
    }

    #[test]
    fn test_function_expression() {
        let ir = parse("const fn = function(@{param}) { return @{result}; }");
        let placeholders = find_placeholders(&ir);
        // Note: The @{result} placeholder is inside the function body which uses
        // a simplified block parsing that doesn't extract statements yet.
        // For now, we just verify the @{param} placeholder is found.
        assert!(placeholders.len() >= 1, "Expected at least 1 placeholder in function expression");
    }

    // =========================================================================
    // Expression-level control flow tests
    // =========================================================================

    #[test]
    fn test_parse_if_expression_debug() {
        // This is the exact input that fails in test_if_expression_in_statement
        let input = r#"const status = {#if cond} "a" {:else} "b" {/if}"#;

        // First, print the tokens to see what the parser sees
        use crate::compiler::lexer::Lexer;
        let tokens = Lexer::new(input).tokenize().expect("lexer should succeed");
        eprintln!("Tokens for if-expression parsing:");
        for (i, t) in tokens.iter().enumerate() {
            eprintln!("  {:3}: {:?} = {:?} (start={})", i, t.kind, t.text, t.start);
        }

        // Now try to parse
        let result = Parser::try_new(input);
        match result {
            Err(e) => panic!("Lexer error: {:?}", e),
            Ok(mut parser) => {
                let parse_result = parser.parse();
                match parse_result {
                    Ok(ir) => {
                        eprintln!("\nParsed IR:");
                        for (i, node) in ir.nodes.iter().enumerate() {
                            eprintln!("  Node {}: {:?}", i, node);
                        }
                        // Check for IfExpr node
                        fn has_if_expr(node: &IrNode) -> bool {
                            match node {
                                IrNode::IfExpr { .. } => true,
                                IrNode::VarDecl { decls, .. } => {
                                    decls.iter().any(|d| d.init.as_ref().map_or(false, |init| has_if_expr(init)))
                                }
                                _ => false,
                            }
                        }
                        let has_if = ir.nodes.iter().any(has_if_expr);
                        assert!(has_if, "Expected IfExpr node in IR");
                    }
                    Err(e) => {
                        panic!("Parse error: {}", e.to_message());
                    }
                }
            }
        }
    }
