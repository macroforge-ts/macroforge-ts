//! Parses Gigaform decorators and field configurations from TypeScript interfaces.

use macroforge_ts::ts_syn::{Attribute, DataClass, DataInterface, DeriveInput, FieldIR, InterfaceFieldIR, Visibility};
use serde::Deserialize;

/// Container-level options from `@gigaform({ ... })` decorator.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GigaformOptions {
    /// i18n key prefix for error messages (e.g., "userForm" -> m.userForm_name_required())
    pub i18n_prefix: Option<String>,
    /// Optional function name to call for default overrides (e.g., "getCustomDefaults")
    pub default_override: Option<String>,
}

impl GigaformOptions {
    /// Returns true if this form uses i18n features.
    pub fn uses_i18n(&self) -> bool {
        self.i18n_prefix.is_some()
    }
}

/// Field-level options from `@gigaform({ ... })` decorator on a field.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GigaformFieldOptions {
    /// Async validators (function names)
    pub validate_async: Option<Vec<String>>,
    /// i18n key for field label
    pub label_key: Option<String>,
    /// i18n key for field description
    pub description_key: Option<String>,
    /// i18n key for field placeholder
    pub placeholder_key: Option<String>,
}

// =============================================================================
// Union Configuration Types
// =============================================================================

/// Configuration for a discriminated union form.
#[derive(Debug, Clone)]
pub struct UnionConfig {
    /// How the union is discriminated
    pub mode: UnionMode,
    /// Per-variant configurations
    pub variants: Vec<UnionVariantConfig>,
}

/// How a union is discriminated.
#[derive(Debug, Clone)]
pub enum UnionMode {
    /// Tagged union with explicit discriminant field: @serde({ tag: "kind" })
    Tagged { field: String },
    /// Untagged union - infer variant from structure: @serde({ untagged: true })
    Untagged,
}

/// Configuration for a single union variant.
#[derive(Debug, Clone)]
pub struct UnionVariantConfig {
    /// The discriminant value for this variant (e.g., "card", "bank")
    pub discriminant_value: String,
    /// Fields specific to this variant
    pub fields: Vec<ParsedField>,
}

// =============================================================================
// Enum Configuration Types
// =============================================================================

/// Configuration for an enum step/variant form.
#[derive(Debug, Clone)]
pub struct EnumFormConfig {
    /// Parsed variants with their metadata
    pub variants: Vec<EnumVariantFormConfig>,
}

/// Configuration for a single enum variant.
#[derive(Debug, Clone)]
pub struct EnumVariantFormConfig {
    /// Variant name (e.g., "PersonalInfo")
    pub name: String,
    /// Display label (Title Case of name, e.g., "Personal Info")
    pub label: String,
    /// Optional description from @gigaform({ description: "..." })
    pub description: Option<String>,
    /// The enum value expression (for accessing EnumName.VariantName)
    pub value_expr: String,
}

// =============================================================================
// Field Configuration Types
// =============================================================================

/// Parsed field information with all metadata.
#[derive(Debug, Clone)]
pub struct ParsedField {
    /// Field name
    pub name: String,
    /// TypeScript type annotation
    pub ts_type: String,
    /// Whether the field is optional (has `?`)
    pub optional: bool,
    /// Whether this is a nested object type (references another Gigaform type)
    pub is_nested: bool,
    /// The nested type name (if is_nested is true)
    pub nested_type: Option<String>,
    /// Whether this is an array type
    pub is_array: bool,
    /// The array element type (if is_array is true)
    pub array_element_type: Option<String>,
    /// Sync validators from @serde
    pub validators: Vec<ValidatorSpec>,
    /// Async validators from @gigaform
    pub async_validators: Vec<String>,
    /// Controller configuration (if any controller decorator present)
    pub controller: Option<ParsedController>,
    /// Field-level gigaform options
    pub field_options: GigaformFieldOptions,
}

impl ParsedField {
    /// Returns true if this field uses i18n features.
    pub fn uses_i18n(&self) -> bool {
        self.field_options.label_key.is_some()
            || self.field_options.description_key.is_some()
            || self.field_options.placeholder_key.is_some()
    }
}

/// A validator specification parsed from @serde({ validate: [...] })
#[derive(Debug, Clone)]
pub struct ValidatorSpec {
    /// The validator name/function (e.g., "minLength", "email", "pattern")
    pub name: String,
    /// Arguments to the validator (e.g., ["2"] for minLength(2))
    pub args: Vec<String>,
    /// Custom error message (if provided)
    pub message: Option<String>,
}

// =============================================================================
// Controller Types and Options (merged from field_controllers)
// =============================================================================

