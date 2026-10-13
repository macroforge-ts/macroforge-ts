//! Field and container options read from `@endec(...)`.

use super::make_decorator;
use crate::builtin::derive::endec::{
    EndecContainerOptions, EndecFieldOptions, EndecFormat, RenameAll, TaggingMode,
};

#[test]
fn test_field_skip() {
    let decorator = make_decorator("skip");
    let result = EndecFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert!(opts.skip);
    assert!(!opts.should_encode());
    assert!(!opts.should_decode());
}

#[test]
fn test_field_skip_encoding() {
    let decorator = make_decorator("skipEncoding");
    let result = EndecFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert!(opts.skip_encoding);
    assert!(!opts.should_encode());
    assert!(opts.should_decode());
}

#[test]
fn test_field_rename() {
    let decorator = make_decorator(r#"{ rename: "user_id" }"#);
    let result = EndecFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert_eq!(opts.rename.as_deref(), Some("user_id"));
}

#[test]
fn a_word_in_a_value_is_no_option() {
    let decorator = make_decorator(r#"{ rename: "skip", default: "\"flatten, (default\"" }"#);
    let opts = EndecFieldOptions::from_decorators(&[decorator], "test_field").options;
    assert!(!opts.skip);
    assert!(!opts.flatten);
    assert_eq!(opts.rename.as_deref(), Some("skip"));
    assert_eq!(opts.default_expr.as_deref(), Some("\"flatten, (default\""));
}

#[test]
fn a_value_of_the_wrong_kind_sets_nothing() {
    let decorator = make_decorator(r#"{ rename: 5, skip: "yes", flatten: 1 }"#);
    let opts = EndecFieldOptions::from_decorators(&[decorator], "test_field").options;
    assert_eq!(opts.rename, None);
    assert!(!opts.skip);
    assert!(!opts.flatten);

    let container = make_decorator(r#"{ tag: 5, renameAll: "weird", denyUnknownFields: "yes" }"#);
    let opts = EndecContainerOptions::from_decorators(&[container]);
    assert_eq!(opts.tagging, TaggingMode::default());
    assert_eq!(opts.rename_all, RenameAll::None);
    assert!(!opts.deny_unknown_fields);
}

#[test]
fn test_field_default_flag() {
    let decorator = make_decorator("default");
    let result = EndecFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert!(opts.default);
    assert!(opts.default_expr.is_none());
}

#[test]
fn test_field_default_expr() {
    let decorator = make_decorator(r#"{ default: "new Date()" }"#);
    let result = EndecFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert!(opts.default);
    assert_eq!(opts.default_expr.as_deref(), Some("new Date()"));
}

#[test]
fn test_field_flatten() {
    let decorator = make_decorator("flatten");
    let result = EndecFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert!(opts.flatten);
}

#[test]
fn test_field_decimal_format() {
    let decorator = make_decorator(r#"{ format: "decimal" }"#);
    let result = EndecFieldOptions::from_decorators(&[decorator], "amount");
    assert_eq!(result.options.format, Some(EndecFormat::Decimal));
    assert!(!result.diagnostics.has_errors());
}

#[test]
fn test_field_unknown_format_is_an_error() {
    let decorator = make_decorator(r#"{ format: "decmial" }"#);
    let result = EndecFieldOptions::from_decorators(&[decorator], "amount");
    assert_eq!(result.options.format, None);
    assert!(result.diagnostics.has_errors());
}

#[test]
fn test_container_rename_all() {
    let decorator = make_decorator(r#"{ renameAll: "camelCase" }"#);
    let opts = EndecContainerOptions::from_decorators(&[decorator]);
    assert_eq!(opts.rename_all, RenameAll::CamelCase);
}

#[test]
fn test_container_deny_unknown_fields() {
    let decorator = make_decorator("denyUnknownFields");
    let opts = EndecContainerOptions::from_decorators(&[decorator]);
    assert!(opts.deny_unknown_fields);
}

#[test]
fn test_container_tag_internally_tagged() {
    let decorator = make_decorator(r#"{ tag: "type" }"#);
    let opts = EndecContainerOptions::from_decorators(&[decorator]);
    assert_eq!(
        opts.tagging,
        TaggingMode::InternallyTagged {
            tag: "type".to_string()
        }
    );
    assert_eq!(opts.tag_field(), Some("type"));
    assert_eq!(opts.tag_field_or_default(), "type");
    assert_eq!(opts.content_field(), None);
}

#[test]
fn test_container_tag_default() {
    let opts = EndecContainerOptions::default();
    assert_eq!(
        opts.tagging,
        TaggingMode::InternallyTagged {
            tag: "__type".to_string()
        }
    );
    assert_eq!(opts.tag_field(), Some("__type"));
    assert_eq!(opts.tag_field_or_default(), "__type");
}

#[test]
fn test_container_externally_tagged() {
    let decorator = make_decorator(r#"{ externallyTagged: true }"#);
    let opts = EndecContainerOptions::from_decorators(&[decorator]);
    assert_eq!(opts.tagging, TaggingMode::ExternallyTagged);
    assert_eq!(opts.tag_field(), None);
    assert_eq!(opts.tag_field_or_default(), "__type");
}

#[test]
fn test_container_adjacently_tagged() {
    let decorator = make_decorator(r#"{ tag: "t", content: "c" }"#);
    let opts = EndecContainerOptions::from_decorators(&[decorator]);
    assert_eq!(
        opts.tagging,
        TaggingMode::AdjacentlyTagged {
            tag: "t".to_string(),
            content: "c".to_string()
        }
    );
    assert_eq!(opts.tag_field(), Some("t"));
    assert_eq!(opts.tag_field_or_default(), "t");
    assert_eq!(opts.content_field(), Some("c"));
}

#[test]
fn test_container_untagged() {
    let decorator = make_decorator(r#"{ untagged: true }"#);
    let opts = EndecContainerOptions::from_decorators(&[decorator]);
    assert_eq!(opts.tagging, TaggingMode::Untagged);
    assert_eq!(opts.tag_field(), None);
    assert_eq!(opts.tag_field_or_default(), "__type");
    assert_eq!(opts.content_field(), None);
}

#[test]
fn test_rename_all_camel_case() {
    assert_eq!(RenameAll::CamelCase.apply("user_name"), "userName");
    assert_eq!(RenameAll::CamelCase.apply("created_at"), "createdAt");
}

#[test]
fn test_rename_all_snake_case() {
    assert_eq!(RenameAll::SnakeCase.apply("userName"), "user_name");
    assert_eq!(RenameAll::SnakeCase.apply("createdAt"), "created_at");
}

#[test]
fn test_rename_all_pascal_case() {
    assert_eq!(RenameAll::PascalCase.apply("user_name"), "UserName");
}

#[test]
fn test_rename_all_kebab_case() {
    assert_eq!(RenameAll::KebabCase.apply("userName"), "user-name");
}

#[test]
fn test_rename_all_screaming_snake_case() {
    assert_eq!(RenameAll::ScreamingSnakeCase.apply("userName"), "USER_NAME");
}

// ========================================================================
// Custom encoder/decoder tests (encodeWith/decodeWith)
// ========================================================================

#[test]
fn test_field_encode_with() {
    let decorator = make_decorator(r#"{ encodeWith: "myEncoder" }"#);
    let result = EndecFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert_eq!(opts.encode_with.as_deref(), Some("myEncoder"));
    assert!(opts.decode_with.is_none());
}

#[test]
fn test_field_decode_with() {
    let decorator = make_decorator(r#"{ decodeWith: "myDecoder" }"#);
    let result = EndecFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert!(opts.encode_with.is_none());
    assert_eq!(opts.decode_with.as_deref(), Some("myDecoder"));
}

#[test]
fn test_field_encode_and_decode_with() {
    let decorator = make_decorator(r#"{ encodeWith: "toJson", decodeWith: "fromJson" }"#);
    let result = EndecFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert_eq!(opts.encode_with.as_deref(), Some("toJson"));
    assert_eq!(opts.decode_with.as_deref(), Some("fromJson"));
}

#[test]
fn test_field_encode_with_combined_with_other_options() {
    let decorator =
        make_decorator(r#"{ encodeWith: "customEncode", rename: "custom_field", skip: false }"#);
    let result = EndecFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert_eq!(opts.encode_with.as_deref(), Some("customEncode"));
    assert_eq!(opts.rename.as_deref(), Some("custom_field"));
    assert!(!opts.skip);
}
