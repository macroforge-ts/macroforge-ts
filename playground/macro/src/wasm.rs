use serde_wasm_bindgen::Serializer;
use ts_macro_host::derived;
use wasm_bindgen::prelude::*;

fn console_log(message: &str) {
    #[allow(unused_unsafe)]
    unsafe {
        web_sys::console::log_1(&JsValue::from_str(message));
    }
}

#[wasm_bindgen(js_name = logDecorator)]
pub fn log_decorator(name: &str) {
    console_log(&format!(
        "[ts-macros] Decorator '{name}' executed in fallback mode"
    ));
}

#[wasm_bindgen(js_name = macroMetadata)]
pub fn macro_metadata() -> JsValue {
    let metadata = derived::decorator_metadata();
    metadata
        .serialize(&Serializer::json_compatible())
        .expect("failed to serialize decorator metadata")
}
