mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, MouseEvent};

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
                .set_attribute("style", "background-color: green;")
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
