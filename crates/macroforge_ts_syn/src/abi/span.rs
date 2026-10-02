//! Source span representation for position tracking.
//!
//! This module provides [`SpanIR`], a stable, serializable span type that
//! represents byte ranges in source code. Unlike a parser's own span type,
//! `SpanIR` is plain byte positions and is designed for ABI stability.
//!
//! ## Positions
//!
//! A position is a byte offset plus one: the first byte of the file is
//! position 1. Byte offsets (not character indices or line/column pairs)
//! stay exact with multi-byte UTF-8 characters. To read the spanned text,
//! use [`SpanIR::source_range`], which gives the 0-based byte range.
//!
//! ## Example
//!
//! ```rust
//! use macroforge_ts_syn::SpanIR;
//!
//! // "x" in "let x = 42;" is byte 4, so its span is positions 5..6.
//! let source = "let x = 42;";
//! let span = SpanIR::new(5, 6);
//! assert_eq!(span.len(), 1);
//! assert_eq!(&source[span.source_range()], "x");
//! ```

use serde::{Deserialize, Serialize};

/// A stable source span using byte offsets.
///
/// Represents a contiguous range in source code from `start` (inclusive)
/// to `end` (exclusive). Positions are byte offsets plus one, as the host
/// produces them from the parser's spans; see the [module docs](self).
///
/// # Fields
///
/// - `start` - The starting position (inclusive)
/// - `end` - The ending position (exclusive)
///
/// # Invariants
///
/// - `start <= end` for a valid, non-empty span
/// - `start == end` represents an empty span (e.g., for insertion points)
///
/// # Example
///
/// ```rust
/// use macroforge_ts_syn::SpanIR;
///
/// // "hello" in "say hello world" is bytes 4..9, so positions 5..10.
/// let span = SpanIR::new(5, 10);
/// assert_eq!(span.len(), 5);
/// assert_eq!(&"say hello world"[span.source_range()], "hello");
/// ```
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SpanIR {
    /// Starting position (inclusive): a byte offset plus one.
    pub start: u32,

    /// Ending position (exclusive): a byte offset plus one.
    pub end: u32,
}

impl SpanIR {
    /// Creates a new span from start and end positions.
    ///
    /// # Arguments
    ///
    /// - `start` - The starting position (inclusive)
    /// - `end` - The ending position (exclusive)
    ///
    /// # Example
    ///
    /// ```rust,no_run
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
    /// ```rust,no_run
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
    /// ```rust,no_run
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

    /// The span's 0-based byte range in the source, for slicing it.
    ///
    /// # Example
    ///
    /// ```rust
    /// use macroforge_ts_syn::SpanIR;
    ///
    /// let source = "let x = 42;";
    /// assert_eq!(&source[SpanIR::new(9, 11).source_range()], "42");
    /// ```
    pub fn source_range(&self) -> std::ops::Range<usize> {
        self.start.saturating_sub(1) as usize..self.end.saturating_sub(1) as usize
    }
}
