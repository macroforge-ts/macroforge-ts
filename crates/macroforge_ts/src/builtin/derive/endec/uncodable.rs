//! Types whose values the endec derives cannot code: ones JSON has no form
//! for, and ones whose members only the type checker can read.

use crate::ts_syn::abi::ir::split_top_level_union;
use oxc::allocator::Allocator;
use oxc::ast::ast::{Statement, TSType, TSTypeName, TSTypeOperatorOperator};
use oxc::parser::Parser;
use oxc::span::SourceType;

/// Why a value of type `ts_type` cannot be coded by its type, when it
/// cannot: it is a function, a symbol or of a type with no value, or its type
/// is one only the type checker can work out. A union is as its members are.
pub(crate) fn uncodable(ts_type: &str) -> Option<String> {
    for member in split_top_level_union(ts_type).unwrap_or_else(|| vec![ts_type]) {
        let member = member.trim();
        if let Some(Err(why)) = template_text(member) {
            return Some(format!(
                "`{member}` is a template literal type {why}; code the field that holds it with `encodeWith` and `decodeWith`, or leave it out with `skip`"
            ));
        }
    }
    let source = format!("type Coded = {ts_type};");
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, &source, SourceType::ts()).parse();
    let Some(Statement::TSTypeAliasDeclaration(alias)) = parsed.program.body.first() else {
        return None;
    };
    let what = reason(&alias.type_annotation)?;
    Some(format!(
        "`{ts_type}` is {what}; code the field that holds it with `encodeWith` and `decodeWith`, or leave it out with `skip`"
    ))
}

/// What the text at a hole of a template literal type has to be.
#[derive(Debug, PartialEq)]
pub(crate) enum TemplateHole {
    /// `${string}`: any text.
    Text,
    /// `${number}`: the text of a decimal number.
    Number,
    /// `${bigint}`: the text of a whole number.
    BigInt,
    /// A literal or a union of literals: one of these texts.
    OneOf(Vec<String>),
}

/// A template literal type as its values are told: the fixed parts of its
/// text in order, and the hole between each two.
#[derive(Debug, PartialEq)]
pub(crate) struct TemplateText {
    pub(crate) texts: Vec<String>,
    pub(crate) holes: Vec<TemplateHole>,
}

/// How the text of the template literal type `ts_type` is told, or why a
/// derive cannot tell it. `None` when `ts_type` is no template literal type.
pub(crate) fn template_text(ts_type: &str) -> Option<Result<TemplateText, String>> {
    let body = ts_type.strip_prefix('`')?.strip_suffix('`')?;
    Some(template_body(body))
}

fn template_body(body: &str) -> Result<TemplateText, String> {
    if body.contains('\\') {
        return Err("that writes an escape, whose text a derive does not read".to_string());
    }
    let mut texts = Vec::new();
    let mut holes = Vec::new();
    let mut rest = body;
    while let Some((text, after)) = rest.split_once("${") {
        let Some((hole, tail)) = after.split_once('}') else {
            return Err("with a hole that does not close".to_string());
        };
        // Nothing says where one hole ends and the next begins.
        if text.is_empty() && !holes.is_empty() {
            return Err("with two holes side by side, which nothing tells apart".to_string());
        }
        texts.push(text.to_string());
        holes.push(template_hole(hole.trim())?);
        rest = tail;
    }
    texts.push(rest.to_string());
    Ok(TemplateText { texts, holes })
}

fn template_hole(hole: &str) -> Result<TemplateHole, String> {
    match hole {
        "string" => return Ok(TemplateHole::Text),
        "number" => return Ok(TemplateHole::Number),
        "bigint" => return Ok(TemplateHole::BigInt),
        _ => {}
    }
    let whole_number = |member: &str| {
        let digits = member.strip_prefix('-').unwrap_or(member);
        !digits.is_empty() && digits.bytes().all(|digit| digit.is_ascii_digit())
    };
    let mut texts = Vec::new();
    for member in split_top_level_union(hole).unwrap_or_else(|| vec![hole]) {
        let member = member.trim();
        let quoted = ['\'', '"'].into_iter().find_map(|quote| {
            member
                .strip_prefix(quote)
                .and_then(|text| text.strip_suffix(quote))
        });
        match (member, quoted) {
            ("boolean", _) => texts.extend(["true".to_string(), "false".to_string()]),
            ("true" | "false" | "null" | "undefined", _) => texts.push(member.to_string()),
            (_, Some(text)) => texts.push(text.to_string()),
            _ if whole_number(member) => texts.push(member.to_string()),
            _ => {
                return Err(format!(
                    "whose `${{{hole}}}` takes text a derive cannot check"
                ));
            }
        }
    }
    Ok(TemplateHole::OneOf(texts))
}

