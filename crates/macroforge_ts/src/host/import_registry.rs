//! # Import Registry (re-exports + foreign types)
//!
//! The core [`ImportRegistry`] lives in `macroforge_ts_syn`. This module re-exports
//! it and adds the `foreign_types` thread-local, which depends on [`ForeignTypeConfig`]
//! from this crate.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Weak};

use super::{ForeignTypeConfig, MacroforgeConfig};

// Re-export everything from macroforge_ts_syn's import_registry
pub use macroforge_ts_syn::import_registry::{
    GeneratedImport, ImportRegistry, SourceImport, clear_registry, install_registry, take_registry,
    with_registry, with_registry_mut,
};

// ============================================================================
// Foreign types thread-local (depends on ForeignTypeConfig from this crate)
// ============================================================================

thread_local! {
    /// The configured foreign types, shared by every reader on the thread.
    static FOREIGN_TYPES: RefCell<Rc<[ForeignTypeConfig]>> = RefCell::new(Rc::from([]));
    /// The config the installed foreign types came from, while it is alive.
    static INSTALLED_FROM: RefCell<Weak<MacroforgeConfig>> = const { RefCell::new(Weak::new()) };
}

/// Read foreign types (immutable access).
pub fn with_foreign_types<R>(f: impl FnOnce(&[ForeignTypeConfig]) -> R) -> R {
    FOREIGN_TYPES.with(|ft| f(&ft.borrow()))
}

/// The configured foreign types, shared rather than copied.
pub fn configured_foreign_types() -> Rc<[ForeignTypeConfig]> {
    FOREIGN_TYPES.with(|ft| Rc::clone(&ft.borrow()))
}

/// Set foreign types.
pub fn set_foreign_types(types: Vec<ForeignTypeConfig>) {
    INSTALLED_FROM.with(|from| *from.borrow_mut() = Weak::new());
    FOREIGN_TYPES.with(|ft| {
        *ft.borrow_mut() = Rc::from(types);
    });
}

/// Install `config`'s foreign types, unless they are installed already: a
/// config is never changed once shared, so the same one holds the same types.
pub fn set_foreign_types_from(config: &Arc<MacroforgeConfig>) {
    let installed = INSTALLED_FROM.with(|from| {
        from.borrow()
            .upgrade()
            .is_some_and(|current| Arc::ptr_eq(&current, config))
    });
    if !installed {
        set_foreign_types(config.foreign_types.clone());
        INSTALLED_FROM.with(|from| *from.borrow_mut() = Arc::downgrade(config));
    }
}

/// Clear foreign types.
pub fn clear_foreign_types() {
    set_foreign_types(Vec::new());
}
