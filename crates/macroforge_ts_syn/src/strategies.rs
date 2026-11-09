//! Proptest strategies for TypeScript type text, each type generated with
//! what a reader of its text should find in it, so a property can hold the
//! hand-written scanners of type text to the structure it was built from.

use proptest::collection::vec;
use proptest::prop_oneof;
use proptest::sample::select;
use proptest::strategy::{BoxedStrategy, Strategy};
use proptest::test_runner::Config;

/// The configuration of a property test of this workspace: a failing case is
/// reported, and pinned as a test of its own, rather than saved beside the
/// source.
pub fn property_config() -> Config {
    Config {
        failure_persistence: None,
        ..Config::default()
    }
}

/// A TypeScript type, built to be written out as text.
#[derive(Debug, Clone)]
pub enum TsType {
    /// A keyword or a type's name: `string`, `User`.
    Name(&'static str),
    /// A string literal type, as written: its quotes and escapes among it.
    Text(String),
    /// `T[]`
    Array(Box<TsType>),
    /// `[A, B?, ...C[]]`
    Tuple(Vec<TupleElement>),
    /// `Name<A, B>`
    Generic(&'static str, Vec<TsType>),
    /// `{ key: A; "other key": B }`, each key as written.
    Object(Vec<(String, TsType)>),
    /// `(a: A, b: B) => R`
    Function(Vec<TsType>, Box<TsType>),
    /// `C extends D ? A : B`
    Conditional(Box<[TsType; 4]>),
    /// `A | B`
    Union(Vec<TsType>),
    /// `A & B`
    Intersection(Vec<TsType>),
    /// `(T)`
    Grouped(Box<TsType>),
    /// `T["key"]`
    Indexed(Box<TsType>, String),
    /// `` `text${Name}` ``, its text as written.
    Template(String, &'static str),
}

/// An element of a [`TsType::Tuple`].
#[derive(Debug, Clone)]
pub struct TupleElement {
    pub element: TsType,
    pub kind: ElementKind,
}

/// How a tuple element is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    /// `T`
    Required,
    /// `T?`
    Optional,
    /// `...T[]`, its array written for it.
    Rest,
}

/// A type written out, with what its text holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Written {
    pub text: String,
    /// How deep it nests: one level for each bracket of any kind it opens,
    /// and one for each `[]` around what that suffix follows.
    pub depth: usize,
}

impl TsType {
    /// The type as TypeScript writes it, grouped where its precedence needs.
    pub fn written(&self) -> Written {
        match self {
            TsType::Name(name) => leaf(name),
            TsType::Text(text) => leaf(text),
            TsType::Array(element) => {
                let element = element.postfix_operand();
                Written {
                    text: format!("{}[]", element.text),
                    depth: element.depth + 1,
                }
            }
            TsType::Tuple(elements) => {
                let written: Vec<Written> = elements.iter().map(TupleElement::written).collect();
                bracketed('[', &written, ", ", ']')
            }
            TsType::Generic(name, arguments) => {
                let written: Vec<Written> = arguments.iter().map(TsType::written).collect();
                let arguments = bracketed('<', &written, ", ", '>');
                Written {
                    text: format!("{name}{}", arguments.text),
                    depth: arguments.depth,
                }
            }
            TsType::Object(members) => {
                let written: Vec<Written> = members
                    .iter()
                    .map(|(key, member)| {
                        let member = member.written();
                        Written {
                            text: format!("{key}: {}", member.text),
                            depth: member.depth,
                        }
                    })
                    .collect();
                let body = joined(written.into_iter(), "; ");
                Written {
                    text: format!("{{ {} }}", body.text),
                    depth: body.depth + 1,
                }
            }
            TsType::Function(parameters, returned) => {
                let written: Vec<Written> = parameters
                    .iter()
                    .enumerate()
                    .map(|(index, parameter)| {
                        let parameter = parameter.written();
                        Written {
                            // The first is named as a type is, which names no type.
                            text: match index {
                                0 => format!("T: {}", parameter.text),
                                _ => format!("p{index}: {}", parameter.text),
                            },
                            depth: parameter.depth,
                        }
                    })
                    .collect();
                let parameters = bracketed('(', &written, ", ", ')');
                let returned = returned.written();
                Written {
                    text: format!("{} => {}", parameters.text, returned.text),
                    depth: parameters.depth.max(returned.depth),
                }
            }
            TsType::Conditional(parts) => {
                let [checked, extended, then, otherwise] = parts.as_ref();
                let checked = checked.operand_where(TsType::runs_to_the_end);
                let extended = extended.operand_where(TsType::runs_to_the_end);
                let then = then.written();
                let otherwise = otherwise.written();
                Written {
                    text: format!(
                        "{} extends {} ? {} : {}",
                        checked.text, extended.text, then.text, otherwise.text
                    ),
                    depth: [checked.depth, extended.depth, then.depth, otherwise.depth]
                        .into_iter()
                        .max()
                        .unwrap_or_default(),
                }
            }
            TsType::Union(members) => joined(members.iter().map(TsType::union_operand), " | "),
            TsType::Intersection(members) => {
                joined(members.iter().map(TsType::intersection_operand), " & ")
            }
            TsType::Grouped(inner) => grouped(inner.written()),
            TsType::Template(text, name) => leaf(&format!("{text}${{{name}}}`")),
            TsType::Indexed(object, key) => {
                let object = object.postfix_operand();
                Written {
                    text: format!("{}[{key}]", object.text),
                    depth: object.depth.max(1),
                }
            }
        }
    }

