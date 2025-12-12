//! Gigaform macro - generates compile-time form handling with Svelte 5 reactive state.
//!
//! This macro **composes with** other Macroforge macros:
//! - `@derive(Default)` provides `defaultValue()`
//! - `@derive(Serialize)` provides `toObject()`
//! - `@derive(Deserialize)` + `@serde` provides `fromObject()` with validation
//!
//! Gigaform adds:
//! - `Errors` type (nested error structure)
//! - `Tainted` type (nested boolean structure)
//! - `FieldController<T>` interface for field controllers
//! - `Gigaform` interface for form instances
//! - `createForm()` factory returning reactive form instance with field controllers
//! - `fromFormData()` (FormData parsing with type coercion)

pub mod field_descriptors;
pub mod form_data;
pub mod i18n;
pub mod parser;
pub mod types;

use macroforge_ts::macros::{ts_macro_derive, ts_template};
use macroforge_ts::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

/// Generates the Gigaform namespace with types, fromFormData, and field descriptors.
pub fn generate(input: DeriveInput) -> Result<TsStream, MacroforgeError> {
    let type_name = input.name();
    let options = parser::parse_gigaform_options(&input);

    // Extract type params from the data variant
    let type_params = extract_type_params(&input.data);
    let generics = GenericInfo::from_params(&type_params);

    // Dispatch based on declaration type
    let fields = match &input.data {
        Data::Interface(interface) => parser::parse_fields(interface, &options),
        Data::Class(class) => {
            // Reject abstract classes
            if class.is_abstract() {
                return Err(MacroforgeError::new(
                    input.decorator_span(),
                    "@derive(Gigaform): Cannot generate form for abstract class",
                ));
            }
            parser::parse_fields_from_class(class, &options)
        }
        Data::TypeAlias(type_alias) => {
            if type_alias.body().is_object() {
                // Object type alias - treat like interface
                let obj_fields = type_alias.body().as_object().unwrap();
                parser::parse_fields_from_type_alias(obj_fields, &options)
            } else if type_alias.body().is_alias() {
                // Non-object type alias (e.g., `type ID = string`) - treat as single field "0"
                let inner_type = type_alias.body().as_alias().unwrap();
                parser::create_synthetic_field("0", inner_type, &input.attrs, &options)
            } else if type_alias.body().is_union() {
                // Discriminated union - parse variants and generate variant-aware form
                let members = type_alias.body().as_union().unwrap();
                let union_config = parser::parse_union_config(members, &input.attrs, &options)
                    .map_err(|e| {
                        MacroforgeError::new(
                            input.decorator_span(),
                            &format!("@derive(Gigaform): {}", e),
                        )
                    })?;

                // Generate union-specific form (different from regular field-based forms)
                return generate_union_form(type_name, &union_config, &options, &generics);
            } else if type_alias.body().is_tuple() {
                return Err(MacroforgeError::new(
                    input.decorator_span(),
                    "@derive(Gigaform): Tuple types not yet supported",
                ));
            } else {
                return Err(MacroforgeError::new(
                    input.decorator_span(),
                    "@derive(Gigaform): Unsupported type alias body",
                ));
            }
        }
        Data::Enum(data_enum) => {
            // Enum step form (wizard-style navigation)
            if data_enum.variants().is_empty() {
                return Err(MacroforgeError::new(
                    input.decorator_span(),
                    "@derive(Gigaform): Enum has no variants",
                ));
            }
            let enum_config = parser::parse_enum_config(&data_enum.inner);
            return generate_enum_form(type_name, &enum_config);
        }
    };

    if fields.is_empty() {
        let msg = match &input.data {
            Data::Class(_) => "@derive(Gigaform): Class has no public fields",
            Data::Interface(_) => "@derive(Gigaform): Interface has no fields",
            Data::TypeAlias(_) => "@derive(Gigaform): Type alias has no fields",
            Data::Enum(_) => "@derive(Gigaform): Enum has no variants",
        };
        return Err(MacroforgeError::new(input.decorator_span(), msg));
    }

    // Generate each section with generics
    let type_defs = types::generate_with_generics(type_name, &fields, &generics);
    let form_data_fn = form_data::generate_with_generics(type_name, &fields, &generics);
    let factory_fn =
        field_descriptors::generate_factory_with_generics(type_name, &fields, &options, &generics);

    // Combine into namespace
    let mut output = ts_template! {
        export namespace @{type_name} {
            {$typescript type_defs}
            {$typescript factory_fn}
            {$typescript form_data_fn}
        }
    };

    // Add required imports
    output.add_import("Result", "macroforge/utils");

    // Add i18n import if used
    if options.uses_i18n() || fields.iter().any(|f| f.uses_i18n()) {
        output.add_import("m", "$lib/paraglide/messages");
    }

    Ok(output)
}

