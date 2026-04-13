//! Boa backend — pure-Rust JS engine.
//!
//! [`boa_engine`] compiles cleanly for wasm32-unknown-unknown (no C
//! code, no libc dependency), so it's the backend the Vite plugin's
//! WASM build ships. It also works on native targets.
//!
//! Scope of this backend:
//! - Synchronous evaluation of a script
//! - Value conversion (JsValue → SandboxValue) for null/undefined,
//!   booleans, numbers, strings, arrays, plain objects
//! - Native implementations of `buildtime.fs.*`, `buildtime.crypto.*`,
//!   `buildtime.time.*`, `buildtime.env`, `buildtime.flags.*`,
//!   `buildtime.location.*`, with capability enforcement
//! - Dependency tracking for `fs` reads

use std::cell::RefCell;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context as TaskContext, Poll, Waker};

// `web_time` is a drop-in for `std::time` types that works on
// wasm32-unknown-unknown. On native targets it re-exports std's
// types. We use it everywhere a clock is read so the WASM build
// doesn't panic.
use web_time::{Instant, SystemTime, UNIX_EPOCH};

use boa_engine::{
    Context, JsError, JsNativeError, JsObject, JsResult, JsValue, NativeFunction, Script, Source,
    context::{
        ContextBuilder,
        time::{Clock, JsInstant},
    },
    job::{JobExecutor, SimpleJobExecutor},
    js_string,
    object::builtins::JsArray,
    property::Attribute,
};

use crate::host::buildtime::sandbox::{
    BuildtimeSandbox, EvalResult, SandboxError, SandboxOptions, SandboxValue,
};

/// State shared between the outer driver and the native API functions
/// installed inside the Boa context. Single-threaded — the Boa context
/// is constructed and consumed on one thread per evaluation.
struct SandboxState {
    options: SandboxOptions,
    dependencies: Vec<PathBuf>,
    side_channel_error: Option<SandboxError>,
}

#[derive(Default)]
pub struct BoaSandbox {
    _private: (),
}

impl BoaSandbox {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl BuildtimeSandbox for BoaSandbox {
    fn name(&self) -> &'static str {
        "boa"
    }

    fn evaluate(
        &self,
        source: &str,
        _origin: &Path,
        options: &SandboxOptions,
    ) -> Result<EvalResult, SandboxError> {
        let state = Rc::new(RefCell::new(SandboxState {
            options: options.clone(),
            dependencies: Vec::new(),
            side_channel_error: None,
        }));

        // Build the context with a SimpleJobExecutor we keep a handle
        // to so we can drive its `run_jobs_async` future with our
        // own deadline-aware poll loop. We also install a custom
        // [`WebTimeClock`] because Boa's default `StdClock` calls
        // `std::time::SystemTime::now()`, which panics on the
        // wasm32-unknown-unknown target the Vite plugin runs on.
        let executor = Rc::new(SimpleJobExecutor::new());
        let mut context = ContextBuilder::default()
            .job_executor(executor.clone())
            .clock(Rc::new(WebTimeClock))
            .build()
            .map_err(|e| SandboxError::Backend(format!("context build: {e}")))?;
        install_buildtime_api(&mut context, Rc::clone(&state))?;

        let wrapped = wrap_source(source);
        let deadline = Instant::now() + options.timeout;

        // Parse + evaluate with a per-budget yielding async future,
        // so we can interrupt long-running scripts at deadline. Boa
        // doesn't expose per-instruction interrupt hooks; instead
        // `Script::evaluate_async_with_budget` yields to the executor
        // every N "clock cycles" — `drive_to_deadline` polls that
        // future with a no-op waker and bails on deadline.
        let script = match Script::parse(
            Source::from_bytes(wrapped.as_bytes()),
            None,
            &mut context,
        ) {
            Ok(s) => s,
            Err(err) => return Err(SandboxError::Backend(err.to_string())),
        };
        {
            let fut = std::pin::pin!(
                script.evaluate_async_with_budget(&mut context, EVAL_BUDGET)
            );
            if let Err(err) = drive_to_deadline(fut, deadline) {
                return Err(err);
            }
        }

        // Drain pending microtasks so the async IIFE wrapper resolves.
        // We use the same deadline so cumulative time matters.
        {
            let ctx_cell = std::cell::RefCell::new(&mut context);
            let fut = std::pin::pin!(executor.clone().run_jobs_async(&ctx_cell));
            if let Err(err) = drive_to_deadline(fut, deadline) {
                return Err(err);
            }
        }

        if let Some(err) = state.borrow_mut().side_channel_error.take() {
            return Err(err);
        }

        let globals = context.global_object();
        let err_value = globals
            .get(js_string!("__macroforgeError"), &mut context)
            .map_err(|e| SandboxError::Backend(format!("global read: {e}")))?;
        if !err_value.is_null_or_undefined() {
            return Err(extract_thrown(err_value, &mut context));
        }

        let result_value = globals
            .get(js_string!("__macroforgeResult"), &mut context)
            .map_err(|e| SandboxError::Backend(format!("result read: {e}")))?;

        let value = js_to_sandbox(&result_value, &mut context, 0)?;
        let deps = std::mem::take(&mut state.borrow_mut().dependencies);
        Ok(EvalResult {
            value,
            dependencies: deps,
        })
    }
}