/// All supported controller types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerType {
    Text,
    TextArea,
    Number,
    Toggle,
    Switch,
    Checkbox,
    Select,
    RadioGroup,
    Combobox,
    ComboboxMultiple,
    Hidden,
    Tags,
    DateTime,
    ArrayFieldset,
    EnumFieldset,
    SiteFieldset,
    Phone,
    Email,
}

impl ControllerType {
    /// Returns the decorator name for this controller type.
    pub fn decorator_name(&self) -> &'static str {
        match self {
            Self::Text => "textController",
            Self::TextArea => "textAreaController",
            Self::Number => "numberController",
            Self::Toggle => "toggleController",
            Self::Switch => "switchController",
            Self::Checkbox => "checkboxController",
            Self::Select => "selectController",
            Self::RadioGroup => "radioGroupController",
            Self::Combobox => "comboboxController",
            Self::ComboboxMultiple => "comboboxMultipleController",
            Self::Hidden => "hiddenController",
            Self::Tags => "tagsController",
            Self::DateTime => "dateTimeController",
            Self::ArrayFieldset => "arrayFieldsetController",
            Self::EnumFieldset => "enumFieldsetController",
            Self::SiteFieldset => "siteFieldsetController",
            Self::Phone => "phoneFieldController",
            Self::Email => "emailFieldController",
        }
    }

    /// Returns the TypeScript controller type name.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Text => "TextFieldController",
            Self::TextArea => "TextAreaFieldController",
            Self::Number => "NumberFieldController",
            Self::Toggle => "ToggleFieldController",
            Self::Switch => "SwitchFieldController",
            Self::Checkbox => "CheckboxFieldController",
            Self::Select => "SelectFieldController",
            Self::RadioGroup => "RadioGroupFieldController",
            Self::Combobox => "ComboboxFieldController",
            Self::ComboboxMultiple => "ComboboxMultipleFieldController",
            Self::Hidden => "HiddenFieldController",
            Self::Tags => "TagsFieldController",
            Self::DateTime => "DateTimeFieldsetController",
            Self::ArrayFieldset => "ArrayFieldsetController",
            Self::EnumFieldset => "EnumFieldsetController",
            Self::SiteFieldset => "SiteFieldsetController",
            Self::Phone => "PhoneFieldController",
            Self::Email => "EmailFieldController",
        }
    }

    /// Returns all controller types for iteration.
    pub fn all() -> &'static [ControllerType] {
        &[
            Self::Text,
            Self::TextArea,
            Self::Number,
            Self::Toggle,
            Self::Switch,
            Self::Checkbox,
            Self::Select,
            Self::RadioGroup,
            Self::Combobox,
            Self::ComboboxMultiple,
            Self::Hidden,
            Self::Tags,
            Self::DateTime,
            Self::ArrayFieldset,
            Self::EnumFieldset,
            Self::SiteFieldset,
            Self::Phone,
            Self::Email,
        ]
    }
}

/// Common options for all controller types.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseControllerOptions {
    /// Label text for the field.
    pub label: Option<String>,
    /// Alternative name for label (labelText).
    pub label_text: Option<String>,
    /// Description/help text.
    pub description: Option<String>,
    /// Placeholder text.
    pub placeholder: Option<String>,
    /// Whether the field is required.
    pub required: Option<bool>,
    /// Whether the field is disabled.
    pub disabled: Option<bool>,
    /// Whether the field is read-only.
    #[serde(alias = "readOnly")]
    pub readonly: Option<bool>,
    /// CSS class for label background.
    pub label_bg_class: Option<String>,
}

impl BaseControllerOptions {
    /// Gets the label text, preferring `label` over `labelText`.
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref().or(self.label_text.as_deref())
    }
}

/// Options specific to text-like controllers.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextOptions {
    #[serde(flatten)]
    pub base: BaseControllerOptions,
    pub formatter: Option<String>,
    pub rounded_class: Option<String>,
    pub hidden: Option<bool>,
}

/// Options specific to number controller.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberOptions {
    #[serde(flatten)]
    pub base: BaseControllerOptions,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub rounded_class: Option<String>,
}

/// Options specific to toggle/switch/checkbox controllers.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleOptions {
    #[serde(flatten)]
    pub base: BaseControllerOptions,
    pub style_classes: Option<String>,
}

/// Options specific to select/radio controllers.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectOptions {
    #[serde(flatten)]
    pub base: BaseControllerOptions,
    pub options: Option<serde_json::Value>,
    pub rounded_class: Option<String>,
}

/// Options specific to combobox controller.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComboboxOptions {
    #[serde(flatten)]
    pub base: BaseControllerOptions,
    pub items: Option<serde_json::Value>,
    pub allow_custom: Option<bool>,
    pub fetch_urls: Option<serde_json::Value>,
    pub item_label_key_name: Option<String>,
    pub item_value_key_name: Option<String>,
    pub rounded_class: Option<String>,
}

