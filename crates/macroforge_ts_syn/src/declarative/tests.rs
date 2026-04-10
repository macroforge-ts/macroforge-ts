//! Unit tests for the declarative macro parser.

use super::parser::parse_macro_def;
use super::types::{
    Body, BodyToken, FragmentKind, MacroDef, Pattern, PatternElement, RepetitionKind,
};
use crate::abi::SpanIR;

fn parse(source: &str) -> MacroDef {
    let span = SpanIR::new(0, source.len() as u32);
    parse_macro_def(source, span).expect("parse should succeed")
}

#[test]
fn parses_single_empty_arm() {
    let def = parse("() => []");
    assert_eq!(def.arms.len(), 1);
    assert_eq!(def.arms[0].pattern, Pattern::Empty);
    // Body is `[]`
    assert_eq!(
        def.arms[0].body,
        Body(vec![BodyToken::Literal("[]".to_string())])
    );
}

#[test]
fn parses_single_fragment_arm() {
    let def = parse("($x:Expr) => $x");
    assert_eq!(def.arms.len(), 1);
    match &def.arms[0].pattern {
        Pattern::Sequence(elems) => {
            assert_eq!(elems.len(), 1);
            match &elems[0] {
                PatternElement::Fragment { name, kind } => {
                    assert_eq!(name, "x");
                    assert_eq!(*kind, FragmentKind::Expr);
                }
                _ => panic!("expected fragment"),
            }
        }
        _ => panic!("expected sequence pattern"),
    }
    assert_eq!(
        def.arms[0].body,
        Body(vec![BodyToken::Substitution("x".to_string())])
    );
}

#[test]
fn parses_multiple_arms_blank_line_separated() {
    let src = "
        () => []

        ($x:Expr) => [$x]

        ($x:Expr, $y:Expr) => [$x, $y]
    ";
    let def = parse(src);
    assert_eq!(def.arms.len(), 3);
    assert_eq!(def.arms[0].pattern, Pattern::Empty);
    match &def.arms[1].pattern {
        Pattern::Sequence(elems) => assert_eq!(elems.len(), 1),
        _ => panic!(),
    }
    match &def.arms[2].pattern {
        Pattern::Sequence(elems) => {
            // Two fragments separated by a literal `,`.
            assert_eq!(elems.len(), 3);
            match &elems[1] {
                PatternElement::Literal(s) => assert_eq!(s, ","),
                _ => panic!("expected literal comma"),
            }
        }
        _ => panic!(),
    }
}

#[test]
fn parses_multiple_arms_adjacent_lines() {
    // The `$opt` example uses adjacent lines (no blank line between arms).
    let src = "
        ()                            => undefined
        ($x:Expr)                     => $x
        ($x:Expr, $default:Expr)      => ($x ?? $default)
    ";
    let def = parse(src);
    assert_eq!(def.arms.len(), 3);
}

#[test]
fn parses_repetition_one_or_more() {
    let def = parse("($($x:Expr),+) => [$( $x ),+]");
    assert_eq!(def.arms.len(), 1);
    match &def.arms[0].pattern {
        Pattern::Sequence(elems) => {
            assert_eq!(elems.len(), 1);
            match &elems[0] {
                PatternElement::Repetition {
                    pattern,
                    separator,
                    kind,
                } => {
                    assert_eq!(*kind, RepetitionKind::OneOrMore);
                    assert_eq!(separator.as_deref(), Some(","));
                    match pattern.as_ref() {
                        Pattern::Sequence(inner) => {
                            assert_eq!(inner.len(), 1);
                            assert!(matches!(
                                inner[0],
                                PatternElement::Fragment {
                                    kind: FragmentKind::Expr,
                                    ..
                                }
                            ));
                        }
                        _ => panic!(),
                    }
                }
                _ => panic!("expected repetition"),
            }
        }
        _ => panic!(),
    }

    // Body has a repetition token.
    let found_rep = def.arms[0].body.0.iter().any(|t| {
        matches!(
            t,
            BodyToken::Repetition {
                kind: RepetitionKind::OneOrMore,
                ..
            }
        )
    });
    assert!(found_rep, "expected repetition body token");
}