/// Extracts type parameters from the data variant.
fn extract_type_params(data: &Data) -> Vec<String> {
    match data {
        Data::Interface(interface) => interface.type_params().to_vec(),
        Data::Class(class) => class.type_params().to_vec(),
        Data::TypeAlias(type_alias) => type_alias.type_params().to_vec(),
        Data::Enum(_) => Vec::new(), // Enums don't have type params in TS
    }
}

/// Generic information for code generation.
#[derive(Debug, Clone, Default)]
pub struct GenericInfo {
    /// Full type parameter declarations (e.g., ["T extends Foo", "U"])
    pub params: Vec<String>,
    /// Just the type parameter names (e.g., ["T", "U"])
    pub names: Vec<String>,
}

impl GenericInfo {
    /// Creates GenericInfo from type parameter strings.
    pub fn from_params(params: &[String]) -> Self {
        if params.is_empty() {
            return Self::default();
        }

        let names = params
            .iter()
            .map(|p| {
                // Extract just the name from "T extends Foo" -> "T"
                p.split_whitespace().next().unwrap_or(p).to_string()
            })
            .collect();

        Self {
            params: params.to_vec(),
            names,
        }
    }

    /// Returns true if there are no type parameters.
    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    /// Returns the type parameter declaration for use in type/function signatures.
    /// e.g., "<T extends Foo, U>" or "" if empty
    pub fn decl(&self) -> String {
        if self.params.is_empty() {
            String::new()
        } else {
            format!("<{}>", self.params.join(", "))
        }
    }

    /// Returns the type arguments for use in type references.
    /// e.g., "<T, U>" or "" if empty
    pub fn args(&self) -> String {
        if self.names.is_empty() {
            String::new()
        } else {
            format!("<{}>", self.names.join(", "))
        }
    }
}

/// Generates a discriminated union form with variant switching.
fn generate_union_form(
    type_name: &str,
    union_config: &parser::UnionConfig,
    options: &parser::GigaformOptions,
    generics: &GenericInfo,
) -> Result<TsStream, MacroforgeError> {
    let type_defs = types::generate_union_with_generics(type_name, union_config, generics);
    let factory_fn = field_descriptors::generate_union_factory_with_generics(
        type_name,
        union_config,
        options,
        generics,
    );
    let form_data_fn = form_data::generate_union_with_generics(type_name, union_config, generics);

    let mut output = ts_template! {
        export namespace @{type_name} {
            {$typescript type_defs}
            {$typescript factory_fn}
            {$typescript form_data_fn}
        }
    };

    output.add_import("Result", "macroforge/utils");

    // Add i18n import if used
    if options.uses_i18n()
        || union_config
            .variants
            .iter()
            .any(|v| v.fields.iter().any(|f| f.uses_i18n()))
    {
        output.add_import("m", "$lib/paraglide/messages");
    }

    Ok(output)
}

/// Generates an enum step form (wizard-style navigation).
fn generate_enum_form(
    enum_name: &str,
    enum_config: &parser::EnumFormConfig,
) -> Result<TsStream, MacroforgeError> {
    let type_defs = types::generate_enum(enum_name, enum_config);
    let factory_fn = field_descriptors::generate_enum_factory(enum_name, enum_config);

    let output = ts_template! {
        export namespace @{enum_name} {
            {$typescript type_defs}
            {$typescript factory_fn}
        }
    };

    Ok(output)
}

