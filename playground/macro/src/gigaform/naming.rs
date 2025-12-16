fn to_camel_case(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

pub fn call_default_value(type_name: &str, generic_args: &str) -> String {
    format!("{}DefaultValue{generic_args}()", to_camel_case(type_name))
}

pub fn call_validate_field(
    type_name: &str,
    field_expr: &str,
    value_expr: &str,
) -> String {
    format!(
        "{}ValidateField({field_expr}, {value_expr})",
        to_camel_case(type_name)
    )
}

#[allow(dead_code)]
pub fn call_validate_fields(
    type_name: &str,
    partial_expr: &str,
) -> String {
    format!("{}ValidateFields({partial_expr})", to_camel_case(type_name))
}

pub fn call_from_object(
    type_name: &str,
    generic_args: &str,
    value_expr: &str,
) -> String {
    format!(
        "{}FromObject{generic_args}({value_expr})",
        to_camel_case(type_name)
    )
}

pub fn call_from_stringified_json(
    type_name: &str,
    generic_args: &str,
    json_expr: &str,
) -> String {
    format!(
        "{}FromStringifiedJSON{generic_args}({json_expr})",
        to_camel_case(type_name)
    )
}

pub fn fn_name_from_form_data(type_name: &str, generic_decl: &str) -> String {
    format!("{}FromFormData{generic_decl}", to_camel_case(type_name))
}

/// Generates a function name with Prefix style.
/// For prefix style: `userCreateForm`, `userGetDefaultForVariant`
pub fn fn_name(base: &str, type_name: &str, generic_decl: &str) -> String {
    format!("{}{}{generic_decl}", to_camel_case(type_name), capitalize_first(base))
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
        return format!("{}DefaultValue{type_args}()", to_camel_case(base_type));
    }

    format!("{}DefaultValue()", to_camel_case(tr))
}
