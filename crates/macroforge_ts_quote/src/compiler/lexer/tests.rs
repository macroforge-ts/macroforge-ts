
     use super::*;

     fn lex(input: &str) -> Vec<(SyntaxKind, String)> {
         let tokens = Lexer::new(input).tokenize();
         tokens.iter().map(|t| (t.kind, t.text.clone())).collect()
     }

     #[test]
     fn test_simple_text() {
         let tokens = lex("hello world");
         assert_eq!(tokens[0].0, SyntaxKind::Ident);
         assert_eq!(tokens[0].1, "hello");
     }

     #[test]
     fn test_normalize_spaced_interpolation() {
         // Rust tokenizer produces `@ { expr }` from `@{expr}`
         // We collapse `@ {` to `@{` but preserve internal whitespace
         let normalized = normalize_template("@ { expr }");
         assert_eq!(normalized, "@{ expr }");
     }

     #[test]
     fn test_normalize_spaced_control_block() {
         let normalized = normalize_template("{ # if cond }");
         assert_eq!(normalized, "{#if cond }");
     }

     #[test]
     fn test_interpolation() {
         let tokens = lex("@{expr}");
         assert_eq!(tokens[0].0, SyntaxKind::At);
         // The lexer consumes interpolation content and closing brace together
         assert_eq!(tokens[1].0, SyntaxKind::RBrace);
         // Verify the content was consumed
         assert_eq!(tokens.len(), 2);
     }

     #[test]
     fn test_control_block() {
         let tokens = lex("{#if cond}");
         assert_eq!(tokens[0].0, SyntaxKind::HashOpen);
         assert_eq!(tokens[1].0, SyntaxKind::IfKw);
     }

     #[test]
     fn test_type_annotation() {
         let tokens = lex("const x: number");
         // Find the colon
         let colon = tokens.iter().find(|(k, _)| *k == SyntaxKind::Colon);
         assert!(colon.is_some());
     }

     #[test]
     fn test_as_keyword() {
         let tokens = lex("value as Type");
         let as_kw = tokens.iter().find(|(k, _)| *k == SyntaxKind::AsKw);
         assert!(as_kw.is_some());
     }

     #[test]
     fn test_for_block_tokens() {
         let tokens = lex("{#for item in items}");
         // Should have: HASH_OPEN, FOR_KW, WHITESPACE, IDENT(item), WHITESPACE, IN_KW, WHITESPACE, IDENT(items), RBRACE
         let in_kw = tokens.iter().find(|(k, _)| *k == SyntaxKind::InKw);
         assert!(in_kw.is_some(), "IN_KW token not found in: {:?}", tokens);
     }

     // ==================== Token Struct Tests ====================

     #[test]
     fn test_token_equality() {
         let t1 = Token {
             kind: SyntaxKind::Ident,
             text: "foo".to_string(),
             start: 0,
         };
         let t2 = Token {
             kind: SyntaxKind::Ident,
             text: "foo".to_string(),
             start: 0,
         };
         assert_eq!(t1, t2);
     }

     #[test]
     fn test_token_inequality_kind() {
         let t1 = Token {
             kind: SyntaxKind::Ident,
             text: "foo".to_string(),
             start: 0,
         };
         let t2 = Token {
             kind: SyntaxKind::Text,
             text: "foo".to_string(),
             start: 0,
         };
         assert_ne!(t1, t2);
     }

     #[test]
     fn test_token_clone() {
         let t1 = Token {
             kind: SyntaxKind::Ident,
             text: "bar".to_string(),
             start: 5,
         };
         let t2 = t1.clone();
         assert_eq!(t1, t2);
     }

     #[test]
     fn test_token_debug() {
         let t = Token {
             kind: SyntaxKind::Ident,
             text: "test".to_string(),
             start: 0,
         };
         let debug = format!("{:?}", t);
         assert!(debug.contains("Token"));
         assert!(debug.contains("Ident"));
     }

     // ==================== Lexer Construction Tests ====================

     #[test]
     fn test_lexer_empty_input() {
         let tokens = lex("");
         assert!(tokens.is_empty());
     }

     #[test]
     fn test_lexer_whitespace_only() {
         let tokens = lex("   \t\n  ");
         assert!(!tokens.is_empty());
         for (kind, _) in &tokens {
             assert_eq!(*kind, SyntaxKind::Whitespace);
         }
     }

     // ==================== Basic Token Tests ====================

     #[test]
     fn test_lex_braces() {
         let tokens = lex("{}");
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::LBrace));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::RBrace));
     }

     #[test]
     fn test_lex_parens() {
         let tokens = lex("()");
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::LParen));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::RParen));
     }

     #[test]
     fn test_lex_brackets() {
         let tokens = lex("[]");
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::LBracket));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::RBracket));
     }

     #[test]
     fn test_lex_angle_brackets() {
         let tokens = lex("<>");
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::Lt));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::Gt));
     }

     #[test]
     fn test_lex_punctuation() {
         let tokens = lex(":;,=? .");
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::Colon));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::Semicolon));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::Comma));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::Eq));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::Question));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::Dot));
     }

     #[test]
     fn test_lex_new_operators() {
         // Test new multi-character operators
         let tokens = lex("?.++--...#!&");
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::QuestionDot));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::PlusPlus));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::MinusMinus));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::DotDotDot));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::Hash));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::Exclaim));
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::Ampersand));
     }

     #[test]
     fn test_lex_quotes() {
         // Test double and single quotes separately
         // Note: " starts string mode so the single quote becomes string content
         let tokens = lex("\"");
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::DoubleQuote));

         let tokens2 = lex("'");
         assert!(tokens2.iter().any(|(k, _)| *k == SyntaxKind::SingleQuote));
     }

     #[test]
     fn test_lex_backtick() {
         let tokens = lex("`");
         assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::Backtick));
     }

     // ==================== Interpolation Tests ====================

     #[test]
     fn test_interpolation_simple() {
         let tokens = lex("@{x}");
         assert_eq!(tokens[0].0, SyntaxKind::At);
         assert_eq!(tokens[1].0, SyntaxKind::RBrace);
     }

     #[test]
     fn test_interpolation_complex_expr() {
         let tokens = lex("@{foo.bar()}");
         assert_eq!(tokens[0].0, SyntaxKind::At);
         // Content is inside RBrace token
         assert!(tokens[1].1.contains("foo"));
     }

     #[test]
     fn test_interpolation_with_generics() {
         let tokens = lex("@{Vec::<i32>::new()}");
         assert_eq!(tokens[0].0, SyntaxKind::At);
     }

     #[test]
     fn test_double_at_escape() {
         let tokens = lex("@@{literal}");
         assert_eq!(tokens[0].0, SyntaxKind::AtAt);
     }

     #[test]
     fn test_single_at_is_text() {
         let tokens = lex("@");
         assert_eq!(tokens[0].0, SyntaxKind::Text);
     }

     // ==================== Control Block Tests ====================

     #[test]
     fn test_hash_open() {
         let tokens = lex("{#");
         assert_eq!(tokens[0].0, SyntaxKind::HashOpen);
     }

     #[test]
     fn test_slash_open() {
         let tokens = lex("{/");
         assert_eq!(tokens[0].0, SyntaxKind::SlashOpen);
     }

     #[test]
     fn test_colon_open() {
         let tokens = lex("{:");
         assert_eq!(tokens[0].0, SyntaxKind::ColonOpen);
     }

     #[test]
     fn test_dollar_open() {
         let tokens = lex("{$");
         assert_eq!(tokens[0].0, SyntaxKind::DollarOpen);
     }

     #[test]
     fn test_pipe_open() {
         let tokens = lex("{|");
         assert_eq!(tokens[0].0, SyntaxKind::PipeOpen);
     }

     #[test]
     fn test_if_control_block() {
         let tokens = lex("{#if condition}");
         assert_eq!(tokens[0].0, SyntaxKind::HashOpen);
         assert_eq!(tokens[1].0, SyntaxKind::IfKw);
     }

     #[test]
     fn test_else_control_block() {
         let tokens = lex("{:else}");
         assert_eq!(tokens[0].0, SyntaxKind::ColonOpen);
         assert_eq!(tokens[1].0, SyntaxKind::ElseKw);
     }

     #[test]
     fn test_else_if_control_block() {
         let tokens = lex("{:else if cond}");
         assert_eq!(tokens[0].0, SyntaxKind::ColonOpen);
         assert_eq!(tokens[1].0, SyntaxKind::ElseKw);
         // Skip whitespace
         let if_kw = tokens.iter().find(|(k, _)| *k == SyntaxKind::IfKw);
         assert!(if_kw.is_some());
     }

     #[test]
     fn test_for_control_block() {
         let tokens = lex("{#for x in xs}");
         assert_eq!(tokens[0].0, SyntaxKind::HashOpen);
         assert_eq!(tokens[1].0, SyntaxKind::ForKw);
     }

     #[test]
     fn test_while_control_block() {
         let tokens = lex("{#while cond}");
         assert_eq!(tokens[0].0, SyntaxKind::HashOpen);
         assert_eq!(tokens[1].0, SyntaxKind::WhileKw);
     }

     #[test]
     fn test_match_control_block() {
         let tokens = lex("{#match expr}");
         assert_eq!(tokens[0].0, SyntaxKind::HashOpen);
         assert_eq!(tokens[1].0, SyntaxKind::MatchKw);
     }

     #[test]
     fn test_case_control_block() {
         let tokens = lex("{:case Some(x)}");
         assert_eq!(tokens[0].0, SyntaxKind::ColonOpen);
         assert_eq!(tokens[1].0, SyntaxKind::CaseKw);
     }

     #[test]
     fn test_end_if_block() {
         let tokens = lex("{/if}");
         assert_eq!(tokens[0].0, SyntaxKind::SlashOpen);
         assert_eq!(tokens[1].0, SyntaxKind::IfKw);
     }

     #[test]
     fn test_end_for_block() {
         let tokens = lex("{/for}");
         assert_eq!(tokens[0].0, SyntaxKind::SlashOpen);
         assert_eq!(tokens[1].0, SyntaxKind::ForKw);
     }

     // ==================== Directive Tests ====================

     #[test]
     fn test_let_directive() {
         let tokens = lex("{$let x = 1}");
         assert_eq!(tokens[0].0, SyntaxKind::DollarOpen);
         assert_eq!(tokens[1].0, SyntaxKind::LetKw);
     }

     #[test]
     fn test_let_mut_directive() {
         let tokens = lex("{$let mut x = 1}");
         assert_eq!(tokens[0].0, SyntaxKind::DollarOpen);
         assert_eq!(tokens[1].0, SyntaxKind::LetKw);
         let mut_kw = tokens.iter().find(|(k, _)| *k == SyntaxKind::MutKw);
         assert!(mut_kw.is_some());
     }

     #[test]
     fn test_do_directive() {
         let tokens = lex("{$do println!(\"test\")}");
         assert_eq!(tokens[0].0, SyntaxKind::DollarOpen);
         assert_eq!(tokens[1].0, SyntaxKind::DoKw);
     }

     #[test]
     fn test_typescript_directive() {
         let tokens = lex("{$typescript foo}");
         assert_eq!(tokens[0].0, SyntaxKind::DollarOpen);
         assert_eq!(tokens[1].0, SyntaxKind::TypeScriptKw);
     }

     // ==================== Comment Tests ====================

     #[test]
     fn test_line_comment_open() {
         let tokens = lex("{>");
         assert_eq!(tokens[0].0, SyntaxKind::CommentLineOpen);
     }

     #[test]
     fn test_line_comment_close() {
         let tokens = lex("<}");
         assert_eq!(tokens[0].0, SyntaxKind::CommentLineClose);
     }

     #[test]
     fn test_block_comment_open() {
         let tokens = lex("{>>");
         assert_eq!(tokens[0].0, SyntaxKind::CommentBlockOpen);
     }

     #[test]
     fn test_block_comment_close() {
         let tokens = lex("<<}");
         assert_eq!(tokens[0].0, SyntaxKind::CommentBlockClose);
     }

     #[test]
     fn test_doc_comment_prefix() {
         let tokens = lex("///");
         assert_eq!(tokens[0].0, SyntaxKind::DocCommentPrefix);
     }

     #[test]
     fn test_jsdoc_open() {
         let tokens = lex("/**");
         assert_eq!(tokens[0].0, SyntaxKind::JsDocOpen);
     }

     #[test]
     fn test_jsdoc_close() {
         let tokens = lex("*/");
         assert_eq!(tokens[0].0, SyntaxKind::JsDocClose);
     }

     // ==================== TypeScript Keyword Tests ====================

     #[test]
     fn test_function_keyword() {
         let tokens = lex("function");
         assert_eq!(tokens[0].0, SyntaxKind::FunctionKw);
     }

     #[test]
     fn test_class_keyword() {
         let tokens = lex("class");
         assert_eq!(tokens[0].0, SyntaxKind::ClassKw);
     }

     #[test]
     fn test_interface_keyword() {
         let tokens = lex("interface");
         assert_eq!(tokens[0].0, SyntaxKind::InterfaceKw);
     }

     #[test]
     fn test_type_keyword() {
         let tokens = lex("type");
         assert_eq!(tokens[0].0, SyntaxKind::TypeKw);
     }

     #[test]
     fn test_const_keyword() {
         let tokens = lex("const");
         assert_eq!(tokens[0].0, SyntaxKind::ConstKw);
     }

     #[test]
     fn test_let_keyword() {
         let tokens = lex("let");
         assert_eq!(tokens[0].0, SyntaxKind::LetKw);
     }

     #[test]
     fn test_var_keyword() {
         let tokens = lex("var");
         assert_eq!(tokens[0].0, SyntaxKind::VarKw);
     }

     #[test]
     fn test_extends_keyword() {
         let tokens = lex("extends");
         assert_eq!(tokens[0].0, SyntaxKind::ExtendsKw);
     }

     #[test]
     fn test_implements_keyword() {
         let tokens = lex("implements");
         assert_eq!(tokens[0].0, SyntaxKind::ImplementsKw);
     }

     #[test]
     fn test_export_keyword() {
         let tokens = lex("export");
         assert_eq!(tokens[0].0, SyntaxKind::ExportKw);
     }

     #[test]
     fn test_async_keyword() {
         let tokens = lex("async");
         assert_eq!(tokens[0].0, SyntaxKind::AsyncKw);
     }

     #[test]
     fn test_static_keyword() {
         let tokens = lex("static");
         assert_eq!(tokens[0].0, SyntaxKind::StaticKw);
     }

     #[test]
     fn test_readonly_keyword() {
         let tokens = lex("readonly");
         assert_eq!(tokens[0].0, SyntaxKind::ReadonlyKw);
     }

     #[test]
     fn test_public_keyword() {
         let tokens = lex("public");
         assert_eq!(tokens[0].0, SyntaxKind::PublicKw);
     }

     #[test]
     fn test_private_keyword() {
         let tokens = lex("private");
         assert_eq!(tokens[0].0, SyntaxKind::PrivateKw);
     }

     #[test]
     fn test_protected_keyword() {
         let tokens = lex("protected");
         assert_eq!(tokens[0].0, SyntaxKind::ProtectedKw);
     }

     #[test]
     fn test_return_keyword() {
         let tokens = lex("return");
         assert_eq!(tokens[0].0, SyntaxKind::ReturnKw);
     }

     #[test]
     fn test_throw_keyword() {
         let tokens = lex("throw");
         assert_eq!(tokens[0].0, SyntaxKind::ThrowKw);
     }

     #[test]
     fn test_if_keyword() {
         let tokens = lex("if");
         assert_eq!(tokens[0].0, SyntaxKind::IfKw);
     }

     #[test]
     fn test_else_keyword() {
         // `else` is only recognized as ElseKw inside control blocks
         // Outside control blocks, it's just an Ident
         let tokens = lex("else");
         assert_eq!(tokens[0].0, SyntaxKind::Ident);

         // Inside a control block, else is recognized as ElseKw
         let tokens2 = lex("{:else}");
         assert_eq!(tokens2[0].0, SyntaxKind::ColonOpen);
         assert_eq!(tokens2[1].0, SyntaxKind::ElseKw);
     }

     #[test]
     fn test_for_keyword() {
         let tokens = lex("for");
         assert_eq!(tokens[0].0, SyntaxKind::ForKw);
     }

     #[test]
     fn test_while_keyword() {
         let tokens = lex("while");
         assert_eq!(tokens[0].0, SyntaxKind::WhileKw);
     }

     #[test]
     fn test_try_keyword() {
         let tokens = lex("try");
         assert_eq!(tokens[0].0, SyntaxKind::TryKw);
     }

     #[test]
     fn test_catch_keyword() {
         let tokens = lex("catch");
         assert_eq!(tokens[0].0, SyntaxKind::CatchKw);
     }

     #[test]
     fn test_finally_keyword() {
         let tokens = lex("finally");
         assert_eq!(tokens[0].0, SyntaxKind::FinallyKw);
     }

     #[test]
     fn test_new_keyword() {
         let tokens = lex("new");
         assert_eq!(tokens[0].0, SyntaxKind::NewKw);
     }

     #[test]
     fn test_get_keyword() {
         let tokens = lex("get");
         assert_eq!(tokens[0].0, SyntaxKind::GetKw);
     }

     #[test]
     fn test_set_keyword() {
         let tokens = lex("set");
         assert_eq!(tokens[0].0, SyntaxKind::SetKw);
     }

     #[test]
     fn test_satisfies_keyword() {
         let tokens = lex("satisfies");
         assert_eq!(tokens[0].0, SyntaxKind::SatisfiesKw);
     }

     #[test]
     fn test_declare_keyword() {
         let tokens = lex("declare");
         assert_eq!(tokens[0].0, SyntaxKind::DeclareKw);
     }

     // ==================== Normalization Tests ====================

     #[test]
     fn test_normalize_preserves_simple() {
         let normalized = normalize_template("hello world");
         assert_eq!(normalized, "hello world");
     }

     #[test]
     fn test_normalize_collapses_at_brace() {
         let normalized = normalize_template("@ { x }");
         assert_eq!(normalized, "@{ x }");
     }

     #[test]
     fn test_normalize_collapses_hash_open() {
         let normalized = normalize_template("{ # if cond }");
         assert_eq!(normalized, "{#if cond }");
     }

     #[test]
     fn test_normalize_collapses_slash_open() {
         let normalized = normalize_template("{ / if }");
         assert_eq!(normalized, "{/if }");
     }

     #[test]
     fn test_normalize_collapses_colon_open() {
         let normalized = normalize_template("{ : else }");
         assert_eq!(normalized, "{:else }");
     }

     #[test]
     fn test_normalize_collapses_dollar_open() {
         let normalized = normalize_template("{ $ let x = 1 }");
         assert_eq!(normalized, "{$let x = 1 }");
     }

     #[test]
     fn test_normalize_collapses_pipe_open() {
         let normalized = normalize_template("{ | name }");
         assert_eq!(normalized, "{|name }");
     }

     #[test]
     fn test_normalize_preserves_content() {
         let normalized = normalize_template("@{ foo . bar ( ) }");
         assert_eq!(normalized, "@{ foo . bar ( ) }");
     }

     // ==================== String Literal Tests ====================

     #[test]
     fn test_string_literal_mode() {
         let tokens = lex("\"hello world\"");
         // First token should be DoubleQuote
         assert_eq!(tokens[0].0, SyntaxKind::DoubleQuote);
         // Last token should also be DoubleQuote (closing)
         assert_eq!(tokens[tokens.len() - 1].0, SyntaxKind::DoubleQuote);
     }

     #[test]
     fn test_string_with_interpolation() {
         let tokens = lex("\"hello @{name}!\"");
         // Should have: DoubleQuote, text, At, RBrace, text, DoubleQuote
         assert_eq!(tokens[0].0, SyntaxKind::DoubleQuote);
         let has_at = tokens.iter().any(|(k, _)| *k == SyntaxKind::At);
         assert!(has_at);
     }

     // ==================== Template Literal Tests ====================

     #[test]
     fn test_template_literal_mode() {
         let tokens = lex("`hello world`");
         assert_eq!(tokens[0].0, SyntaxKind::Backtick);
         assert_eq!(tokens[tokens.len() - 1].0, SyntaxKind::Backtick);
     }

     #[test]
     fn test_template_with_interpolation() {
         let tokens = lex("`hello @{name}!`");
         assert_eq!(tokens[0].0, SyntaxKind::Backtick);
         let has_at = tokens.iter().any(|(k, _)| *k == SyntaxKind::At);
         assert!(has_at);
     }

     // ==================== Ident Block Tests ====================

     #[test]
     fn test_ident_block_simple() {
         let tokens = lex("{|name|}");
         assert_eq!(tokens[0].0, SyntaxKind::PipeOpen);
         let has_pipe_close = tokens.iter().any(|(k, _)| *k == SyntaxKind::PipeClose);
         assert!(has_pipe_close);
     }

     #[test]
     fn test_ident_block_with_interpolation() {
         let tokens = lex("{|prefix@{var}Suffix|}");
         assert_eq!(tokens[0].0, SyntaxKind::PipeOpen);
         let has_at = tokens.iter().any(|(k, _)| *k == SyntaxKind::At);
         assert!(has_at);
     }

     // ==================== Edge Cases ====================

     #[test]
     fn test_identifier_underscore_prefix() {
         let tokens = lex("_private");
         assert_eq!(tokens[0].0, SyntaxKind::Ident);
         assert_eq!(tokens[0].1, "_private");
     }

     #[test]
     fn test_identifier_with_numbers() {
         let tokens = lex("var123");
         assert_eq!(tokens[0].0, SyntaxKind::Ident);
         assert_eq!(tokens[0].1, "var123");
     }

     #[test]
     fn test_multiple_interpolations() {
         let tokens = lex("@{a}@{b}@{c}");
         let at_count = tokens.iter().filter(|(k, _)| *k == SyntaxKind::At).count();
         assert_eq!(at_count, 3);
     }

     #[test]
     fn test_nested_braces_in_interpolation() {
         let tokens = lex("@{vec![1, 2, 3]}");
         assert_eq!(tokens[0].0, SyntaxKind::At);
         // The content should be preserved
         assert!(tokens[1].1.contains("vec!"));
     }

     #[test]
     fn test_complex_typescript_code() {
         let tokens = lex("export class Foo extends Bar implements Baz {}");
         let kinds: Vec<_> = tokens.iter().map(|(k, _)| *k).collect();
         assert!(kinds.contains(&SyntaxKind::ExportKw));
         assert!(kinds.contains(&SyntaxKind::ClassKw));
         assert!(kinds.contains(&SyntaxKind::ExtendsKw));
         assert!(kinds.contains(&SyntaxKind::ImplementsKw));
     }

     // ==================== Rust Doc Attr Tests ====================

     #[test]
     fn test_rust_doc_attr() {
         let tokens = lex("#[doc = \"This is a doc\"]");
         assert_eq!(tokens[0].0, SyntaxKind::RustDocAttr);
         assert_eq!(tokens[0].1, "This is a doc");
     }

     #[test]
     fn test_rust_doc_attr_with_quotes() {
         let tokens = lex("#[doc = \"Test with \\\"quotes\\\"\"]");
         assert_eq!(tokens[0].0, SyntaxKind::RustDocAttr);
     }

     // ==================== Position Tracking Tests ====================

     #[test]
     fn test_token_start_positions() {
         let lexer = Lexer::new("ab cd");
         let tokens = lexer.tokenize();

         // First token "ab" starts at 0
         assert_eq!(tokens[0].start, 0);
         assert_eq!(tokens[0].text, "ab");

         // Find the "cd" token and check its position
         let cd_token = tokens.iter().find(|t| t.text == "cd");
         assert!(cd_token.is_some());
         // Position should be after "ab" and whitespace
         assert!(cd_token.unwrap().start > 2);
     }