    /// This type with each `from` it names by itself named `to`: a key, a
    /// string and a member of a qualified name are no such name.
    pub fn substituted(&self, from: &str, to: &'static str) -> TsType {
        let each = |members: &[TsType]| -> Vec<TsType> {
            members
                .iter()
                .map(|member| member.substituted(from, to))
                .collect()
        };
        let boxed = |inner: &TsType| Box::new(inner.substituted(from, to));
        match self {
            TsType::Name(name) if *name == from => TsType::Name(to),
            TsType::Name(_) | TsType::Text(_) => self.clone(),
            TsType::Template(text, name) if *name == from => TsType::Template(text.clone(), to),
            TsType::Template(..) => self.clone(),
            TsType::Array(inner) => TsType::Array(boxed(inner)),
            TsType::Tuple(elements) => TsType::Tuple(
                elements
                    .iter()
                    .map(|element| TupleElement {
                        element: element.element.substituted(from, to),
                        kind: element.kind,
                    })
                    .collect(),
            ),
            TsType::Generic(name, arguments) => TsType::Generic(name, each(arguments)),
            TsType::Object(members) => TsType::Object(
                members
                    .iter()
                    .map(|(key, member)| (key.clone(), member.substituted(from, to)))
                    .collect(),
            ),
            TsType::Function(parameters, returned) => {
                TsType::Function(each(parameters), boxed(returned))
            }
            TsType::Conditional(parts) => {
                let [checked, extended, then, otherwise] = parts.as_ref();
                TsType::Conditional(Box::new([
                    checked.substituted(from, to),
                    extended.substituted(from, to),
                    then.substituted(from, to),
                    otherwise.substituted(from, to),
                ]))
            }
            TsType::Union(members) => TsType::Union(each(members)),
            TsType::Intersection(members) => TsType::Intersection(each(members)),
            TsType::Grouped(inner) => TsType::Grouped(boxed(inner)),
            TsType::Indexed(object, key) => TsType::Indexed(boxed(object), key.clone()),
        }
    }

