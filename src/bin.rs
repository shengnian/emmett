use futures::future::lazy;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use emmett::Pipeline;

fn main() {
    let config_path = Path::new("./example_configs/full.toml");
    let mut config_file = File::open(config_path).expect("Couldn't open config file.");
    let mut config = String::new();

    config_file
        .read_to_string(&mut config)
        .expect("Couldn't read config file.");

    let toml: toml::Value = config.parse().expect("Couldn't parse config TOML.");

    tokio::run(lazy(move || {
        Pipeline::from_toml(toml)
            .expect("Couldn't create Pipeline from TOML")
            .run();
        Ok(())
    }));
}
