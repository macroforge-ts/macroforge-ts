use super::*;
use ::quote::quote;

/// Test the actual token stream structure from @{...}
#[test]
fn test_token_stream_structure() {
    let tokens: TokenStream2 = quote! { @{name} };
    let tokens_vec: Vec<TokenTree> = tokens.into_iter().collect();

    eprintln!("Token count: {}", tokens_vec.len());
    for (i, tt) in tokens_vec.iter().enumerate() {
        match tt {
            TokenTree::Punct(p) => {
                eprintln!(
                    "  Token {}: Punct('{}', spacing={:?})",
                    i,
                    p.as_char(),
                    p.spacing()
                );
            }
            TokenTree::Group(g) => {
                eprintln!(
                    "  Token {}: Group(delimiter={:?}, content={:?})",
                    i,
                    g.delimiter(),
                    g.stream().to_string()
                );
            }
            TokenTree::Ident(id) => {
                eprintln!("  Token {}: Ident({})", i, id);
            }
            TokenTree::Literal(lit) => {
                eprintln!("  Token {}: Literal({})", i, lit);
            }
        }
    }

    // Should have 2 tokens: @ (Punct) and {name} (Group with Brace delimiter)
    assert_eq!(tokens_vec.len(), 2, "Should have exactly 2 tokens");
    assert!(
        matches!(&tokens_vec[0], TokenTree::Punct(p) if p.as_char() == '@'),
        "First token should be @"
    );
    assert!(
        matches!(&tokens_vec[1], TokenTree::Group(g) if g.delimiter() == Delimiter::Brace),
        "Second token should be brace group"
    );
}

/// Test the is_group check in isolation
#[test]
fn test_is_group_check() {
    let tokens: TokenStream2 = quote! { @{name} };
    let mut iter = tokens.into_iter().peekable();

    // First token should be @
    let first = iter.next();
    eprintln!("First token: {:?}", first);
    assert!(matches!(first, Some(TokenTree::Punct(p)) if p.as_char() == '@'));

    // After consuming @, peek should show the brace group
    let peeked = iter.peek();
    eprintln!("Peeked after @: {:?}", peeked);

    // Test the is_group check - same code as in parse_segments
    let is_group = iter
        .peek()
        .is_some_and(|t| matches!(t, TokenTree::Group(g) if g.delimiter() == Delimiter::Brace));
    eprintln!("is_group result: {}", is_group);
    assert!(is_group, "is_group should be true");
}

/// Test that parse_segments correctly identifies @{...} as interpolation
#[test]
fn test_parse_at_brace_interpolation() {
    // Create tokens: foo @{bar} baz
    let tokens: TokenStream2 = quote! { foo @{bar} baz };
    let mut ids = IdGen::new();
    let (segments, _) = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
        .expect("parse_segments should succeed");

    // Debug output
    eprintln!("Number of segments: {}", segments.len());
    for (i, seg) in segments.iter().enumerate() {
        match seg {
            Segment::Static(s) => eprintln!("  Segment {}: Static({:?})", i, s),
            Segment::Interpolation { id, .. } => {
                eprintln!("  Segment {}: Interpolation(id={})", i, id)
            }
            Segment::Control { id, .. } => eprintln!("  Segment {}: Control(id={})", i, id),
            _ => eprintln!("  Segment {}: Other", i),
        }
    }

    // We expect: Static("foo "), Interpolation, Static(" baz")
    let has_interpolation = segments
        .iter()
        .any(|s| matches!(s, Segment::Interpolation { .. }));
    assert!(
        has_interpolation,
        "Should have at least one Interpolation segment"
    );

    // Check that no Static segment contains '@'
    for seg in &segments {
        if let Segment::Static(s) = seg {
            assert!(
                !s.contains('@'),
                "Static segment should not contain '@': {:?}",
                s
            );
        }
    }
}

/// Test multiple @{...} interpolations
#[test]
fn test_multiple_interpolations() {
    let tokens: TokenStream2 = quote! { foo @{a} bar @{b} baz };
    let mut ids = IdGen::new();
    let (segments, _) = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
        .expect("parse_segments should succeed");

    eprintln!("Number of segments: {}", segments.len());
    for (i, seg) in segments.iter().enumerate() {
        match seg {
            Segment::Static(s) => eprintln!("  Segment {}: Static({:?})", i, s),
            Segment::Interpolation { id, .. } => {
                eprintln!("  Segment {}: Interpolation(id={})", i, id)
            }
            _ => eprintln!("  Segment {}: Other", i),
        }
    }

    let interp_count = segments
        .iter()
        .filter(|s| matches!(s, Segment::Interpolation { .. }))
        .count();
    assert_eq!(interp_count, 2, "Should have exactly 2 interpolations");

    // Check no @ in static segments
    for seg in &segments {
        if let Segment::Static(s) = seg {
            assert!(
                !s.contains('@'),
                "Static segment should not contain '@': {:?}",
                s
            );
        }
    }
}

