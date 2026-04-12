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
fn parses_body_with_macro_call() {
    // `$double($x)` inside a body should parse as MacroCall, not
    // Substitution of `double` followed by literal `($x)`.
    let def = parse("($x:Expr) => $double($x)");
    let tokens = &def.arms[0].body.0;
    let mut found = false;
    for t in tokens {
        if let BodyToken::MacroCall { name, args } = t {
            assert_eq!(name, "double");
            // `args` should contain a Substitution of `x`.
            let inner_sub = args
                .iter()
                .any(|a| matches!(a, BodyToken::Substitution(s) if s == "x"));
            assert!(inner_sub, "expected $x substitution inside call args");
            found = true;
        }
    }
    assert!(found, "expected a MacroCall token in body: {:?}", tokens);
}

#[test]
fn parses_body_with_nested_macro_calls() {
    // `$outer($inner($x))` — two levels of MacroCall.
    let def = parse("($x:Expr) => $outer($inner($x))");
    let tokens = &def.arms[0].body.0;
    let outer = tokens
        .iter()
        .find_map(|t| match t {
            BodyToken::MacroCall { name, args } if name == "outer" => Some(args),
            _ => None,
        })
        .expect("outer MacroCall");
    let inner = outer
        .iter()
        .find_map(|t| match t {
            BodyToken::MacroCall { name, args } if name == "inner" => Some(args),
            _ => None,
        })
        .expect("inner MacroCall");
    let has_x = inner
        .iter()
        .any(|t| matches!(t, BodyToken::Substitution(s) if s == "x"));
    assert!(has_x);
}

