//! Validator types and parsing for field validation during decoding.

mod extract;
pub(super) mod parse;
mod suggest;

pub use extract::extract_validators;
use suggest::find_similar_validator;

// ============================================================================
// Validator types for field validation
// ============================================================================

/// A single validator with optional custom message
#[derive(Debug, Clone)]
pub struct ValidatorSpec {
    pub validator: Validator,
    pub custom_message: Option<String>,
}

/// All supported validators for field validation during decoding
#[derive(Debug, Clone, PartialEq)]
pub enum Validator {
    // String validators
    Email,
    Url,
    Uuid,
    MaxLength(usize),
    MinLength(usize),
    Length(usize),
    LengthRange(usize, usize),
    Pattern(String),
    NonEmpty,
    Trimmed,
    Lowercase,
    Uppercase,
    Capitalized,
    Uncapitalized,
    StartsWith(String),
    EndsWith(String),
    Includes(String),

    // Number validators
    GreaterThan(f64),
    GreaterThanOrEqualTo(f64),
    LessThan(f64),
    LessThanOrEqualTo(f64),
    Between(f64, f64),
    Int,
    NonNaN,
    Finite,
    Positive,
    NonNegative,
    Negative,
    NonPositive,
    MultipleOf(f64),
    Uint8,
    /// Combined check: must be an integer AND >= 0 (like `Uint8` without the upper bound)
    NonNegativeInt,

    // Array validators
    MaxItems(usize),
    MinItems(usize),
    ItemsCount(usize),

    // Date validators
    ValidDate,
    GreaterThanDate(String),
    GreaterThanOrEqualToDate(String),
    LessThanDate(String),
    LessThanOrEqualToDate(String),
    BetweenDate(String, String),

    // BigInt validators
    GreaterThanBigInt(String),
    GreaterThanOrEqualToBigInt(String),
    LessThanBigInt(String),
    LessThanOrEqualToBigInt(String),
    BetweenBigInt(String, String),
    PositiveBigInt,
    NonNegativeBigInt,
    NegativeBigInt,
    NonPositiveBigInt,

    // Custom validator
    Custom(CustomValidator),
}

/// What kind of value a validator looks at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Checked {
    Text,
    /// The length of text or of an array.
    Length,
    Number,
    Items,
    Date,
    BigInt,
    Anything,
}

impl Validator {
    fn checks(&self) -> Checked {
        match self {
            Self::Email
            | Self::Url
            | Self::Uuid
            | Self::Pattern(_)
            | Self::Trimmed
            | Self::Lowercase
            | Self::Uppercase
            | Self::Capitalized
            | Self::Uncapitalized
            | Self::StartsWith(_)
            | Self::EndsWith(_)
            | Self::Includes(_) => Checked::Text,
            Self::MaxLength(_)
            | Self::MinLength(_)
            | Self::Length(_)
            | Self::LengthRange(_, _)
            | Self::NonEmpty => Checked::Length,
            Self::GreaterThan(_)
            | Self::GreaterThanOrEqualTo(_)
            | Self::LessThan(_)
            | Self::LessThanOrEqualTo(_)
            | Self::Between(_, _)
            | Self::Int
            | Self::NonNaN
            | Self::Finite
            | Self::Positive
            | Self::NonNegative
            | Self::Negative
            | Self::NonPositive
            | Self::MultipleOf(_)
            | Self::Uint8
            | Self::NonNegativeInt => Checked::Number,
            Self::MaxItems(_) | Self::MinItems(_) | Self::ItemsCount(_) => Checked::Items,
            Self::ValidDate
            | Self::GreaterThanDate(_)
            | Self::GreaterThanOrEqualToDate(_)
            | Self::LessThanDate(_)
            | Self::LessThanOrEqualToDate(_)
            | Self::BetweenDate(_, _) => Checked::Date,
            Self::GreaterThanBigInt(_)
            | Self::GreaterThanOrEqualToBigInt(_)
            | Self::LessThanBigInt(_)
            | Self::LessThanOrEqualToBigInt(_)
            | Self::BetweenBigInt(_, _)
            | Self::PositiveBigInt
            | Self::NonNegativeBigInt
            | Self::NegativeBigInt
            | Self::NonPositiveBigInt => Checked::BigInt,
            Self::Custom(_) => Checked::Anything,
        }
    }

