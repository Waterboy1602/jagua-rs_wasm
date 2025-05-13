use serde::{Deserialize, Serialize};

use jagua_rs::collision_detection::CDEConfig;
use jagua_rs::geometry::fail_fast::SPSurrogateConfig;

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
