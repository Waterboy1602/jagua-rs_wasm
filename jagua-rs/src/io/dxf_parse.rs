use dxf::Drawing;
use dxf::entities::*;

use crate::io::json_instance::JsonInstance;
use crate::io::json_instance::JsonStrip;
use crate::io::json_instance::JsonBin;

use log::error;

use crate::fsize;

pub struct DxfInstance {
    /// The name of the instance
    pub name: String,
    /// Set of items to be produced
    pub items: Vec<DxfItem>,
    /// Containers for a Bin Packing Problem
    pub bins: Option<Vec<JsonBin>>,
    /// Container for a Strip Packing Problem
    pub strip: Option<JsonStrip>,
}

pub struct DxfItem {
    /// Number of times this item should be produced
    pub demand: u64,
    /// List of allowed orientations angles (in degrees). If none any orientation is allowed
    pub allowed_orientations: Option<Vec<fsize>>,
    /// Polygon shape of the item
    pub shape: EntityType,
    /// The value of the item (for knapsack problems)
    pub value: Option<u64>,
    /// The quality required for the entire item, if not defined maximum quality is required
    pub base_quality: Option<usize>,
}



pub fn parse_dxf(json_with_dxf_instance: &JsonInstance) -> DxfInstance {
    let mut dxf_items = Vec::new();

    for item in &json_with_dxf_instance.items {
        let dxf_path = match &item.dxf {
            Some(path) => path,
            None => {
                error!("DXF path is missing");
                break;
            }
        };

        let demand = item.demand;
        let allowed_orientations = &item.allowed_orientations;

        // Process entities in the DXF file

        let drawing = match Drawing::load_file(dxf_path) {
            Ok(drawing) => drawing,
            Err(err) => {
                error!("Failed to load DXF file: {}", err);
                continue;
            }
        };

        for e in drawing.entities() {
            println!("found entity on layer {}", e.common.layer);
            let dxf_item = DxfItem {
                demand: demand,
                allowed_orientations: allowed_orientations.clone(),
                shape: e.specific.clone(),
                value: item.value,
                base_quality: item.base_quality,
            };

            dxf_items.push(dxf_item);
        }  
    }
    
    DxfInstance {
        name: json_with_dxf_instance.name.clone(),
        items: dxf_items,
        bins: json_with_dxf_instance.bins.clone(),
        strip: json_with_dxf_instance.strip.clone(),
    }
}