#[test]
fn parses_macro_call_inside_repetition() {
    // `$( $double($x); )+` — repetition over a MacroCall.
    let def = parse("($($x:Expr),+) => { $( $double($x); )+ }");
    let tokens = &def.arms[0].body.0;
    let rep_body = tokens
        .iter()
        .find_map(|t| match t {
            BodyToken::Repetition { body, .. } => Some(body),
            _ => None,
        })
        .expect("repetition in body");
    let has_call = rep_body
        .iter()
        .any(|t| matches!(t, BodyToken::MacroCall { name, .. } if name == "double"));
    assert!(
        has_call,
        "expected $double call inside repetition, got: {:?}",
        rep_body
    );
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

// ---------------------------------------------------------------------------
// Phase F — parser edge cases (newline-sensitive disambiguation +
// string-literal awareness). See plans/melodic-rolling-falcon.md.
// ---------------------------------------------------------------------------

#[test]
fn substitution_followed_by_paren_on_new_line_is_not_a_macro_call() {
    // `$foo\n  (thing)` inside a brace-wrapped body — the newline
    // between `$foo` and `(` must break the substitution/macro-call
    // disambiguation so `$foo` stays a plain substitution and `(thing)`
    // is literal text. The outer `{}` keeps the `(` at brace-depth > 0
    // so the arm-splitter doesn't misread it as a new arm header.
    let def = parse("($foo:Expr) => { $foo\n  (thing) }");
    let tokens = &def.arms[0].body.0;
    // Expect a Substitution("foo") followed by Literal("(thing)")-ish.
    let has_sub = tokens
        .iter()
        .any(|t| matches!(t, BodyToken::Substitution(s) if s == "foo"));
    assert!(
        has_sub,
        "expected bare $foo substitution, got: {:?}",
        tokens
    );
    let has_call = tokens
        .iter()
        .any(|t| matches!(t, BodyToken::MacroCall { .. }));
    assert!(
        !has_call,
        "expected NO MacroCall — newline should break the association, got: {:?}",
        tokens
    );
    // The literal portion should include the trailing `(thing)`.
    let has_paren_literal = tokens
        .iter()
        .any(|t| matches!(t, BodyToken::Literal(s) if s.contains("(thing)")));
    assert!(
        has_paren_literal,
        "expected the `(thing)` text to appear as a literal, got: {:?}",
        tokens
    );
}

#[test]
fn same_line_paren_still_parses_as_macro_call() {
    // Regression guard: `$foo(stmt)` on one line is still a call.
    let def = parse("($x:Expr) => $foo($x)");
    let has_call = def.arms[0]
        .body
        .0
        .iter()
        .any(|t| matches!(t, BodyToken::MacroCall { name, .. } if name == "foo"));
    assert!(has_call, "expected MacroCall for `$foo($x)` on one line");
}

#[test]
fn dollar_inside_double_quoted_string_stays_literal() {
    // The `$$` inside a double-quoted string is not the macro escape —
    // it's literal text that should round-trip verbatim.
    let def = parse(r#"($x:Expr) => "abc $$ def""#);
    let tokens = &def.arms[0].body.0;
    // No Literal("$") token should appear (which is what the `$$`
    // escape produces when parsed as an escape).
    let escape_count = tokens
        .iter()
        .filter(|t| matches!(t, BodyToken::Literal(s) if s == "$"))
        .count();
    assert_eq!(
        escape_count, 0,
        "`$$` inside a string should not be parsed as an escape: {:?}",
        tokens
    );
    // The full literal text must still contain `$$`.
    let joined: String = tokens
        .iter()
        .filter_map(|t| match t {
            BodyToken::Literal(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();
    assert!(
        joined.contains("$$"),
        "expected body literal to contain `$$`, got: {:?}",
        joined
    );
}

#[test]
fn dollar_ident_inside_single_quoted_string_stays_literal() {
    // `'$name'` inside the body should remain literal text, not a
    // substitution reference to `name`.
    let def = parse("($x:Expr) => '$name'");
    let tokens = &def.arms[0].body.0;
    let has_sub_name = tokens
        .iter()
        .any(|t| matches!(t, BodyToken::Substitution(s) if s == "name"));
    assert!(
        !has_sub_name,
        "string-interior `$name` should not be a Substitution: {:?}",
        tokens
    );
    // The `$name` text should be preserved in a literal.
    let joined: String = tokens
        .iter()
        .filter_map(|t| match t {
            BodyToken::Literal(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();
    assert!(
        joined.contains("$name"),
        "expected literal `$name` to survive in the body, got: {:?}",
        joined
    );
}

#[test]
fn template_expression_slot_still_parses_substitutions() {
    // Regression guard for the template-literal state machine: inside
    // `${...}` we're back in code context, so `$x` there must still be
    // a substitution.
    let def = parse("($x:Expr) => `hello ${$x} world`");
    let tokens = &def.arms[0].body.0;
    let has_sub = tokens
        .iter()
        .any(|t| matches!(t, BodyToken::Substitution(s) if s == "x"));
    assert!(
        has_sub,
        "expected `$x` inside ${{...}} to be a Substitution, got: {:?}",
        tokens
    );
}

#[test]
fn reserved_cluster_name_parses_as_substitution_even_with_paren() {
    // `$__cluster__` is reserved by the cluster-aware runtime-name
    // template feature. In a call_arms body, the natural form
    // `__helper_$__cluster__($args)` must parse as Literal + Substitution
    // + Literal + Substitution + Literal, NOT as a call to a macro
    // named "__cluster__".
    let def = parse("($x:Expr) => __h_$__cluster__($x)");
    let tokens = &def.arms[0].body.0;
    let has_cluster_sub = tokens
        .iter()
        .any(|t| matches!(t, BodyToken::Substitution(s) if s == "__cluster__"));
    let has_cluster_call = tokens
        .iter()
        .any(|t| matches!(t, BodyToken::MacroCall { name, .. } if name == "__cluster__"));
    assert!(
        has_cluster_sub,
        "expected `$__cluster__` to parse as a substitution, got: {:?}",
        tokens
    );
    assert!(
        !has_cluster_call,
        "expected no MacroCall for the reserved `$__cluster__` name, got: {:?}",
        tokens
    );
}

#[test]
fn short_cluster_name_is_now_a_regular_metavariable() {
    // PR 12 freed up the short `cluster` name — users can now use
    // `$cluster` as a regular metavariable or declare a macro called
    // `$cluster`. Only `$__cluster__` is reserved.
    let def = parse("($cluster:Expr) => ($cluster + 1)");
    // The pattern should bind a fragment named "cluster".
    match &def.arms[0].pattern {
        Pattern::Sequence(elems) => {
            assert!(
                elems.iter().any(|e| matches!(
                    e,
                    PatternElement::Fragment { name, .. } if name == "cluster"
                )),
                "expected fragment `$cluster`, got: {:?}",
                elems
            );
        }
        _ => panic!("expected sequence pattern"),
    }
    // The body should reference `$cluster` as a substitution.
    let has_sub = def.arms[0]
        .body
        .0
        .iter()
        .any(|t| matches!(t, BodyToken::Substitution(s) if s == "cluster"));
    assert!(
        has_sub,
        "expected `$cluster` substitution in body, got: {:?}",
        def.arms[0].body.0
    );
}

#[test]
fn template_text_portion_keeps_dollar_ident_literal() {
    // `$name` in the text part of a template literal (OUTSIDE any
    // `${...}` slot) is literal — the user would write `${$name}` if
    // they meant a substitution.
    let def = parse("($x:Expr) => `hello $name world`");
    let tokens = &def.arms[0].body.0;
    let has_sub_name = tokens
        .iter()
        .any(|t| matches!(t, BodyToken::Substitution(s) if s == "name"));
    assert!(
        !has_sub_name,
        "template-text `$name` should NOT be a Substitution: {:?}",
        tokens
    );
}

// ---------------------------------------------------------------------------
// PR 18 — DeclarativeError help/notes + MacroDef::default()
// ---------------------------------------------------------------------------

#[test]
fn declarative_error_with_help_round_trip() {
    use super::errors::DeclarativeError;
    let err = DeclarativeError::new(SpanIR::new(5, 10), "something went wrong")
        .with_help("try doing X instead")
        .with_note("note: Y is also relevant");
    assert_eq!(err.message, "something went wrong");
    assert_eq!(err.span, SpanIR::new(5, 10));
    assert_eq!(err.help.as_deref(), Some("try doing X instead"));
    assert_eq!(err.notes, vec!["note: Y is also relevant".to_string()]);
}

#[test]
fn declarative_error_chainable_notes() {
    use super::errors::DeclarativeError;
    let err = DeclarativeError::new(SpanIR::new(0, 0), "m")
        .with_note("first")
        .with_note("second")
        .with_note("third");
    assert_eq!(err.notes, vec!["first", "second", "third"]);
}

#[test]
fn macro_def_default_and_from_arms_agree() {
    use super::types::{MacroArm, MacroMode};
    let default = MacroDef::default();
    let from_arms: MacroDef = MacroDef::from_arms(
        String::new(),
        Vec::<MacroArm>::new(),
        MacroMode::ExpandOnly,
        SpanIR::new(0, 0),
    );
    // Every field that Default produces should match what from_arms
    // produces when given the same trivial inputs. If a new field is
    // added to MacroDef, either both or neither must update —
    // preventing silent drift.
    assert_eq!(default.name, from_arms.name);
    assert_eq!(default.arms, from_arms.arms);
    assert_eq!(default.mode, from_arms.mode);
    assert_eq!(default.kind, from_arms.kind);
    assert_eq!(default.runtime, from_arms.runtime);
    assert_eq!(default.call_arms, from_arms.call_arms);
    assert_eq!(
        default.megamorphism_threshold,
        from_arms.megamorphism_threshold
    );
    assert_eq!(
        default.runtime_name_template,
        from_arms.runtime_name_template
    );
    assert_eq!(default.span, from_arms.span);
}
