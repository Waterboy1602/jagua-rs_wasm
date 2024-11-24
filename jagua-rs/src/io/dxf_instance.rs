use serde::{Deserialize, Serialize};

use crate::fsize;

/// The DXF representation of a problem instance
#[derive(Serialize, Deserialize, Clone)]
pub struct DxfInstance {
    #[serde(rename = "Name")]
    /// The name of the instance
    pub name: String,
    /// Set of items to be produced
    #[serde(rename = "Items")]
    pub items: Vec<DxfItem>,
    /// Containers for a Bin Packing
    #[serde(rename = "Objects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bins: Option<Vec<DxfBin>>,
    /// Container for a Strip Packing Problem
    #[serde(rename = "Strip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip: Option<DxfStrip>,
}


/// The JSON representation of a bin
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DxfBin {
    /// The cost of using this bin
    pub cost: u64,
    /// Number of this bin available, if not present, it is assumed to be unlimited
    pub stock: Option<u64>,
    /// Polygon shape of the bin
    pub shape: EntityType,
    /// A list of zones with different quality levels
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub zones: Vec<DxfQualityZone>,
}

/// The JSON representation of a strip with fixed height and variable width
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DxfStrip {
    pub height: fsize,
}

/// The JSON representation of an item
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DxfItem {
    /// Number of times this item should be produced
    pub demand: u64,
    /// List of allowed orientations angles (in degrees). If none any orientation is allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_orientations: Option<Vec<fsize>>,
    /// Polygon shape of the item
    pub shape: EntityType,
    /// The value of the item (for knapsack problems)
    pub value: Option<u64>,
    /// The quality required for the entire item, if not defined maximum quality is required
    pub base_quality: Option<usize>,
}

/// Different ways to represent a shape
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "Type", content = "Data")]
#[serde(rename_all_fields = "PascalCase")]
pub enum EntityType {
    /// Axis-aligned rectangle. With the left-bottom corner at (0, 0)
    Rectangle { width: fsize, height: fsize },
    /// Polygon with a single outer boundary
    SimplePolygon(DxfSimplePoly),
    /// Polygon with a single outer boundary and a list of holes
    Polygon(DxfPoly),
    /// Multiple disjoint polygons
    MultiPolygon(Vec<DxfPoly>),
}

/// A polygon represented as an outer boundary and a list of holes
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DxfPoly {
    /// The outer boundary of the polygon
    pub outer: DxfSimplePoly,
    /// A list of holes in the polygon
    #[serde(default)]
    pub inner: Vec<DxfSimplePoly>,
}

/// A simple polygon represented as a list of points (x, y)
#[derive(Serialize, Deserialize, Clone)]
pub struct DxfSimplePoly(pub Vec<(fsize, fsize)>);

/// A zone with a specific quality level
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DxfQualityZone {
    /// The quality level of this zone
    pub quality: usize,
    /// The polygon shape of this zone
    pub shape: EntityType,
}