    /// The words this type is written with where a type stands, in order:
    /// the keyword `extends` among them, the first part of a qualified name,
    /// and no key, label or string.
    pub fn named(&self) -> Vec<&'static str> {
        let all = |members: &[TsType]| -> Vec<&'static str> {
            members.iter().flat_map(TsType::named).collect()
        };
        match self {
            TsType::Name(name) => name.split('.').take(1).collect(),
            TsType::Text(_) => Vec::new(),
            TsType::Template(_, name) => vec![name],
            TsType::Array(inner) | TsType::Grouped(inner) | TsType::Indexed(inner, _) => {
                inner.named()
            }
            TsType::Tuple(elements) => elements
                .iter()
                .flat_map(|element| element.element.named())
                .collect(),
            TsType::Generic(name, arguments) => {
                std::iter::once(*name).chain(all(arguments)).collect()
            }
            TsType::Object(members) => members
                .iter()
                .flat_map(|(_, member)| member.named())
                .collect(),
            TsType::Function(parameters, returned) => all(parameters)
                .into_iter()
                .chain(returned.named())
                .collect(),
            TsType::Conditional(parts) => {
                let [checked, extended, then, otherwise] = parts.as_ref();
                checked
                    .named()
                    .into_iter()
                    .chain(["extends"])
                    .chain(extended.named())
                    .chain(then.named())
                    .chain(otherwise.named())
                    .collect()
            }
            TsType::Union(members) | TsType::Intersection(members) => all(members),
        }
    }

    /// The members of the union this type is at its top level, as written.
    pub fn union_members(&self) -> Option<Vec<String>> {
        match self {
            TsType::Union(members) => Some(
                members
                    .iter()
                    .map(|member| member.union_operand().text)
                    .collect(),
            ),
            _ => None,
        }
    }

    /// The members of the intersection this type is at its top level, as
    /// written.
    pub fn intersection_members(&self) -> Option<Vec<String>> {
        match self {
            TsType::Intersection(members) => Some(
                members
                    .iter()
                    .map(|member| member.intersection_operand().text)
                    .collect(),
            ),
            _ => None,
        }
    }

    /// Whether a tuple in this type has a rest element other elements follow.
    pub fn has_early_rest(&self) -> bool {
        match self {
            TsType::Name(_) | TsType::Text(_) | TsType::Template(..) => false,
            TsType::Array(inner) | TsType::Grouped(inner) | TsType::Indexed(inner, _) => {
                inner.has_early_rest()
            }
            TsType::Tuple(elements) => {
                let last = elements.len().saturating_sub(1);
                elements.iter().enumerate().any(|(index, element)| {
                    (element.kind == ElementKind::Rest && index < last)
                        || element.element.has_early_rest()
                })
            }
            TsType::Generic(_, members)
            | TsType::Union(members)
            | TsType::Intersection(members) => members.iter().any(TsType::has_early_rest),
            TsType::Object(members) => members.iter().any(|(_, member)| member.has_early_rest()),
            TsType::Function(parameters, returned) => {
                parameters.iter().any(TsType::has_early_rest) || returned.has_early_rest()
            }
            TsType::Conditional(parts) => parts.iter().any(TsType::has_early_rest),
        }
    }

    /// Whether this type, unbracketed, would take a `|` or `&` written after
    /// it, or before it, as its own: a function's return type and a
    /// conditional's last branch run to the end of the text.
    fn runs_to_the_end(&self) -> bool {
        matches!(self, TsType::Function(..) | TsType::Conditional(_))
    }

    fn union_operand(&self) -> Written {
        self.operand_where(|member| member.runs_to_the_end() || matches!(member, TsType::Union(_)))
    }

    fn intersection_operand(&self) -> Written {
        self.operand_where(|member| {
            member.runs_to_the_end() || matches!(member, TsType::Union(_) | TsType::Intersection(_))
        })
    }

    fn postfix_operand(&self) -> Written {
        self.operand_where(|inner| {
            inner.runs_to_the_end() || matches!(inner, TsType::Union(_) | TsType::Intersection(_))
        })
    }

    fn operand_where(&self, needs_grouping: fn(&TsType) -> bool) -> Written {
        if needs_grouping(self) {
            grouped(self.written())
        } else {
            self.written()
        }
    }
}

