//! Did-you-mean suggestions for misspelled validator names.

/// Every validator name, for typo detection.
pub(super) const KNOWN_VALIDATORS: &[&str] = &[
    "email",
    "url",
    "uuid",
    "maxLength",
    "minLength",
    "length",
    "pattern",
    "nonEmpty",
    "trimmed",
    "lowercase",
    "uppercase",
    "capitalized",
    "uncapitalized",
    "startsWith",
    "endsWith",
    "includes",
    "greaterThan",
    "greaterThanOrEqualTo",
    "lessThan",
    "lessThanOrEqualTo",
    "between",
    "int",
    "nonNaN",
    "finite",
    "positive",
    "nonNegative",
    "negative",
    "nonPositive",
    "multipleOf",
    "uint8",
    "nonNegativeInt",
    "maxItems",
    "minItems",
    "itemsCount",
    "validDate",
    "greaterThanDate",
    "greaterThanOrEqualToDate",
    "lessThanDate",
    "lessThanOrEqualToDate",
    "betweenDate",
    "positiveBigInt",
    "nonNegativeBigInt",
    "negativeBigInt",
    "nonPositiveBigInt",
    "greaterThanBigInt",
    "greaterThanOrEqualToBigInt",
    "lessThanBigInt",
    "lessThanOrEqualToBigInt",
    "betweenBigInt",
    "custom",
];

/// A known validator within two edits of `name`, for typo suggestions.
pub(super) fn find_similar_validator(name: &str) -> Option<&'static str> {
    let name_lower = name.to_lowercase();
    KNOWN_VALIDATORS
        .iter()
        .filter_map(|v| {
            let dist = strsim::levenshtein(&v.to_lowercase(), &name_lower);
            if dist <= 2 { Some((*v, dist)) } else { None }
        })
        .min_by_key(|(_, dist)| *dist)
        .map(|(v, _)| v)
}
