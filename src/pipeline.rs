#![allow(unused)]

#[derive(Parser)]
#[grammar = "logstash.pest"]
pub struct ConfigParser;

mod filters;
mod inputs;
mod outputs;
use filters::{Filter, FilterBlock};
use inputs::{Input, InputBlock};
use outputs::{Output, OutputBlock};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use pest_derive::Parser;
use futures::{sync::mpsc};
use std::convert::TryFrom;

use inputs::Generator;
use inputs::HttpPoller;

pub struct Pipeline(pub InputBlock, pub FilterBlock, pub OutputBlock);

impl Pipeline {

    pub fn run(self) {
        let filter_receiver = self.0.run();
        let outputs_receiver = self.1.run(filter_receiver);
        self.2.run(outputs_receiver);
    }

    pub fn from_toml(path: &Path) -> Pipeline {
                
        // inputs
        // let poller = Input::HttpPoller(inputs::HttpPoller::new(
        //     1000,
        //     vec!["https://jsonplaceholder.typicode.com/posts/1"],
        // ));
        // let generator = Input::Generator(inputs::Generator::new());
        // let exec = Input::Exec(inputs::Exec::new("ls"));

        // filters
        // let geoip = Filter::Geoip(filters::Geoip::new("ip"));
        
        // let clone = Filter::Clone(filters::Clone::new(Vec::new()));
        // let fingerprint = Filter::Fingerprint(filters::Fingerprint::new());

        let json = Filter::Json(filters::Json {
            skip_on_invalid_json: false,
            source: "message",
            tag_on_failure: vec!["_jsonparsefailure"],
            target: "jsonString",
            _receiver: None,
            _sender: None,
        });

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

        // pipeline
        let pipeline = Pipeline(inputs, filters, outputs);

        pipeline
        
    }
    
}

trait InputPlugin {
    fn to_input(&self) -> Input;
}

impl InputPlugin for toml::Value {
    fn to_input(&self) -> Input {
        if let Some(generator) = self.get("generator") {
            let plugin = Generator::try_from(generator.to_owned()).unwrap();
            let generator = Input::Generator(plugin, None);
            return generator
        };
        if let Some(poller) = self.get("http_poller") {
            let plugin = HttpPoller::try_from(poller.to_owned()).unwrap();
            let poller = Input::HttpPoller(plugin, None);
            return poller
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
            let plugin = filters::Mutate::try_from(mutate.to_owned()).unwrap();
            let mutate = Filter::Mutate(plugin);
            return mutate
        };
        panic!("Bad config.");
    }
}
