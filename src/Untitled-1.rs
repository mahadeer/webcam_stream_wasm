extern crate wasm_bindgen;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, ImageCapture, MediaStream,
    MediaStreamConstraints, VideoStreamTrack,
};

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
    let canvas_elem = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    container.append_child(&canvas_elem)?;

    let window_ref = window.clone();
    let context = canvas_elem
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let grab_frame_clsr =
        Closure::wrap(Box::new(move |img_capture: JsValue| {
            let image_capture = img_capture.dyn_into::<ImageCapture>().unwrap();
            image_capture.take_photo();
        }) as Box<dyn FnMut(JsValue)>);
    let resolve_clsr = Closure::wrap(Box::new(move |media_stream: JsValue| {
        let media_streaming_parsed: MediaStream =
            MediaStream::new_with_tracks(&media_stream).unwrap();
        let track_arr = media_streaming_parsed.get_video_tracks();
        let track = track_arr.shift();
        let video_stream = track.dyn_into::<VideoStreamTrack>().unwrap();
        let image_capture = ImageCapture::new(&video_stream).unwrap();
        window_ref.set_interval_with_callback_and_timeout_and_arguments_1(
            grab_frame_clsr.as_ref().unchecked_ref(),
            200,
            &JsValue::from(image_capture),
        ).unwrap();
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
