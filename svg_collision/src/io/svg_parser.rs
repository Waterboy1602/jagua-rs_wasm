use std::fs::File;
use std::io::Read;

use slotmap::{DefaultKey, SlotMap};
use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::{Event, Parser};

// use jagua_rs::collision_detection::cd_engine::CDEngine;
// use jagua_rs::collision_detection::hazard::HazardEntity;
// use jagua_rs::collision_detection::quadtree::qt_hazard::QTHazPresence;
// use jagua_rs::collision_detection::quadtree::qt_node::QTNode;
use jagua_rs::entities::bin::Bin;
use jagua_rs::entities::placed_item::PlacedItem;
// use jagua_rs::geometry::primitives::edge::Edge;
use jagua_rs::entities::instances::instance::Instance;
// use jagua_rs::entities::layout::Layout;
use crate::config::Config;
use jagua_rs::geometry::primitives::point::Point;
use jagua_rs::geometry::primitives::simple_polygon::SimplePolygon;
use jagua_rs::geometry::transformation::Transformation;

pub struct SvgParser {
    config: Config,
}

impl SvgParser {
    pub fn new(config: Config) -> SvgParser {
        SvgParser { config }
    }

    // Parses an SVG file and converts it into a `Layout` object.
    pub fn svg_to_layout_from_file(path: &str, id: usize) -> Result<(), String> {
        let mut file = File::open(path).map_err(|e| format!("Failed to open SVG file: {}", e))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| format!("Failed to read SVG file: {}", e))?;

        let mut bin_shape: Option<SimplePolygon> = None; // You might want to define a type for bin_shape
        let mut placed_items: SlotMap<DefaultKey, Vec<(f64, f64)>> = SlotMap::with_key(); // Assuming polygon is Vec<(f64, f64)>

        let mut inside_defs = false;
        let mut inside_group = false;

        for event in Parser::new(&content) {
            match event {
                Event::Tag("defs", _, attributes) => {
                    inside_defs = !inside_group;
                    if let Some(id) = attributes.get("id") {
                        println!("Found <defs> with id: {:?}", id.to_string());
                    }
                }
                Event::Tag("g", _, attributes) if inside_defs => {
                    inside_group = !inside_group;
                    if let Some(id) = attributes.get("id") {
                        println!("Found <g> inside <defs> with id: {:?}", id.to_string());
                    }
                }
                Event::Tag("path", _, attributes) if inside_defs && inside_group => {
                    if let Some(d) = attributes.get("d") {
                        println!(
                            "Found <path> inside <g> inside <defs> with d: {:?}",
                            d.to_string()
                        );
                        if let Ok(polygon) = Self::parse_path_data(d) {
                            println!("{:?}", polygon);
                        }
                    }
                }
                Event::Tag("polygon", _, attributes) => {
                    if let Some(points) = attributes.get("points") {
                        println!("Found <polygon> with points: {:?}", points.to_string());
                    }
                }
                _ => {}
            }
        }

