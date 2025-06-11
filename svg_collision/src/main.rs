use std::env;
use std::fs::File;
use std::io::Read;
use svg_collision::io::svg_parser::run_cde;

fn main() {
    let args: Vec<String> = env::args().collect();
    let svg_path = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("./assets/swim.svg");

    let mut file = File::open(svg_path).expect("Failed to open SVG file");
    let mut svg_str = String::new();
    file.read_to_string(&mut svg_str)
        .expect("Failed to read SVG file");

    run_cde(&svg_str);
}
