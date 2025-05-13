use jagua_rs::entities::problems::problem::Problem;
use log::{error, warn};
use rand::SeedableRng;
use rand::prelude::SmallRng;
use std::fs::File;
use std::io::BufReader;

// use svgtypes::{PathParser, PathSegment};
use jagua_rs::entities::instances::instance::Instance;
use jagua_rs::entities::instances::instance_generic::InstanceGeneric;

use jagua_rs::collision_detection::hazards::filter::NoHazardFilter;
use jagua_rs::entities::layout::Layout;
use jagua_rs::entities::problems::bin_packing::BPProblem;
use jagua_rs::entities::problems::strip_packing::SPProblem;
use jagua_rs::probs::spp::entities::{SPInstance, SPPlacement, SPProblem, SPSolution};
use jagua_rs::probs::{bpp, spp};
use svg_collision::config::Config;
use svg_collision::io::svg_parser::SvgParser;

fn main() {
    // let config = match args.config_file {
    //     None => {
    //         warn!("No config file provided, use --config-file to provide a custom config");
    //         warn!(
    //             "Falling back default config:\n{}",
    //             serde_json::to_string(&LBFConfig::default()).unwrap()
    //         );
    //         Config::default()
    //     }
    //     Some(config_file) => {
    //         let file = File::open(config_file).unwrap_or_else(|err| {
    //             panic!("Config file could not be opened: {}", err);
    //         });
    //         let reader = BufReader::new(file);
    //         serde_json::from_reader(reader).unwrap_or_else(|err| {
    //             error!("Config file could not be parsed: {}", err);
    //             error!("Omit the --config-file argument to use the default config");
    //             panic!();
    //         })
    //     }
    // };

    // ! HOE HET VROEGER WAS (Voor 06/05)
    // let config = Config::default();
    // let parser = SvgParser::new(config);
    // let instance = parser.svg_to_layout_from_file("./assets/1.svg").unwrap();

    // let rng = match config.prng_seed {
    //     Some(seed) => SmallRng::seed_from_u64(seed),
    //     None => SmallRng::from_entropy(),
    // };

    // let problem: Problem = match instance.clone() {
    //     Instance::BP(bpi) => BPProblem::new(bpi.clone()).into(),
    //     Instance::SP(spi) => {
    //         let strip_width = instance.item_area() * 2.0 / spi.strip_height; //initiate with 50% usage
    //         SPProblem::new(spi.clone(), strip_width, config.cde_config).into()
    //     }
    // };

    // let layout: &Layout = problem.get_layout(layout_idx);
    // let cde = layout.cde();
    // let irrel_hazards = match item.hazard_filter.as_ref() {
    //     None => vec![],
    //     Some(hf) => hazard_filter::generate_irrelevant_hazards(hf, layout.cde().all_hazards()),
    // };

    let importer = Importer::new(
        config.cde_config,
        config.poly_simpl_tolerance,
        config.min_item_separation,
    );

    let rng = match config.prng_seed {
        Some(seed) => SmallRng::seed_from_u64(seed),
        None => SmallRng::from_os_rng(),
    };

    // TODO spp::io::import bekijken en gebruiken voor svg_to_layout
    // ! impl Importer {}
    let instance = SvgParser::svg_to_layout_from_file("./assets/1.svg").unwrap();

    let problem = SPProblem::new(instance.clone());

    let cde = problem.layout.cde();

    // TODO: HEEFT NOG NOOIT GEWERKT
    let mut hpg_sampler = HPGSampler::new(item, layout)?;
    let transform = hpg_sampler.sample(rng);

    let surrogate = item.shape_cd.surrogate();
    let mut buffer = {
        let mut buffer = (*item.shape).clone();
        buffer.surrogate = None; //strip the surrogate for faster transforms, we don't need it for the buffer shape
        buffer
    };
    buffer.transform_from(&item.shape, &transform);
    if !cde.poly_collides(&buffer, &NoHazardFilter) {
        print!("no collision detected");
    } else {
        print!("collision detected");
    }
}
