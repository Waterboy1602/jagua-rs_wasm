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
// use jagua_rs::geometry::primitives::edge::Edge;
use crate::config::Config;
use jagua_rs::entities::Instance;
use jagua_rs::entities::Item;
use jagua_rs::geometry::Transformation;
use jagua_rs::probs::spp::entities::SPInstance;

pub struct SvgParser {
    config: Config,
}

impl SvgParser {
    pub fn new(config: Config) -> SvgParser {
        SvgParser { config }
    }

    // Parses an SVG file and converts it into a `Layout` object.
    // TODO START MET Strip Packing (SP)
    pub fn svg_to_layout_from_file(&self, path: &str) -> Result<SPInstance, String> {
        let mut file = File::open(path).map_err(|e| format!("Failed to open SVG file: {}", e))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| format!("Failed to read SVG file: {}", e))?;

        let mut inside_defs = false;
        let mut inside_group = false;

        let mut item_id = 0; // Initialize item_id
        let mut item_type = String::new(); // Initialize item_type with an empty string
        let mut items = Vec::new();
        let mut bins = Vec::new();

        for event in Parser::new(&content) {
            match event {
                Event::Tag("defs", _, attributes) => {
                    inside_defs = !inside_group;
                }
                Event::Tag("g", _, attributes) if inside_defs => {
                    inside_group = !inside_group;
                    if let Some(id) = attributes.get("id") {
                        println!("Found <g> inside <defs> with id: {:?}", id.to_string());
                        let parts: Vec<&str> = id.split('_').collect();

                        if parts.is_empty() {
                            return Err("Missing item id/type".to_string());
                        }

                        item_type = parts[0].to_string();

                        item_id = if parts.len() > 1 {
                            parts[1].parse::<usize>().unwrap_or(0)
                        } else {
                            0 // Default to 0 if no id part is found
                        };
                    }
                }
                Event::Tag("path", _, attributes) if inside_defs && inside_group => {
                    if let Some(d) = attributes.get("d") {
                        println!(
                            "Found <path> inside <g> inside <defs> with d: {:?}",
                            d.to_string()
                        );

                        if item_type.to_lowercase() == "bin" {
                            let bin_shape = self.parse_bin_data(d, item_id);
                            bins.push(bin_shape);
                        } else if item_type.to_lowercase() == "item" {
                            let item_shape = self.parse_path_data(d, item_id);
                            items.push(item_shape);
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

        let instance = SPInstance::new(items, bins).into();

        println!("Created instance: {:?}", instance);

        Ok(instance)
    }

    fn parse_path_data(&self, data: &str, item_id: usize) -> (Item, usize) {
        let mut points = Vec::new();
        let mut parts = data.split_whitespace().peekable();

        while let Some(command) = parts.next() {
            match command {
                s if s.starts_with("M") || s.starts_with("L") => {
                    let command = command.replace("M", "");
                    let command = command.replace("L", "");
                    let mut coord_parts = command.split(',');
                    let x = coord_parts.next().unwrap().parse::<f32>().unwrap();
                    let y = coord_parts.next().unwrap().parse::<f32>().unwrap();

                    let point = Point(x, y);
                    points.push(point);
                }
                "z" => {
                    // Close path, no need to add points here
                }
                _ => {
                    // Handle or ignore other commands
                }
            }
        }
        let shape = SimplePolygon::new(points.clone());

        let allowed_orientations = AllowedRotation::Continuous; // Allow any rotation
        let base_quality = 1; // Quality of the item (not yet supported) - max is = 1
        let item_value = 0; // Value for knapsack problem (not yet supported)

        let item = Item::new(
            item_id,
            shape,
            allowed_orientations,
            Some(base_quality),
            item_value,
            Transformation::empty(),
            self.config.cde_config.item_surrogate_config.clone(),
        );

        (item, 1 as usize)
    }

    fn parse_bin_data(&self, data: &str, bin_id: usize) -> (Bin, usize) {
        let mut points = Vec::new();
        let mut parts = data.split_whitespace().peekable();

        while let Some(command) = parts.next() {
            match command {
                s if s.starts_with("M") || s.starts_with("L") => {
                    let command = command.replace("M", "");
                    let command = command.replace("L", "");
                    let mut coord_parts = command.split(',');
                    let x = coord_parts.next().unwrap().parse::<f32>().unwrap();
                    let y = coord_parts.next().unwrap().parse::<f32>().unwrap();

                    let point = Point(x, y);
                    points.push(point);
                }
                "z" => {
                    // Close path, no need to add points here
                }
                _ => {
                    // Handle or ignore other commands
                }
            }
        }
        let bin_shape = SimplePolygon::new(points.clone());

        let bin = Bin::new(
            bin_id,
            bin_shape,
            1,
            Transformation::empty(),
            vec![],
            vec![],
            self.config.cde_config.clone(),
        );
        (bin, 1)
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