    /// Why this validator cannot check a value of type `ts_type`, when the
    /// type is plainly of another kind than the validator looks at: `email`
    /// on a `number`, `positive` on a `string[]`. `None` for a type whose
    /// kind is not written in it, such as a named type.
    pub fn misapplied_to(&self, ts_type: &str) -> Option<String> {
        let (wants, takes): (&str, &[Held]) = match self.checks() {
            Checked::Text => ("text", &[Held::Text]),
            Checked::Length => (
                "the length of text or of an array",
                &[Held::Text, Held::Array],
            ),
            Checked::Number => ("a number", &[Held::Number]),
            Checked::Items => ("the items of an array", &[Held::Array]),
            Checked::Date => ("a date", &[Held::Date]),
            Checked::BigInt => ("a bigint", &[Held::BigInt]),
            Checked::Anything => return None,
        };
        let held = Held::of(ts_type)?;
        (!takes.contains(&held)).then(|| {
            format!(
                "validator `{self:?}` checks {wants}, and `{}` is {}",
                ts_type.trim(),
                held.described()
            )
        })
    }
}

/// The kind of value a type plainly holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Held {
    Text,
    Number,
    Boolean,
    BigInt,
    Date,
    Array,
    Collection,
    Object,
}

impl Held {
    fn described(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Number => "a number",
            Self::Boolean => "a boolean",
            Self::BigInt => "a bigint",
            Self::Date => "a date",
            Self::Array => "an array",
            Self::Collection => "a collection",
            Self::Object => "an object",
        }
    }

    /// What `ts_type` holds once its `null` and `undefined` are set aside,
    /// when every other member is of one kind.
    fn of(ts_type: &str) -> Option<Self> {
        use crate::ts_syn::abi::ir::split_top_level_union;

        let ts_type = ts_type.trim();
        let mut kinds = split_top_level_union(ts_type)
            .unwrap_or_else(|| vec![ts_type])
            .into_iter()
            .map(str::trim)
            .filter(|member| !matches!(*member, "null" | "undefined"))
            .map(Self::of_member);
        let first = kinds.next()??;
        kinds.all(|kind| kind == Some(first)).then_some(first)
    }

    fn of_member(member: &str) -> Option<Self> {
        let quoted = ['"', '\'', '`']
            .into_iter()
            .any(|quote| member.len() > 1 && member.starts_with(quote) && member.ends_with(quote));
        Some(match member {
            "string" => Self::Text,
            _ if quoted => Self::Text,
            "number" => Self::Number,
            _ if member.parse::<f64>().is_ok() => Self::Number,
            "boolean" | "true" | "false" => Self::Boolean,
            "bigint" => Self::BigInt,
            _ => match super::TypeCategory::from_ts_type(member) {
                super::TypeCategory::Date => Self::Date,
                super::TypeCategory::Array(_) => Self::Array,
                super::TypeCategory::Map(_, _)
                | super::TypeCategory::Set(_)
                | super::TypeCategory::Record(_, _) => Self::Collection,
                super::TypeCategory::Primitive
                | super::TypeCategory::Optional(_)
                | super::TypeCategory::Nullable(_)
                | super::TypeCategory::Wrapper(_)
                | super::TypeCategory::Encodable(_)
                | super::TypeCategory::Unknown => {
                    if member.starts_with('[') {
                        Self::Array
                    } else if member.starts_with('{') {
                        Self::Object
                    } else {
                        return None;
                    }
                }
            },
        })
    }
}

/// A validator function the user supplies: `custom(isEven)` for one in scope
/// in the file, or `custom({ function: "isEven", source: "./validators" })`
/// for one the expansion imports itself.
#[derive(Debug, Clone, PartialEq)]
pub struct CustomValidator {
    /// The function to call, as written when it is in scope; with `source`,
    /// the name the module exports.
    pub function: String,
    /// The module to import `function` from, as written in an import of the
    /// file being expanded.
    pub source: Option<String>,
}

impl CustomValidator {
    /// The expression the generated check calls: `function` itself, or the
    /// private name its import is bound to.
    pub fn callee(&self) -> String {
        match &self.source {
            None => self.function.clone(),
            Some(source) => {
                let module: String = source
                    .trim_start_matches(['.', '/'])
                    .chars()
                    .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                    .collect();
                format!("__mf_{}__{module}", self.function)
            }
        }
    }
}

// ============================================================================
// Validator parsing errors
// ============================================================================

/// Error information from parsing a validator string
#[derive(Debug, Clone)]
pub struct ValidatorParseError {
    pub message: String,
    pub help: Option<String>,
}

impl ValidatorParseError {
    /// Create error for an unknown validator name
    pub fn unknown_validator(name: &str) -> Self {
        let similar = find_similar_validator(name);
        Self {
            message: format!("unknown validator '{}'", name),
            help: similar.map(|s| format!("did you mean '{}'?", s)),
        }
    }

    /// Create error for invalid arguments
    pub fn invalid_args(name: &str, reason: &str) -> Self {
        Self {
            message: format!("invalid arguments for '{}': {}", name, reason),
            help: None,
        }
    }
}
