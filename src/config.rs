use std::fs::File;
use std::io::Read;

pub fn get() -> toml::Value {

    let mut config_file = File::open("./example_configs/full.toml")
        .expect("Couldn't read config file.");

    let mut config = String::new();
    config_file.read_to_string(&mut config)
        .expect("Couldn't read config file.");

    let config: toml::Value = config.parse()
        .expect("Couldn't parse config TOML");

    config

}
