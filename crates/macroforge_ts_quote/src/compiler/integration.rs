//! Integration between the new compiler and the existing template system.
//!
//! This module provides functions to bridge the gap between the segment-based
//! template representation and the new Rowan-based compiler.
//!
//! ## Output Modes
//!
//! The new compiler supports two output modes:
//!
//! 1. **String mode** (`compile_template_string`, `compile_segments_with_new_compiler`):
//!    Generates code that builds a TypeScript string at runtime.
//!
//! 2. **SWC AST mode** (`compile_segments_to_swc_ast`):
//!    Generates code that builds a TypeScript string and parses it with SWC at runtime,
//!    producing `Vec<ModuleItem>`. This is compatible with the existing template system.

use super::codegen::{Codegen, CodegenConfig};
use super::ir::lower;
use super::parser::Parser;
use super::semantic::analyze;
use super::syntax::SyntaxNode;
use crate::template::{Segment, ControlNode, CommentStyle, StringPart, IdentPart};
use proc_macro2::{TokenStream, Span};
use quote::quote;

/// Converts segments to a template string that the new compiler can process.
///
/// This reconstructs the original template syntax from parsed segments.
pub fn segments_to_template_string(segments: &[&Segment]) -> String {
    let mut result = String::new();

    for segment in segments {
        segment_to_string(segment, &mut result);
    }

    result
}

fn segment_to_string(segment: &Segment, out: &mut String) {
    match segment {
        Segment::Static(text) => {
            out.push_str(text);
        }

        Segment::Interpolation { id, expr: _ } => {
            // Convert back to @{expr} syntax
            // The expr is a TokenStream, convert to string
            out.push_str("@{");
            out.push_str(&format!("__mf_hole_{}", id));
            out.push('}');
        }

        Segment::Control { node } => {
            control_node_to_string(node, out);
        }

        Segment::Comment { style, text } => {
            match style {
                CommentStyle::Line => {
                    out.push_str("{> ");
                    out.push_str(text);
                    out.push_str(" <}");
                }
                CommentStyle::Block => {
                    out.push_str("{>> ");
                    out.push_str(text);
                    out.push_str(" <<}");
                }
                CommentStyle::DocBlock => {
                    out.push_str("/** ");
                    out.push_str(text);
                    out.push_str(" */");
                }
            }
        }

        Segment::StringInterp { parts } => {
            out.push('"');
            for part in parts {
                string_part_to_string(part, out);
            }
            out.push('"');
        }

        Segment::TemplateInterp { parts } => {
            out.push('`');
            for part in parts {
                string_part_to_string(part, out);
            }
            out.push('`');
        }

        Segment::IdentBlock { parts } => {
            out.push_str("{|");
            for part in parts {
                ident_part_to_string(part, out);
            }
            out.push_str("|}");
        }

        Segment::BraceBlock { inner } => {
            out.push('{');
            for seg in inner {
                segment_to_string(seg, out);
            }
            out.push('}');
        }

        Segment::ObjectPropLoop { pat, iter, key_expr, value_expr } => {
            // Object property loop: generates { key: value } pairs in a loop
            // Emit as a for loop that builds key-value pairs
            out.push_str("{#for ");
            out.push_str(&pat.to_string());
            out.push_str(" in ");
            out.push_str(&iter.to_string());
            out.push_str("}@{");
            out.push_str(&key_expr.to_string());
            out.push_str("}: @{");
            out.push_str(&value_expr.to_string());
            out.push_str("},{/for}");
        }

        Segment::Let { tokens } => {
            // Let directive contains the full let statement tokens
            out.push_str("{$let ");
            out.push_str(&tokens.to_string());
            out.push('}');
        }

        Segment::Do { expr } => {
            out.push_str("{$do ");
            out.push_str(&expr.to_string());
            out.push('}');
        }

        Segment::Typescript { expr } => {
            out.push_str("{$typescript ");
            out.push_str(&expr.to_string());
            out.push('}');
        }
    }
}

/// Converts a StringPart to its string representation.
fn string_part_to_string(part: &StringPart, out: &mut String) {
    match part {
        StringPart::Text(text) => {
            out.push_str(text);
        }
        StringPart::Expr(expr) => {
            // String interpolation expression: ${expr}
            out.push_str("${");
            out.push_str(&expr.to_string());
            out.push('}');
        }
    }
}

/// Converts an IdentPart to its string representation.
fn ident_part_to_string(part: &IdentPart, out: &mut String) {
    match part {
        IdentPart::Static(text) => {
            out.push_str(text);
        }
        IdentPart::Interpolation { expr } => {
            // Identifier interpolation: @{expr}
            out.push_str("@{");
            out.push_str(&expr.to_string());
            out.push('}');
        }
    }
}

