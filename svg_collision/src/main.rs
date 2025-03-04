use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use slotmap::SlotMap;

use log::{error, warn};

// use svgtypes::{PathParser, PathSegment};

use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::Event;

use jagua_rs::collision_detection::cd_engine::CDEngine;
// use jagua_rs::collision_detection::hazard::HazardEntity;
// use jagua_rs::collision_detection::quadtree::qt_hazard::QTHazPresence;
// use jagua_rs::collision_detection::quadtree::qt_node::QTNode;
use jagua_rs::entities::bin::Bin;
use jagua_rs::entities::placed_item::PlacedItem;
// use jagua_rs::geometry::primitives::edge::Edge;
use jagua_rs::entities::instances::instance::Instance;
use jagua_rs::entities::layout::Layout;
use jagua_rs::geometry::primitives::point::Point;
use jagua_rs::geometry::primitives::simple_polygon::SimplePolygon;

use crate::config::Config;
use svg_collision::io::svg_parser::svg_to_layout_from_file;

fn main() {
    let config = match args.config_file {
        None => {
            warn!("No config file provided, use --config-file to provide a custom config");
            warn!(
                "Falling back default config:\n{}",
                serde_json::to_string(&LBFConfig::default()).unwrap()
            );
            Config::default()
        }
        Some(config_file) => {
            let file = File::open(config_file).unwrap_or_else(|err| {
                panic!("Config file could not be opened: {}", err);
            });
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|err| {
                error!("Config file could not be parsed: {}", err);
                error!("Omit the --config-file argument to use the default config");
                panic!();
            })
        }
    };

    let instance: Instance;

    match svg_to_layout_from_file("./assets/1.svg", 1) {
        Ok(_layout) => println!("Successfully loaded layout!"),
        Err(err) => eprintln!("Error: {}", err),
    }
}
