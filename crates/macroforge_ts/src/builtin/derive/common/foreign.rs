//! The handlers a configured foreign type runs in place of a derived
//! function.

use crate::builtin::derive::endec::{TypeCategory, get_foreign_types};
use crate::ts_syn::config::ForeignHandler;

/// What generated code calls to run `handler` on a value of `ts_type`, when
/// a config declares that type: its handler, or the error that it configures
/// none. `None` for a type no config declares; a built-in foreign type is
/// handled by its JavaScript class.
pub fn foreign_handler(ts_type: &str, handler: ForeignHandler) -> Option<Result<String, String>> {
    let foreign_types = get_foreign_types();
    let config = TypeCategory::match_foreign_type(ts_type, &foreign_types).config?;
    if config.builtin {
        return None;
    }
    Some(config.handler_callee(handler).ok_or_else(|| {
        format!(
            "the foreign type '{}' configures no `{}` handler, which a value of it needs here",
            config.name,
            handler.key()
        )
    }))
}
