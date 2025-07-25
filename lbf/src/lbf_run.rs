use std::path::{Path, PathBuf};

use jagua_rs::io::json_instance::JsonInstance;
use log::{error, warn};
use rand::prelude::SmallRng;
use rand::SeedableRng;

use crate::io::json_output::JsonOutput;
use crate::io::layout_to_svg::s_layout_to_svg;
use crate::lbf_config::LBFConfig;
use crate::lbf_optimizer::LBFOptimizer;
use crate::{io, EPOCH};
use jagua_rs::entities::instances::instance::Instance;
use jagua_rs::io::parser;
use jagua_rs::io::parser::Parser;
use jagua_rs::util::polygon_simplification::PolySimplConfig;

pub fn solve_json(config_json: String, input_json: String, path_sol: String) -> Vec<Vec<String>> {
    let config = if config_json.is_empty() {
        warn!("No config file provided");
        warn!(
            "Falling back default config:\n{}",
            serde_json::to_string(&LBFConfig::default()).unwrap()
        );
        LBFConfig::default()
    } else {
        serde_json::from_str(&config_json).unwrap_or_else(|err| {
            error!("Config json could not be parsed: {}", err);
            panic!();
        })
    };

    let json_instance: JsonInstance;
    let instance: Instance;

    json_instance = io::read_json_instance(None, Some(&input_json));
    let poly_simpl_config = match config.poly_simpl_tolerance {
        Some(tolerance) => PolySimplConfig::Enabled { tolerance },
        None => PolySimplConfig::Disabled,
    };

    // let parser = Parser::new(poly_simpl_config, config.cde_config, true, PathBuf::new());
    let parser = Parser::new(poly_simpl_config, config.cde_config, true);

    instance = parser.parse(&json_instance);

    let rng = match config.prng_seed {
        Some(seed) => SmallRng::seed_from_u64(seed),
        None => SmallRng::from_entropy(),
    };

    let mut optimizer = LBFOptimizer::new(instance.clone(), config, rng);
    let solution = optimizer.solve();

    let json_output = JsonOutput {
        instance: json_instance.clone(),
        solution: parser::compose_json_solution(&solution, &instance, EPOCH.clone()),
        config: config.clone(),
    };

    let json_sol_path: String = format!("{}sol_{}.json", path_sol, "web");
    io::write_json_output(&json_output, Path::new(&json_sol_path));

    let mut svg_sol_paths = Vec::new();
    for (i, s_layout) in solution.layout_snapshots.iter().enumerate() {
        let svg_path = format!("{}sol_{}_{}.svg", path_sol, "web", i);
        io::write_svg(
            &s_layout_to_svg(s_layout, &instance, config.svg_draw_options),
            Path::new(&svg_path),
        );
        svg_sol_paths.push(svg_path);
    }

    vec![svg_sol_paths.clone(), vec![json_sol_path.clone()]]
}