/// Custom Boa clock that reads time via `web_time::SystemTime`.
///
/// Boa's default `StdClock` calls `std::time::SystemTime::now()`,
/// which panics on `wasm32-unknown-unknown` (no monotonic clock in
/// std). `web_time` is a drop-in shim that uses `performance.now()`
/// in the browser; on native it re-exports std's clock.
struct WebTimeClock;

impl Clock for WebTimeClock {
    fn now(&self) -> JsInstant {
        let dur = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        JsInstant::new(dur.as_secs(), dur.subsec_nanos())
    }
}

/// Number of Boa "clock cycles" the VM runs between executor yields.
/// Lower = more responsive to timeout but higher per-iteration overhead.
/// 1024 gives ~10–100 µs between yields on a typical laptop.
const EVAL_BUDGET: u32 = 1024;

/// Drive a Boa-produced future to completion, returning [`SandboxError::Timeout`]
/// once the deadline passes. Uses a no-op waker because Boa's
/// `evaluate_async_with_budget` is pure-CPU work — there's nothing to
/// wake when it returns Pending; the caller just polls again.
///
/// Takes a stack-pinned `Pin<&mut F>` so non-`'static` futures (which
/// Boa's APIs return — they borrow the Context) work without boxing.
fn drive_to_deadline<F, T>(
    mut future: Pin<&mut F>,
    deadline: Instant,
) -> Result<T, SandboxError>
where
    F: Future<Output = JsResult<T>>,
{
    let waker = Waker::noop();
    let mut cx = TaskContext::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(Ok(v)) => return Ok(v),
            Poll::Ready(Err(err)) => return Err(SandboxError::Backend(err.to_string())),
            Poll::Pending => {
                if Instant::now() >= deadline {
                    return Err(SandboxError::Timeout {
                        duration: std::time::Duration::ZERO,
                    });
                }
            }
        }
    }
}

/// Wrap user source in an async IIFE so `await` works at the top
/// level of the user's body. 5 lines of preamble keep
/// `WRAPPER_PREAMBLE_LINES` in prepass.rs correct for stack remapping.
fn wrap_source(source: &str) -> String {
    let mut out = String::with_capacity(source.len() + 256);
    out.push_str(
        "globalThis.__macroforgeResult = undefined;\n\
         globalThis.__macroforgeError = undefined;\n\
         (async function () {\n\
         try {\n\
         globalThis.__macroforgeResult = await (async function () {\n",
    );
    out.push_str(source);
    out.push_str(
        "\n})();\n\
         } catch (e) {\n\
             globalThis.__macroforgeError = {\n\
                 message: (e && e.message) || String(e),\n\
                 stack: (e && e.stack) || \"\",\n\
             };\n\
         }\n\
         })();\n",
    );
    out
}

// ---------------------------------------------------------------------
// Native `buildtime` API installation
// ---------------------------------------------------------------------

fn install_buildtime_api(
    context: &mut Context,
    state: Rc<RefCell<SandboxState>>,
) -> Result<(), SandboxError> {
    let buildtime = JsObject::with_null_proto();

    install_fs(&buildtime, context, Rc::clone(&state))?;
    install_crypto(&buildtime, context)?;
    install_time(&buildtime, context)?;
    install_env(&buildtime, context, &state.borrow().options)?;
    install_flags(&buildtime, context)?;
    install_location(&buildtime, context, &state.borrow().options)?;

    context
        .register_global_property(js_string!("buildtime"), buildtime, Attribute::all())
        .map_err(|e| SandboxError::Backend(e.to_string()))?;
    Ok(())
}

