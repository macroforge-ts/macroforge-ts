//! Endec container-level and field-level option types and parsing.

use super::helpers::extract_named_value;
use super::validators::{ValidatorSpec, extract_validators};
use crate::builtin::derive_common::has_flag;
use crate::ts_syn::abi::{DecoratorIR, DiagnosticCollector};

/// Naming convention for JSON field renaming
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum RenameAll {
    #[default]
    None,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    PascalCase,
}

impl std::str::FromStr for RenameAll {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().replace(['-', '_'], "").as_str() {
            "camelcase" => Ok(Self::CamelCase),
            "snakecase" => Ok(Self::SnakeCase),
            "screamingsnakecase" => Ok(Self::ScreamingSnakeCase),
            "kebabcase" => Ok(Self::KebabCase),
            "pascalcase" => Ok(Self::PascalCase),
            _ => Err(()),
        }
    }
}

impl RenameAll {
    pub fn apply(&self, name: &str) -> String {
        use convert_case::{Case, Casing};
        match self {
            Self::None => name.to_string(),
            Self::CamelCase => name.to_case(Case::Camel),
            Self::SnakeCase => name.to_case(Case::Snake),
            Self::ScreamingSnakeCase => name.to_case(Case::UpperSnake),
            Self::KebabCase => name.to_case(Case::Kebab),
            Self::PascalCase => name.to_case(Case::Pascal),
        }
    }
}

/// Enum tagging strategy for encoded unions.
/// Mirrors Rust endec's enum representation options.
#[derive(Debug, Clone, PartialEq)]
pub enum TaggingMode {
    /// Internally tagged: `{ tag: "TypeName", ...fields }`
    /// This is the default (using `__type` as the tag field).
    InternallyTagged { tag: String },
    /// Externally tagged: `{ "TypeName": { ...fields } }` or just `"TypeName"` for unit variants
    ExternallyTagged,
    /// Adjacently tagged: `{ tag: "TypeName", content: { ...fields } }`
    AdjacentlyTagged { tag: String, content: String },
    /// Untagged: raw value with no type information wrapper
    Untagged,
}

impl Default for TaggingMode {
    fn default() -> Self {
        Self::InternallyTagged {
            tag: "__type".to_string(),
        }
    }
}

/// Container-level endec options (on the class/interface itself)
#[derive(Debug, Clone, Default)]
pub struct EndecContainerOptions {
    pub rename_all: RenameAll,
    pub deny_unknown_fields: bool,
    /// The tagging mode for this container. Determines how type discrimination
    /// is handled in encoded union representations.
    pub tagging: TaggingMode,
}

impl EndecContainerOptions {
    pub fn from_decorators(decorators: &[DecoratorIR]) -> Self {
        let mut opts = Self::default();
        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case("endec") {
                continue;
            }
            let args = decorator.args_src.trim();

            if let Some(rename_all) = extract_named_value(args, "renameAll")
                && let Ok(convention) = rename_all.parse::<RenameAll>()
            {
                opts.rename_all = convention;
            }

            if has_flag(args, "denyUnknownFields") {
                opts.deny_unknown_fields = true;
            }

            // Resolve tagging mode from combination of attributes
            let tag = extract_named_value(args, "tag");
            let content = extract_named_value(args, "content");
            let untagged = has_flag(args, "untagged");
            let externally_tagged = has_flag(args, "externallyTagged");

            if untagged {
                opts.tagging = TaggingMode::Untagged;
            } else if externally_tagged {
                opts.tagging = TaggingMode::ExternallyTagged;
            } else if let Some(tag_name) = tag {
                if let Some(content_name) = content {
                    opts.tagging = TaggingMode::AdjacentlyTagged {
                        tag: tag_name,
                        content: content_name,
                    };
                } else {
                    opts.tagging = TaggingMode::InternallyTagged { tag: tag_name };
                }
            }
            // else: default remains InternallyTagged { tag: "__type" }
        }
        opts
    }

    /// Returns the tag field name for internally-tagged or adjacently-tagged modes.
    /// Returns None for externally-tagged and untagged modes.
    pub fn tag_field(&self) -> Option<&str> {
        match &self.tagging {
            TaggingMode::InternallyTagged { tag } => Some(tag.as_str()),
            TaggingMode::AdjacentlyTagged { tag, .. } => Some(tag.as_str()),
            _ => None,
        }
    }

    /// Returns the discriminator field name with a fallback default of `"__type"`.
    /// Used by individual class/interface encoders that always embed a tag.
    pub fn tag_field_or_default(&self) -> &str {
        match &self.tagging {
            TaggingMode::InternallyTagged { tag } => tag.as_str(),
            TaggingMode::AdjacentlyTagged { tag, .. } => tag.as_str(),
            _ => "__type",
        }
    }

    /// Returns the content field name for adjacently-tagged mode.
    pub fn content_field(&self) -> Option<&str> {
        match &self.tagging {
            TaggingMode::AdjacentlyTagged { content, .. } => Some(content.as_str()),
            _ => None,
        }
    }
}

