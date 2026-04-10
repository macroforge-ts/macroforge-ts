//! Thread-local storage for the [`MacroContextIR`](crate::abi::ir::context::MacroContextIR)
//! currently being processed.
//!
//! Mirrors the pattern in [`crate::import_registry`]: the host installs the
//! current macro's context before invoking the macro, and helper APIs (notably
//! [`crate::stream::TsStream::add_import_for`]) read it back without forcing
//! every consumer to plumb the context through their own data structures.
//!
//! The host is responsible for installing and clearing the context around each
//! macro dispatch — see `host::expand`'s per-macro loop in `macroforge_ts`.

use std::cell::RefCell;

use crate::abi::ir::context::MacroContextIR;

thread_local! {
    static CURRENT_CONTEXT: RefCell<Option<MacroContextIR>> = const { RefCell::new(None) };
}

/// Install the given context as the current thread's active macro context.
/// Replaces any previously installed context.
pub fn install_context(ctx: MacroContextIR) {
    CURRENT_CONTEXT.with(|slot| *slot.borrow_mut() = Some(ctx));
}

/// Remove the current thread's active macro context. Called by the host
/// after each macro dispatch returns.
pub fn clear_context() {
    CURRENT_CONTEXT.with(|slot| *slot.borrow_mut() = None);
}

/// Borrow the current thread's active macro context, if any.
pub fn with_context<R>(f: impl FnOnce(Option<&MacroContextIR>) -> R) -> R {
    CURRENT_CONTEXT.with(|slot| f(slot.borrow().as_ref()))
}