/// Wrap a stateful closure as a NativeFunction. The closure captures
/// `Rc<RefCell<SandboxState>>`, which isn't `Copy` and must be passed
/// via the unsafe constructor. `Rc<RefCell<SandboxState>>` contains no
/// JS values, so there's nothing for the GC to trace — the unsafe
/// invariant (no Trace captures) holds.
fn stateful_fn<F>(closure: F) -> NativeFunction
where
    F: Fn(&JsValue, &[JsValue], &mut Context) -> JsResult<JsValue> + 'static,
{
    // SAFETY: The captured `Rc<RefCell<SandboxState>>` owns only Rust
    // data (Vec<PathBuf>, SandboxOptions, Option<SandboxError>) — no
    // `JsObject`, `JsValue`, or any GC-traceable type. Boa's Trace
    // invariant therefore holds vacuously.
    unsafe { NativeFunction::from_closure(closure) }
}

fn install_fs(
    buildtime: &JsObject,
    context: &mut Context,
    state: Rc<RefCell<SandboxState>>,
) -> Result<(), SandboxError> {
    let fs = JsObject::with_null_proto();

    let st = Rc::clone(&state);
    let read_text = stateful_fn(move |_this, args, ctx| {
        let path = arg_as_string(args, 0, ctx)?;
        match fs_read_text_impl(&st, &path) {
            Ok(text) => Ok(JsValue::from(js_string!(text.as_str()))),
            Err(err) => stash_and_throw(&st, err),
        }
    });
    register_method(&fs, "readText", read_text, context)?;

    let st = Rc::clone(&state);
    let read_json = stateful_fn(move |_this, args, ctx| {
        let path = arg_as_string(args, 0, ctx)?;
        match fs_read_text_impl(&st, &path) {
            Ok(text) => {
                // Parse JSON via JSON.parse so we return a real JS value.
                let json_global = ctx
                    .global_object()
                    .get(js_string!("JSON"), ctx)?
                    .as_object()
                    .ok_or_else(|| {
                        JsError::from_native(JsNativeError::typ().with_message("JSON global"))
                    })?;
                let parse = json_global.get(js_string!("parse"), ctx)?;
                let parse_obj = parse.as_object().ok_or_else(|| {
                    JsError::from_native(
                        JsNativeError::typ().with_message("JSON.parse not an object"),
                    )
                })?;
                parse_obj.call(
                    &JsValue::from(json_global),
                    &[JsValue::from(js_string!(text.as_str()))],
                    ctx,
                )
            }
            Err(err) => stash_and_throw(&st, err),
        }
    });
    register_method(&fs, "readJson", read_json, context)?;

    let st = Rc::clone(&state);
    let exists = stateful_fn(move |_this, args, ctx| {
        let path = arg_as_string(args, 0, ctx)?;
        match fs_exists_impl(&st, &path) {
            Ok(b) => Ok(JsValue::from(b)),
            Err(err) => stash_and_throw(&st, err),
        }
    });
    register_method(&fs, "exists", exists, context)?;

    let st = state;
    let list_dir = stateful_fn(move |_this, args, ctx| {
        let path = arg_as_string(args, 0, ctx)?;
        match fs_list_dir_impl(&st, &path) {
            Ok(names) => {
                let arr = JsArray::new(ctx);
                for (i, n) in names.into_iter().enumerate() {
                    arr.set(i as u32, js_string!(n.as_str()), true, ctx)?;
                }
                Ok(JsValue::from(arr))
            }
            Err(err) => stash_and_throw(&st, err),
        }
    });
    register_method(&fs, "listDir", list_dir, context)?;

    let _ = buildtime
        .set(js_string!("fs"), fs, true, context)
        .map_err(to_sandbox_err)?;
    Ok(())
}