fn reason(ts_type: &TSType<'_>) -> Option<&'static str> {
    const FUNCTION: &str = "a function, which JSON has no form for";
    const SYMBOL: &str = "a symbol, which JSON has no form for";
    match ts_type {
        TSType::TSFunctionType(_) | TSType::TSConstructorType(_) => Some(FUNCTION),
        TSType::TSSymbolKeyword(_) => Some(SYMBOL),
        TSType::TSVoidKeyword(_) | TSType::TSNeverKeyword(_) => {
            Some("a type with no value for JSON to hold")
        }
        TSType::TSMappedType(_) => {
            Some("a mapped type, whose members only the type checker can read")
        }
        TSType::TSConditionalType(_) => {
            Some("a conditional type, which only the type checker can resolve")
        }
        TSType::TSTypeQuery(_) => Some("a `typeof` type, which only the type checker can resolve"),
        TSType::TSIndexedAccessType(_) => {
            Some("an indexed access type, which only the type checker can resolve")
        }
        TSType::TSTypeOperatorType(operator) => match operator.operator {
            TSTypeOperatorOperator::Keyof => {
                Some("a `keyof` type, which only the type checker can resolve")
            }
            TSTypeOperatorOperator::Unique => Some(SYMBOL),
            TSTypeOperatorOperator::Readonly => reason(&operator.type_annotation),
        },
        TSType::TSParenthesizedType(grouped) => reason(&grouped.type_annotation),
        TSType::TSUnionType(union) => union.types.iter().find_map(reason),
        TSType::TSTypeReference(reference) => match &reference.type_name {
            TSTypeName::IdentifierReference(name) if name.name == "Function" => Some(FUNCTION),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{TemplateHole, TemplateText, template_text, uncodable};

    #[test]
    fn a_template_literal_type_is_told_by_its_text_and_holes() {
        assert_eq!(template_text("string"), None);
        assert_eq!(
            template_text("`id-${number}`"),
            Some(Ok(TemplateText {
                texts: vec!["id-".to_string(), String::new()],
                holes: vec![TemplateHole::Number],
            }))
        );
        assert_eq!(
            template_text("`${string}/${'a' | \"b\" | 3 | boolean}`"),
            Some(Ok(TemplateText {
                texts: vec![String::new(), "/".to_string(), String::new()],
                holes: vec![
                    TemplateHole::Text,
                    TemplateHole::OneOf(
                        ["a", "b", "3", "true", "false"]
                            .map(str::to_string)
                            .to_vec()
                    ),
                ],
            }))
        );
        assert_eq!(
            template_text("`plain`"),
            Some(Ok(TemplateText {
                texts: vec!["plain".to_string()],
                holes: vec![],
            }))
        );
        for unread in [
            "`v-${Version}`",
            "`${number}${number}`",
            "`a\\n${string}`",
            "`${1.5}`",
        ] {
            assert!(matches!(template_text(unread), Some(Err(_))), "{unread}");
            assert!(uncodable(unread).is_some(), "{unread}");
        }
        assert!(uncodable("string | `v-${Version}`").is_some());
    }

    #[test]
    fn a_type_json_cannot_hold_or_a_derive_cannot_read_is_uncodable() {
        for ts_type in [
            "() => void",
            "(value: number) => string",
            "new () => Date",
            "Function",
            "symbol",
            "unique symbol",
            "void",
            "never",
            "{ [K in Keys]: number }",
            "T extends string ? A : B",
            "typeof defaults",
            "Config['mode']",
            "keyof Config",
            "string | (() => void)",
            "readonly (() => void)[] | (() => void)",
        ] {
            assert!(uncodable(ts_type).is_some(), "{ts_type}");
        }
    }

    #[test]
    fn a_type_a_derive_reads_or_carries_is_left_alone() {
        for ts_type in [
            "string",
            "number | null",
            "unknown",
            "any",
            "object",
            "Date",
            "User",
            "string[]",
            "readonly string[]",
            "Map<string, () => void>",
            "{ run: () => void }",
            "[number, string]",
            "`id-${number}`",
            "Record<'a' | 'b', number>",
        ] {
            assert_eq!(uncodable(ts_type), None, "{ts_type}");
        }
    }
}
