mod filters;
mod inputs;
mod outputs;
use filters::*;
use inputs::*;
use outputs::*;

use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Pipeline(pub InputBlock, pub FilterBlock, pub OutputBlock);

impl Pipeline {
    pub fn run(self) {
        let filter_receiver = self.0.run();
        let output_receiver = self.1.run(filter_receiver);
        self.2.run(output_receiver);
    }

    pub fn from_toml(path: &Path) -> Pipeline {
        // outputs
        let stdout = Output::Stdout(outputs::Stdout {
            ..Default::default()
        });

        // blocks
        let mut inputs = InputBlock(vec![]);
        let mut filters = FilterBlock(vec![]);
        let outputs = OutputBlock(vec![stdout]);

        // read pipeline config
        let mut config_file = File::open(path).expect("Couldn't open config file.");

        let mut config = String::new();
        config_file
            .read_to_string(&mut config)
            .expect("Couldn't read config file.");

        let toml: toml::Value = config.parse().expect("Couldn't parse config TOML.");

        if let Some(input_block) = toml.get("inputs") {
            if let Some(input_block) = input_block.as_array() {
                input_block.iter().for_each(|input| {
                    if let Some(input_block) = input.as_table() {
                        input_block.iter().for_each(|input| {
                            inputs.0.push(input.to_input());
                        })
                    }
                });
            }
        }

        if let Some(filter_block) = toml.get("filters") {
            if let Some(filter_block) = filter_block.as_array() {
                filter_block.iter().for_each(|filter| {
                    if let Some(filter_block) = filter.as_table() {
                        filter_block.iter().for_each(|filter| {
                            filters.0.push(filter.to_filter());
                        })
                    }
                });
            }
        }

        // dbg!(&filters.0);

        Pipeline(inputs, filters, outputs)
    }
}

trait InputPlugin {
    fn to_input(&self) -> Input;
}

impl InputPlugin for (&String, &toml::Value) {
    fn to_input(&self) -> Input {
        if self.0 == "generator" {
            let plugin = Generator::try_from(self.1).unwrap();
            return Input::Generator(plugin, None);
        };
        if self.0 == "http_poller" {
            let plugin = HttpPoller::try_from(self.1).unwrap();
            return Input::HttpPoller(Box::new(plugin), None);
        };
        panic!("Bad configuration for {} input block.", self.0);
    }
}

trait FilterPlugin {
    fn to_filter(&self) -> Filter;
}

impl FilterPlugin for (&String, &toml::Value) {
    fn to_filter(&self) -> Filter {
        if self.0 == "mutate" {
            let plugin = Mutate::try_from(self.1).expect("Incorrect Mutate filter config.");
            return Filter::Mutate(Box::new(plugin));
        };
        if self.0 == "json" {
            let plugin = Json::try_from(self.1).expect("Incorrect Json filter config.");
            return Filter::Json(plugin);
        };
        panic!("Bad configuration for {} filter block.", self.0);
    }
}
