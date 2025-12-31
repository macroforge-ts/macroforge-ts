
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
