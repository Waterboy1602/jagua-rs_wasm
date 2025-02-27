use std::fs::File;
use std::io::BufReader;

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
use jagua_rs::entities::layout::Layout;
use jagua_rs::geometry::primitives::point::Point;
use jagua_rs::geometry::primitives::simple_polygon::SimplePolygon;

fn main() {
    match svg_to_layout_from_file("./assets/1.svg", 1) {
        Ok(_layout) => println!("Successfully loaded layout!"),
        Err(err) => eprintln!("Error: {}", err),
    }
}

/// Parses an SVG file and converts it into a `Layout` object.
pub fn svg_to_layout_from_file(path: &str, id: usize) -> Result<Layout, String> {
    let file = File::open(path).map_err(|e| format!("Failed to open SVG file: {}", e))?;
    let reader = BufReader::new(file);

    let mut bin_shape = None;
    // TODO Slotmap - undeclared type???
    let mut placed_items = SlotMap::with_key();

    for event in svg::parser::Parser::new(reader) {
        match event {
            Event::Tag("path", _, attributes) => {
                if let Some(data) = attributes.get("d") {
                    if let Ok(polygon) = parse_path_data(data) {
                        if bin_shape.is_none() {
                            bin_shape = Some(polygon); // First shape is the bin
                        } else {
                            // TODO from_polygon doesnt exist??
                            let placed_item = PlacedItem::from_polygon(polygon);
                            placed_items.insert(placed_item);
                        }
                    }
                }
            }
            Event::Tag("polygon", _, attributes) => {
                if let Some(points) = attributes.get("points") {
                    if let Ok(polygon) = parse_polygon_points(points) {
                        if bin_shape.is_none() {
                            bin_shape = Some(polygon);
                        } else {
                            // TODO from_polygon doesnt exist??
                            let placed_item = PlacedItem::from_polygon(polygon);
                            placed_items.insert(placed_item);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let bin_shape = bin_shape.ok_or("No valid bin shape found in SVG")?;
    // TODO fix construct of bin: more arguments needed
    let bin = Bin::new(bin_shape);
    // TODO fix construct of CDengine: more arguments needed
    let cde = CDEngine::new(); // Initialize collision detection engine

    Ok(Layout {
        id,
        bin,
        placed_items,
        cde,
    })
}

fn parse_path_data(data: &str) -> Result<SimplePolygon, String> {
    // Convert SVG path data to a SimplePolygon
    let mut points = Vec::new();

    let commands = svgtypes::PathParser::from(data);
    for command in commands {
        match command {
            Ok(svgtypes::PathSegment::MoveTo { x, y, .. }) => {
                points.push(Point(x as f32, y as f32));
            }
            Ok(svgtypes::PathSegment::LineTo { x, y, .. }) => {
                points.push(Point(x as f32, y as f32));
            }
            _ => {}
        }
    }

    if points.len() >= 3 {
        Ok(SimplePolygon::new(points))
    } else {
        Err("Invalid path data: Not enough points to form a polygon".into())
    }
}

fn parse_polygon_points(points_str: &str) -> Result<SimplePolygon, String> {
    let points: Vec<Point> = points_str
        .split_whitespace()
        .filter_map(|pair| {
            let mut coords = pair.split(',');
            let x = coords.next()?.parse().ok()?;
            let y = coords.next()?.parse().ok()?;
            Some(Point(x as f32, y as f32))
        })
        .collect();

    if points.len() >= 3 {
        Ok(SimplePolygon::new(points))
    } else {
        Err("Invalid polygon data: Not enough points".into())
    }
}

pub fn read_svg() -> String {
    let path = "./assets/1.svg";
    let mut content = String::new();
    for event in svg::open(path, &mut content).unwrap() {
        match event {
            Event::Tag(Path, _, attributes) => {
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();
                for command in data.iter() {
                    match command {
                        &Command::Move(..) => { /* … */ }
                        &Command::Line(..) => { /* … */ }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    content
}
