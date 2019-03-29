extern crate wasm_bindgen;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlElement, HtmlMediaElement, MediaStream, MediaStreamConstraints};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace =console)]
    fn log(message: &str);
    #[wasm_bindgen(js_namespace =console, js_name="log")]
    fn log_obj(msg_obj: JsValue);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoOptions {
    width: i32,
    height: i32,
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let container = document
        .get_element_by_id("container")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    let video_elem = document
        .create_element("video")
        .unwrap()
        .dyn_into::<HtmlMediaElement>()
        .unwrap();
    video_elem.set_attribute("id", "video-src")?;
    container.append_child(&video_elem)?;

    let video_elem_1 = video_elem.clone();
    let metadata_loaded_clsr = Closure::wrap(Box::new(move || {
        video_elem_1.play().expect("Play back error");
    }) as Box<FnMut()>);
    let resolve_clsr = Closure::wrap(Box::new(move |media_stream: JsValue| {
        let media_streaming_parsed: MediaStream =
            MediaStream::new_with_tracks(&media_stream).unwrap();
        video_elem.set_src_object(Some(&media_streaming_parsed));
        video_elem.set_onloadedmetadata(Some(&metadata_loaded_clsr.as_ref().unchecked_ref()));
    }) as Box<dyn FnMut(JsValue)>);
    let reject_clsr = Closure::wrap(Box::new(move |err: JsValue| {
        log_obj(err);
    }) as Box<dyn FnMut(JsValue)>);
    let finally_clsr = Closure::wrap(Box::new(move || {
        log("Finally called");
    }) as Box<dyn FnMut()>);

    let media_devices = window.navigator().media_devices().unwrap();
    let media_options = VideoOptions {
        width: 1280,
        height: 720,
    };
    let mut media_constraints = MediaStreamConstraints::new();
    let settings = media_constraints.video(&JsValue::from_serde(&media_options).unwrap());
    let request = media_devices
        .get_user_media_with_constraints(settings)
        .unwrap();
    request.then2(&resolve_clsr, &reject_clsr);
    request.finally(&finally_clsr);
    resolve_clsr.forget();
    reject_clsr.forget();
    finally_clsr.forget();
    Ok(())
}
