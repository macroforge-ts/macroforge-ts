//! A property by its name, rendered so that a name which is not an
//! identifier, such as `first-name` or `0`, still names the property.

use crate::ts_syn::ToTsString;
use crate::ts_syn::scan::{Token, tokens};

use super::js_string;

/// Whether `name` can follow a `.` or stand unquoted as an object key.
/// Anything outside ASCII is quoted rather than judged.
pub fn is_identifier_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || matches!(first, '_' | '$'))
        && chars.all(|letter| letter.is_ascii_alphanumeric() || matches!(letter, '_' | '$'))
}

/// A property reached off an object: `.name`, or `["first-name"]`. It follows
/// the object in a template with nothing between them: `value@{member}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Member(String);

impl Member {
    pub fn of(name: &str) -> Self {
        Self(if is_identifier_name(name) {
            format!(".{name}")
        } else {
            format!("[{}]", js_string(name))
        })
    }
}

impl Member {
    /// A member of a dictionary by its key: `["name"]`. A type with a string
    /// index signature reads `.constructor` and `.toString` as the members
    /// every object has, and a key in brackets as one of its own.
    pub fn keyed(name: &str) -> Self {
        Self(format!("[{}]", js_string(name)))
    }
}

impl ToTsString for Member {
    fn to_ts_string(&self) -> String {
        self.0.clone()
    }
}

/// A property's key in an object literal or an object type: `name`, or
/// `"first-name"`.
pub fn property_key(name: &str) -> String {
    if is_identifier_name(name) {
        name.to_string()
    } else {
        js_string(name)
    }
}

/// Whether `source` reads the binding `name`: the word on its own, not as a
/// property of something else, a key of an object, or text in a string. A
/// spread (`...name`) and a template's `${name}` read it.
pub fn mentions(source: &str, name: &str) -> bool {
    let tokens = tokens(source);
    let significant = |token: &&Token<'_>| !token.text().trim().is_empty();
    tokens.iter().enumerate().any(|(index, token)| {
        if *token != Token::Ident(name) {
            return false;
        }
        let before: Vec<&str> = tokens
            .get(..index)
            .unwrap_or_default()
            .iter()
            .rev()
            .filter(significant)
            .take(3)
            .map(Token::text)
            .collect();
        let after = tokens
            .get(index + 1..)
            .unwrap_or_default()
            .iter()
            .find(significant)
            .map(Token::text);
        let property = before.first() == Some(&".") && before != [".", ".", "."];
        let key = after == Some(":") && matches!(before.first().copied(), Some("{" | ","));
        !property && !key
    })
}

/// `name` as a parameter of a function whose body is `body`: itself where
/// the body reads it, and `_name` where it does not, which is how an unread
/// parameter passes `noUnusedParameters`.
pub fn parameter_name(name: &str, body: &str) -> String {
    if mentions(body, name) {
        name.to_string()
    } else {
        format!("_{name}")
    }
}

/// `name` as part of a generated local's name: itself when it is an
/// identifier, and otherwise with each other character spelled as `$` and its
/// code point, so two names never share a local.
pub fn local_name_part(name: &str) -> String {
    if is_identifier_name(name) {
        return name.to_string();
    }
    name.chars()
        .map(|letter| {
            if letter.is_ascii_alphanumeric() || letter == '_' {
                letter.to_string()
            } else {
                format!("${:x}", u32::from(letter))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{mentions, parameter_name};

    #[test]
    fn a_binding_is_read_only_as_a_word_of_its_own() {
        assert!(mentions("f(value.id, ctx)", "ctx"));
        assert!(mentions("ctx.register(value)", "ctx"));
        assert!(!mentions(r#"result["ctx"] = value.ctx;"#, "ctx"));
        assert!(!mentions(r#"__fields.keeps("ctx")"#, "ctx"));
        assert!(!mentions("const ctxt = 1; context();", "ctx"));
        assert!(!mentions("", "ctx"));
        assert!(mentions("return [...value];", "value"));
        assert!(mentions("f(...ctx)", "ctx"));
        assert!(!mentions("other?.value", "value"));
        assert!(!mentions("['value', `value`, { value: 1 }]", "value"));
        assert!(mentions("`${value}`", "value"));
        assert!(mentions("{ value }", "value"));
        assert!(mentions("ok ? value : other", "value"));
    }

    #[test]
    fn an_unread_parameter_is_named_as_one() {
        assert_eq!(parameter_name("value", "return [value];"), "value");
        assert_eq!(parameter_name("value", "return [];"), "_value");
        assert_eq!(parameter_name("value", "[...value]"), "value");
    }

    mod properties {
        use oxc::allocator::Allocator;
        use oxc::ast::ast::IdentifierReference;
        use oxc::ast_visit::Visit;
        use oxc::parser::Parser;
        use oxc::span::SourceType;
        use proptest::collection::vec;
        use proptest::sample::select;
        use proptest::{prop_assert_eq, prop_assume, proptest};

        use super::super::mentions;
        use crate::ts_syn::strategies::property_config;

        /// Whether `source` reads the binding `name`, as oxc reads it.
        struct Reads<'name> {
            name: &'name str,
            found: bool,
        }

        impl<'a> Visit<'a> for Reads<'_> {
            fn visit_identifier_reference(&mut self, reference: &IdentifierReference<'a>) {
                self.found |= reference.name == self.name;
            }
        }

        fn read_by_oxc(source: &str, name: &str) -> Option<bool> {
            let allocator = Allocator::default();
            let parsed = Parser::new(&allocator, source, SourceType::ts()).parse();
            if !parsed.diagnostics.is_empty() {
                return None;
            }
            let mut reads = Reads { name, found: false };
            reads.visit_program(&parsed.program);
            Some(reads.found)
        }

        proptest! {
            #![proptest_config(property_config())]

            #[test]
            fn a_binding_is_mentioned_where_the_code_reads_it(
                elements in vec(
                    select(&[
                        "value", "valueX", "xvalue", "_value", "$value", "value2", "a.value", "a?.value",
                        "...value", "value.x", "value[0]", "\"value\"", "'value'", "`value`", "`${value}`",
                        "{ value }", "{ value: 1 }", "{ a: value }", "{ 'value': 1 }", "{ ...value }",
                        "f(value)", "x.value.y", "a['value']", "other", "1",
                    ][..]),
                    0..5,
                ),
            ) {
                let source = format!("const read = [{}];", elements.join(", "));
                let read = read_by_oxc(&source, "value");
                prop_assume!(read.is_some());
                prop_assert_eq!(Some(mentions(&source, "value")), read, "{}", source);
            }
        }
    }
}