fn install_crypto(buildtime: &JsObject, context: &mut Context) -> Result<(), SandboxError> {
    let crypto = JsObject::with_null_proto();

    let sha256 = NativeFunction::from_copy_closure(|_this, args, ctx| {
        let input = arg_as_string(args, 0, ctx)?;
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        Ok(JsValue::from(js_string!(hex::encode(hasher.finalize()))))
    });
    register_method(&crypto, "sha256", sha256, context)?;

    let sha512 = NativeFunction::from_copy_closure(|_this, args, ctx| {
        let input = arg_as_string(args, 0, ctx)?;
        use sha2::{Digest, Sha512};
        let mut hasher = Sha512::new();
        hasher.update(input.as_bytes());
        Ok(JsValue::from(js_string!(hex::encode(hasher.finalize()))))
    });
    register_method(&crypto, "sha512", sha512, context)?;

    let _ = buildtime
        .set(js_string!("crypto"), crypto, true, context)
        .map_err(to_sandbox_err)?;
    Ok(())
}

fn install_time(buildtime: &JsObject, context: &mut Context) -> Result<(), SandboxError> {
    let time = JsObject::with_null_proto();

    let now = NativeFunction::from_copy_closure(|_this, _args, _ctx| {
        Ok(JsValue::from(js_string!(iso_now())))
    });
    register_method(&time, "now", now, context)?;

    let iso = NativeFunction::from_copy_closure(|_this, _args, _ctx| {
        Ok(JsValue::from(js_string!(iso_now())))
    });
    register_method(&time, "iso", iso, context)?;

    let unix = NativeFunction::from_copy_closure(|_this, _args, _ctx| {
        Ok(JsValue::from(unix_seconds() as f64))
    });
    register_method(&time, "unix", unix, context)?;

    let _ = buildtime
        .set(js_string!("time"), time, true, context)
        .map_err(to_sandbox_err)?;
    Ok(())
}

fn install_env(
    buildtime: &JsObject,
    context: &mut Context,
    options: &SandboxOptions,
) -> Result<(), SandboxError> {
    let env = JsObject::with_null_proto();
    for var in &options.capabilities.env_allow {
        let value = std::env::var(var).ok();
        let js_val = match value {
            Some(v) => JsValue::from(js_string!(v.as_str())),
            None => JsValue::undefined(),
        };
        let _ = env
            .set(js_string!(var.as_str()), js_val, true, context)
            .map_err(to_sandbox_err)?;
    }
    let _ = buildtime
        .set(js_string!("env"), env, true, context)
        .map_err(to_sandbox_err)?;
    Ok(())
}

fn install_flags(buildtime: &JsObject, context: &mut Context) -> Result<(), SandboxError> {
    let flags = JsObject::with_null_proto();
    let has = NativeFunction::from_copy_closure(|_this, _args, _ctx| Ok(JsValue::from(false)));
    register_method(&flags, "has", has, context)?;
    let get = NativeFunction::from_copy_closure(|_this, _args, _ctx| Ok(JsValue::undefined()));
    register_method(&flags, "get", get, context)?;
    let _ = buildtime
        .set(js_string!("flags"), flags, true, context)
        .map_err(to_sandbox_err)?;
    Ok(())
}

fn install_location(
    buildtime: &JsObject,
    context: &mut Context,
    options: &SandboxOptions,
) -> Result<(), SandboxError> {
    let location = JsObject::with_null_proto();
    let file = options.source_file.to_string_lossy().into_owned();
    let _ = location
        .set(
            js_string!("file"),
            JsValue::from(js_string!(file.as_str())),
            true,
            context,
        )
        .map_err(to_sandbox_err)?;
    let _ = location
        .set(
            js_string!("line"),
            JsValue::from(options.source_line as f64),
            true,
            context,
        )
        .map_err(to_sandbox_err)?;
    let _ = location
        .set(
            js_string!("column"),
            JsValue::from(options.source_column as f64),
            true,
            context,
        )
        .map_err(to_sandbox_err)?;
    let _ = buildtime
        .set(js_string!("location"), location, true, context)
        .map_err(to_sandbox_err)?;
    Ok(())
}

fn register_method(
    obj: &JsObject,
    name: &str,
    func: NativeFunction,
    context: &mut Context,
) -> Result<(), SandboxError> {
    let realm = context.realm().clone();
    let _ = obj
        .set(js_string!(name), func.to_js_function(&realm), true, context)
        .map_err(to_sandbox_err)?;
    Ok(())
}

// ---------------------------------------------------------------------
// fs native implementations (capability-enforced, dep-tracking)
// ---------------------------------------------------------------------

