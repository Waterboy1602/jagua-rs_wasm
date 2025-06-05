use serde::{Deserialize, Serialize};

use jagua_rs::collision_detection::CDEConfig;
use jagua_rs::geometry::fail_fast::SPSurrogateConfig;
use jagua_rs::io::svg::{SvgDrawOptions, SvgLayoutTheme};

/// Configuration for the LBF optimizer
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Config {
    /// Configuration of the Collision Detection Engine
    pub cde_config: CDEConfig,
    pub prng_seed: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cde_config: CDEConfig {
                quadtree_depth: 5,
                item_surrogate_config: SPSurrogateConfig {
                    n_pole_limits: [(100, 0.0), (20, 0.75), (10, 0.90)],
                    n_ff_poles: 2,
                    n_ff_piers: 0,
                },
            },
            prng_seed: Some(0),
        }
    }
}

pub const OUTPUT_DIR: &str = "../solutions";

pub const DRAW_OPTIONS_WITH_HIGHLIGHT: SvgDrawOptions = SvgDrawOptions {
    theme: SvgLayoutTheme::GRAY,
    quadtree: false,
    surrogate: false,
    highlight_collisions: true,
    draw_cd_shapes: false,
    highlight_cd_shapes: true,
};

pub const DRAW_OPTIONS_WITHOUT_HIGHLIGHT: SvgDrawOptions = SvgDrawOptions {
    theme: SvgLayoutTheme::GRAY,
    quadtree: false,
    surrogate: false,
    highlight_collisions: false,
    draw_cd_shapes: false,
    highlight_cd_shapes: true,
};
