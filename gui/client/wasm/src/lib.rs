mod enums;
mod utils;

use crate::enums::Status;

use serde_wasm_bindgen::{from_value, to_value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

use svg_collision::io::svg_parser::run_cde_wasm;

#[wasm_bindgen]
pub fn run() {
    let document = window().unwrap().document().unwrap();
    let light = document
        .get_element_by_id("traffic-light")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    let light_rc = Rc::new(RefCell::new(light));

    let moving = Rc::new(RefCell::new(false));
    let moving_clone = moving.clone();
    let light_clone1 = light_rc.clone();

    let closure = Closure::<dyn FnMut(MouseEvent)>::new(move |_| {
        *moving_clone.borrow_mut() = true;
        light_clone1
            .borrow_mut()
            .set_attribute(
                "style",
                "background-color: red;
                width: 100px;
                height: 100px;
                backgroundColor: green;
                borderRadius: 50%;
                margin: 50px auto;",
            )
            .unwrap();
    });

    document
        .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

    let moving_clone = moving.clone();
    let light_clone2 = light_rc.clone();

    let stop_checker = Closure::<dyn FnMut()>::new(move || {
        if *moving_clone.borrow() {
            *moving_clone.borrow_mut() = false;
        } else {
            light_clone2
                .borrow_mut()
                .set_attribute(
                    "style",
                    "background-color: red;
                    width: 100px;
                    height: 100px;
                    backgroundColor: red;
                    borderRadius: 50%;
                    margin: 50px auto;",
                )
                .unwrap();
        }
    });

    window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            stop_checker.as_ref().unchecked_ref(),
            1000,
        )
        .unwrap();
    stop_checker.forget();
}

#[wasm_bindgen]
pub fn toggle_box() {
    console::log_1(&"toggle_box".into());

    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("should have a document on window");

    let test_box_element = document
        .get_element_by_id("testBox")
        .expect("should have #testBox element on the page");

    let test_box: HtmlElement = match test_box_element.dyn_into::<HtmlElement>() {
        Ok(element) => element,
        Err(_) => {
            console::error_1(&"Could not cast testBox to HtmlElement".into());
            return;
        }
    };

    let class_name = test_box
        .get_attribute("class")
        .unwrap_or_else(|| String::from(""));

    if class_name == "green" {
        test_box.set_attribute("class", "red").unwrap();
    } else {
        test_box.set_attribute("class", "green").unwrap();
    }
}

#[wasm_bindgen]
pub fn svg_collision(js_value: JsValue) -> Result<JsValue, JsValue> {
    let moved_element: HashMap<String, Option<String>> = match from_value(js_value) {
        Ok(val) => val,
        Err(e) => return Err(JsValue::from_str(&format!("Error deserializing: {}", e))),
    };
    console::log_1(&"svg_collision_testtttt".into());

    match to_value(&moved_element) {
        Ok(js_val) => {
            console::log_1(&js_val); // log the value to the console.
            return Ok(js_val); // Return the serialized JsValue
        }
        Err(e) => return Err(JsValue::from_str(&format!("Error serializing: {}", e))),
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_name = postMessage)]
    fn post_message_to_js(s: &str);

    #[wasm_bindgen(js_name = postMessage)]
    fn post_message_object_to_js(val: &JsValue);
}

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen]
pub fn make_jaguars_instance(svg_input: JsValue) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // console::log_1(&"run cde".into());

    let progress_msg = "PROGRESS: Started processing";
    let progress_obj = js_sys::Object::new();
    js_sys::Reflect::set(
        &progress_obj,
        &JsValue::from_str("type"),
        &JsValue::from_str(&Status::Processing.to_string()),
    )
    .unwrap();

    js_sys::Reflect::set(
        &progress_obj,
        &JsValue::from_str("message"),
        &JsValue::from_str(&progress_msg),
    )
    .unwrap();

    post_message_object_to_js(&progress_obj);
    // post_message_to_js(&"run cde");

    let svg_str: String = match from_value(svg_input) {
        Ok(val) => val,
        Err(e) => {
            let err_msg = format!("Error deserializing SVG input: {}", e);
            let error_obj = js_sys::Object::new();
            js_sys::Reflect::set(
                &error_obj,
                &JsValue::from_str("type"),
                &JsValue::from_str(&Status::Error.to_string()),
            )
            .unwrap();
            js_sys::Reflect::set(
                &error_obj,
                &JsValue::from_str("message"),
                &JsValue::from_str(&err_msg),
            )
            .unwrap();
            post_message_object_to_js(&error_obj);

            return Ok(());
        }
    };
    // console::log_1(&svg_str.clone().into());
    // post_message_to_js(&svg_str);

    let svg_result = run_cde_wasm(&svg_str);

    match svg_result {
        Ok(svg_result) => {
            // console::log_1(&svg_result.clone().into());

            let final_obj = js_sys::Object::new();
            js_sys::Reflect::set(
                &final_obj,
                &JsValue::from_str("type"),
                &JsValue::from_str(&Status::Finished.to_string()),
            )
            .unwrap();
            js_sys::Reflect::set(
                &final_obj,
                &JsValue::from_str("result"),
                &JsValue::from_str(&svg_result),
            )
            .unwrap();
            post_message_object_to_js(&final_obj);
        }
        Err(e) => {
            let err_msg = format!("Error during WASM computation: {}", e);
            let error_obj = js_sys::Object::new();
            js_sys::Reflect::set(
                &error_obj,
                &JsValue::from_str("type"),
                &JsValue::from_str(&Status::Error.to_string()),
            )
            .unwrap();
            js_sys::Reflect::set(
                &error_obj,
                &JsValue::from_str("message"),
                &JsValue::from_str(&err_msg),
            )
            .unwrap();
            post_message_object_to_js(&error_obj);

            return Ok(());
        }
    };

    Ok(())
}
