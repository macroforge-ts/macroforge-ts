#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Stable span for macro ABI.
/// Host maps SWC spans <-> SpanIR.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpanIR {
    pub start: u32, // byte offset
    pub end: u32,   // byte offset
}

impl SpanIR {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}
