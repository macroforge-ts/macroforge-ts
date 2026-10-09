use crate::builtin::derive::common::{extract_named_string, field_value_type, has_flag};
use crate::ts_syn::abi::DecoratorIR;

/// Options parsed from @Debug decorator on fields
#[derive(Default)]
pub(super) struct DebugFieldOptions {
    pub(super) skip: bool,
    pub(super) rename: Option<String>,
}

impl DebugFieldOptions {
    pub(super) fn from_decorators(decorators: &[DecoratorIR]) -> Self {
        let mut opts = DebugFieldOptions::default();
        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case("debug") {
                continue;
            }

            let args = decorator.args_src.trim();
            if args.is_empty() {
                continue;
            }

            if has_flag(args, "skip") {
                opts.skip = true;
            }

            if let Some(rename) = extract_named_string(args, "rename") {
                opts.rename = Some(rename);
            }
        }
        opts
    }
}

/// A field shown in debug output.
pub(super) struct DebugField {
    /// The label printed before the value: the field name, or its `rename`.
    pub(super) label: String,
    pub(super) name: String,
    /// The field's type as its values have it, `undefined` included when the
    /// field is optional.
    pub(super) ts_type: String,
}

impl DebugField {
    /// The field, unless `@debug(skip)` hides it.
    pub(super) fn shown(
        name: &str,
        ts_type: &str,
        optional: bool,
        decorators: &[DecoratorIR],
    ) -> Option<Self> {
        let opts = DebugFieldOptions::from_decorators(decorators);
        if opts.skip {
            return None;
        }
        Some(Self {
            label: opts.rename.unwrap_or_else(|| name.to_string()),
            name: name.to_string(),
            ts_type: field_value_type(ts_type, optional),
        })
    }
}
