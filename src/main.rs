#![allow(unused)]

use futures::{future::lazy, sync::mpsc};
use pest_derive::Parser;

mod filters;
mod inputs;
mod outputs;
use filters::{Filter, FilterBlock};
use inputs::{Input, InputBlock};
use outputs::{Output, OutputBlock};

#[derive(Parser)]
#[grammar = "logstash.pest"]
pub struct ConfigParser;

fn main() {
    // inputs
    let poller = Input::HttpPoller(inputs::HttpPoller::new(
        1000,
        vec!["https://jsonplaceholder.typicode.com/posts/1"],
    ));
    let generator = Input::Generator(inputs::Generator::new());
    let exec = Input::Exec(inputs::Exec::new("ls"));

    // filters
    let geoip = Filter::Geoip(filters::Geoip::new("ip"));
    let mutate = Filter::Mutate(filters::Mutate::new());
    let clone = Filter::Clone(filters::Clone::new(Vec::new()));
    let fingerprint = Filter::Fingerprint(filters::Fingerprint::new());

    let json = Filter::Json(filters::Json {
        skip_on_invalid_json: Some(false),
        source: "jsonString",
        tag_on_failure: Some(vec!["_jsonparsefailure"]),
        target: Some("jsonString"),
        _receiver: None,
        _sender: None,
    });

    // outputs
    let stdout = Output::Stdout(outputs::Stdout::new());

    // communication channels
    let (input_sender, filter_receiver) = mpsc::channel(1_024);
    let (filter_sender, output_receiver) = mpsc::channel(1_024);

    // blocks
    let inputs = InputBlock(vec![generator], input_sender);
    let filters = FilterBlock(vec![json, fingerprint], filter_receiver, filter_sender);
    let outputs = OutputBlock(vec![stdout], output_receiver);

    // pipeline
    let pipeline = Pipeline(inputs, filters, outputs);

    tokio::run(lazy(move || {
        pipeline.run();
        Ok(())
    }));
}

pub struct Pipeline(InputBlock, FilterBlock, OutputBlock);

impl Pipeline {
    pub fn run(self) {
        self.0.run();
        self.1.run();
        self.2.run();
    }
}