/// Options specific to date-time controller.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeOptions {
    #[serde(flatten)]
    pub base: BaseControllerOptions,
    pub initial_date: Option<serde_json::Value>,
}

/// Options specific to array fieldset controller.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrayFieldsetOptions {
    #[serde(flatten)]
    pub base: BaseControllerOptions,
    pub item_structure: Option<serde_json::Value>,
    pub element_controllers: Option<serde_json::Value>,
}

/// Options specific to enum fieldset controller.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumFieldsetOptions {
    #[serde(flatten)]
    pub base: BaseControllerOptions,
    pub variants: Option<serde_json::Value>,
    pub default_variant: Option<String>,
}

/// Parsed and typed controller options.
#[derive(Debug, Clone)]
pub enum ControllerOptions {
    Text(TextOptions),
    TextArea(TextOptions),
    Number(NumberOptions),
    Toggle(ToggleOptions),
    Switch(ToggleOptions),
    Checkbox(ToggleOptions),
    Select(SelectOptions),
    RadioGroup(SelectOptions),
    Combobox(ComboboxOptions),
    ComboboxMultiple(ComboboxOptions),
    Hidden(BaseControllerOptions),
    Tags(BaseControllerOptions),
    DateTime(DateTimeOptions),
    ArrayFieldset(ArrayFieldsetOptions),
    EnumFieldset(EnumFieldsetOptions),
    SiteFieldset(BaseControllerOptions),
    Phone(BaseControllerOptions),
    Email(BaseControllerOptions),
}

impl ControllerOptions {
    /// Returns the base options for any controller type.
    pub fn base(&self) -> &BaseControllerOptions {
        match self {
            Self::Text(o) | Self::TextArea(o) => &o.base,
            Self::Number(o) => &o.base,
            Self::Toggle(o) | Self::Switch(o) | Self::Checkbox(o) => &o.base,
            Self::Select(o) | Self::RadioGroup(o) => &o.base,
            Self::Combobox(o) | Self::ComboboxMultiple(o) => &o.base,
            Self::Hidden(o)
            | Self::Tags(o)
            | Self::SiteFieldset(o)
            | Self::Phone(o)
            | Self::Email(o) => o,
            Self::DateTime(o) => &o.base,
            Self::ArrayFieldset(o) => &o.base,
            Self::EnumFieldset(o) => &o.base,
        }
    }

    /// Returns the controller type for this options variant.
    pub fn controller_type(&self) -> ControllerType {
        match self {
            Self::Text(_) => ControllerType::Text,
            Self::TextArea(_) => ControllerType::TextArea,
            Self::Number(_) => ControllerType::Number,
            Self::Toggle(_) => ControllerType::Toggle,
            Self::Switch(_) => ControllerType::Switch,
            Self::Checkbox(_) => ControllerType::Checkbox,
            Self::Select(_) => ControllerType::Select,
            Self::RadioGroup(_) => ControllerType::RadioGroup,
            Self::Combobox(_) => ControllerType::Combobox,
            Self::ComboboxMultiple(_) => ControllerType::ComboboxMultiple,
            Self::Hidden(_) => ControllerType::Hidden,
            Self::Tags(_) => ControllerType::Tags,
            Self::DateTime(_) => ControllerType::DateTime,
            Self::ArrayFieldset(_) => ControllerType::ArrayFieldset,
            Self::EnumFieldset(_) => ControllerType::EnumFieldset,
            Self::SiteFieldset(_) => ControllerType::SiteFieldset,
            Self::Phone(_) => ControllerType::Phone,
            Self::Email(_) => ControllerType::Email,
        }
    }
}

/// Parsed controller configuration for a field.
#[derive(Debug, Clone)]
pub struct ParsedController {
    pub options: ControllerOptions,
}

impl ParsedController {
    pub fn controller_type(&self) -> ControllerType {
        self.options.controller_type()
    }

    pub fn base(&self) -> &BaseControllerOptions {
        self.options.base()
    }
}

// =============================================================================
// Parsing Functions
// =============================================================================

/// Parses container-level @gigaform options.
pub fn parse_gigaform_options(input: &DeriveInput) -> GigaformOptions {
    // Look for @gigaform decorator on the interface
    for attr in &input.attrs {
        if attr.name() == "gigaform" {
            let json_value = parse_decorator_args(&attr.inner.args_src);
            if let Ok(opts) = serde_json::from_value(json_value) {
                return opts;
            }
        }
    }
    GigaformOptions::default()
}

/// Parses all fields from the interface.
pub fn parse_fields(interface: &DataInterface, _options: &GigaformOptions) -> Vec<ParsedField> {
    interface
        .fields()
        .iter()
        .map(parse_interface_field)
        .collect()
}

