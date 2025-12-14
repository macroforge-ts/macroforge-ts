//! Source span representation for position tracking.
//!
//! This module provides [`SpanIR`], a stable, serializable span type that
//! represents byte ranges in source code. Unlike SWC's `Span` type, `SpanIR`
//! uses simple byte offsets and is designed for ABI stability.
//!
//! ## Byte Offsets
//!
//! Spans use byte offsets (not character indices or line/column pairs) for:
//! - Precision with multi-byte UTF-8 characters
//! - Efficient substring operations
//! - Simple arithmetic for position calculations
//!
//! ## Example
//!
//! ```rust,ignore
//! use macroforge_ts_syn::SpanIR;
//!
//! // Create a span for bytes 10-25
//! let span = SpanIR::new(10, 25);
//! assert_eq!(span.len(), 15);
//! assert!(!span.is_empty());
//!
//! // Extract the spanned text from source
//! let source = "let x = 42;";
//! let text = &source[span.start as usize..span.end as usize];
//! ```

use serde::{Deserialize, Serialize};

/// A stable source span using byte offsets.
///
/// Represents a contiguous range in source code from `start` (inclusive)
/// to `end` (exclusive). The host system maps between SWC's internal
/// spans and `SpanIR` for macro communication.
///
/// # Fields
///
/// - `start` - The starting byte offset (inclusive)
/// - `end` - The ending byte offset (exclusive)
///
/// # Invariants
///
/// - `start <= end` for a valid, non-empty span
/// - `start == end` represents an empty span (e.g., for insertion points)
///
/// # Example
///
/// ```rust,ignore
/// use macroforge_ts_syn::SpanIR;
///
/// // Span covering "hello" in "say hello world"
/// //                    ^^^^^
/// // byte indices:  4   5678 9
/// let span = SpanIR::new(4, 9);
/// assert_eq!(span.len(), 5);
///
/// // Extract the text
/// let source = "say hello world";
/// let text = &source[span.start as usize..span.end as usize];
/// assert_eq!(text, "hello");
/// ```
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpanIR {
    /// Starting byte offset (inclusive).
    pub start: u32,

    /// Ending byte offset (exclusive).
    pub end: u32,
}

impl SpanIR {
    /// Creates a new span from start and end byte offsets.
    ///
    /// # Arguments
    ///
    /// - `start` - The starting byte offset (inclusive)
    /// - `end` - The ending byte offset (exclusive)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use macroforge_ts_syn::SpanIR;
    ///
    /// let span = SpanIR::new(0, 10);
    /// assert_eq!(span.start, 0);
    /// assert_eq!(span.end, 10);
    /// ```
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    /// Returns the length of the span in bytes.
    ///
    /// Uses saturating subtraction to handle edge cases where
    /// `end < start` (returns 0 in that case).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use macroforge_ts_syn::SpanIR;
    ///
    /// let span = SpanIR::new(5, 15);
    /// assert_eq!(span.len(), 10);
    /// ```
    pub fn len(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }

    /// Returns `true` if the span is empty (zero length).
    ///
    /// Empty spans are useful for marking insertion points where
    /// no existing code is being replaced.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use macroforge_ts_syn::SpanIR;
    ///
    /// let insertion_point = SpanIR::new(42, 42);
    /// assert!(insertion_point.is_empty());
    ///
    /// let region = SpanIR::new(10, 20);
    /// assert!(!region.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}