fn fs_read_text_impl(
    state: &Rc<RefCell<SandboxState>>,
    path: &str,
) -> Result<String, SandboxError> {
    let resolved = resolve_and_check_read(state, path)?;
    let text = crate::host::buildtime::host_fs::read_text(&resolved)
        .map_err(|e| SandboxError::Backend(format!("fs.readText({}): {}", resolved.display(), e)))?;
    state.borrow_mut().dependencies.push(resolved);
    Ok(text)
}

fn fs_exists_impl(state: &Rc<RefCell<SandboxState>>, path: &str) -> Result<bool, SandboxError> {
    let resolved = resolve_and_check_read(state, path)?;
    let exists = crate::host::buildtime::host_fs::exists(&resolved)
        .map_err(|e| SandboxError::Backend(format!("fs.exists({}): {}", resolved.display(), e)))?;
    state.borrow_mut().dependencies.push(resolved);
    Ok(exists)
}

fn fs_list_dir_impl(
    state: &Rc<RefCell<SandboxState>>,
    path: &str,
) -> Result<Vec<String>, SandboxError> {
    let resolved = resolve_and_check_read(state, path)?;
    let names = crate::host::buildtime::host_fs::list_dir(&resolved)
        .map_err(|e| SandboxError::Backend(format!("fs.listDir({}): {}", resolved.display(), e)))?;
    state.borrow_mut().dependencies.push(resolved);
    Ok(names)
}

fn resolve_and_check_read(
    state: &Rc<RefCell<SandboxState>>,
    path: &str,
) -> Result<PathBuf, SandboxError> {
    let borrowed = state.borrow();
    let candidate = PathBuf::from(path);
    let resolved = if candidate.is_absolute() {
        candidate
    } else {
        borrowed
            .options
            .source_file
            .parent()
            .map(|p| p.join(&candidate))
            .unwrap_or(candidate)
    };
    let normalized = normalize_path(&resolved);
    borrowed
        .options
        .capabilities
        .check_read(&normalized)
        .map_err(|_| SandboxError::UnauthorizedRead {
            path: normalized.clone(),
        })?;
    Ok(normalized)
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for comp in path.components() {
        match comp {
            std::path::Component::ParentDir => {
                out.pop();
            }
            std::path::Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

// ---------------------------------------------------------------------
// Time helpers
// ---------------------------------------------------------------------

fn iso_now() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let (year, month, day, hour, minute, second) = unix_to_civil(duration.as_secs() as i64);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        year,
        month,
        day,
        hour,
        minute,
        second,
        duration.subsec_millis()
    )
}

fn unix_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn unix_to_civil(unix_secs: i64) -> (i32, u32, u32, u32, u32, u32) {
    let days = unix_secs.div_euclid(86400);
    let secs_of_day = unix_secs.rem_euclid(86400);
    let hour = (secs_of_day / 3600) as u32;
    let minute = ((secs_of_day % 3600) / 60) as u32;
    let second = (secs_of_day % 60) as u32;

    let z = days + 719_468;
    let era = if z >= 0 {
        z / 146_097
    } else {
        (z - 146_096) / 146_097
    };
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let year = if month <= 2 { y + 1 } else { y } as i32;
    (year, month, day, hour, minute, second)
}

// ---------------------------------------------------------------------
// Value conversion + error helpers
// ---------------------------------------------------------------------

const MAX_SERIALIZE_DEPTH: usize = 512;

fn js_to_sandbox(
    value: &JsValue,
    context: &mut Context,
    depth: usize,
) -> Result<SandboxValue, SandboxError> {
    if depth > MAX_SERIALIZE_DEPTH {
        return Err(SandboxError::UnserializableResult {
            kind: "deeply nested or circular value".to_string(),
        });
    }
    if value.is_null() {
        return Ok(SandboxValue::Null);
    }
    if value.is_undefined() {
        return Ok(SandboxValue::Undefined);
    }
    if let Some(b) = value.as_boolean() {
        return Ok(SandboxValue::Bool(b));
    }
    if let Some(n) = value.as_number() {
        return Ok(SandboxValue::Number(n));
    }
    if let Some(s) = value.as_string() {
        return Ok(SandboxValue::String(s.to_std_string_lossy()));
    }
    if let Some(obj) = value.as_object() {
        return object_to_sandbox(&obj, context, depth);
    }
    Err(SandboxError::UnserializableResult {
        kind: "symbol or host value".to_string(),
    })
}

fn object_to_sandbox(
    obj: &JsObject,
    context: &mut Context,
    depth: usize,
) -> Result<SandboxValue, SandboxError> {
    if obj.is_array() {
        let arr = JsArray::from_object(obj.clone())
            .map_err(|e| SandboxError::Backend(format!("array cast: {e}")))?;
        let len =
            arr.length(context)
                .map_err(|e| SandboxError::Backend(format!("array len: {e}")))? as u32;
        let mut out = Vec::with_capacity(len as usize);
        for i in 0..len {
            let item = arr
                .get(i, context)
                .map_err(|e| SandboxError::Backend(format!("array get: {e}")))?;
            out.push(js_to_sandbox(&item, context, depth + 1)?);
        }
        return Ok(SandboxValue::Array(out));
    }
    if obj.is_callable() {
        return Err(SandboxError::UnserializableResult {
            kind: "function".to_string(),
        });
    }
    let tag = object_kind_name(obj, context);
    if !matches!(tag, "Object" | "null-proto") {
        return Err(SandboxError::UnserializableResult {
            kind: tag.to_string(),
        });
    }
    let keys = obj
        .own_property_keys(context)
        .map_err(|e| SandboxError::Backend(format!("own keys: {e}")))?;
    let mut out = std::collections::BTreeMap::new();
    for key in keys {
        let key_str = match &key {
            boa_engine::property::PropertyKey::String(s) => s.to_std_string_lossy(),
            boa_engine::property::PropertyKey::Index(idx) => idx.get().to_string(),
            boa_engine::property::PropertyKey::Symbol(_) => continue,
        };
        let val = obj
            .get(key.clone(), context)
            .map_err(|e| SandboxError::Backend(format!("prop get: {e}")))?;
        out.insert(key_str, js_to_sandbox(&val, context, depth + 1)?);
    }
    Ok(SandboxValue::Object(out))
}

fn object_kind_name(obj: &JsObject, context: &mut Context) -> &'static str {
    if obj.is_array() {
        return "array";
    }
    if obj.is_callable() {
        return "function";
    }
    if let Ok(ctor) = obj.get(js_string!("constructor"), context) {
        if let Some(ctor_obj) = ctor.as_object() {
            if let Ok(name) = ctor_obj.get(js_string!("name"), context) {
                if let Some(s) = name.as_string() {
                    let s = s.to_std_string_lossy();
                    return match s.as_str() {
                        "Object" => "Object",
                        "Date" => "Date",
                        "RegExp" => "RegExp",
                        "Map" => "Map",
                        "Set" => "Set",
                        "Promise" => "Promise",
                        "Error" => "Error",
                        _ => "class instance",
                    };
                }
            }
        }
    }
    "null-proto"
}

