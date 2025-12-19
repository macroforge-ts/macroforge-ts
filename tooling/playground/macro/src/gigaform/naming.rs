use convert_case::Casing;

pub fn call_default_value(type_name: &str, generic_args: &str) -> String {
    format!(
        "{}DefaultValue{generic_args}()",
        type_name.to_case(convert_case::Case::Camel)
    )
}

pub fn call_validate_field(type_name: &str, field_expr: &str, value_expr: &str) -> String {
    format!(
        "{}ValidateField({field_expr}, {value_expr})",
        type_name.to_case(convert_case::Case::Camel)
    )
}

pub fn call_deserialize(type_name: &str, generic_args: &str, value_expr: &str) -> String {
    format!(
        "{}Deserialize{generic_args}({value_expr})",
        type_name.to_case(convert_case::Case::Camel)
    )
}

pub fn fn_name_from_form_data(type_name: &str, generic_decl: &str) -> String {
    format!(
        "{}FromFormData{generic_decl}",
        type_name.to_case(convert_case::Case::Camel)
    )
}

/// Generates a function name with Prefix style.
/// For prefix style: `userCreateForm`, `userGetDefaultForVariant`
pub fn fn_name(base: &str, type_name: &str, generic_decl: &str) -> String {
    format!(
        "{}{}{generic_decl}",
        type_name.to_case(convert_case::Case::Camel),
        capitalize_first(base)
    )
}

/// Generates a type name with prefix style.
/// `AccountErrors`, `AccountTainted`, `AccountGigaform`, etc.
pub fn type_name_prefixed(type_name: &str, suffix: &str) -> String {
    format!("{type_name}{suffix}")
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

pub fn call_default_value_for_type_ref(type_ref: &str) -> String {
    let tr = type_ref.trim();
    if let Some(bracket_pos) = tr.find('<') {
        let base_type = &tr[..bracket_pos];
        let type_args = &tr[bracket_pos..]; // includes <>
        return format!(
            "{}DefaultValue{type_args}()",
            base_type.to_case(convert_case::Case::Camel)
        );
    }

    format!("{}DefaultValue()", tr.to_case(convert_case::Case::Camel))
}