impl TupleElement {
    /// The element as a tuple writes it.
    pub fn written(&self) -> Written {
        match self.kind {
            ElementKind::Required => self.element.written(),
            ElementKind::Optional => {
                let element = self.element.postfix_operand();
                Written {
                    text: format!("{}?", element.text),
                    depth: element.depth,
                }
            }
            ElementKind::Rest => {
                let array = TsType::Array(Box::new(self.element.clone())).written();
                Written {
                    text: format!("...{}", array.text),
                    depth: array.depth,
                }
            }
        }
    }
}

fn leaf(text: &str) -> Written {
    Written {
        text: text.to_string(),
        depth: 0,
    }
}

fn grouped(inner: Written) -> Written {
    Written {
        text: format!("({})", inner.text),
        depth: inner.depth + 1,
    }
}

fn joined(parts: impl Iterator<Item = Written>, separator: &str) -> Written {
    let parts: Vec<Written> = parts.collect();
    Written {
        text: parts
            .iter()
            .map(|part| part.text.as_str())
            .collect::<Vec<_>>()
            .join(separator),
        depth: parts
            .iter()
            .map(|part| part.depth)
            .max()
            .unwrap_or_default(),
    }
}

fn bracketed(open: char, parts: &[Written], separator: &str, close: char) -> Written {
    let inner = joined(parts.iter().cloned(), separator);
    Written {
        text: format!("{open}{}{close}", inner.text),
        depth: inner.depth + 1,
    }
}

/// Letters that mean something to a scanner of type text outside a string.
const HOSTILE: &[char] = &[
    '|', '&', ',', '<', '>', '(', ')', '[', ']', '{', '}', '=', '?', ':', ';', '.', '"', '\'', '`',
    '\\', '$', '/', '*', '@', '\n', ' ', 'a', 'Z', '0', 'é', '😀',
];

/// Text of any letters, those meaning something to a scanner most often.
pub fn hostile_text() -> impl Strategy<Value = String> {
    let letter = prop_oneof![
        4 => select(HOSTILE),
        1 => proptest::char::any(),
    ];
    vec(letter, 0..12).prop_map(|letters| letters.into_iter().collect())
}

/// `text` as a TypeScript string literal quoted with `quote`.
pub fn quoted(text: &str, quote: char) -> String {
    let mut literal = String::with_capacity(text.len() + 2);
    literal.push(quote);
    for letter in text.chars() {
        match letter {
            '\\' => literal.push_str("\\\\"),
            '\n' => literal.push_str("\\n"),
            '\r' => literal.push_str("\\r"),
            '\u{2028}' => literal.push_str("\\u2028"),
            '\u{2029}' => literal.push_str("\\u2029"),
            '$' if quote == '`' => literal.push_str("\\$"),
            _ if letter == quote => {
                literal.push('\\');
                literal.push(letter);
            }
            _ => literal.push(letter),
        }
    }
    literal.push(quote);
    literal
}

/// A string literal type of hostile text, in any of the three quotes.
pub fn string_literal() -> impl Strategy<Value = String> {
    (hostile_text(), select(&['"', '\'', '`'][..])).prop_map(|(text, quote)| quoted(&text, quote))
}

/// A type that holds no other.
fn leaf_type() -> impl Strategy<Value = TsType> {
    prop_oneof![
        select(
            &[
                "string",
                "number",
                "boolean",
                "null",
                "undefined",
                "User",
                "T",
                "NS.T",
                "Date"
            ][..]
        )
        .prop_map(TsType::Name),
        string_literal().prop_map(TsType::Text),
        (hostile_text(), select(&["T", "string", "User"][..])).prop_map(|(text, name)| {
            // The text of a template up to a substitution: quoted, its
            // closing backtick left off.
            let quoted = quoted(&text, '`');
            TsType::Template(
                quoted.get(..quoted.len() - 1).unwrap_or("`").to_string(),
                name,
            )
        }),
    ]
}

