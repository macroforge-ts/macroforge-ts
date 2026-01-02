
	use super::*;

	fn lex(input: &str) -> Vec<(SyntaxKind, String)> {
		let tokens = Lexer::new(input).tokenize().expect("lexer should not fail on test input");
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
		// {#if is now a single BraceHashIf token
		assert_eq!(tokens[0].0, SyntaxKind::BraceHashIf);
		// The next non-whitespace token should be an Ident for "cond"
		let cond_token = tokens.iter().find(|(k, t)| *k == SyntaxKind::Ident && t == "cond");
		assert!(cond_token.is_some());
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
		// Should have: BraceHashFor, WHITESPACE, IDENT(item), WHITESPACE, IN_KW, WHITESPACE, IDENT(items), RBRACE
		assert_eq!(tokens[0].0, SyntaxKind::BraceHashFor);
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
	fn test_brace_hash_if() {
		// {#if without a recognized keyword after just returns separate tokens
		let tokens = lex("{#if");
		assert_eq!(tokens[0].0, SyntaxKind::BraceHashIf);
	}

	#[test]
	fn test_brace_slash_if() {
		// {/if} is now a single BraceSlashIfBrace token (includes closing brace)
		let tokens = lex("{/if}");
		assert_eq!(tokens[0].0, SyntaxKind::BraceSlashIfBrace);
		assert_eq!(tokens.len(), 1);
	}

	#[test]
	fn test_brace_colon_else() {
		// {:else} is now a single BraceColonElseBrace token (includes closing brace)
		let tokens = lex("{:else}");
		assert_eq!(tokens[0].0, SyntaxKind::BraceColonElseBrace);
		assert_eq!(tokens.len(), 1);
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
		// {#if is now a single BraceHashIf token
		assert_eq!(tokens[0].0, SyntaxKind::BraceHashIf);
		// Condition follows
		let condition_token = tokens.iter().find(|(k, t)| *k == SyntaxKind::Ident && t == "condition");
		assert!(condition_token.is_some());
	}

	#[test]
	fn test_else_control_block() {
		let tokens = lex("{:else}");
		// {:else} is now a single BraceColonElseBrace token (includes closing brace)
		assert_eq!(tokens[0].0, SyntaxKind::BraceColonElseBrace);
		assert_eq!(tokens.len(), 1);
	}

	#[test]
	fn test_else_if_control_block() {
		let tokens = lex("{:else if cond}");
		// {:else if is now a single BraceColonElseIf token
		assert_eq!(tokens[0].0, SyntaxKind::BraceColonElseIf);
		// Condition follows
		let cond_token = tokens.iter().find(|(k, t)| *k == SyntaxKind::Ident && t == "cond");
		assert!(cond_token.is_some());
	}

	#[test]
	fn test_for_control_block() {
		let tokens = lex("{#for x in xs}");
		// {#for is now a single BraceHashFor token
		assert_eq!(tokens[0].0, SyntaxKind::BraceHashFor);
		// Variable and iterator follow
		let x_token = tokens.iter().find(|(k, t)| *k == SyntaxKind::Ident && t == "x");
		assert!(x_token.is_some());
	}

	#[test]
	fn test_while_control_block() {
		let tokens = lex("{#while cond}");
		// {#while is now a single BraceHashWhile token
		assert_eq!(tokens[0].0, SyntaxKind::BraceHashWhile);
		// Condition follows
		let cond_token = tokens.iter().find(|(k, t)| *k == SyntaxKind::Ident && t == "cond");
		assert!(cond_token.is_some());
	}

	#[test]
	fn test_match_control_block() {
		let tokens = lex("{#match expr}");
		// {#match is now a single BraceHashMatch token
		assert_eq!(tokens[0].0, SyntaxKind::BraceHashMatch);
		// Expression follows
		let expr_token = tokens.iter().find(|(k, t)| *k == SyntaxKind::Ident && t == "expr");
		assert!(expr_token.is_some());
	}

	#[test]
	fn test_case_control_block() {
		let tokens = lex("{:case Some(x)}");
		// {:case is now a single BraceColonCase token
		assert_eq!(tokens[0].0, SyntaxKind::BraceColonCase);
		// Pattern follows
		let some_token = tokens.iter().find(|(k, t)| *k == SyntaxKind::Ident && t == "Some");
		assert!(some_token.is_some());
	}

	#[test]
	fn test_end_if_block() {
		let tokens = lex("{/if}");
		// {/if} is now a single BraceSlashIfBrace token (includes closing brace)
		assert_eq!(tokens[0].0, SyntaxKind::BraceSlashIfBrace);
		assert_eq!(tokens.len(), 1);
	}

	#[test]
	fn test_end_for_block() {
		let tokens = lex("{/for}");
		// {/for} is now a single BraceSlashForBrace token (includes closing brace)
		assert_eq!(tokens[0].0, SyntaxKind::BraceSlashForBrace);
		assert_eq!(tokens.len(), 1);
	}

	#[test]
	fn test_end_while_block() {
		let tokens = lex("{/while}");
		// {/while} is now a single BraceSlashWhileBrace token (includes closing brace)
		assert_eq!(tokens[0].0, SyntaxKind::BraceSlashWhileBrace);
		assert_eq!(tokens.len(), 1);
	}

	#[test]
	fn test_end_match_block() {
		let tokens = lex("{/match}");
		// {/match} is now a single BraceSlashMatchBrace token (includes closing brace)
		assert_eq!(tokens[0].0, SyntaxKind::BraceSlashMatchBrace);
		assert_eq!(tokens.len(), 1);
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
		// Test complete JSDoc comment - */ only appears after /** in JsDoc mode
		let tokens = lex("/** doc */");
		assert_eq!(tokens[0].0, SyntaxKind::JsDocOpen);
		assert_eq!(tokens[1].0, SyntaxKind::Text);
		assert_eq!(tokens[1].1, " doc ");
		assert_eq!(tokens[2].0, SyntaxKind::JsDocClose);
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
		// `else` is a JavaScript keyword, recognized as ElseKw
		let tokens = lex("else");
		assert_eq!(tokens[0].0, SyntaxKind::ElseKw);

		// Template {:else} returns complete BraceColonElseBrace token
		let tokens2 = lex("{:else}");
		assert_eq!(tokens2[0].0, SyntaxKind::BraceColonElseBrace);
		assert_eq!(tokens2.len(), 1);
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
	fn test_normalize_collapses_slash_close() {
		// Complete closing tags like { / if } become {/if}
		let normalized = normalize_template("{ / if }");
		assert_eq!(normalized, "{/if}");

		// All closing keywords work
		assert_eq!(normalize_template("{ / for }"), "{/for}");
		assert_eq!(normalize_template("{ / while }"), "{/while}");
		assert_eq!(normalize_template("{ / match }"), "{/match}");
	}

	#[test]
	fn test_normalize_collapses_colon_open() {
		// {:else} is now fully normalized (including the closing brace) so it matches the lexer's expected pattern
		let normalized = normalize_template("{ : else }");
		assert_eq!(normalized, "{:else}");
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
	fn test_debug_control_flow_in_function() {
		// This is the input after TokenStream stringification
		let input = "function test () { { # if true } console . log (\"hi\") ; { / if } }";
		let tokens = lex(input);
		eprintln!("Tokens for control flow in function:");
		for (i, (kind, text)) in tokens.iter().enumerate() {
			eprintln!("  {}: {:?} = {:?}", i, kind, text);
		}
		// Check that we get BraceHashIf
		let has_brace_hash_if = tokens.iter().any(|(k, _)| *k == SyntaxKind::BraceHashIf);
		assert!(has_brace_hash_if, "Expected BraceHashIf token");
	}

	#[test]
	fn test_token_start_positions() {
		let lexer = Lexer::new("ab cd");
		let tokens = lexer.tokenize().expect("lexer should not fail");

		// First token "ab" starts at 0
		assert_eq!(tokens[0].start, 0);
		assert_eq!(tokens[0].text, "ab");

		// Find the "cd" token and check its position
		let cd_token = tokens.iter().find(|t| t.text == "cd");
		assert!(cd_token.is_some());
		// Position should be after "ab" and whitespace
		assert!(cd_token.unwrap().start > 2);
	}

	#[test]
	fn test_if_expression_with_else_tokens() {
		// This is the exact input that fails in test_if_expression_in_statement
		let input = r#"const status = {#if cond} "a" {:else} "b" {/if}"#;
		let tokens = lex(input);

		// Debug output
		eprintln!("Tokens for if-expression:");
		for (i, (kind, text)) in tokens.iter().enumerate() {
			eprintln!("  {:3}: {:?} = {:?}", i, kind, text);
		}

		// Check all expected tokens are present
		assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::BraceHashIf),
			"Expected BraceHashIf token");
		assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::BraceColonElseBrace),
			"Expected BraceColonElseBrace token. Tokens: {:?}", tokens);
		assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::BraceSlashIfBrace),
			"Expected BraceSlashIfBrace token");
	}

	// ==================== TypeScript Comment Tests ====================

	#[test]
	fn test_ts_line_comment() {
		let tokens = lex("// this is a comment\nconst x = 1");
		// First token should be TsLineComment
		assert_eq!(tokens[0].0, SyntaxKind::TsLineComment);
		assert_eq!(tokens[0].1, "// this is a comment");
		// Should have newline (whitespace) after comment
		assert_eq!(tokens[1].0, SyntaxKind::Whitespace);
		// Then const keyword
		assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::ConstKw));
	}

	#[test]
	fn test_ts_line_comment_at_eof() {
		let tokens = lex("const x = 1 // comment at end");
		// Last token should be TsLineComment
		let last = tokens.last().unwrap();
		assert_eq!(last.0, SyntaxKind::TsLineComment);
		assert_eq!(last.1, "// comment at end");
	}

	#[test]
	fn test_ts_line_comment_vs_doc_comment() {
		// /// is a doc comment prefix, not a line comment
		let tokens = lex("/// doc comment");
		assert_eq!(tokens[0].0, SyntaxKind::DocCommentPrefix);

		// // is a regular line comment
		let tokens = lex("// regular comment");
		assert_eq!(tokens[0].0, SyntaxKind::TsLineComment);
	}

	#[test]
	fn test_ts_block_comment() {
		let tokens = lex("/* block comment */ const x = 1");
		// First token should be TsBlockComment
		assert_eq!(tokens[0].0, SyntaxKind::TsBlockComment);
		assert_eq!(tokens[0].1, "/* block comment */");
		// Should have whitespace after comment
		assert_eq!(tokens[1].0, SyntaxKind::Whitespace);
		// Then const keyword
		assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::ConstKw));
	}

	#[test]
	fn test_ts_block_comment_multiline() {
		let tokens = lex("/* multi\nline\ncomment */ x");
		assert_eq!(tokens[0].0, SyntaxKind::TsBlockComment);
		assert_eq!(tokens[0].1, "/* multi\nline\ncomment */");
	}

	#[test]
	fn test_ts_block_comment_vs_jsdoc() {
		// /** is JSDoc, not a block comment
		let tokens = lex("/** jsdoc */");
		assert_eq!(tokens[0].0, SyntaxKind::JsDocOpen);

		// /* is a regular block comment
		let tokens = lex("/* comment */");
		assert_eq!(tokens[0].0, SyntaxKind::TsBlockComment);
	}

	#[test]
	fn test_jsdoc_with_dots_and_at_symbols() {
		// JSDoc with dots and @ symbols should be collected as text
		let tokens = lex("/** @param value - The value. @returns string */");
		assert_eq!(tokens[0].0, SyntaxKind::JsDocOpen);
		assert_eq!(tokens[1].0, SyntaxKind::Text);
		// The dot should be inside the text, not a separate Dot token
		assert!(tokens[1].1.contains("."));
		assert!(tokens[1].1.contains("@param"));
		assert_eq!(tokens[2].0, SyntaxKind::JsDocClose);
		// Should be exactly 3 tokens: JsDocOpen, Text, JsDocClose
		assert_eq!(tokens.len(), 3);
	}

	#[test]
	fn test_ts_block_comment_unterminated() {
		// Unterminated block comment should consume to EOF
		let tokens = lex("/* unterminated");
		assert_eq!(tokens[0].0, SyntaxKind::TsBlockComment);
		assert_eq!(tokens[0].1, "/* unterminated");
	}

	#[test]
	fn test_ts_comments_with_interpolation() {
		// Comments before interpolation
		let tokens = lex("// comment\n@{expr}");
		assert_eq!(tokens[0].0, SyntaxKind::TsLineComment);
		assert!(tokens.iter().any(|(k, _)| *k == SyntaxKind::At));
	}

	#[test]
	fn test_ts_line_comment_is_trivia() {
		assert!(SyntaxKind::TsLineComment.is_trivia());
	}

	#[test]
	fn test_ts_block_comment_is_trivia() {
		assert!(SyntaxKind::TsBlockComment.is_trivia());
	}