/// Parses all fields from a class (public fields only).
pub fn parse_fields_from_class(class: &DataClass, _options: &GigaformOptions) -> Vec<ParsedField> {
    class
        .fields()
        .iter()
        .filter(|f| f.visibility == Visibility::Public)
        .map(parse_class_field)
        .collect()
}

/// Parses all fields from a type alias object body.
pub fn parse_fields_from_type_alias(fields: &[InterfaceFieldIR], _options: &GigaformOptions) -> Vec<ParsedField> {
    fields.iter().map(parse_interface_field).collect()
}

/// Creates a synthetic field for non-object type aliases (e.g., `type ID = string` → field "0").
/// This allows treating scalar type aliases as single-field forms.
pub fn create_synthetic_field(
    field_name: &str,
    inner_type: &str,
    container_attrs: &[Attribute],
    _options: &GigaformOptions,
) -> Vec<ParsedField> {
    // Convert Attributes to DecoratorIRs for the synthetic field
    // (e.g., @serde validators on the type alias apply to field "0")
    let decorators: Vec<_> = container_attrs.iter().map(|a| a.inner.clone()).collect();
    let field = parse_field_common(field_name, inner_type, false, &decorators);
    vec![field]
}

// =============================================================================
// Union Parsing Functions
// =============================================================================

/// Classification of union type for Gigaform handling
#[derive(Debug, Clone, PartialEq)]
pub enum UnionKind {
    /// All members are string/number literals (e.g., `"Home" | "About"`)
    LiteralUnion,
    /// All members are type references (e.g., `DailyRule | WeeklyRule`)
    TypeRefUnion,
    /// All members are inline object types (discriminated union)
    ObjectUnion,
    /// Mixed member types
    Mixed,
}

/// Classifies a union type based on its members
pub fn classify_union(members: &[macroforge_ts::ts_syn::TypeMember]) -> UnionKind {
    if members.is_empty() {
        return UnionKind::Mixed;
    }

    let all_literals = members.iter().all(|m| m.is_literal());
    let all_type_refs = members.iter().all(|m| m.is_type_ref());
    let all_objects = members.iter().all(|m| m.is_object());

    if all_literals {
        UnionKind::LiteralUnion
    } else if all_type_refs {
        UnionKind::TypeRefUnion
    } else if all_objects {
        UnionKind::ObjectUnion
    } else {
        UnionKind::Mixed
    }
}

/// Parses a discriminated union type alias.
/// Returns UnionConfig if successfully parsed, or an error message.
pub fn parse_union_config(
    members: &[macroforge_ts::ts_syn::TypeMember],
    container_attrs: &[Attribute],
    _options: &GigaformOptions,
) -> Result<UnionConfig, String> {
    let union_kind = classify_union(members);

    match union_kind {
        UnionKind::LiteralUnion => {
            // String/number literal union - use literals as variant names
            let variants = parse_literal_union_variants(members)?;
            Ok(UnionConfig {
                mode: UnionMode::Tagged { field: "_value".to_string() },
                variants,
            })
        }
        UnionKind::TypeRefUnion => {
            // Type reference union - use type names as variant discriminants
            let variants = parse_type_ref_union_variants(members)?;
            Ok(UnionConfig {
                mode: UnionMode::Tagged { field: "_type".to_string() },
                variants,
            })
        }
        UnionKind::ObjectUnion => {
            // Inline object union - requires @serde({ tag }) or @serde({ untagged })
            let serde_opts = parse_serde_container_options(container_attrs);

            let mode = if let Some(tag_field) = serde_opts.tag {
                UnionMode::Tagged { field: tag_field }
            } else if serde_opts.untagged {
                UnionMode::Untagged
            } else {
                return Err("Union types with inline objects require @serde({ tag: \"fieldName\" }) or @serde({ untagged: true })".to_string());
            };

            let variants = match &mode {
                UnionMode::Tagged { field } => parse_tagged_union_variants(members, field)?,
                UnionMode::Untagged => parse_untagged_union_variants(members)?,
            };

            Ok(UnionConfig { mode, variants })
        }
        UnionKind::Mixed => {
            Err("Union types with mixed member types (literals, type refs, objects) are not supported".to_string())
        }
    }
}

/// Parses variants from a literal union (string or number literals).
fn parse_literal_union_variants(
    members: &[macroforge_ts::ts_syn::TypeMember],
) -> Result<Vec<UnionVariantConfig>, String> {
    let mut variants = Vec::new();

    for member in members {
        let literal = member.as_literal().ok_or_else(|| {
            "Expected literal type in union".to_string()
        })?;

        // Extract the literal value (remove quotes if present)
        let discriminant_value = extract_literal_value(literal);

        variants.push(UnionVariantConfig {
            discriminant_value,
            fields: Vec::new(), // Literal unions have no additional fields
        });
    }

    Ok(variants)
}

