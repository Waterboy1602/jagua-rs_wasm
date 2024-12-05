#[macro_use]
extern crate rocket;

use std::sync::Mutex;

use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::http::Method;
use rocket::response::{Flash, Redirect};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

use lbf::lbf_run::solve_json;

type SvgFiles = Mutex<Vec<String>>; // Define a type alias for shared state.

#[derive(Deserialize, Serialize)]
pub struct InputData {
    pub json_str: String,
}

#[post("/json", format = "json", data = "<input_data>")]
async fn json(input_data: Json<InputData>, svg_state: &State<SvgFiles>) -> Result<String, String> {
    let json = input_data.into_inner();
    if json.json_str.is_empty() {
        return Err("JSON cannot be empty".to_string());
    }

    let svg_files = solve_json(json.json_str.clone(), "./static/solutions/".to_string());
    if svg_files.is_empty() {
        return Err("No solution found.".to_string());
    } else {
        let mut state = svg_state.lock().expect("State lock poisoned");
        *state = svg_files;
        println!("SVG files: {:?}", state);
        return Ok(state[0].clone());
    }
}

#[get("/file")]
fn sol(svg_state: &State<SvgFiles>) {
    let svg_files = svg_state.lock().expect("State lock poisoned");
    let path = svg_files.get(0).unwrap();
    let adjusted_path_svg = path.replace("/static", "");
    let adjusted_path_json = path
        .replace(".svg", ".json")
        .replace("/static", "")
        .replace("_0", "");
    // let adjusted_path_svg = "/solutions/sol_web_0.svg";
    println!("{}", adjusted_path_svg);
}

#[launch]
fn rocket() -> _ {
    // Configure CORS options
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&["http://localhost:5173"]),
        allowed_methods: vec![Method::Get, Method::Post, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS configuration failed");

    rocket::build()
        .manage(SvgFiles::default()) // Initialize shared state.
        .mount("/", routes![json, sol])
        .mount("/", FileServer::from(relative!("./static")))
        .attach(cors)
}