/// Test tokens_to_ts_string doesn't inject @
#[test]
fn test_tokens_to_ts_string_no_at() {
    let tokens: TokenStream2 = quote! { foo bar };
    let result = tokens_to_ts_string(tokens);
    eprintln!("tokens_to_ts_string result: {:?}", result);
    assert!(!result.contains('@'), "Should not contain @");
}

/// Test using from_str (like the actual macro does)
#[test]
fn test_from_str_tokenization() {
    use std::str::FromStr;

    let tokens = TokenStream2::from_str("@{name}").unwrap();
    let tokens_vec: Vec<TokenTree> = tokens.into_iter().collect();

    eprintln!("from_str Token count: {}", tokens_vec.len());
    for (i, tt) in tokens_vec.iter().enumerate() {
        match tt {
            TokenTree::Punct(p) => {
                eprintln!(
                    "  Token {}: Punct('{}', spacing={:?})",
                    i,
                    p.as_char(),
                    p.spacing()
                );
            }
            TokenTree::Group(g) => {
                eprintln!(
                    "  Token {}: Group(delimiter={:?}, content={:?})",
                    i,
                    g.delimiter(),
                    g.stream().to_string()
                );
            }
            TokenTree::Ident(id) => {
                eprintln!("  Token {}: Ident({})", i, id);
            }
            TokenTree::Literal(lit) => {
                eprintln!("  Token {}: Literal({})", i, lit);
            }
        }
    }

    // Should have 2 tokens like quote! does
    assert_eq!(tokens_vec.len(), 2, "Should have exactly 2 tokens");
}

/// Test parse_segments with from_str input
#[test]
fn test_parse_segments_from_str() {
    use std::str::FromStr;

    let tokens = TokenStream2::from_str("foo @{bar} baz").unwrap();
    let mut ids = IdGen::new();
    let (segments, _) = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
        .expect("parse_segments should succeed");

    eprintln!("from_str segments count: {}", segments.len());
    for (i, seg) in segments.iter().enumerate() {
        match seg {
            Segment::Static(s) => eprintln!("  Segment {}: Static({:?})", i, s),
            Segment::Interpolation { id, .. } => {
                eprintln!("  Segment {}: Interpolation(id={})", i, id)
            }
            _ => eprintln!("  Segment {}: Other", i),
        }
    }

    // Check that no Static segment contains '@'
    for seg in &segments {
        if let Segment::Static(s) = seg {
            assert!(
                !s.contains('@'),
                "Static segment should not contain '@': {:?}",
                s
            );
        }
    }
}

/// Test build_template_and_bindings produces correct template string
#[test]
fn test_build_template_and_bindings() {
    use std::str::FromStr;

    let tokens = TokenStream2::from_str("foo @{bar} baz").unwrap();
    let mut ids = IdGen::new();
    let (segments, _) = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
        .expect("parse_segments should succeed");

    // Create an empty context map (no special placeholder classification)
    let context_map = HashMap::new();

    let template_result = build_template_and_bindings(segments.iter(), &context_map)
        .expect("build_template_and_bindings should succeed");

    eprintln!("Template string: {:?}", template_result.template);
    eprintln!("Bindings count: {}", template_result.bindings.len());

    // Template should NOT contain '@'
    assert!(
        !template_result.template.contains('@'),
        "Template should not contain '@': {:?}",
        template_result.template
    );

    // Template should contain placeholder
    assert!(
        template_result.template.contains("__mf_hole_"),
        "Template should contain placeholder: {:?}",
        template_result.template
    );
}

/// Test the full template string for the derive_clone pattern
#[test]
fn test_derive_clone_full_template() {
    use std::str::FromStr;

    let code = r#"export function @{fn_name}(value: @{class_name}): @{class_name} {
            const cloned = Object.create(Object.getPrototypeOf(value));
            return cloned;
        }"#;

    let tokens = TokenStream2::from_str(code).unwrap();
    let mut ids = IdGen::new();
    let (segments, _) = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
        .expect("parse_segments should succeed");

    let context_map = classify_placeholders_module(&segments)
        .expect("classify_placeholders_module should succeed");

    let template_result = build_template_and_bindings(segments.iter(), &context_map)
        .expect("build_template_and_bindings should succeed");

    eprintln!("Full template string:\n{}", template_result.template);
    eprintln!("\nBindings count: {}", template_result.bindings.len());

    // Template should NOT contain '@'
    assert!(
        !template_result.template.contains('@'),
        "Template should not contain '@'"
    );
}

