use macroforge_ts::ts_syn::abi::FunctionNamingStyle;

fn to_camel_case(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

pub fn call_default_value(type_name: &str, generic_args: &str, naming_style: FunctionNamingStyle) -> String {
    let applied_type = format!("{type_name}{generic_args}");
    match naming_style {
        FunctionNamingStyle::Namespace => format!("{type_name}.defaultValue{generic_args}()"),
        FunctionNamingStyle::Suffix => format!("defaultValue{type_name}{generic_args}()"),
        FunctionNamingStyle::Prefix => format!("{}DefaultValue{generic_args}()", to_camel_case(type_name)),
        FunctionNamingStyle::Generic => format!("defaultValue<{applied_type}>()"),
    }
}

pub fn call_validate_field(
    type_name: &str,
    naming_style: FunctionNamingStyle,
    field_expr: &str,
    value_expr: &str,
) -> String {
    match naming_style {
        FunctionNamingStyle::Namespace => {
            format!("{type_name}.validateField({field_expr}, {value_expr})")
        }
        FunctionNamingStyle::Suffix => format!("validateField{type_name}({field_expr}, {value_expr})"),
        FunctionNamingStyle::Prefix => format!(
            "{}ValidateField({field_expr}, {value_expr})",
            to_camel_case(type_name)
        ),
        FunctionNamingStyle::Generic => format!("validateField({field_expr}, {value_expr})"),
    }
}

#[allow(dead_code)]
pub fn call_validate_fields(
    type_name: &str,
    naming_style: FunctionNamingStyle,
    partial_expr: &str,
) -> String {
    match naming_style {
        FunctionNamingStyle::Namespace => format!("{type_name}.validateFields({partial_expr})"),
        FunctionNamingStyle::Suffix => format!("validateFields{type_name}({partial_expr})"),
        FunctionNamingStyle::Prefix => {
            format!("{}ValidateFields({partial_expr})", to_camel_case(type_name))
        }
        FunctionNamingStyle::Generic => format!("validateFields({partial_expr})"),
    }
}

pub fn call_from_object(
    type_name: &str,
    generic_args: &str,
    naming_style: FunctionNamingStyle,
    value_expr: &str,
) -> String {
    let applied_type = format!("{type_name}{generic_args}");
    match naming_style {
        FunctionNamingStyle::Namespace => format!("{type_name}.fromObject{generic_args}({value_expr})"),
        FunctionNamingStyle::Suffix => format!("fromObject{type_name}{generic_args}({value_expr})"),
        FunctionNamingStyle::Prefix => format!(
            "{}FromObject{generic_args}({value_expr})",
            to_camel_case(type_name)
        ),
        FunctionNamingStyle::Generic => format!("fromObject<{applied_type}>({value_expr})"),
    }
}

pub fn call_from_stringified_json(
    type_name: &str,
    generic_args: &str,
    naming_style: FunctionNamingStyle,
    json_expr: &str,
) -> String {
    let applied_type = format!("{type_name}{generic_args}");
    match naming_style {
        FunctionNamingStyle::Namespace => {
            format!("{type_name}.fromStringifiedJSON{generic_args}({json_expr})")
        }
        FunctionNamingStyle::Suffix => format!("fromStringifiedJSON{type_name}{generic_args}({json_expr})"),
        FunctionNamingStyle::Prefix => format!(
            "{}FromStringifiedJSON{generic_args}({json_expr})",
            to_camel_case(type_name)
        ),
        FunctionNamingStyle::Generic => format!("fromStringifiedJSON<{applied_type}>({json_expr})"),
    }
}

pub fn fn_name_from_form_data(type_name: &str, generic_decl: &str, naming_style: FunctionNamingStyle) -> String {
    match naming_style {
        FunctionNamingStyle::Namespace => format!("fromFormData{generic_decl}"),
        FunctionNamingStyle::Suffix => format!("fromFormData{type_name}{generic_decl}"),
        FunctionNamingStyle::Prefix => format!("{}FromFormData{generic_decl}", to_camel_case(type_name)),
        FunctionNamingStyle::Generic => format!("fromFormData{generic_decl}"),
    }
}

/// Generates a function name with the correct style.
/// For prefix style: `userCreateForm`, `userGetDefaultForVariant`
/// For suffix style: `createFormUser`, `getDefaultForVariantUser`
pub fn fn_name(base: &str, type_name: &str, generic_decl: &str, naming_style: FunctionNamingStyle) -> String {
    match naming_style {
        FunctionNamingStyle::Namespace => format!("{base}{generic_decl}"),
        FunctionNamingStyle::Suffix => format!("{base}{type_name}{generic_decl}"),
        FunctionNamingStyle::Prefix => format!("{}{}{generic_decl}", to_camel_case(type_name), capitalize_first(base)),
        FunctionNamingStyle::Generic => format!("{base}{generic_decl}"),
    }
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

pub fn call_default_value_for_type_ref(type_ref: &str, naming_style: FunctionNamingStyle) -> String {
    let tr = type_ref.trim();
    if let Some(bracket_pos) = tr.find('<') {
        let base_type = &tr[..bracket_pos];
        let type_args = &tr[bracket_pos..]; // includes <>
        return match naming_style {
            FunctionNamingStyle::Namespace => format!("{base_type}.defaultValue{type_args}()"),
            FunctionNamingStyle::Suffix => format!("defaultValue{base_type}{type_args}()"),
            FunctionNamingStyle::Prefix => format!("{}DefaultValue{type_args}()", to_camel_case(base_type)),
            FunctionNamingStyle::Generic => format!("defaultValue<{tr}>()"),
        };
    }

    match naming_style {
        FunctionNamingStyle::Namespace => format!("{tr}.defaultValue()"),
        FunctionNamingStyle::Suffix => format!("defaultValue{tr}()"),
        FunctionNamingStyle::Prefix => format!("{}DefaultValue()", to_camel_case(tr)),
        FunctionNamingStyle::Generic => format!("defaultValue<{tr}>()"),
    }
}
