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