fn control_node_to_string(node: &ControlNode, out: &mut String) {
    match node {
        ControlNode::If { cond, then_branch, else_branch } => {
            out.push_str("{#if ");
            out.push_str(&cond.to_string());
            out.push('}');
            for seg in then_branch {
                segment_to_string(seg, out);
            }
            if let Some(else_segs) = else_branch {
                out.push_str("{:else}");
                for seg in else_segs {
                    segment_to_string(seg, out);
                }
            }
            out.push_str("{/if}");
        }

        ControlNode::IfLet { pattern, expr, then_branch, else_branch } => {
            out.push_str("{#if let ");
            out.push_str(&pattern.to_string());
            out.push_str(" = ");
            out.push_str(&expr.to_string());
            out.push('}');
            for seg in then_branch {
                segment_to_string(seg, out);
            }
            if let Some(else_segs) = else_branch {
                out.push_str("{:else}");
                for seg in else_segs {
                    segment_to_string(seg, out);
                }
            }
            out.push_str("{/if}");
        }

        ControlNode::For { pat, iter, body } => {
            out.push_str("{#for ");
            out.push_str(&pat.to_string());
            out.push_str(" in ");
            out.push_str(&iter.to_string());
            out.push('}');
            for seg in body {
                segment_to_string(seg, out);
            }
            out.push_str("{/for}");
        }

        ControlNode::While { cond, body } => {
            out.push_str("{#while ");
            out.push_str(&cond.to_string());
            out.push('}');
            for seg in body {
                segment_to_string(seg, out);
            }
            out.push_str("{/while}");
        }

        ControlNode::WhileLet { pattern, expr, body } => {
            out.push_str("{#while let ");
            out.push_str(&pattern.to_string());
            out.push_str(" = ");
            out.push_str(&expr.to_string());
            out.push('}');
            for seg in body {
                segment_to_string(seg, out);
            }
            out.push_str("{/while}");
        }

        ControlNode::Match { expr, cases } => {
            out.push_str("{#match ");
            out.push_str(&expr.to_string());
            out.push('}');
            for case in cases {
                out.push_str("{:case ");
                out.push_str(&case.pattern.to_string());
                out.push('}');
                for seg in &case.body {
                    segment_to_string(seg, out);
                }
            }
            out.push_str("{/match}");
        }
    }
}