#[test]
fn parses_repetition_zero_or_more() {
    let def = parse("($($x:Expr),*) => {}");
    match &def.arms[0].pattern {
        Pattern::Sequence(elems) => match &elems[0] {
            PatternElement::Repetition { kind, .. } => {
                assert_eq!(*kind, RepetitionKind::ZeroOrMore);
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
}

#[test]
fn parses_repetition_zero_or_one_no_separator() {
    let def = parse("($x:Expr $( , )?) => [$x]");
    match &def.arms[0].pattern {
        Pattern::Sequence(elems) => {
            // $x fragment, then `$( , )?` repetition.
            assert_eq!(elems.len(), 2);
            match &elems[1] {
                PatternElement::Repetition { kind, .. } => {
                    assert_eq!(*kind, RepetitionKind::ZeroOrOne);
                }
                _ => panic!("expected repetition"),
            }
        }
        _ => panic!(),
    }
}

#[test]
fn parses_body_with_substitution_and_literal() {
    let def = parse("($x:Expr) => (() => { return $x + 1; })()");
    let tokens = &def.arms[0].body.0;
    // We expect at least one literal and one substitution.
    let has_sub = tokens
        .iter()
        .any(|t| matches!(t, BodyToken::Substitution(name) if name == "x"));
    assert!(has_sub, "expected `$x` substitution");
    let has_lit = tokens
        .iter()
        .any(|t| matches!(t, BodyToken::Literal(s) if s.contains("return")));
    assert!(has_lit, "expected literal text");
}

#[test]
fn parses_body_with_repetition() {
    let def = parse("($($x:Expr),+) => { let __acc = 0; $( __acc += $x; )+ __acc }");
    let has_body_rep = def.arms[0].body.0.iter().any(|t| {
        matches!(
            t,
            BodyToken::Repetition {
                kind: RepetitionKind::OneOrMore,
                ..
            }
        )
    });
    assert!(has_body_rep);
}

#[test]
fn parses_vec_example_from_design_doc() {
    let src = "
        () => []

        ($($x:Expr),+) => {
            const __v = [];
            $( __v.push($x); )+
            __v
        }
    ";
    let def = parse(src);
    assert_eq!(def.arms.len(), 2);
    assert_eq!(def.arms[0].pattern, Pattern::Empty);
    match &def.arms[1].pattern {
        Pattern::Sequence(elems) => {
            assert_eq!(elems.len(), 1);
            assert!(matches!(elems[0], PatternElement::Repetition { .. }));
        }
        _ => panic!(),
    }
    // The second arm's body should contain a repetition and at least one literal.
    let body = &def.arms[1].body.0;
    let has_rep = body
        .iter()
        .any(|t| matches!(t, BodyToken::Repetition { .. }));
    assert!(has_rep);
}

#[test]
fn rejects_unknown_fragment_kind() {
    let src = "($x:Banana) => $x";
    let err = parse_macro_def(src, SpanIR::new(0, src.len() as u32)).unwrap_err();
    assert!(
        err.message.contains("Banana"),
        "expected diagnostic mentioning the unknown kind, got: {}",
        err.message
    );
    assert!(
        err.message.contains("known:"),
        "expected diagnostic to list known kinds"
    );
}

#[test]
fn rejects_missing_arrow() {
    let src = "($x:Expr) $x";
    let err = parse_macro_def(src, SpanIR::new(0, src.len() as u32)).unwrap_err();
    assert!(err.message.contains("=>"));
}

#[test]
fn rejects_unbalanced_parens() {
    let src = "($x:Expr => $x";
    let err = parse_macro_def(src, SpanIR::new(0, src.len() as u32)).unwrap_err();
    // Could hit either the arm matcher or the outer balance check.
    assert!(
        err.message.contains("unbalanced") || err.message.contains("=>"),
        "got: {}",
        err.message
    );
}

#[test]
fn rejects_missing_fragment_kind() {
    let src = "($x) => $x";
    let err = parse_macro_def(src, SpanIR::new(0, src.len() as u32)).unwrap_err();
    assert!(err.message.contains(":<Kind>") || err.message.contains("Kind"));
}

#[test]
fn empty_template_is_error() {
    let src = "";
    let err = parse_macro_def(src, SpanIR::new(0, 0)).unwrap_err();
    assert!(err.message.to_lowercase().contains("no arms"));
}

#[test]
fn span_offsets_track_template_base() {
    // Pretend the template body lives at offset 100 in the source file.
    let src = "($x:Expr) => $x";
    let base = 100u32;
    let def = parse_macro_def(src, SpanIR::new(base, base + src.len() as u32)).unwrap();
    assert_eq!(def.span.start, base);
    assert_eq!(def.arms[0].span.start, base);
}