/// Test classify_placeholders_module
#[test]
fn test_classify_placeholders() {
    use std::str::FromStr;

    let tokens = TokenStream2::from_str("const x = @{bar};").unwrap();
    let mut ids = IdGen::new();
    let (segments, _) = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
        .expect("parse_segments should succeed");

    eprintln!("Segments for classification:");
    for (i, seg) in segments.iter().enumerate() {
        match seg {
            Segment::Static(s) => eprintln!("  {}: Static({:?})", i, s),
            Segment::Interpolation { id, .. } => eprintln!("  {}: Interpolation(id={})", i, id),
            _ => eprintln!("  {}: Other", i),
        }
    }

    let context_map = classify_placeholders_module(&segments)
        .expect("classify_placeholders_module should succeed");

    eprintln!("Context map: {:?}", context_map);
}

/// Test full parse_template flow
#[test]
fn test_full_parse_template() {
    use std::str::FromStr;

    let tokens = TokenStream2::from_str("const x = @{bar};").unwrap();
    let result = parse_template(tokens);

    match result {
        Ok(output) => {
            let s = output.to_string();
            eprintln!(
                "parse_template output (first 500 chars): {}",
                &s[..s.len().min(500)]
            );
            // If it succeeds, the template was parsed correctly
        }
        Err(e) => {
            eprintln!("parse_template error: {}", e);
            // This is the error we're trying to fix
            panic!("parse_template failed: {}", e);
        }
    }
}

/// Test case matching the actual failing code from derive_clone.rs
#[test]
fn test_derive_clone_like_template() {
    // Match the exact pattern from derive_clone.rs
    let tokens: TokenStream2 = quote! {
        export function @{fn_name}(value: @{class_name}): @{class_name} {
            const cloned = Object.create(Object.getPrototypeOf(value));
            return cloned;
        }
    };

    let mut ids = IdGen::new();
    let (segments, _) = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
        .expect("parse_segments should succeed");

    eprintln!("derive_clone-like segments:");
    for (i, seg) in segments.iter().enumerate() {
        match seg {
            Segment::Static(s) => {
                eprintln!("  {}: Static({:?})", i, s);
                if s.contains('@') {
                    eprintln!("    ^^^ CONTAINS @ ^^^");
                }
            }
            Segment::Interpolation { id, .. } => eprintln!("  {}: Interpolation(id={})", i, id),
            Segment::Control { id, .. } => eprintln!("  {}: Control(id={})", i, id),
            _ => eprintln!("  {}: Other", i),
        }
    }

    // Should have 3 interpolations
    let interp_count = segments
        .iter()
        .filter(|s| matches!(s, Segment::Interpolation { .. }))
        .count();
    assert_eq!(
        interp_count, 3,
        "Should have exactly 3 interpolations for @{{fn_name}}, @{{class_name}}, @{{class_name}}"
    );

    // No Static segment should contain '@'
    for seg in &segments {
        if let Segment::Static(s) = seg {
            assert!(!s.contains('@'), "Static segment contains '@': {:?}", s);
        }
    }
}

/// Test with from_str matching derive_clone
#[test]
fn test_derive_clone_like_from_str() {
    use std::str::FromStr;

    let code = r#"export function @{fn_name}(value: @{class_name}): @{class_name} {
            const cloned = Object.create(Object.getPrototypeOf(value));
            return cloned;
        }"#;

    let tokens = TokenStream2::from_str(code).unwrap();

    let mut ids = IdGen::new();
    let (segments, _) = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
        .expect("parse_segments should succeed");

    eprintln!("from_str derive_clone-like segments:");
    for (i, seg) in segments.iter().enumerate() {
        match seg {
            Segment::Static(s) => {
                eprintln!("  {}: Static({:?})", i, s);
                if s.contains('@') {
                    eprintln!("    ^^^ CONTAINS @ ^^^");
                }
            }
            Segment::Interpolation { id, .. } => eprintln!("  {}: Interpolation(id={})", i, id),
            Segment::Control { id, .. } => eprintln!("  {}: Control(id={})", i, id),
            _ => eprintln!("  {}: Other", i),
        }
    }

    // Should have 3 interpolations
    let interp_count = segments
        .iter()
        .filter(|s| matches!(s, Segment::Interpolation { .. }))
        .count();
    assert_eq!(interp_count, 3, "Should have exactly 3 interpolations");
}