/// Compiles segments to SWC AST nodes using the new compiler.
///
/// This generates code that:
/// 1. Builds a TypeScript string using the new compiler's string-building approach
/// 2. Parses that string with SWC at runtime
/// 3. Pushes the resulting ModuleItems to the output vector
///
/// This is compatible with the existing template system which expects `Vec<ModuleItem>`.
pub fn compile_segments_to_swc_ast(
    segments: &[&Segment],
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
) -> syn::Result<TokenStream> {
    // Step 1: Convert segments to template string for the new compiler
    let template_str = segments_to_template_string(segments);

    if template_str.trim().is_empty() {
        return Ok(TokenStream::new());
    }

    // Step 2: Parse with new compiler
    let parser = Parser::new(&template_str);
    let green = parser.parse();
    let root = SyntaxNode::new_root(green);

    // Step 3: Semantic analysis
    let analysis = analyze(&root);

    // Step 4: Lower to IR
    let ir = lower(&root, analysis);

    // Step 5: Generate string-building code
    // Use a temporary variable name for the string
    let string_var = proc_macro2::Ident::new("__mf_ts_source", Span::call_site());
    let config = CodegenConfig {
        module_level: false, // Don't wrap in module-level block
        output_var: string_var.to_string(),
        module_var: "__mf_module".to_string(),
    };

    let string_building_code = Codegen::with_config(config).generate(&ir);

    // Step 6: Wrap with SWC parsing
    Ok(quote! {{
        // Build the TypeScript source string
        let #string_var: String = #string_building_code;

        // Parse with SWC at runtime
        use swc_core::common::{FileName, SourceMap, sync::Lrc};
        use swc_core::ecma::parser::{Parser as SwcParser, StringInput, Syntax, TsSyntax, lexer::Lexer as SwcLexer};
        use swc_core::ecma::visit::{VisitMut, VisitMutWith};
        use swc_core::ecma::ast::*;

        let __mf_cm: Lrc<SourceMap> = Lrc::new(SourceMap::default());
        let __mf_fm = __mf_cm.new_source_file(
            FileName::Custom("template.ts".into()).into(),
            #string_var.clone(),
        );
        let __mf_syntax = Syntax::Typescript(TsSyntax {
            tsx: true,
            decorators: true,
            ..Default::default()
        });
        let __mf_lexer = SwcLexer::new(
            __mf_syntax,
            EsVersion::latest(),
            StringInput::from(&*__mf_fm),
            None,
        );
        let mut __mf_parser = SwcParser::new_from(__mf_lexer);
        let __mf_module = __mf_parser
            .parse_module()
            .unwrap_or_else(|e| panic!(
                "Failed to parse TypeScript template: {:?}\n\nGenerated source:\n{}",
                e, #string_var
            ));

        // Process each module item
        for mut __mf_item in __mf_module.body {
            // Fix spans to unique positions
            let __mf_pos = swc_core::common::BytePos(#pos_ident);
            #pos_ident += 1;
            {
                struct __MfSpanFix {
                    span: swc_core::common::Span,
                }
                impl VisitMut for __MfSpanFix {
                    fn visit_mut_span(&mut self, span: &mut swc_core::common::Span) {
                        *span = self.span;
                    }
                }
                let mut __mf_span_fix = __MfSpanFix {
                    span: swc_core::common::Span::new(__mf_pos, __mf_pos),
                };
                __mf_item.visit_mut_with(&mut __mf_span_fix);
            }

            // Attach pending comments
            if !#pending_ident.is_empty() {
                use swc_core::common::comments::Comments;
                use swc_core::common::Spanned;
                for __mf_comment in #pending_ident.drain(..) {
                    #comments_ident.add_leading(__mf_item.span().lo(), __mf_comment);
                }
            }

            #out_ident.push(__mf_item);
        }
    }})
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;
    use quote::quote;

    fn ident(name: &str) -> proc_macro2::Ident {
        proc_macro2::Ident::new(name, Span::call_site())
    }

    #[test]
    fn test_segments_to_template_simple() {
        let seg = Segment::Static("const x = 1;".to_string());
        let segments = vec![&seg];

        let template = segments_to_template_string(&segments);
        assert_eq!(template, "const x = 1;");
    }

    #[test]
    fn test_segments_to_template_with_interpolation() {
        let seg1 = Segment::Static("const x = ".to_string());
        let seg2 = Segment::Interpolation {
            id: 0,
            expr: quote! { value },
        };
        let seg3 = Segment::Static(";".to_string());
        let segments = vec![&seg1, &seg2, &seg3];

        let template = segments_to_template_string(&segments);
        assert!(template.contains("@{__mf_hole_0}"));
    }


    #[test]
    fn test_compile_segments_to_swc_ast() {
        let seg = Segment::Static("const x = 1;".to_string());
        let segments = vec![&seg];

        let out = ident("__mf_out");
        let comments = ident("__mf_comments");
        let pending = ident("__mf_pending");
        let pos = ident("__mf_pos");

        let result = compile_segments_to_swc_ast(
            &segments,
            &out,
            &comments,
            &pending,
            &pos,
        );

        assert!(result.is_ok());
        let code = result.unwrap();
        let code_str = code.to_string();

        // Should contain SWC parsing code
        assert!(code_str.contains("swc_core"), "Should use swc_core");
        assert!(code_str.contains("SourceMap"), "Should create SourceMap");
        assert!(code_str.contains("parse_module"), "Should parse module");
        assert!(code_str.contains("__mf_out"), "Should push to output");
    }

    #[test]
    fn test_compile_segments_to_swc_ast_empty() {
        let segments: Vec<&Segment> = vec![];

        let out = ident("__mf_out");
        let comments = ident("__mf_comments");
        let pending = ident("__mf_pending");
        let pos = ident("__mf_pos");

        let result = compile_segments_to_swc_ast(
            &segments,
            &out,
            &comments,
            &pending,
            &pos,
        );

        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.is_empty(), "Empty segments should produce empty code");
    }

    #[test]
    fn test_compile_segments_to_swc_ast_with_control_flow() {
        let seg1 = Segment::Static("const items = [".to_string());
        let seg2 = Segment::Control {
            node: ControlNode::For {
                pat: quote! { item },
                iter: quote! { items },
                body: vec![Segment::Static("item,".to_string())],
            },
        };
        let seg3 = Segment::Static("];".to_string());
        let segments = vec![&seg1, &seg2, &seg3];

        let out = ident("__mf_out");
        let comments = ident("__mf_comments");
        let pending = ident("__mf_pending");
        let pos = ident("__mf_pos");

        let result = compile_segments_to_swc_ast(
            &segments,
            &out,
            &comments,
            &pending,
            &pos,
        );

        assert!(result.is_ok());
        let code = result.unwrap();
        let code_str = code.to_string();

        // Should contain for loop in the generated code
        assert!(code_str.contains("for"), "Should generate for loop");
    }
}