        Ok(()) // Return Ok(()) if successful
    }

    fn parse_path_data(data: &str) -> Result<Vec<(f64, f64)>, String> {
        let mut points = Vec::new();
        let mut parts = data.split_whitespace().peekable();

        while let Some(command) = parts.next() {
            match command {
                "M" | "L" => {
                    let coords = parts
                        .next()
                        .ok_or_else(|| "Missing coordinates".to_string())?;
                    let mut coord_parts = coords.split(',');
                    let x = coord_parts
                        .next()
                        .ok_or_else(|| "Missing x coordinate".to_string())?
                        .parse::<f64>()
                        .map_err(|_| "Invalid x coordinate".to_string())?;
                    let y = coord_parts
                        .next()
                        .ok_or_else(|| "Missing y coordinate".to_string())?
                        .parse::<f64>()
                        .map_err(|_| "Invalid y coordinate".to_string())?;
                    points.push((x, y));
                    println!("Parsed point: ({}, {})", x, y);
                }
                "z" => {
                    // Close path, no need to add points here
                }
                _ => {
                    // Handle or ignore other commands
                }
            }
        }
        Ok(points)
    }

    fn parse_points_data(data: &str) -> Result<Vec<(f64, f64)>, String> {
        // Implement the parsing logic for points data (e.g., "10,20 30,40 ...")
        let mut points = Vec::new();
        for pair in data.split_whitespace() {
            let coords: Vec<&str> = pair.split(',').collect();
            if coords.len() == 2 {
                let x = coords[0]
                    .parse()
                    .map_err(|_| format!("Invalid x coordinate: {}", coords[0]))?;
                let y = coords[1]
                    .parse()
                    .map_err(|_| format!("Invalid y coordinate: {}", coords[1]))?;
                points.push((x, y));
            } else {
                return Err(format!("Invalid point format: {}", pair));
            }
        }
        Ok(points)
    }

    // Parses an SVG file and converts it into a `Layout` object.
    // pub fn svg_to_layout_from_file_OLD(path: &str, id: usize) -> Instance {
    //     let file = File::open(path).map_err(|e| format!("Failed to open SVG file: {}", e))?;
    //     let reader = BufReader::new(file);

    //     let mut bin_shape = None;
    //     let mut placed_items = SlotMap::with_key();

    //     for event in svg::parser::Parser::new(reader) {
    //         match event {
    //             Event::Tag("path", _, attributes) => {
    //                 if let Some(data) = attributes.get("d") {
    //                     if let Ok(polygon) = parse_path_data(data) {
    //                         if bin_shape.is_none() {
    //                             bin_shape = Some(polygon); // First shape is the bin
    //                         } else {
    //                             // TODO from_polygon doesnt exist??
    //                             let placed_item = PlacedItem::from_polygon(polygon);
    //                             placed_items.insert(placed_item);
    //                         }
    //                     }
    //                 }
    //             }
    //             Event::Tag("polygon", _, attributes) => {
    //                 if let Some(points) = attributes.get("points") {
    //                     if let Ok(polygon) = parse_polygon_points(points) {
    //                         if bin_shape.is_none() {
    //                             bin_shape = Some(polygon);
    //                         } else {
    //                             // TODO from_polygon doesnt exist??
    //                             let placed_item = PlacedItem::from_polygon(polygon);
    //                             placed_items.insert(placed_item);
    //                         }
    //                     }
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }

    //     let bin_shape = bin_shape.ok_or("No valid bin shape found in SVG")?;
    //     // TODO fix construct of bin: more arguments needed
    //     // let bin = Bin::new(bin_shape);
    //     let bin = Bin::new(
    //         id,
    //         bin_shape,
    //         0,
    //         Transformation::empty(),
    //         vec![],
    //         vec![],
    //         cde_config,
    //     );
    //     // TODO fix construct of CDengine: more arguments needed

    //     let instance: Instance = Instance::new(id, bin, placed_items);

    //     instance
    // }

    // fn parse_path_data(data: &str) -> Result<SimplePolygon, String> {
    //     // Convert SVG path data to a SimplePolygon
    //     let mut points = Vec::new();

    //     let commands = svgtypes::PathParser::from(data);
    //     for command in commands {
    //         match command {
    //             Ok(svgtypes::PathSegment::MoveTo { x, y, .. }) => {
    //                 points.push(Point(x as f32, y as f32));
    //             }
    //             Ok(svgtypes::PathSegment::LineTo { x, y, .. }) => {
    //                 points.push(Point(x as f32, y as f32));
    //             }
    //             _ => {}
    //         }
    //     }

    //     if points.len() >= 3 {
    //         Ok(SimplePolygon::new(points))
    //     } else {
    //         Err("Invalid path data: Not enough points to form a polygon".into())
    //     }
    // }

    // fn parse_polygon_points(points_str: &str) -> Result<SimplePolygon, String> {
    //     let points: Vec<Point> = points_str
    //         .split_whitespace()
    //         .filter_map(|pair| {
    //             let mut coords = pair.split(',');
    //             let x = coords.next()?.parse::<f32>().ok()?;
    //             let y = coords.next()?.parse::<f32>().ok()?;
    //             Some(Point(x, y))
    //         })
    //         .collect();

    //     if points.len() >= 3 {
    //         Ok(SimplePolygon::new(points))
    //     } else {
    //         Err("Invalid polygon data: Not enough points".into())
    //     }
    // }

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
}
