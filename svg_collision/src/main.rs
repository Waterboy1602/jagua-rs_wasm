use jagua_rs::entities::Layout;
use jagua_rs::io::svg::layout_to_svg;

use std::env;
use std::path::Path;
use svg_collision::config::Config;
use svg_collision::config::DRAW_OPTIONS_WITHOUT_HIGHLIGHT;
use svg_collision::config::OUTPUT_DIR;
use svg_collision::io::svg_parser::SvgParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let svg_path = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("./assets/swim.svg");

    let config = Config::default();

    let mut svg_parser = SvgParser::new(config, None, None);

    let instance = SvgParser::svg_file_to_instance(&mut svg_parser, svg_path).unwrap();

    let mut layout = Layout::new(instance.bins[0].container.clone());

    for (item, _item_demand) in instance.items.iter() {
        for transformation in svg_parser.item_transformations[&item.id].iter() {
            layout.place_item(item, transformation.clone());
        }
    }

    let svg = layout_to_svg(
        &layout,
        &instance,
        DRAW_OPTIONS_WITHOUT_HIGHLIGHT,
        "SVG TEST",
    );

    svg::save(
        Path::new(&*format!(
            "{}/svg_test_without_collision_highlight.svg",
            OUTPUT_DIR
        )),
        &svg,
    )
    .expect("failed to save SVG document");
}