/// An object key: a name, or a string literal.
fn object_key() -> impl Strategy<Value = String> {
    prop_oneof![
        select(&["a", "b", "value", "$key", "_key", "T"][..]).prop_map(str::to_string),
        (hostile_text(), select(&['"', '\''][..])).prop_map(|(text, quote)| quoted(&text, quote)),
    ]
}

/// Any type, nested up to a few levels.
pub fn ts_type() -> BoxedStrategy<TsType> {
    leaf_type()
        .prop_recursive(5, 48, 4, |inner| {
            prop_oneof![
                inner
                    .clone()
                    .prop_map(|element| TsType::Array(Box::new(element))),
                vec(tuple_element(inner.clone()), 0..4).prop_map(TsType::Tuple),
                (
                    select(&["Array", "Map", "Set", "Record", "Promise"][..]),
                    vec(inner.clone(), 1..3)
                )
                    .prop_map(|(name, arguments)| TsType::Generic(name, arguments)),
                vec((object_key(), inner.clone()), 0..3).prop_map(TsType::Object),
                (vec(inner.clone(), 0..3), inner.clone()).prop_map(|(parameters, returned)| {
                    TsType::Function(parameters, Box::new(returned))
                }),
                [inner.clone(), inner.clone(), inner.clone(), inner.clone()]
                    .prop_map(|parts| TsType::Conditional(Box::new(parts))),
                vec(inner.clone(), 2..4).prop_map(TsType::Union),
                vec(inner.clone(), 2..4).prop_map(TsType::Intersection),
                inner
                    .clone()
                    .prop_map(|grouped| TsType::Grouped(Box::new(grouped))),
                (inner, string_literal())
                    .prop_map(|(object, key)| TsType::Indexed(Box::new(object), key)),
            ]
        })
        .boxed()
}

/// Any tuple type with at least one element.
pub fn tuple_type() -> impl Strategy<Value = TsType> {
    vec(tuple_element(ts_type()), 1..5).prop_map(TsType::Tuple)
}

/// An element of any kind over a type of `element`.
fn tuple_element(element: impl Strategy<Value = TsType>) -> impl Strategy<Value = TupleElement> {
    let kind = select(
        &[
            ElementKind::Required,
            ElementKind::Optional,
            ElementKind::Rest,
        ][..],
    );
    (element, kind).prop_map(|(element, kind)| TupleElement { element, kind })
}

/// Any union type, its members of any type.
pub fn union_type() -> impl Strategy<Value = TsType> {
    vec(ts_type(), 2..5).prop_map(TsType::Union)
}

/// Any intersection type, its members of any type.
pub fn intersection_type() -> impl Strategy<Value = TsType> {
    vec(ts_type(), 2..5).prop_map(TsType::Intersection)
}

/// A JavaScript value as a decorator's arguments write one: strings of
/// hostile text, numbers, names, and arrays, objects, calls and groups of
/// them. No string holds `*/`, which would end the comment it is written in.
pub fn js_value() -> BoxedStrategy<String> {
    let string = (hostile_text(), select(&['"', '\''][..]))
        .prop_map(|(text, quote)| quoted(&text.replace("*/", "* /"), quote));
    prop_oneof![
        string,
        select(&["1", "-2.5", "0x10", "x", "value", "null"][..]).prop_map(str::to_string),
    ]
    .prop_recursive(3, 16, 3, |inner| {
        prop_oneof![
            vec(inner.clone(), 0..3).prop_map(|items| format!("[{}]", items.join(", "))),
            vec(
                (select(&["a", "rename", "\"key\""][..]), inner.clone()),
                0..3
            )
            .prop_map(|members| {
                let members: Vec<String> = members
                    .into_iter()
                    .map(|(key, value)| format!("{key}: {value}"))
                    .collect();
                format!("{{ {} }}", members.join(", "))
            }),
            vec(inner.clone(), 0..3).prop_map(|items| format!("call({})", items.join(", "))),
            inner.prop_map(|grouped| format!("({grouped})")),
        ]
    })
    .boxed()
}
