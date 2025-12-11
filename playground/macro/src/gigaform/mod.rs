//! Gigaform macro - generates compile-time form handling with validation.
//!
//! This macro **composes with** other Macroforge macros:
//! - `@derive(Default)` provides `default_()`
//! - `@derive(Serialize)` provides `toJSON()`
//! - `@derive(Deserialize)` + `@serde` provides `fromJSON()` with validation
//!
//! Gigaform adds:
//! - `Errors` type (nested error structure)
//! - `Tainted` type (nested boolean structure)
//! - `fromFormData()` (FormData parsing with type coercion)
//! - `fields.*` (field descriptors with get/set/constraints/UI metadata)

pub mod field_descriptors;
pub mod form_data;
pub mod i18n;
pub mod parser;
pub mod types;

use macroforge_ts::macros::{ts_macro_derive, ts_template};
use macroforge_ts::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

/// Generates the Gigaform namespace with types, fromFormData, and field descriptors.
pub fn generate(input: DeriveInput) -> Result<TsStream, MacroforgeError> {
    let interface = match &input.data {
        Data::Interface(i) => i,
        _ => {
            return Err(MacroforgeError::new(
                input.decorator_span(),
                "@derive(Gigaform) can only be used on interfaces",
            ));
        }
    };

    let interface_name = input.name();

    // Parse gigaform options from container-level decorator
    let options = parser::parse_gigaform_options(&input);

    // Parse all fields with their types, validators, and controller configs
    let fields = parser::parse_fields(interface, &options);

    if fields.is_empty() {
        return Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(Gigaform): interface has no fields",
        ));
    }

    // Generate each section
    let type_defs = types::generate(interface_name, &fields);
    let form_data_fn = form_data::generate(interface_name, &fields);
    let field_descriptors = field_descriptors::generate(interface_name, &fields, &options);

    // Combine into namespace
    let mut output = ts_template! {
        export namespace @{interface_name} {
            {$typescript type_defs}
            {$typescript form_data_fn}
            {$typescript field_descriptors}
        }
    };

    // Add required imports
    output.add_import("Result", "macroforge/result");

    // Add i18n import if used
    if options.uses_i18n() || fields.iter().any(|f| f.uses_i18n()) {
        output.add_import("m", "$lib/paraglide/messages");
    }

    Ok(output)
}

/// Generates a complete form namespace with types, validation, field descriptors, and controllers.
///
/// This macro **composes with** other Macroforge macros:
/// - `@derive(Default)` provides `default_()`
/// - `@derive(Serialize)` provides `toJSON()`
/// - `@derive(Deserialize)` + `@serde({ validate: [...] })` provides `fromJSON()` with validation
///
/// **Gigaform adds:**
/// - `Errors` type (nested error structure)
/// - `Tainted` type (nested boolean structure for dirty tracking)
/// - `fromFormData()` (FormData parsing with type coercion)
/// - `fields.*` (field descriptors with get/set/validate/constraints)
/// - Controller factory functions (merged from FieldController)
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
///   export type Data = UserForm;
///   export type Errors = { _errors?: string[]; name?: string[]; email?: string[]; age?: string[] };
///   export type Tainted = { name?: boolean; email?: boolean; age?: boolean };
///
///   // FormData parsing (delegates to fromJSON for validation)
///   export function fromFormData(fd: FormData): Result<Data, Errors>;
///
///   // Field descriptors with type-safe accessors
///   export const fields = {
///     name: { get, set, getError, setError, getTainted, setTainted, validate, constraints },
///     email: { ... },
///     age: { ... },
///   };
///
///   // Controller factories
///   export function nameController(form: SuperForm<UserForm>): TextFieldController<...>;
///   export function emailController(form: SuperForm<UserForm>): EmailFieldController<...>;
///   export function ageController(form: SuperForm<UserForm>): NumberFieldController<...>;
/// }
/// ```
///
/// # Container Options
///
/// Use `@gigaform({ ... })` decorator on the interface for container-level options:
///
/// ```typescript
/// /** @derive(Gigaform) */
/// /** @gigaform({ i18nPrefix: "userForm" }) */
/// export interface UserForm { ... }
/// ```
///
/// - `i18nPrefix`: Prefix for Paraglide message keys (e.g., `m.userForm_name_minLength()`)
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
        serde,
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