fn extract_thrown(err_value: JsValue, context: &mut Context) -> SandboxError {
    let Some(obj) = err_value.as_object() else {
        return SandboxError::Threw {
            message: "(non-object thrown)".to_string(),
            stack: String::new(),
        };
    };
    let message = obj
        .get(js_string!("message"), context)
        .ok()
        .and_then(|v| v.as_string().map(|s| s.to_std_string_lossy()))
        .unwrap_or_else(|| "unknown".to_string());
    let stack = obj
        .get(js_string!("stack"), context)
        .ok()
        .and_then(|v| v.as_string().map(|s| s.to_std_string_lossy()))
        .unwrap_or_default();
    SandboxError::Threw { message, stack }
}

fn arg_as_string(args: &[JsValue], idx: usize, _context: &mut Context) -> JsResult<String> {
    let Some(arg) = args.get(idx) else {
        return Err(JsError::from_native(
            JsNativeError::typ().with_message("expected string argument"),
        ));
    };
    let Some(s) = arg.as_string() else {
        return Err(JsError::from_native(
            JsNativeError::typ().with_message("argument is not a string"),
        ));
    };
    Ok(s.to_std_string_lossy())
}

fn stash_and_throw(state: &Rc<RefCell<SandboxState>>, err: SandboxError) -> JsResult<JsValue> {
    let msg = err.to_string();
    state.borrow_mut().side_channel_error = Some(err);
    Err(JsError::from_native(
        JsNativeError::error().with_message(msg),
    ))
}

fn to_sandbox_err(err: JsError) -> SandboxError {
    SandboxError::Backend(err.to_string())
}