/// Parses variants from a type reference union.
fn parse_type_ref_union_variants(
    members: &[macroforge_ts::ts_syn::TypeMember],
) -> Result<Vec<UnionVariantConfig>, String> {
    let mut variants = Vec::new();

    for member in members {
        let type_ref = member.as_type_ref().ok_or_else(|| {
            "Expected type reference in union".to_string()
        })?;

        variants.push(UnionVariantConfig {
            discriminant_value: type_ref.to_string(),
            fields: Vec::new(), // Fields come from the referenced type
        });
    }

    Ok(variants)
}

/// Extracts the actual value from a literal type string.
/// e.g., `"Home"` -> `Home`, `42` -> `42`
fn extract_literal_value(literal: &str) -> String {
    let trimmed = literal.trim();

    // Remove surrounding quotes
    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        return trimmed[1..trimmed.len()-1].to_string();
    }

    trimmed.to_string()
}

/// Serde container-level options relevant to unions.
#[derive(Debug, Default)]
struct SerdeContainerOpts {
    tag: Option<String>,
    untagged: bool,
}

/// Parses @serde({ tag: "...", untagged: true }) from container attributes.
fn parse_serde_container_options(attrs: &[Attribute]) -> SerdeContainerOpts {
    let mut opts = SerdeContainerOpts::default();

    for attr in attrs {
        if attr.name() == "serde" {
            let json = parse_decorator_args(&attr.inner.args_src);
            if let Some(tag) = json.get("tag").and_then(|v| v.as_str()) {
                opts.tag = Some(tag.to_string());
            }
            if let Some(untagged) = json.get("untagged").and_then(|v| v.as_bool()) {
                opts.untagged = untagged;
            }
        }
    }

    opts
}

/// Parses variants from a tagged union.
fn parse_tagged_union_variants(
    members: &[macroforge_ts::ts_syn::TypeMember],
    tag_field: &str,
) -> Result<Vec<UnionVariantConfig>, String> {
    let mut variants = Vec::new();

    for member in members {
        let fields = member.as_object().ok_or_else(|| {
            "Tagged union variants must be object types".to_string()
        })?;

        // Find the discriminant field
        let discriminant_field = fields.iter()
            .find(|f| f.name == tag_field)
            .ok_or_else(|| format!("Variant missing discriminant field '{}'", tag_field))?;

        // Extract literal value from the discriminant field type (e.g., `"card"` from `type: "card"`)
        let discriminant_value = extract_literal_from_type(&discriminant_field.ts_type)
            .ok_or_else(|| format!("Discriminant field '{}' must have a literal type", tag_field))?;

        // Parse other fields (excluding the discriminant)
        let variant_fields: Vec<ParsedField> = fields.iter()
            .filter(|f| f.name != tag_field)
            .map(parse_interface_field)
            .collect();

        variants.push(UnionVariantConfig {
            discriminant_value,
            fields: variant_fields,
        });
    }

    Ok(variants)
}

/// Parses variants from an untagged union (infer by unique fields).
fn parse_untagged_union_variants(
    members: &[macroforge_ts::ts_syn::TypeMember],
) -> Result<Vec<UnionVariantConfig>, String> {
    // For untagged unions, we use the first unique field of each variant as an identifier
    let mut variants = Vec::new();
    let mut all_field_names: Vec<Vec<String>> = Vec::new();

    // First pass: collect all field names per variant
    for member in members {
        let fields = member.as_object().ok_or_else(|| {
            "Untagged union variants must be object types".to_string()
        })?;
        all_field_names.push(fields.iter().map(|f| f.name.clone()).collect());
    }

    // Second pass: find unique field for each variant
    for (i, member) in members.iter().enumerate() {
        let fields = member.as_object().unwrap();
        let my_fields = &all_field_names[i];

        // Find a field that's unique to this variant
        let unique_field = my_fields.iter().find(|field_name| {
            all_field_names.iter().enumerate()
                .filter(|(j, _)| *j != i)
                .all(|(_, other_fields)| !other_fields.contains(field_name))
        });

        let discriminant_value = unique_field
            .ok_or_else(|| "Untagged union variants must be distinguishable by unique fields".to_string())?
            .clone();

        let variant_fields: Vec<ParsedField> = fields.iter()
            .map(parse_interface_field)
            .collect();

        variants.push(UnionVariantConfig {
            discriminant_value,
            fields: variant_fields,
        });
    }

    Ok(variants)
}

