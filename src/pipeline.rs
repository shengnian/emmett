#![allow(unused)]

mod inputs;
mod filters;
mod outputs;
use inputs::*;
use filters::*;
use outputs::*;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::convert::TryFrom;

use futures::{sync::mpsc};

pub struct Pipeline(pub InputBlock, pub FilterBlock, pub OutputBlock);

impl Pipeline {

    pub fn run(self) {
        let filter_receiver = self.0.run();
        let outputs_receiver = self.1.run(filter_receiver);
        self.2.run(outputs_receiver);
    }

    pub fn from_toml(path: &Path) -> Pipeline {

        // outputs
        let stdout = Output::Stdout(outputs::Stdout::new());

        // blocks
        let mut inputs = InputBlock(vec![]);
        let mut filters = FilterBlock(vec![]);
        let mut outputs = OutputBlock(vec![stdout]);

        // read pipeline config
        let mut config_file = File::open(path)
            .expect("Couldn't open config file.");

        let mut config = String::new();
        config_file.read_to_string(&mut config)
            .expect("Couldn't read config file.");

        let toml: toml::Value = config.parse()
            .expect("Couldn't parse config TOML.");

        if let Some(input_block) = toml.get("inputs") {
            if let Some(input_block) = input_block.as_array() {
                input_block.into_iter().for_each(|input| {
                    inputs.0.push(input.to_input());
                });
            }
        }

        if let Some(filter_block) = toml.get("filters") {
            if let Some(filter_block) = filter_block.as_array() {
                filter_block.into_iter().for_each(|filter| {
                    filters.0.push(filter.to_filter());
                });
            }
        }

        Pipeline(inputs, filters, outputs)
        
    }
    
}

trait InputPlugin {
    fn to_input(&self) -> Input;
}

impl InputPlugin for toml::Value {
    fn to_input(&self) -> Input {
        if let Some(generator) = self.get("generator") {
            let plugin = Generator::try_from(generator.to_owned()).unwrap();
            return Input::Generator(plugin, None)
        };
        if let Some(poller) = self.get("http_poller") {
            let plugin = HttpPoller::try_from(poller.to_owned()).unwrap();
            return Input::HttpPoller(plugin, None)
        };
        panic!("Bad config.");
    }
}

trait FilterPlugin {
    fn to_filter(&self) -> Filter;
}

impl FilterPlugin for toml::Value {
    fn to_filter(&self) -> Filter {
        if let Some(mutate) = self.get("mutate") {
            let plugin = Mutate::try_from(mutate)
                .expect("Incorrect Mutate filter config.");
            return Filter::Mutate(plugin)
        };
        panic!("Bad config.");
    }
}
