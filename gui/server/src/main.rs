#[macro_use]
extern crate rocket;

use std::sync::Mutex;
use std::path::PathBuf;

use rocket::form::Form;
use rocket::fs::{relative, FileServer, NamedFile};
use rocket::http::{Method, Status};
use rocket::response::{Flash, Redirect};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

use lbf::lbf_run::solve_json;

type SvgFiles = Mutex<Vec<String>>; // Define a type alias for shared state.

#[derive(Deserialize, Serialize)]
pub struct InputData {
    pub config: String,
    pub input: String,
}

#[post("/json", format = "json", data = "<input_data>")]
async fn json(input_data: Json<InputData>, svg_state: &State<SvgFiles>) -> Result<Json<Vec<Vec<String>>>, String> {
    let json = input_data.into_inner();

    if json.input.is_empty() {
        return Err("JSON cannot be empty".to_string());
    }

    let mut svg_files = solve_json(json.config, json.input.clone(), "static/solutions/".to_string());
    if svg_files.is_empty() {
        return Err("No solution found.".to_string());
    } else {
        println!("SVG files: {:?}", svg_files.clone());
        return Ok(Json(svg_files.clone()));
    }
}

#[get("/file?<path>")]
async fn file(path: String) -> Result<NamedFile, Status> {
    let file_path = PathBuf::from(path);

    // Ensure the file exists and is accessible
    if !file_path.exists() || !file_path.is_file() {
        return Err(Status::NotFound);
    }

    // Serve the file
    NamedFile::open(file_path)
        .await
        .map_err(|_| Status::InternalServerError)
}

// #[get("/file")]
// fn sol(svg_state: &State<SvgFiles>) {
//     let svg_files = svg_state.lock().expect("State lock poisoned");
//     let path = svg_files.get(0).unwrap();
//     let adjusted_path_svg = path.replace("/static", "");
//     let adjusted_path_json = path
//         .replace(".svg", ".json")
//         .replace("/static", "")
//         .replace("_0", "");
//     // let adjusted_path_svg = "/solutions/sol_web_0.svg";
//     println!("{}", adjusted_path_svg);
// }

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


    // TODO: fix the path to the static files
    // Prevent access to all files in ./static
    rocket::build()
        .manage(SvgFiles::default()) // Initialize shared state.
        .mount("/", routes![json, file])
        .mount("/", FileServer::from(relative!("./")))
        .attach(cors)
}