/// Extracts a literal string value from a TypeScript type annotation.
/// e.g., `"card"` -> Some("card"), `string` -> None
fn extract_literal_from_type(ts_type: &str) -> Option<String> {
    let trimmed = ts_type.trim();

    // Check for string literal: "value" or 'value'
    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        return Some(trimmed[1..trimmed.len()-1].to_string());
    }

    None
}

// =============================================================================
// Field Parsing Functions
// =============================================================================

/// Parses a single interface field.
fn parse_interface_field(field: &InterfaceFieldIR) -> ParsedField {
    parse_field_common(&field.name, &field.ts_type, field.optional, &field.decorators)
}

/// Parses a single class field.
fn parse_class_field(field: &FieldIR) -> ParsedField {
    parse_field_common(&field.name, &field.ts_type, field.optional, &field.decorators)
}

/// Common field parsing logic shared by interface and class fields.
fn parse_field_common(
    name: &str,
    ts_type: &str,
    optional: bool,
    decorators: &[macroforge_ts::ts_syn::DecoratorIR],
) -> ParsedField {
    let name = name.to_string();
    let ts_type = ts_type.to_string();

    // Determine if this is a nested or array type
    let (is_nested, nested_type) = detect_nested_type(&ts_type);
    let (is_array, array_element_type) = detect_array_type(&ts_type);

    // Parse @serde validators
    let validators = parse_serde_validators_from_decorators(decorators);

    // Parse @gigaform field options
    let field_options = parse_gigaform_field_options_from_decorators(decorators);
    let async_validators = field_options.validate_async.clone().unwrap_or_default();

    // Parse controller
    let controller = parse_field_controller_from_decorators(decorators, &ts_type);

    ParsedField {
        name,
        ts_type,
        optional,
        is_nested,
        nested_type,
        is_array,
        array_element_type,
        validators,
        async_validators,
        controller,
        field_options,
    }
}

/// Detects if a type is a nested object type (references another type with Gigaform).
fn detect_nested_type(ts_type: &str) -> (bool, Option<String>) {
    let trimmed = ts_type.trim();

    // Skip primitives
    let primitives = [
        "string",
        "number",
        "boolean",
        "Date",
        "bigint",
        "symbol",
        "undefined",
        "null",
        "unknown",
        "any",
        "never",
        "void",
    ];
    if primitives.contains(&trimmed) {
        return (false, None);
    }

    // Skip array types
    if trimmed.ends_with("[]") || trimmed.starts_with("Array<") {
        return (false, None);
    }

    // Skip union/intersection types for now
    if trimmed.contains('|') || trimmed.contains('&') {
        return (false, None);
    }

    // Skip inline object types
    if trimmed.starts_with('{') {
        return (false, None);
    }

    // If it's a PascalCase identifier, assume it's a nested type
    if trimmed
        .chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
    {
        return (true, Some(trimmed.to_string()));
    }

    (false, None)
}

/// Detects if a type is an array type and extracts the element type.
fn detect_array_type(ts_type: &str) -> (bool, Option<String>) {
    let trimmed = ts_type.trim();

    // Check for T[] syntax
    if trimmed.ends_with("[]") {
        let element = trimmed.trim_end_matches("[]").trim();
        return (true, Some(element.to_string()));
    }

    // Check for Array<T> syntax
    if trimmed.starts_with("Array<") && trimmed.ends_with('>') {
        let inner = &trimmed[6..trimmed.len() - 1];
        return (true, Some(inner.trim().to_string()));
    }

    // Check for ReadonlyArray<T> syntax
    if trimmed.starts_with("ReadonlyArray<") && trimmed.ends_with('>') {
        let inner = &trimmed[14..trimmed.len() - 1];
        return (true, Some(inner.trim().to_string()));
    }

    (false, None)
}

/// Parses @serde({ validate: [...] }) validators from decorators.
fn parse_serde_validators_from_decorators(decorators: &[macroforge_ts::ts_syn::DecoratorIR]) -> Vec<ValidatorSpec> {
    let mut validators = Vec::new();

    for decorator in decorators {
        if decorator.name == "serde" {
            let json_value = parse_decorator_args(&decorator.args_src);
            if let Some(validate_array) = json_value.get("validate").and_then(|v| v.as_array()) {
                for item in validate_array {
                    if let Some(spec) = parse_validator_item(item) {
                        validators.push(spec);
                    }
                }
            }
        }
    }

    validators
}

/// Parses a single validator item (string or object with custom message).
fn parse_validator_item(item: &serde_json::Value) -> Option<ValidatorSpec> {
    match item {
        serde_json::Value::String(s) => Some(parse_validator_string(s)),
        serde_json::Value::Object(obj) => {
            let validate = obj.get("validate")?.as_str()?;
            let message = obj.get("message").and_then(|m| m.as_str()).map(String::from);
            let mut spec = parse_validator_string(validate);
            spec.message = message;
            Some(spec)
        }
        _ => None,
    }
}

