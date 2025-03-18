use log::{error, warn};
use std::fs::File;
use std::io::BufReader;

// use svgtypes::{PathParser, PathSegment};

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

    let config = Config::default();
    let parser = SvgParser::new(config);
    let instance = SvgParser::svg_to_layout_from_file("./assets/1.svg", 1);
}
