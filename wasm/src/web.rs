use js_sys::ArrayBuffer;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, Request, RequestInit, RequestMode, Response, WorkerGlobalScope};

pub async fn load_image(url: &str) -> Result<image::RgbaImage, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    println!("{}", url);
    let request = Request::new_with_str_and_init(url, &opts).unwrap();

    request.headers().set("Accept", "*/*")?;

    let global_scope = js_sys::global();
    console::log_1(&global_scope.to_string());
    let worker_scope = global_scope
        .dyn_into::<WorkerGlobalScope>()
        .expect("Expected global scope to be WorkerGlobalScope");

    let resp_value = JsFuture::from(worker_scope.fetch_with_request(&request))
        .await
        .unwrap();

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let bytes_value = JsFuture::from(resp.array_buffer()?).await?;
    let bytes: ArrayBuffer = bytes_value.dyn_into().unwrap();

    let array = Uint8Array::new(&bytes);
    let bytes: Vec<u8> = array.to_vec();

    match image::load_from_memory(&bytes) {
        Ok(dyn_image) => Ok(dyn_image.into_rgba8()),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

pub fn load_image_from_buffer(buffer: &ArrayBuffer) -> Result<image::RgbaImage, JsValue> {
    let array = Uint8Array::new(buffer);
    let bytes: Vec<u8> = array.to_vec();

    match image::load_from_memory(&bytes) {
        Ok(dyn_image) => Ok(dyn_image.into_rgba8()),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}