/// Parses a validator string like "minLength(2)" into name and args.
fn parse_validator_string(s: &str) -> ValidatorSpec {
    let trimmed = s.trim();

    // Check for function-style validator: name(args)
    if let Some(paren_idx) = trimmed.find('(')
        && trimmed.ends_with(')')
    {
        let name = trimmed[..paren_idx].to_string();
        let args_str = &trimmed[paren_idx + 1..trimmed.len() - 1];
        let args = parse_validator_args(args_str);
        return ValidatorSpec {
            name,
            args,
            message: None,
        };
    }

    // Simple validator without args
    ValidatorSpec {
        name: trimmed.to_string(),
        args: Vec::new(),
        message: None,
    }
}

/// Parses validator arguments (handles strings, numbers, multiple args).
fn parse_validator_args(args_str: &str) -> Vec<String> {
    if args_str.trim().is_empty() {
        return Vec::new();
    }

    // Simple comma split (doesn't handle nested commas in strings perfectly)
    args_str
        .split(',')
        .map(|s| {
            let trimmed = s.trim();
            // Remove surrounding quotes if present
            if (trimmed.starts_with('"') && trimmed.ends_with('"'))
                || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
            {
                trimmed[1..trimmed.len() - 1].to_string()
            } else {
                trimmed.to_string()
            }
        })
        .collect()
}

/// Parses @gigaform field-level options from decorators.
fn parse_gigaform_field_options_from_decorators(decorators: &[macroforge_ts::ts_syn::DecoratorIR]) -> GigaformFieldOptions {
    for decorator in decorators {
        if decorator.name == "gigaform" {
            let json_value = parse_decorator_args(&decorator.args_src);
            if let Ok(opts) = serde_json::from_value(json_value) {
                return opts;
            }
        }
    }
    GigaformFieldOptions::default()
}

/// Parses a field's controller decorator from decorators.
fn parse_field_controller_from_decorators(
    decorators: &[macroforge_ts::ts_syn::DecoratorIR],
    ts_type: &str,
) -> Option<ParsedController> {
    // Check each controller type's decorator
    for controller_type in ControllerType::all() {
        let decorator_name = controller_type.decorator_name();
        if let Some(decorator) = decorators.iter().find(|d| d.name == decorator_name) {
            let json_value = parse_decorator_args(&decorator.args_src);
            let options = deserialize_controller_options(*controller_type, json_value);
            return Some(ParsedController { options });
        }
    }

    // Fallback: infer controller type from TypeScript type
    infer_controller_from_type(ts_type)
}

