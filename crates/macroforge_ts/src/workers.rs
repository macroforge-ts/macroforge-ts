//! The threads the engine's parallel work runs on.
//!
//! Expansion and lowering recurse as deep as the code they walk, which
//! overflows rayon's default 2 MB stack as a crash rather than a diagnostic,
//! so the workers get 32 MB. One pool serves the whole process and lives as
//! long as it, so no call pays for starting threads.

static WORKER_POOL: std::sync::LazyLock<Result<rayon::ThreadPool, String>> =
    std::sync::LazyLock::new(|| {
        rayon::ThreadPoolBuilder::new()
            .stack_size(32 * 1024 * 1024)
            .thread_name(|index| format!("macroforge-worker-{index}"))
            .build()
            .map_err(|err| format!("failed to start the macroforge worker pool: {err}"))
    });

/// The process's worker pool.
pub fn worker_pool() -> Result<&'static rayon::ThreadPool, String> {
    WORKER_POOL.as_ref().map_err(Clone::clone)
}
