//! What a file looked like when something derived from it was cached, so the
//! cache can tell when the file changed.

use std::path::Path;

/// A file's modification time and length. Two stamps of one file differ once
/// it is rewritten, which is what process-lifetime caches key on to follow
/// a rebuilt package or an edited config.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileStamp {
    /// Nanoseconds since the Unix epoch.
    modified: u128,
    len: u64,
}

impl FileStamp {
    /// The stamp of the file at `path` as it is now.
    ///
    /// # Errors
    ///
    /// Fails when the file cannot be read, or the platform keeps no
    /// modification time, since a stamp that cannot change would pin a stale
    /// cache entry forever.
    pub fn of(path: &Path) -> std::io::Result<Self> {
        let (modified, len) = crate::host::file_access::modified_and_len(path)?;
        Ok(Self { modified, len })
    }
}