/// Generates a form namespace with reactive state, field controllers, and validation.
///
/// This macro **composes with** other Macroforge macros:
/// - `@derive(Default)` provides `defaultValue()`
/// - `@derive(Serialize)` provides `toObject()`
/// - `@derive(Deserialize)` + `@serde({ validate: [...] })` provides `fromObject()` with validation
///
/// **Gigaform adds:**
/// - `Errors` type (nested error structure)
/// - `Tainted` type (nested boolean structure for dirty tracking)
/// - `FieldController<T>` interface for individual field controllers
/// - `Gigaform` interface for form instance
/// - `createForm()` factory returning reactive Gigaform with field controllers
/// - `fromFormData()` (FormData parsing with type coercion)
///
/// # Example
///
/// ```typescript
/// /** @derive(Default, Serialize, Deserialize, Gigaform) */
/// export interface UserForm {
///   /** @serde({ validate: ["minLength(2)"] }) */
///   /** @textController({ label: "Full Name" }) */
///   name: string;
///
///   /** @serde({ validate: ["email"] }) */
///   /** @emailFieldController({ label: "Email" }) */
///   email: string;
///
///   /** @serde({ validate: ["positive", "int"] }) */
///   /** @numberController({ label: "Age", min: 0 }) */
///   age: number;
/// }
///
/// // Generates namespace with:
/// export namespace UserForm {
///   // Types
///   export type Errors = { _errors?: string[]; name?: string[]; email?: string[]; age?: string[] };
///   export type Tainted = { name?: boolean; email?: boolean; age?: boolean };
///   export interface FieldController<T> { get(), set(), getError(), setError(), ... };
///   export interface Gigaform { data, errors, tainted, fields, validate(), reset() };
///
///   // Factory function - creates reactive form instance
///   export function createForm(overrides?: Partial<UserForm>): Gigaform;
///
///   // FormData parsing
///   export function fromFormData(fd: FormData): Result<UserForm, Array<{ field: string; message: string }>>;
/// }
///
/// // Usage in Svelte component:
/// const form = UserForm.createForm();
/// <TextField fieldController={form.fields.name} />
/// <TextField fieldController={form.fields.email} />
/// <button onclick={() => form.validate()}>Submit</button>
/// ```
///
/// # Container Options
///
/// Use `@gigaform({ ... })` decorator on the interface for container-level options:
///
/// ```typescript
/// /** @derive(Gigaform) */
/// /** @gigaform({ i18nPrefix: "userForm", defaultOverride: "getFormDefaults" }) */
/// export interface UserForm { ... }
/// ```
///
/// - `i18nPrefix`: Prefix for Paraglide message keys (e.g., `m.userForm_name_minLength()`)
/// - `defaultOverride`: Function name that returns partial defaults to merge with `defaultValue()`
///
/// # Field Options
///
/// Use `@gigaform({ ... })` decorator on fields for field-level options:
///
/// ```typescript
/// /** @gigaform({ validateAsync: ["checkEmailUnique"], labelKey: "email_label" }) */
/// email: string;
/// ```
///
/// - `validateAsync`: Array of async validator function names
/// - `labelKey`: i18n key for the field label
/// - `descriptionKey`: i18n key for the field description
/// - `placeholderKey`: i18n key for the field placeholder
#[ts_macro_derive(
    Gigaform,
    description = "Generates form namespace with types, validation, field descriptors, and controllers",
    attributes(
        gigaform,
        textController,
        textAreaController,
        numberController,
        toggleController,
        switchController,
        checkboxController,
        selectController,
        radioGroupController,
        comboboxController,
        comboboxMultipleController,
        hiddenController,
        tagsController,
        dateTimeController,
        arrayFieldsetController,
        enumFieldsetController,
        siteFieldsetController,
        phoneFieldController,
        emailFieldController
    )
)]
pub fn derive_gigaform(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let derive_input = parse_ts_macro_input!(input as DeriveInput);
    generate(derive_input)
}
