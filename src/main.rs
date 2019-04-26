// use log::debug;
use env_logger::{Builder, Env};
use structopt::{self, StructOpt};
use pest_derive::Parser;
use futures::{future::lazy, sync::mpsc};

mod inputs;
mod filters;
mod outputs;

use inputs::{Input, InputBlock};
use filters::{Filter, FilterBlock};
use outputs::{Output, OutputBlock};

#[derive(Parser)]
#[grammar = "logstash.pest"]
pub struct ConfigParser;

#[derive(Debug, StructOpt)]
#[structopt(name = "(=^•ω•^=) emmett")]
struct Opt {
    #[structopt(short = "d", long = "debug")]
    debug: bool
}

fn main() {

    let opt = Opt::from_args();

    if opt.debug {
        let debug = Env::default().default_filter_or("debug");
        Builder::from_env(debug).init();
    }

    // inputs
    let poller = Input::HttpPoller(
        inputs::HttpPoller::new(3, vec!["https://jsonplaceholder.typicode.com/posts/1"])
    );
    let generator = Input::Generator(inputs::Generator::new());
    let exec = Input::Exec(inputs::Exec::new("ls"));

    // filters
    let geoip = Filter::Geoip(filters::GeoipFilter::new("ip"));
    let mutate = Filter::MutateFilter(filters::MutateFilter::new());
    
    // outputs
    let stdout = Output::Stdout(outputs::Stdout::new());

    // communication channels
    let (input_sender, filter_receiver) = mpsc::channel(1_024);
    let (filter_sender, output_receiver) = mpsc::channel(1_024);

    // blocks
    let inputs = InputBlock(vec![poller, exec, generator], input_sender);
    let filters = FilterBlock(vec![geoip, mutate], filter_receiver, filter_sender);
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

