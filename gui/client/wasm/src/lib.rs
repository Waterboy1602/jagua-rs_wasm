mod enums;
mod utils;

use crate::enums::Status;

use log::{log, Level, LevelFilter};
use serde_wasm_bindgen::{from_value, to_value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
use web_time::Instant;

use anyhow::{Context, Result};
use jagua_rs::io::import::Importer;
use jagua_rs::io::svg::s_layout_to_svg;
use jagua_rs::probs::spp;
use jagua_rs::probs::spp::io::ext_repr::ExtSPInstance;
use lbf::config::LBFConfig;
use lbf::opt::lbf_spp::LBFOptimizerSP;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use svg_collision::io::svg_parser::run_cde_wasm;

lazy_static::lazy_static! {
    static ref EPOCH: Instant = Instant::now();
}

#[wasm_bindgen]
pub fn init_logger(level_filter_u8: u8) -> Result<(), JsValue> {
    let level_filter = match level_filter_u8 {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 => LevelFilter::Trace,
        _ => {
            let error_msg = format!("Invalid LevelFilter value: {}", level_filter_u8);
            console::error_1(&error_msg.into());
            return Ok(());
        }
    };

    fern::Dispatch::new()
        .level(level_filter)
        .chain(fern::Output::call(|record| {
            let duration = EPOCH.elapsed();
            let sec = duration.as_secs() % 60;
            let min = (duration.as_secs() / 60) % 60;
            let hours = (duration.as_secs() / 60) / 60;

            let prefix = format!(
                "[{}] [{:0>2}:{:0>2}:{:0>2}]",
                record.level(),
                hours,
                min,
                sec
            );

            let full_log_message = format!("{prefix:<27}{}", record.args());

            let log_obj = js_sys::Object::new();
            js_sys::Reflect::set(
                &log_obj,
                &JsValue::from_str("type"),
                &JsValue::from_str(&Status::Processing.to_string()),
            )
            .unwrap();
            js_sys::Reflect::set(
                &log_obj,
                &JsValue::from_str("level"),
                &JsValue::from_str(&record.level().to_string()),
            )
            .unwrap();
            js_sys::Reflect::set(
                &log_obj,
                &JsValue::from_str("message"),
                &JsValue::from_str(&full_log_message.to_string()),
            )
            .unwrap();

            post_message_object_to_js(&log_obj.into());
        }))
        .apply()
        .map_err(|e| JsValue::from_str(&format!("Failed to apply logger: {}", e)))?;

    log!(Level::Info, "Epoch: {}", EPOCH.elapsed().as_secs_f64());
    Ok(())
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
pub fn svg_collision_test(svg_input: JsValue) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    log!(Level::Info, "Started SVG collision test");

    let svg_str: String = match from_value(svg_input) {
        Ok(val) => val,
        Err(e) => {
            log!(Level::Error, "Error deserializing SVG input: {}", e);
            return Ok(());
        }
    };
    // console::log_1(&svg_str.clone().into());

    let svg_result = run_cde_wasm(&svg_str);

    match svg_result {
        Ok(svg_result) => {
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
            log!(Level::Error, "Error during WASM computation: {}", e);

            return Ok(());
        }
    };

    Ok(())
}

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen]
pub fn run_lbf(json_input: JsValue) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    log!(Level::Info, "Started LBF optimization");

    let json_str: String = match from_value(json_input) {
        Ok(val) => val,
        Err(e) => {
            log!(Level::Error, "Error deserializing JSON input: {}", e);

            return Ok(());
        }
    };

    let config = LBFConfig::default();

    let ext_sp_instance: ExtSPInstance = serde_json::from_str(&json_str)
        .context("not a valid strip packing instance (ExtSPInstance)")
        .unwrap();

    let importer = Importer::new(
        config.cde_config,
        config.poly_simpl_tolerance,
        config.min_item_separation,
    );
    let rng = match config.prng_seed {
        Some(seed) => SmallRng::seed_from_u64(seed),
        None => SmallRng::from_os_rng(),
    };
    let instance = spp::io::import(&importer, &ext_sp_instance).unwrap();
    let sol = LBFOptimizerSP::new(instance.clone(), config, rng).solve();

    let svg_result =
        s_layout_to_svg(&sol.layout_snapshot, &instance, config.svg_draw_options, "").to_string();

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

    Ok(())
}

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen]
pub fn run_sparrow(json_input: JsValue) -> Result<(), JsValue> {
    log!(Level::Info, "Sparrow not yet implemented");

    Ok(())
}
