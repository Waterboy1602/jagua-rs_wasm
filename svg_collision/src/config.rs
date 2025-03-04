use serde::{Deserialize, Serialize};

use jagua_rs::util::config::{CDEConfig, SPSurrogateConfig};

/// Configuration for the LBF optimizer
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Config {
    /// Configuration of the Collision Detection Engine
    pub cde_config: CDEConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cde_config: CDEConfig {
                quadtree_depth: 5,
                hpg_n_cells: 2000,
                item_surrogate_config: SPSurrogateConfig {
                    pole_coverage_goal: 0.9,
                    max_poles: 10,
                    n_ff_poles: 2,
                    n_ff_piers: 0,
                },
            },
        }
    }
}
