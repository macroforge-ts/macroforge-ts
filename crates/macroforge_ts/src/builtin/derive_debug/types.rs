use crate::builtin::derive_common::{extract_named_string, has_flag};

/// Options parsed from @Debug decorator on fields
#[derive(Default)]
pub(super) struct DebugFieldOptions {
    pub(super) skip: bool,
    pub(super) rename: Option<String>,
}

impl DebugFieldOptions {
    pub(super) fn from_decorators(decorators: &[crate::ts_syn::abi::DecoratorIR]) -> Self {
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

/// Debug field info: (label, field_name, ts_type)
pub(super) type DebugField = (String, String, String);