/// Field-level endec options
#[derive(Debug, Clone, Default)]
pub struct EndecFieldOptions {
    pub skip: bool,
    pub skip_encoding: bool,
    pub skip_decoding: bool,
    pub rename: Option<String>,
    pub default: bool,
    pub default_expr: Option<String>,
    pub flatten: bool,
    pub validators: Vec<ValidatorSpec>,
    /// Custom encoding function name (like Rust's `#[serde(serialize_with)]`)
    pub encode_with: Option<String>,
    /// Custom decoding function name (like Rust's `#[serde(deserialize_with)]`)
    pub decode_with: Option<String>,
    /// Format hint for encoding/decoding (e.g., "decimal" to encode numbers as strings)
    pub format: Option<String>,
}

/// Result of parsing field options, containing both options and any diagnostics
#[derive(Debug, Clone, Default)]
pub struct EndecFieldParseResult {
    pub options: EndecFieldOptions,
    pub diagnostics: DiagnosticCollector,
}

impl EndecFieldOptions {
    /// Parse field options from decorators, collecting diagnostics for invalid configurations
    pub fn from_decorators(decorators: &[DecoratorIR], field_name: &str) -> EndecFieldParseResult {
        let mut opts = Self::default();
        let mut diagnostics = DiagnosticCollector::new();

        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case("endec") {
                continue;
            }
            let args = decorator.args_src.trim();
            let decorator_span = decorator.span;

            if has_flag(args, "skip") {
                opts.skip = true;
            }
            if has_flag(args, "skipEncoding") {
                opts.skip_encoding = true;
            }
            if has_flag(args, "skipDecoding") {
                opts.skip_decoding = true;
            }
            if has_flag(args, "flatten") {
                opts.flatten = true;
            }

            // Check for default (both boolean flag and expression)
            if let Some(default_expr) = extract_named_value(args, "default") {
                opts.default = true;
                opts.default_expr = Some(default_expr);
            } else if has_flag(args, "default") {
                opts.default = true;
            }

            if let Some(rename) = extract_named_value(args, "rename") {
                opts.rename = Some(rename);
            }

            // Parse custom encoding/decoding functions (like Rust's endec)
            if let Some(fn_name) = extract_named_value(args, "encodeWith") {
                opts.encode_with = Some(fn_name);
            }
            if let Some(fn_name) = extract_named_value(args, "decodeWith") {
                opts.decode_with = Some(fn_name);
            }

            if let Some(format) = extract_named_value(args, "format") {
                opts.format = Some(format);
            }

            // Extract validators with diagnostic collection
            let validators = extract_validators(args, decorator_span, field_name, &mut diagnostics);
            opts.validators.extend(validators);
        }

        EndecFieldParseResult {
            options: opts,
            diagnostics,
        }
    }

    pub fn should_encode(&self) -> bool {
        !self.skip && !self.skip_encoding
    }

    pub fn should_decode(&self) -> bool {
        !self.skip && !self.skip_decoding
    }
}
