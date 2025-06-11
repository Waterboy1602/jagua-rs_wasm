use svg::parser::{Event, Parser};

use itertools::Itertools;
use jagua_rs::entities::Item;
use jagua_rs::entities::Layout;
use jagua_rs::geometry::DTransformation;
use jagua_rs::geometry::shape_modification::ShapeModifyConfig;
use jagua_rs::io::ext_repr::{ExtContainer, ExtItem, ExtSPolygon, ExtShape};
use jagua_rs::io::import::{Importer, ext_to_int_transformation};
use jagua_rs::io::svg::layout_to_svg;
use jagua_rs::probs::bpp::entities::{BPInstance, Bin};

use std::collections::HashMap;

use crate::config::Config;
use crate::config::DRAW_OPTIONS_WITHOUT_HIGHLIGHT;
use crate::config::OUTPUT_DIR;

use std::path::Path;

pub struct SvgParser {
    pub shape_modify_config: ShapeModifyConfig,
    pub config: Config,
    pub item_transformations: HashMap<usize, Vec<DTransformation>>,
    pub importer: Importer,
}

impl SvgParser {
    pub fn new(
        config: Config,
        poly_simpl_tolerance: Option<f32>,
        min_item_separation: Option<f32>,
    ) -> SvgParser {
        SvgParser {
            shape_modify_config: ShapeModifyConfig {
                offset: min_item_separation.map(|f| f / 2.0),
                simplify_tolerance: poly_simpl_tolerance,
            },
            config,
            item_transformations: HashMap::new(),
            importer: Importer::new(
                config.cde_config.clone(),
                poly_simpl_tolerance,
                min_item_separation,
            ),
        }
    }

    // Parses an SVG file and converts it into a `Layout` object.
    // TODO START MET Strip Packing (SP)
    pub fn svg_file_to_instance(&mut self, svg_str: &str) -> Result<BPInstance, String> {
        let mut inside_defs = false;
        let mut inside_group = false;

        let mut object_type: String = String::new();
        let mut object_id: usize = 0;
        let mut bins: Vec<Bin> = Vec::new();
        let mut item_map: HashMap<usize, (Item, usize)> = HashMap::new();

        for event in Parser::new(&svg_str) {
            match event {
                Event::Tag("defs", _, _attributes) => {
                    inside_defs = !inside_defs;
                }
                Event::Tag("g", _, attributes) => {
                    if !inside_defs {
                        inside_group = !inside_group;
                    }

                    if let Some(id) = attributes.get("id") {
                        let parts: Vec<&str> = id.split('_').collect();

                        if parts.is_empty() {
                            return Err("Missing item id/type".to_string());
                        }

                        object_type = parts[0].to_string().to_lowercase();

                        if object_type == "item" {
                            object_id = if parts.len() > 1 {
                                parts[1].parse::<usize>().unwrap()
                            } else {
                                None.unwrap()
                            };
                        }
                    }
                }
                Event::Tag("path", _, attributes) if inside_group => {
                    if let Some(d) = attributes.get("d") {
                        if object_type == "container" && bins.is_empty() {
                            let container = self
                                .importer
                                .import_container(&self.parse_container_data(d, object_id))
                                .unwrap();
                            bins.push(Bin::new(container, 1, 1));
                        } else if object_type == "item" {
                            let item = self
                                .importer
                                .import_item(&self.parse_path_data(d, object_id))
                                .unwrap();
                            item_map.insert(object_id, (item, 1));
                        }
                    }
                }
                Event::Tag("use", _, attributes) => {
                    if let Some(href) = attributes.get("xlink:href") {
                        if let Some(transform) = attributes.get("transform") {
                            let transform_values = transform
                                .split(", ")
                                .map(|s| s.trim().to_string())
                                .collect::<Vec<String>>();

                            self.parse_transform_data(transform_values, &href, &item_map);
                        }
                    }
                }
                _ => {}
            }
        }

        println!("Number of bins: {:?}", bins.len());
        println!("Number of parsed items: {:?}", item_map.len());

        let instance = BPInstance::new(
            item_map
                .iter()
                .sorted_by_key(|(k, _)| *k)
                .map(|(_, v)| v.clone())
                .collect(),
            bins,
        );

        Ok(instance)
    }

    fn parse_path_data(&self, data: &str, item_id: usize) -> ExtItem {
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

                    points.push((x, y));
                }
                "z" => {
                    // Close path, no need to add points here
                }
                _ => {
                    // Handle or ignore other commands
                }
            }
        }

        let ext_shape = ExtShape::SimplePolygon(ExtSPolygon { 0: points.clone() });

        let item = ExtItem {
            id: item_id as u64,
            allowed_orientations: None,
            shape: ext_shape,
            min_quality: None,
        };

        item
    }

    fn parse_container_data(&self, data: &str, container_id: usize) -> ExtContainer {
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

                    points.push((x, y));
                }
                "z" => {
                    // Close path, no need to add points here
                }
                _ => {
                    // Handle or ignore other commands
                }
            }
        }

        let ext_shape = ExtShape::SimplePolygon(ExtSPolygon { 0: points.clone() });

        let container = ExtContainer {
            id: container_id as u64,
            shape: ext_shape,
            zones: Vec::new(),
        };

        container
    }

    fn parse_transform_data(
        &mut self,
        transform: Vec<String>,
        href: &str,
        item_map: &HashMap<usize, (Item, usize)>,
    ) {
        let translate_vec: Vec<f32> = transform[0]
            .replace("translate(", "")
            .replace(")", "")
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse translation value"))
            .collect();

        let translation: (f32, f32) = if translate_vec.len() == 2 {
            (translate_vec[0], translate_vec[1])
        } else {
            panic!("Expected two translation values, got {:?}", translate_vec);
        };

        let rotation_deg: f32 = transform[1]
            .replace("rotate(", "")
            .replace(")", "")
            .parse()
            .expect("Failed to parse string to f32");

        let rotation_rad = rotation_deg.to_radians();

        let pre_transform = item_map
            .get(&href.split('_').last().unwrap().parse::<usize>().unwrap())
            .map(|(item, _)| item.shape_orig.pre_transform.clone())
            .unwrap_or(DTransformation::empty());

        let transformation = ext_to_int_transformation(
            &DTransformation::new(rotation_rad, translation),
            &pre_transform,
        );

        let item_id = href.split('_').last().unwrap().parse::<usize>().unwrap();

        self.item_transformations
            .entry(item_id)
            .or_insert_with(Vec::new)
            .push(transformation);
    }
}

pub fn run_cde(svg_str: &str) {
    let config = Config::default();

    let mut svg_parser = SvgParser::new(config, None, None);

    let instance = SvgParser::svg_file_to_instance(&mut svg_parser, svg_str).unwrap();

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

pub fn run_cde_wasm(svg_str: &str) -> Result<String, String> {
    let config = Config::default();

    let mut svg_parser = SvgParser::new(config, None, None);

    let instance = SvgParser::svg_file_to_instance(&mut svg_parser, svg_str).unwrap();

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

    let svg_string = svg.to_string();
    Ok(svg_string)
}
