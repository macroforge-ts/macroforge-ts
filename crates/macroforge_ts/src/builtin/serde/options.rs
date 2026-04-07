//! Serde container-level and field-level option types and parsing.

use super::helpers::{extract_named_string, has_flag};
use super::validators::{ValidatorSpec, extract_validators};
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

/// Enum tagging strategy for serialized unions.
/// Mirrors Rust serde's enum representation options.
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

/// Container-level serde options (on the class/interface itself)
#[derive(Debug, Clone, Default)]
pub struct SerdeContainerOptions {
    pub rename_all: RenameAll,
    pub deny_unknown_fields: bool,
    /// The tagging mode for this container. Determines how type discrimination
    /// is handled in serialized union representations.
    pub tagging: TaggingMode,
}

impl SerdeContainerOptions {
    pub fn from_decorators(decorators: &[DecoratorIR]) -> Self {
        let mut opts = Self::default();
        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case("serde") {
                continue;
            }
            let args = decorator.args_src.trim();

            if let Some(rename_all) = extract_named_string(args, "renameAll")
                && let Ok(convention) = rename_all.parse::<RenameAll>()
            {
                opts.rename_all = convention;
            }

            if has_flag(args, "denyUnknownFields") {
                opts.deny_unknown_fields = true;
            }

            // Resolve tagging mode from combination of attributes
            let tag = extract_named_string(args, "tag");
            let content = extract_named_string(args, "content");
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
    /// Used by individual class/interface serializers that always embed a tag.
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

/// Field-level serde options
#[derive(Debug, Clone, Default)]
pub struct SerdeFieldOptions {
    pub skip: bool,
    pub skip_serializing: bool,
    pub skip_deserializing: bool,
    pub rename: Option<String>,
    pub default: bool,
    pub default_expr: Option<String>,
    pub flatten: bool,
    pub validators: Vec<ValidatorSpec>,
    /// Custom serialization function name (like Rust's `#[serde(serialize_with)]`)
    pub serialize_with: Option<String>,
    /// Custom deserialization function name (like Rust's `#[serde(deserialize_with)]`)
    pub deserialize_with: Option<String>,
    /// Format hint for serialization/deserialization (e.g., "decimal" to serialize numbers as strings)
    pub format: Option<String>,
}

/// Result of parsing field options, containing both options and any diagnostics
#[derive(Debug, Clone, Default)]
pub struct SerdeFieldParseResult {
    pub options: SerdeFieldOptions,
    pub diagnostics: DiagnosticCollector,
}

impl SerdeFieldOptions {
    /// Parse field options from decorators, collecting diagnostics for invalid configurations
    pub fn from_decorators(decorators: &[DecoratorIR], field_name: &str) -> SerdeFieldParseResult {
        let mut opts = Self::default();
        let mut diagnostics = DiagnosticCollector::new();

        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case("serde") {
                continue;
            }
            let args = decorator.args_src.trim();
            let decorator_span = decorator.span;

            if has_flag(args, "skip") {
                opts.skip = true;
            }
            if has_flag(args, "skipSerializing") {
                opts.skip_serializing = true;
            }
            if has_flag(args, "skipDeserializing") {
                opts.skip_deserializing = true;
            }
            if has_flag(args, "flatten") {
                opts.flatten = true;
            }

            // Check for default (both boolean flag and expression)
            if let Some(default_expr) = extract_named_string(args, "default") {
                opts.default = true;
                opts.default_expr = Some(default_expr);
            } else if has_flag(args, "default") {
                opts.default = true;
            }

            if let Some(rename) = extract_named_string(args, "rename") {
                opts.rename = Some(rename);
            }

            // Parse custom serialization/deserialization functions (like Rust's serde)
            if let Some(fn_name) = extract_named_string(args, "serializeWith") {
                opts.serialize_with = Some(fn_name);
            }
            if let Some(fn_name) = extract_named_string(args, "deserializeWith") {
                opts.deserialize_with = Some(fn_name);
            }

            if let Some(format) = extract_named_string(args, "format") {
                opts.format = Some(format);
            }

            // Extract validators with diagnostic collection
            let validators = extract_validators(args, decorator_span, field_name, &mut diagnostics);
            opts.validators.extend(validators);
        }

        SerdeFieldParseResult {
            options: opts,
            diagnostics,
        }
    }

    pub fn should_serialize(&self) -> bool {
        !self.skip && !self.skip_serializing
    }

    pub fn should_deserialize(&self) -> bool {
        !self.skip && !self.skip_deserializing
    }
}
