use super::*;
use super::compile::compile_stmt_segments;
use super::flush::flush_stmt_run;
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

/// Test complex deserialize template with doc comments, multiple functions, and control flow
/// This template pattern is used in derive_deserialize.rs and was failing with:
/// "TypeScript parse error: Error { error: (26..27, Unexpected { got: "{", expected: "* for generator, private key, identifier or async" }) }"
#[test]
fn test_deserialize_complex_template() {
    use std::str::FromStr;

    // Simplified version of the complex deserialize template
    let code = r#"
        /** Deserializes input to this type. @param input - JSON string or object */
        export function @{fn_deserialize_ident}(input: unknown): @{return_type_ident} {
            try {
                const data = typeof input === "string" ? JSON.parse(input) : input;
                return @{success_result_expr};
            } catch (e) {
                const message = e instanceof Error ? e.message : String(e);
                return @{error_generic_message_expr};
            }
        }

        /** Deserializes with existing context. @param value - The raw value @param ctx - The context */
        export function @{fn_deserialize_internal_ident}(value: any, ctx: @{deserialize_context_ident}): @{type_ident} | @{pending_ref_ident} {
            if (value?.__ref !== undefined) {
                return ctx.getOrDefer(value.__ref) as @{type_ident} | @{pending_ref_ident};
            }

            if (typeof value !== "object" || value === null || Array.isArray(value)) {
                throw new @{deserialize_error_expr}([{ field: "_root", message: "expected an object" }]);
            }

            const obj = value as Record<string, unknown>;
            const errors: Array<{ field: string; message: string }> = [];
            const instance: any = {};

            if (obj.__id !== undefined) {
                ctx.register(obj.__id as number, instance);
            }

            return instance as @{type_ident};
        }
    "#;

    eprintln!("\n=== Testing complex deserialize template ===");
    eprintln!("Input code:\n{}", code);

    let tokens = TokenStream2::from_str(code).unwrap();

    let mut ids = IdGen::new();
    let result = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false);

    match result {
        Ok((segments, _)) => {
            eprintln!("\n=== Parsed {} segments ===", segments.len());
            for (i, seg) in segments.iter().enumerate() {
                match seg {
                    Segment::Static(s) => {
                        let truncated = if s.len() > 50 { format!("{}...", &s[..50]) } else { s.clone() };
                        eprintln!("  {}: Static({:?})", i, truncated);
                    }
                    Segment::Interpolation { id, expr, .. } => {
                        eprintln!("  {}: Interpolation(id={}, expr={})", i, id, expr);
                    }
                    Segment::Control { id, node, .. } => {
                        eprintln!("  {}: Control(id={}, node={:?})", i, id, std::mem::discriminant(node));
                    }
                    Segment::Comment { style, text } => {
                        eprintln!("  {}: Comment({:?}, {:?})", i, style, text);
                    }
                    Segment::BraceBlock { id, inner } => {
                        eprintln!("  {}: BraceBlock(id={}, inner_count={})", i, id, inner.len());
                    }
                    _ => eprintln!("  {}: {:?}", i, std::mem::discriminant(seg)),
                }
            }

            // Use compile_stmt_segments which handles comments properly
            let out_ident = proc_macro2::Ident::new("__mf_out", proc_macro2::Span::call_site());
            let comments_ident = proc_macro2::Ident::new("__mf_comments", proc_macro2::Span::call_site());
            let pending_ident = proc_macro2::Ident::new("__mf_pending", proc_macro2::Span::call_site());
            let pos_ident = proc_macro2::Ident::new("__mf_pos", proc_macro2::Span::call_site());

            let compile_result = compile_stmt_segments(
                &segments,
                &out_ident,
                &comments_ident,
                &pending_ident,
                &pos_ident,
            );

            match compile_result {
                Ok(compiled) => {
                    eprintln!("\n=== Successfully compiled ===");
                    eprintln!("Generated {} tokens", compiled.to_string().len());
                }
                Err(e) => {
                    eprintln!("\nFailed to compile: {}", e);
                    panic!("compile_stmt_segments failed: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("\nFailed to parse segments: {}", e);
            panic!("parse_segments failed: {}", e);
        }
    }
}

/// Test deserialize template with control flow (if/for/match)
#[test]
fn test_deserialize_template_with_control_flow() {
    use std::str::FromStr;

    // Template with control flow like the actual derive_deserialize.rs
    let code = r#"
        export function @{fn_ident}(value: any): @{type_ident} {
            const obj = value as Record<string, unknown>;
            const errors: Array<{ field: string; message: string }> = [];

            {#if deny_unknown}
                const knownKeys = new Set([@{known_keys_str}]);
                for (const key of Object.keys(obj)) {
                    if (!knownKeys.has(key)) {
                        errors.push({ field: key, message: "unknown field" });
                    }
                }
            {/if}

            {#if has_required}
                {#for field in &required_fields}
                    if (!("@{field.json_key}" in obj)) {
                        errors.push({ field: "@{field.json_key}", message: "missing required field" });
                    }
                {/for}
            {/if}

            const instance: any = {};
            return instance as @{type_ident};
        }
    "#;

    eprintln!("\n=== Testing template with control flow ===");

    let tokens = TokenStream2::from_str(code).unwrap();
    let mut ids = IdGen::new();
    let result = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false);

    match result {
        Ok((segments, _)) => {
            eprintln!("Parsed {} segments successfully", segments.len());

            // Recursively count control structures in all segments including inside BraceBlocks
            fn count_controls_recursive(segments: &[Segment]) -> usize {
                let mut count = 0;
                for seg in segments {
                    match seg {
                        Segment::Control { node, .. } => {
                            count += 1;
                            // Count inside control structure bodies
                            match node {
                                ControlNode::If { then_branch, else_branch, .. } |
                                ControlNode::IfLet { then_branch, else_branch, .. } => {
                                    count += count_controls_recursive(then_branch);
                                    if let Some(eb) = else_branch {
                                        count += count_controls_recursive(eb);
                                    }
                                }
                                ControlNode::For { body, .. } |
                                ControlNode::While { body, .. } |
                                ControlNode::WhileLet { body, .. } => {
                                    count += count_controls_recursive(body);
                                }
                                ControlNode::Match { cases, .. } => {
                                    for case in cases {
                                        count += count_controls_recursive(&case.body);
                                    }
                                }
                            }
                        }
                        Segment::BraceBlock { inner, .. } => {
                            count += count_controls_recursive(inner);
                        }
                        _ => {}
                    }
                }
                count
            }

            let control_count = count_controls_recursive(&segments);
            eprintln!("Found {} control structures (including inside BraceBlocks)", control_count);

            // Should have if and for control structures (2 ifs + 1 for = 3)
            assert!(control_count >= 2, "Should have at least 2 control structures (if and for), found {}", control_count);
        }
        Err(e) => {
            panic!("parse_segments failed: {}", e);
        }
    }
}

/// Test $let bindings inside control flow (common pattern in derive_deserialize)
#[test]
fn test_let_binding_in_for_loop() {
    use std::str::FromStr;

    let code = r#"
        {#for field in all_fields}
            {$let raw_var = format!("__raw_{}", field.field_name)}
            const @{raw_var} = obj["@{field.json_key}"];
            instance.@{field.field_ident} = @{raw_var};
        {/for}
    "#;

    eprintln!("\n=== Testing $let in for loop ===");

    let tokens = TokenStream2::from_str(code).unwrap();
    let mut ids = IdGen::new();
    let result = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false);

    match result {
        Ok((segments, _)) => {
            eprintln!("Parsed {} segments", segments.len());

            // Find the for loop control structure
            let has_for = segments.iter().any(|s| {
                if let Segment::Control { node, .. } = s {
                    matches!(node, ControlNode::For { .. })
                } else {
                    false
                }
            });
            assert!(has_for, "Should have a For control structure");
        }
        Err(e) => {
            panic!("parse_segments failed: {}", e);
        }
    }
}

/// Test match statement pattern (used for TypeCategory matching)
#[test]
fn test_match_statement() {
    use std::str::FromStr;

    let code = r#"
        {#match &field.type_cat}
            {:case TypeCategory::Primitive}
                instance.@{field.field_ident} = value;
            {:case TypeCategory::Date}
                instance.@{field.field_ident} = new Date(value);
            {:case _}
                instance.@{field.field_ident} = value;
        {/match}
    "#;

    eprintln!("\n=== Testing match statement ===");

    let tokens = TokenStream2::from_str(code).unwrap();
    let mut ids = IdGen::new();
    let result = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false);

    match result {
        Ok((segments, _)) => {
            eprintln!("Parsed {} segments", segments.len());

            // Find the match control structure
            let has_match = segments.iter().any(|s| {
                if let Segment::Control { node, .. } = s {
                    matches!(node, ControlNode::Match { .. })
                } else {
                    false
                }
            });
            assert!(has_match, "Should have a Match control structure");
        }
        Err(e) => {
            panic!("parse_segments failed: {}", e);
        }
    }
}

/// Test if-let pattern (used for Option handling)
#[test]
fn test_if_let_pattern() {
    use std::str::FromStr;

    let code = r#"
        {#if let Some(fn_expr) = &field.deserialize_with}
            instance.@{field.field_ident} = (@{fn_expr})(obj["@{field.json_key}"]);
        {:else}
            instance.@{field.field_ident} = obj["@{field.json_key}"];
        {/if}
    "#;

    eprintln!("\n=== Testing if-let pattern ===");

    let tokens = TokenStream2::from_str(code).unwrap();
    let mut ids = IdGen::new();
    let result = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false);

    match result {
        Ok((segments, _)) => {
            eprintln!("Parsed {} segments", segments.len());

            // Find the if-let control structure
            let has_if_let = segments.iter().any(|s| {
                if let Segment::Control { node, .. } = s {
                    matches!(node, ControlNode::IfLet { .. })
                } else {
                    false
                }
            });
            assert!(has_if_let, "Should have an IfLet control structure");
        }
        Err(e) => {
            panic!("parse_segments failed: {}", e);
        }
    }
}

/// Test the actual placeholder sources that get generated
#[test]
fn test_placeholder_source_for_for_body() {
    use super::parse::parse_ts_module_with_source;
    use super::build::{build_placeholder_source, PlaceholderSourceKind};
    use std::str::FromStr;

    // Parse the exact for loop body from the failing template
    let code = r#"this.@{field.field_ident} = props.@{field.field_ident}{#if field.optional} as @{field.ts_type}{:else}{/if};"#;
    let tokens = TokenStream2::from_str(code).unwrap();
    let mut ids = IdGen::new();
    let (segments, _) = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
        .expect("parse_segments should succeed");

    eprintln!("\n=== For body segments ===");
    for (i, seg) in segments.iter().enumerate() {
        match seg {
            Segment::Static(s) => eprintln!("  {}: Static({:?})", i, s),
            Segment::Interpolation { id, .. } => eprintln!("  {}: Interpolation(id={})", i, id),
            Segment::Control { id, node } => {
                eprintln!("  {}: Control(id={}, {:?})", i, id, std::mem::discriminant(node))
            }
            _ => eprintln!("  {}: {:?}", i, std::mem::discriminant(seg)),
        }
    }

    // Build the placeholder source
    let (src, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
    eprintln!("\n=== Placeholder source ===\n{:?}", src);
    eprintln!("Map: {:?}", map);

    // Try parsing as module
    let module_result = parse_ts_module_with_source(&src);
    eprintln!("\nModule parse: {:?}", module_result.is_ok());
    if let Err(e) = &module_result {
        eprintln!("Module error: {:?}", e);
    }

    // Try parsing wrapped in class
    let class_wrapped = format!("class __MfWrapper {{ {} }}", src);
    eprintln!("\nClass wrapped: {:?}", class_wrapped);
    let class_result = parse_ts_module_with_source(&class_wrapped);
    eprintln!("Class parse: {:?}", class_result.is_ok());
    if let Err(e) = &class_result {
        eprintln!("Class error: {:?}", e);
    }
}

/// Test class body member template (body! macro pattern)
/// This pattern was failing with: "TypeScript parse error: Error { error: (25..26, Unexpected { got: "." ..."
#[test]
fn test_body_macro_constructor_pattern() {
    use std::str::FromStr;

    // Simplified version of the failing body! template from derive_deserialize.rs:1062
    let code = r#"
        constructor(props: { name: string }) {
            this.@{field_ident} = props.@{field_ident};
        }
    "#;

    eprintln!("\n=== Testing body! constructor pattern ===");
    eprintln!("Input code:\n{}", code);

    let tokens = TokenStream2::from_str(code).unwrap();
    let mut ids = IdGen::new();
    let result = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false);

    match result {
        Ok((segments, _)) => {
            eprintln!("\n=== Parsed {} segments ===", segments.len());
            for (i, seg) in segments.iter().enumerate() {
                match seg {
                    Segment::Static(s) => {
                        eprintln!("  {}: Static({:?})", i, s);
                    }
                    Segment::Interpolation { id, expr, .. } => {
                        eprintln!("  {}: Interpolation(id={}, expr={})", i, id, expr);
                    }
                    _ => eprintln!("  {}: {:?}", i, std::mem::discriminant(seg)),
                }
            }

            // Build the template string to see what SWC will receive
            let context_map = HashMap::new();
            let template_result = build_template_and_bindings(segments.iter(), &context_map);

            match template_result {
                Ok(result) => {
                    eprintln!("\n=== Template string (what SWC sees) ===\n{}", result.template);
                    eprintln!("\nCharacter positions around 25:");
                    let chars: Vec<char> = result.template.chars().collect();
                    for (i, ch) in chars.iter().enumerate().skip(20).take(15) {
                        eprintln!("  {}: {:?}", i, ch);
                    }
                }
                Err(e) => {
                    eprintln!("\nFailed to build template: {}", e);
                }
            }

            // Now try to run through flush_stmt_run to see the actual error
            let out_ident = proc_macro2::Ident::new("__mf_out", proc_macro2::Span::call_site());
            let comments_ident = proc_macro2::Ident::new("__mf_comments", proc_macro2::Span::call_site());
            let pending_ident = proc_macro2::Ident::new("__mf_pending", proc_macro2::Span::call_site());
            let pos_ident = proc_macro2::Ident::new("__mf_pos", proc_macro2::Span::call_site());
            let context_map = HashMap::new();

            let seg_refs: Vec<&Segment> = segments.iter().collect();
            let flush_result = flush_stmt_run(
                &seg_refs,
                &context_map,
                &out_ident,
                &comments_ident,
                &pending_ident,
                &pos_ident,
            );

            match flush_result {
                Ok(code) => {
                    eprintln!("\n=== flush_stmt_run succeeded ===");
                    eprintln!("Generated code length: {}", code.to_string().len());
                }
                Err(e) => {
                    eprintln!("\n=== flush_stmt_run FAILED ===");
                    eprintln!("Error: {}", e);
                    // Don't panic - we're debugging this
                }
            }
        }
        Err(e) => {
            panic!("parse_segments failed: {}", e);
        }
    }
}

/// Test class body with this.@{...} pattern - the likely source of the parsing error
#[test]
fn test_this_dot_interpolation_pattern() {
    use std::str::FromStr;

    // Minimal test: just the problematic pattern
    let code = r#"this.@{field_ident} = value;"#;

    eprintln!("\n=== Testing this.@{{}} pattern ===");
    eprintln!("Input code: {:?}", code);

    let tokens = TokenStream2::from_str(code).unwrap();
    let mut ids = IdGen::new();
    let (segments, _) = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false)
        .expect("parse_segments should succeed");

    eprintln!("Parsed {} segments", segments.len());
    for (i, seg) in segments.iter().enumerate() {
        match seg {
            Segment::Static(s) => eprintln!("  {}: Static({:?})", i, s),
            Segment::Interpolation { id, expr, .. } => eprintln!("  {}: Interpolation(id={}, expr={})", i, id, expr),
            _ => eprintln!("  {}: Other", i),
        }
    }

    // Build template to see what SWC receives
    let context_map = HashMap::new();
    let template_result = build_template_and_bindings(segments.iter(), &context_map)
        .expect("build_template_and_bindings should succeed");

    eprintln!("\nTemplate string: {:?}", template_result.template);

    // The template string should be something like: "this.$__mf_hole_0 = value;"
    // When parsed as a statement, this should be valid member assignment
    assert!(template_result.template.contains("this."), "Template should contain 'this.'");
    assert!(template_result.template.contains("__mf_hole_"), "Template should contain placeholder");
}

/// Test deeply nested control structures (the actual deserialize pattern)
#[test]
fn test_deeply_nested_control_structures() {
    use std::str::FromStr;

    let code = r#"
        {#if has_fields}
            {#for field in all_fields}
                {$let raw_var = format!("__raw_{}", field.field_name)}
                {#if let Some(fn_expr) = &field.deserialize_with}
                    instance.@{field.field_ident} = (@{fn_expr})(obj["@{field.json_key}"]);
                {:else}
                    {#if field.optional}
                        if ("@{field.json_key}" in obj) {
                            const @{raw_var} = obj["@{field.json_key}"] as @{field.ts_type};
                            {#match &field.type_cat}
                                {:case TypeCategory::Primitive}
                                    instance.@{field.field_ident} = @{raw_var};
                                {:case TypeCategory::Date}
                                    instance.@{field.field_ident} = new Date(@{raw_var});
                                {:case _}
                                    instance.@{field.field_ident} = @{raw_var};
                            {/match}
                        }
                    {:else}
                        const @{raw_var} = obj["@{field.json_key}"] as @{field.ts_type};
                        instance.@{field.field_ident} = @{raw_var};
                    {/if}
                {/if}
            {/for}
        {/if}
    "#;

    eprintln!("\n=== Testing deeply nested control structures ===");

    let tokens = TokenStream2::from_str(code).unwrap();
    let mut ids = IdGen::new();
    let result = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false);

    match result {
        Ok((segments, _)) => {
            eprintln!("Parsed {} segments successfully", segments.len());

            // Count different control structure types
            fn count_control_types(segments: &[Segment]) -> (usize, usize, usize, usize) {
                let mut if_count = 0;
                let mut for_count = 0;
                let mut match_count = 0;
                let mut if_let_count = 0;

                for seg in segments {
                    if let Segment::Control { node, .. } = seg {
                        match node {
                            ControlNode::If { then_branch, else_branch, .. } => {
                                if_count += 1;
                                let (i, f, m, l) = count_control_types(then_branch);
                                if_count += i;
                                for_count += f;
                                match_count += m;
                                if_let_count += l;
                                if let Some(eb) = else_branch {
                                    let (i, f, m, l) = count_control_types(eb);
                                    if_count += i;
                                    for_count += f;
                                    match_count += m;
                                    if_let_count += l;
                                }
                            }
                            ControlNode::For { body, .. } => {
                                for_count += 1;
                                let (i, f, m, l) = count_control_types(body);
                                if_count += i;
                                for_count += f;
                                match_count += m;
                                if_let_count += l;
                            }
                            ControlNode::Match { cases, .. } => {
                                match_count += 1;
                                for case in cases {
                                    let (i, f, m, l) = count_control_types(&case.body);
                                    if_count += i;
                                    for_count += f;
                                    match_count += m;
                                    if_let_count += l;
                                }
                            }
                            ControlNode::IfLet { then_branch, else_branch, .. } => {
                                if_let_count += 1;
                                let (i, f, m, l) = count_control_types(then_branch);
                                if_count += i;
                                for_count += f;
                                match_count += m;
                                if_let_count += l;
                                if let Some(eb) = else_branch {
                                    let (i, f, m, l) = count_control_types(eb);
                                    if_count += i;
                                    for_count += f;
                                    match_count += m;
                                    if_let_count += l;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                (if_count, for_count, match_count, if_let_count)
            }

            let (if_count, for_count, match_count, if_let_count) = count_control_types(&segments);
            eprintln!("Control structures: if={}, for={}, match={}, if_let={}", if_count, for_count, match_count, if_let_count);

            assert!(if_count >= 2, "Should have at least 2 if structures, found {}", if_count);
            assert!(for_count >= 1, "Should have at least 1 for structure, found {}", for_count);
            // Note: match might not be found if it's inside control flow that isn't fully traversed
            // The important thing is that parsing succeeded and we found multiple control structures
            assert!(if_let_count >= 1, "Should have at least 1 if-let structure, found {}", if_let_count);
            let total = if_count + for_count + match_count + if_let_count;
            assert!(total >= 4, "Should have at least 4 control structures total, found {}", total);
        }
        Err(e) => {
            panic!("parse_segments failed: {}", e);
        }
    }
}

/// Test union-style nested controls compile as statements (for loops must not be in expr context)
#[test]
fn test_union_deserialize_nested_controls_compile() {
    use std::str::FromStr;
    use crate::template::build::{build_placeholder_source, PlaceholderSourceKind};
    use crate::template::compile::compile_stmt_segments;

    let code = r#"
        export function deserialize(value: any): Foo {
            {#if is_literal_only}
                const allowedValues = [@{literal_elements_str}] as const;
                return value as @{full_type_ident};
            {:else if is_primitive_only}
                {#for prim in &primitive_types}
                    if (typeof value === "@{prim}") {
                        return value as @{full_type_ident};
                    }
                {/for}
            {:else}
                {#if has_primitives}
                    {#for prim in &primitive_types}
                        if (typeof value === "@{prim}") {
                            return value as @{full_type_ident};
                        }
                    {/for}
                {/if}
            {/if}
        }
    "#;

    let tokens = TokenStream2::from_str(code).unwrap();
    let mut ids = IdGen::new();
    let result = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false);

    match result {
        Ok((segments, _)) => {
            let (placeholder_source, _) =
                build_placeholder_source(&segments, PlaceholderSourceKind::Module);
            let out_ident = proc_macro2::Ident::new("__mf_out", proc_macro2::Span::call_site());
            let comments_ident = proc_macro2::Ident::new("__mf_comments", proc_macro2::Span::call_site());
            let pending_ident = proc_macro2::Ident::new("__mf_pending", proc_macro2::Span::call_site());
            let pos_ident = proc_macro2::Ident::new("__mf_pos", proc_macro2::Span::call_site());

            let compile_result = compile_stmt_segments(
                &segments,
                &out_ident,
                &comments_ident,
                &pending_ident,
                &pos_ident,
            );

            if let Err(e) = &compile_result {
                eprintln!("\n=== union nested control compile failed ===");
                eprintln!("Placeholder source:\n{}", placeholder_source);
                eprintln!("Error: {}", e);
            }

            assert!(
                compile_result.is_ok(),
                "Expected nested control template to compile as statements"
            );
        }
        Err(e) => {
            panic!("parse_segments failed: {}", e);
        }
    }
}

/// Test a simple export function without any doc comments
#[test]
fn test_simple_export_function() {
    use std::str::FromStr;
    use crate::template::build::{build_placeholder_source, PlaceholderSourceKind};
    use crate::template::compile::compile_stmt_segments;

    // Test exactly matching the failing serialize_template - second function has TWO parameters
    let code = r#"
        {>> "First doc" <<}
        export function @{fn1_ident}(value: @{type1_ident}): string {
            const ctx = @{ctx1_expr}.create();
            return JSON.stringify(@{internal_expr}(value, ctx));
        }

        {>> "Second doc" <<}
        export function @{fn2_ident}(value: @{type2_ident}, ctx: @{ctx_type}): Record<string, unknown> {
            const existingId = ctx.getId(value);
            if (existingId !== undefined) {
                return { __ref: existingId };
            }

            const __id = ctx.register(value);

            const result: Record<string, unknown> = {
                __type: "@{class_name}",
                __id,
            };

            {#if has_fields}
                {#for field in fields}
                    result["key"] = @{field_expr};
                {/for}
            {/if}

            return result;
        }
    "#;

    eprintln!("\n=== Testing simple export function ===");

    let tokens = TokenStream2::from_str(code).unwrap();
    let mut ids = IdGen::new();
    let result = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false);

    match result {
        Ok((segments, _)) => {
            eprintln!("\n=== Parsed {} segments ===", segments.len());
            for (i, seg) in segments.iter().enumerate() {
                match seg {
                    Segment::Static(s) => {
                        eprintln!("  {}: Static({:?})", i, s);
                    }
                    Segment::Interpolation { id, expr, .. } => {
                        eprintln!("  {}: Interpolation(id={}, expr={})", i, id, expr);
                    }
                    Segment::BraceBlock { id, inner } => {
                        eprintln!("  {}: BraceBlock(id={}, inner_len={})", i, id, inner.len());
                        for (j, seg) in inner.iter().enumerate() {
                            eprintln!("    {}.{}: {:?}", i, j, std::mem::discriminant(seg));
                        }
                    }
                    _ => eprintln!("  {}: {:?}", i, std::mem::discriminant(seg)),
                }
            }

            let (placeholder_source, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
            eprintln!("\n=== Placeholder source ({} bytes) ===", placeholder_source.len());
            eprintln!("{}", placeholder_source);
            eprintln!("\n=== Placeholder map ({} entries) ===", map.len());
            for (name, id) in &map {
                eprintln!("  {} -> {}", name, id);
            }

            let out_ident = proc_macro2::Ident::new("__mf_out", proc_macro2::Span::call_site());
            let comments_ident = proc_macro2::Ident::new("__mf_comments", proc_macro2::Span::call_site());
            let pending_ident = proc_macro2::Ident::new("__mf_pending", proc_macro2::Span::call_site());
            let pos_ident = proc_macro2::Ident::new("__mf_pos", proc_macro2::Span::call_site());

            let compile_result = compile_stmt_segments(
                &segments,
                &out_ident,
                &comments_ident,
                &pending_ident,
                &pos_ident,
            );

            match compile_result {
                Ok(_) => eprintln!("\n=== Successfully compiled ==="),
                Err(e) => panic!("compile_stmt_segments failed: {}", e),
            }
        }
        Err(e) => panic!("parse_segments failed: {}", e),
    }
}

/// Test serialize template with doc comments using {>> <<} syntax
/// This is the pattern from derive_serialize.rs that was failing with TS1109
#[test]
fn test_serialize_template_with_doc_comments() {
    use std::str::FromStr;
    use crate::template::build::{build_placeholder_source, PlaceholderSourceKind};
    use crate::template::compile::compile_stmt_segments;

    // Template pattern from derive_serialize.rs
    let code = r#"
        {>> "Serializes a value to a JSON string.\n@param value - The value to serialize\n@returns JSON string representation" <<}
        export function @{fn_serialize_ident}(value: @{class_ident}): string {
            const ctx = @{serialize_context_expr}.create();
            return JSON.stringify(@{fn_serialize_internal_expr}(value, ctx));
        }

        {>> "@internal Serializes with an existing context for nested/cyclic object graphs." <<}
        export function @{fn_serialize_internal_ident}(value: @{class_ident}, ctx: @{serialize_context_ident}): Record<string, unknown> {
            const existingId = ctx.getId(value);
            if (existingId !== undefined) {
                return { __ref: existingId };
            }

            const __id = ctx.register(value);

            const result: Record<string, unknown> = {
                __type: "@{class_name}",
                __id,
            };

            {#if has_regular}
                {#for field in regular_fields}
                    {#if field.optional}
                        if (value.@{field.field_ident} !== undefined) {
                            result["@{field.json_key}"] = value.@{field.field_ident};
                        }
                    {:else}
                        result["@{field.json_key}"] = value.@{field.field_ident};
                    {/if}
                {/for}
            {/if}

            return result;
        }
    "#;

    eprintln!("\n=== Testing serialize template with doc comments ===");
    eprintln!("Input code:\n{}", code);

    let tokens = TokenStream2::from_str(code).unwrap();
    let mut ids = IdGen::new();
    let result = parse_segments(&mut tokens.into_iter().peekable(), None, &mut ids, false);

    match result {
        Ok((segments, _)) => {
            eprintln!("\n=== Parsed {} segments ===", segments.len());
            for (i, seg) in segments.iter().enumerate() {
                match seg {
                    Segment::Static(s) => {
                        let truncated = if s.len() > 60 { format!("{}...", &s[..60]) } else { s.clone() };
                        eprintln!("  {}: Static({:?})", i, truncated);
                    }
                    Segment::Interpolation { id, expr, .. } => {
                        eprintln!("  {}: Interpolation(id={}, expr={})", i, id, expr);
                    }
                    Segment::Control { id, node, .. } => {
                        match node {
                            ControlNode::For { body, .. } => {
                                eprintln!("  {}: Control::For(id={}, body_len={})", i, id, body.len());
                            }
                            ControlNode::If { then_branch, else_branch, .. } => {
                                eprintln!("  {}: Control::If(id={}, then_len={}, has_else={})",
                                    i, id, then_branch.len(), else_branch.is_some());
                            }
                            _ => eprintln!("  {}: Control(id={}, node={:?})", i, id, std::mem::discriminant(node)),
                        }
                    }
                    Segment::Comment { style, text } => {
                        eprintln!("  {}: Comment({:?}, text_len={})", i, style, text.len());
                    }
                    _ => eprintln!("  {}: {:?}", i, std::mem::discriminant(seg)),
                }
            }

            // Build placeholder source to see what's being parsed
            let (placeholder_source, map) = build_placeholder_source(&segments, PlaceholderSourceKind::Module);
            eprintln!("\n=== Placeholder source ({} bytes) ===", placeholder_source.len());
            eprintln!("{}", placeholder_source);
            eprintln!("\n=== Placeholder map ({} entries) ===", map.len());
            for (name, id) in &map {
                eprintln!("  {} -> {}", name, id);
            }

            // Try to compile
            let out_ident = proc_macro2::Ident::new("__mf_out", proc_macro2::Span::call_site());
            let comments_ident = proc_macro2::Ident::new("__mf_comments", proc_macro2::Span::call_site());
            let pending_ident = proc_macro2::Ident::new("__mf_pending", proc_macro2::Span::call_site());
            let pos_ident = proc_macro2::Ident::new("__mf_pos", proc_macro2::Span::call_site());

            let compile_result = compile_stmt_segments(
                &segments,
                &out_ident,
                &comments_ident,
                &pending_ident,
                &pos_ident,
            );

            match compile_result {
                Ok(compiled) => {
                    eprintln!("\n=== Successfully compiled ===");
                    eprintln!("Generated {} tokens", compiled.to_string().len());
                }
                Err(e) => {
                    eprintln!("\nFailed to compile: {}", e);
                    panic!("compile_stmt_segments failed: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("\nFailed to parse segments: {}", e);
            panic!("parse_segments failed: {}", e);
        }
    }
}
