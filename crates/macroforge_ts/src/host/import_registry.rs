//! # Import Registry (re-exports + foreign types)
//!
//! The core [`ImportRegistry`] lives in `macroforge_ts_syn`. This module re-exports
//! it and adds the `foreign_types` thread-local, which depends on [`ForeignTypeConfig`]
//! from this crate.

use std::cell::RefCell;

use super::ForeignTypeConfig;

// Re-export everything from macroforge_ts_syn's import_registry
pub use macroforge_ts_syn::import_registry::{
    GeneratedImport, ImportRegistry, SourceImport, clear_registry, install_registry, take_registry,
    with_registry, with_registry_mut,
};

// ============================================================================
// Foreign types thread-local (depends on ForeignTypeConfig from this crate)
// ============================================================================

thread_local! {
    static FOREIGN_TYPES: RefCell<Vec<ForeignTypeConfig>> = const { RefCell::new(Vec::new()) };
}

/// Read foreign types (immutable access).
pub fn with_foreign_types<R>(f: impl FnOnce(&[ForeignTypeConfig]) -> R) -> R {
    FOREIGN_TYPES.with(|ft| f(&ft.borrow()))
}

/// Write foreign types (mutable access).
pub fn with_foreign_types_mut<R>(f: impl FnOnce(&mut Vec<ForeignTypeConfig>) -> R) -> R {
    FOREIGN_TYPES.with(|ft| f(&mut ft.borrow_mut()))
}

/// Set foreign types.
pub fn set_foreign_types(types: Vec<ForeignTypeConfig>) {
    FOREIGN_TYPES.with(|ft| {
        *ft.borrow_mut() = types;
    });
}

/// Clear foreign types.
pub fn clear_foreign_types() {
    FOREIGN_TYPES.with(|ft| {
        ft.borrow_mut().clear();
    });
}