/// Test derive_clone with control flow - matches actual macro usage
#[test]
fn test_derive_clone_with_control_flow() {
    use std::str::FromStr;

    // This is the EXACT template from derive_clone.rs line 110-122
    let code = r#"export function @{fn_name}(value: @{class_name}): @{class_name} {
            const cloned = Object.create(Object.getPrototypeOf(value));

            {#if has_fields}
                {#for field in field_names}
                    cloned.@{field} = value.@{field};
                {/for}
            {/if}

            return cloned;
        }"#;

    eprintln!("\n=== Testing derive_clone with control flow ===");
    eprintln!("Input code:\n{}", code);

    let tokens = TokenStream2::from_str(code).unwrap();

    let mut ids = IdGen::new();
    let result = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false);

    match result {
        Ok((segments, _)) => {
            eprintln!("\nParsed segments:");
            for (i, seg) in segments.iter().enumerate() {
                match seg {
                    Segment::Static(s) => {
                        eprintln!("  {}: Static({:?})", i, s);
                        if s.contains('@') {
                            eprintln!("    ^^^ CONTAINS @ - BUG! ^^^");
                        }
                        if s.contains('#') {
                            eprintln!("    ^^^ CONTAINS # - control flow not parsed! ^^^");
                        }
                    }
                    Segment::Interpolation { id, expr, .. } => {
                        eprintln!("  {}: Interpolation(id={}, expr={})", i, id, expr);
                    }
                    Segment::Control { id, node, .. } => {
                        eprintln!(
                            "  {}: Control(id={}, node={:?})",
                            i,
                            id,
                            std::mem::discriminant(node)
                        );
                    }
                    _ => eprintln!("  {}: {:?}", i, seg),
                }
            }

            // Build the template string
            let context_map = HashMap::new();
            let template_result = build_template_and_bindings(segments.iter(), &context_map);

            match template_result {
                Ok(result) => {
                    eprintln!("\nFull template string:\n{}", result.template);
                    eprintln!("\nBindings: {}", result.bindings.len());
                    for b in &result.bindings {
                        eprintln!("  {} : {:?}", b.name, b.ty);
                    }
                }
                Err(e) => {
                    eprintln!("\nFailed to build template: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("\nFailed to parse segments: {}", e);
        }
    }
}

/// Debug: trace through "value: @{class_name}" token by token
#[test]
fn test_debug_colon_at_sequence() {
    use std::str::FromStr;

    // The parentheses case - this might be the issue!
    let code = "(value: @{class_name})";
    let tokens = TokenStream2::from_str(code).unwrap();
    let tokens_vec: Vec<TokenTree> = tokens.into_iter().collect();

    eprintln!("Tokens for 'value: @{{class_name}}':");
    for (i, tt) in tokens_vec.iter().enumerate() {
        match tt {
            TokenTree::Punct(p) => {
                eprintln!(
                    "  {}: Punct('{}', spacing={:?})",
                    i,
                    p.as_char(),
                    p.spacing()
                );
            }
            TokenTree::Group(g) => {
                eprintln!(
                    "  {}: Group(delimiter={:?}, content={:?})",
                    i,
                    g.delimiter(),
                    g.stream().to_string()
                );
            }
            TokenTree::Ident(id) => {
                eprintln!("  {}: Ident({})", i, id);
            }
            TokenTree::Literal(lit) => {
                eprintln!("  {}: Literal({})", i, lit);
            }
        }
    }

    // Now simulate what parse_segments does
    let tokens = TokenStream2::from_str(code).unwrap();
    let mut iter = tokens.into_iter().peekable();

    eprintln!("\nSimulating parse_segments:");
    while let Some(token) = iter.peek().cloned() {
        match &token {
            TokenTree::Punct(p) if p.as_char() == '@' => {
                eprintln!("  Found @, consuming it...");
                iter.next();
                eprintln!("  After consuming @, peeking next token...");
                let peeked = iter.peek();
                eprintln!("  Peeked: {:?}", peeked);

                let is_group = iter.peek().is_some_and(|t| {
                    let result =
                        matches!(t, TokenTree::Group(g) if g.delimiter() == Delimiter::Brace);
                    eprintln!("  is_group check result: {}", result);
                    result
                });

                if is_group {
                    eprintln!("  -> IS a brace group, consuming it");
                    iter.next();
                } else {
                    eprintln!("  -> NOT a brace group!");
                }
            }
            _ => {
                eprintln!("  Token: {:?}, consuming...", token);
                iter.next();
            }
        }
    }
}