/// Deserializes JSON into the appropriate controller options struct.
fn deserialize_controller_options(
    controller_type: ControllerType,
    value: serde_json::Value,
) -> ControllerOptions {
    match controller_type {
        ControllerType::Text => {
            ControllerOptions::Text(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::TextArea => {
            ControllerOptions::TextArea(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::Number => {
            ControllerOptions::Number(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::Toggle => {
            ControllerOptions::Toggle(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::Switch => {
            ControllerOptions::Switch(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::Checkbox => {
            ControllerOptions::Checkbox(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::Select => {
            ControllerOptions::Select(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::RadioGroup => {
            ControllerOptions::RadioGroup(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::Combobox => {
            ControllerOptions::Combobox(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::ComboboxMultiple => {
            ControllerOptions::ComboboxMultiple(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::Hidden => {
            ControllerOptions::Hidden(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::Tags => {
            ControllerOptions::Tags(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::DateTime => {
            ControllerOptions::DateTime(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::ArrayFieldset => {
            ControllerOptions::ArrayFieldset(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::EnumFieldset => {
            ControllerOptions::EnumFieldset(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::SiteFieldset => {
            ControllerOptions::SiteFieldset(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::Phone => {
            ControllerOptions::Phone(serde_json::from_value(value).unwrap_or_default())
        }
        ControllerType::Email => {
            ControllerOptions::Email(serde_json::from_value(value).unwrap_or_default())
        }
    }
}

/// Infers a default controller type based on the TypeScript type.
fn infer_controller_from_type(ts_type: &str) -> Option<ParsedController> {
    let ts_type = ts_type.trim();

    let options = match ts_type {
        "string" => ControllerOptions::Text(TextOptions::default()),
        t if t == "string | null" || t == "null | string" => {
            ControllerOptions::Text(TextOptions::default())
        }
        "number" => ControllerOptions::Number(NumberOptions::default()),
        t if t == "number | null" || t == "null | number" => {
            ControllerOptions::Number(NumberOptions::default())
        }
        "boolean" => ControllerOptions::Toggle(ToggleOptions::default()),
        t if t.starts_with("Array<") || t.ends_with("[]") => {
            ControllerOptions::Tags(BaseControllerOptions::default())
        }
        t if t.starts_with("ReadonlyArray<") => {
            ControllerOptions::Tags(BaseControllerOptions::default())
        }
        _ => return None,
    };

    Some(ParsedController { options })
}

/// Parses decorator arguments from the raw source string.
fn parse_decorator_args(args_src: &str) -> serde_json::Value {
    let trimmed = args_src.trim();
    if trimmed.is_empty() {
        return serde_json::Value::Object(serde_json::Map::new());
    }

    // Try to parse as JSON directly
    if let Ok(value) = serde_json::from_str(trimmed) {
        return value;
    }

    // Try to convert JS object literal to JSON
    let json_str = convert_js_object_to_json(trimmed);
    serde_json::from_str(&json_str).unwrap_or_else(|_| serde_json::Value::Object(serde_json::Map::new()))
}

/// Converts a JavaScript object literal to valid JSON.
fn convert_js_object_to_json(js_obj: &str) -> String {
    let mut result = String::with_capacity(js_obj.len() * 2);
    let mut chars = js_obj.chars().peekable();
    let mut in_string = false;
    let mut string_char = '"';

    while let Some(c) = chars.next() {
        match c {
            '"' if !in_string => {
                in_string = true;
                string_char = '"';
                result.push('"');
            }
            '"' if in_string && string_char == '"' => {
                in_string = false;
                result.push('"');
            }
            '\'' if !in_string => {
                in_string = true;
                string_char = '\'';
                result.push('"');
            }
            '\'' if in_string && string_char == '\'' => {
                in_string = false;
                result.push('"');
            }
            ':' if !in_string => {
                result.push(':');
            }
            _ if !in_string && c.is_alphabetic() => {
                let mut word = String::new();
                word.push(c);
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' {
                        word.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                match word.as_str() {
                    "true" | "false" | "null" => result.push_str(&word),
                    _ => {
                        let rest: String = chars.clone().collect();
                        let next_meaningful = rest.trim_start().chars().next();
                        if next_meaningful == Some(':') {
                            result.push('"');
                            result.push_str(&word);
                            result.push('"');
                        } else {
                            result.push_str(&word);
                        }
                    }
                }
            }
            _ => result.push(c),
        }
    }

    result
}

// =============================================================================
// Enum Parsing Functions
// =============================================================================

use macroforge_ts::ts_syn::EnumIR;

/// Parses an enum into form configuration.
pub fn parse_enum_config(enum_ir: &EnumIR) -> EnumFormConfig {
    let variants = enum_ir
        .variants
        .iter()
        .map(|variant| {
            // Try to get custom label from @gigaform({ label: "..." }) decorator
            let label = parse_variant_label(&variant.decorators).unwrap_or_else(|| {
                to_title_case(&variant.name)
            });

            // Try to get description from @gigaform({ description: "..." }) decorator
            let description = parse_variant_description(&variant.decorators);

            EnumVariantFormConfig {
                name: variant.name.clone(),
                label,
                description,
                value_expr: format!("{}.{}", enum_ir.name, variant.name),
            }
        })
        .collect();

    EnumFormConfig { variants }
}

/// Parses a custom label from @gigaform decorator on a variant.
fn parse_variant_label(decorators: &[macroforge_ts::ts_syn::DecoratorIR]) -> Option<String> {
    for decorator in decorators {
        if decorator.name == "gigaform" {
            let json_value = parse_decorator_args(&decorator.args_src);
            if let Some(label) = json_value.get("label").and_then(|v| v.as_str()) {
                return Some(label.to_string());
            }
        }
    }
    None
}

/// Parses a description from @gigaform decorator on a variant.
fn parse_variant_description(decorators: &[macroforge_ts::ts_syn::DecoratorIR]) -> Option<String> {
    for decorator in decorators {
        if decorator.name == "gigaform" {
            let json_value = parse_decorator_args(&decorator.args_src);
            if let Some(desc) = json_value.get("description").and_then(|v| v.as_str()) {
                return Some(desc.to_string());
            }
        }
    }
    None
}

/// Converts a PascalCase or camelCase identifier to Title Case.
/// e.g., "PersonalInfo" -> "Personal Info", "firstName" -> "First Name"
pub fn to_title_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_upper = false;

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            // Add space before uppercase if not at start and previous wasn't uppercase
            if i > 0 && !prev_was_upper {
                result.push(' ');
            }
            // Also add space if this uppercase follows uppercase but next is lowercase
            // e.g., "HTMLParser" -> "HTML Parser"
            result.push(c);
            prev_was_upper = true;
        } else {
            // Lowercase first char should be uppercased
            if i == 0 {
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c);
            }
            prev_was_upper = false;
        }
    }

    result
}
