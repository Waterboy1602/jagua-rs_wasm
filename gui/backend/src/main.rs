#[macro_use] extern crate rocket;

use std::sync::Mutex;

use rocket::State;
use rocket::form::Form;
use rocket::fs::{FileServer, relative};
use rocket::response::{Flash, Redirect};
use rocket::serde::Serialize;

use rocket_dyn_templates::{Template, context};

use lbf::lbf_run::solve_json;

type SvgFiles = Mutex<Vec<String>>; // Define a type alias for shared state.


#[derive(Debug, FromForm)]
pub struct Json {
    pub json_str: String,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {field: "value"})
}

#[post("/json", data = "<form_data>")]
async fn json(form_data: Form<Json>, svg_state: &State<SvgFiles>) -> Flash<Redirect> {
    let json = form_data.into_inner();
    if json.json_str.is_empty() {
        return Flash::error(Redirect::to("/"), "JSON cannot be empty.")
    }

    let svg_files = solve_json(json.json_str.clone(), "./static/assets/solutions/".to_string());
    if svg_files.is_empty() {
        return Flash::error(Redirect::to("/"), "No solution found.")
    } else {
        let mut state = svg_state.lock().expect("State lock poisoned");
        *state = svg_files;
        println!("SVG files: {:?}", state);
        return Flash::success(Redirect::to("/solution"), "Solution found.")
    }
}

#[get("/solution")]
fn sol(svg_state: &State<SvgFiles>) -> Template {
    let svg_files = svg_state.lock().expect("State lock poisoned");
    let path = svg_files.get(0).unwrap();
    let adjusted_path_svg  = path.replace("/static", "");
    let adjusted_path_json = path.replace(".svg", ".json")
                                    .replace("/static", "")
                                    .replace("_0", "");
    // let adjusted_path_svg = "/assets/solutions/sol_web_0.svg";
    println!("{}", adjusted_path_svg);

    Template::render("solution", context! {path_svg: adjusted_path_svg, path_json: adjusted_path_json})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(SvgFiles::default()) // Initialize shared state.
        .mount("/", routes![index, json, sol])
        .mount("/", FileServer::from(relative!("./static")))
        .attach(Template::fairing())
}