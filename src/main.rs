// use log::debug;
use env_logger::{Builder, Env};
use structopt::{self, StructOpt};
use pest_derive::Parser;
use futures::{future::lazy, sync::mpsc};

pub mod plugins;
use plugins::{input,
              filter,
              output,
              input::{Input, InputBlock},
              filter::{Filter, FilterBlock},
              output::{Output, OutputBlock}};

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
        input::HttpPoller::new(3000, vec!["https://jsonplaceholder.typicode.com/todos/1"])
    );
    let generator = Input::Generator(input::Generator::new());
    let exec = Input::Exec(input::Exec::new("ls"));

    // filters
    let geoip = Filter::Geoip(filter::GeoipFilter::new("ip"));
    
    // outputs
    let stdout = Output::Stdout(output::Stdout::new());

    // communication channels
    let (input_sender, filter_receiver) = mpsc::channel(1_024);
    let (filter_sender, output_receiver) = mpsc::channel(1_024);

    // blocks
    let inputs = InputBlock(vec![exec, generator], input_sender);
    let filters = FilterBlock(vec![geoip], filter_receiver, filter_sender);
    let _outputs = OutputBlock(vec![stdout], output_receiver);
    
    tokio::run(lazy(move || {

        inputs.run();
        filters.run();

        // for output in outputs.0 { tokio::spawn(output); }
        
        // // send every message to every output
        // let output_stream = output_receiver.for_each(|message| {

        //     outputs.0.iter().map(|output| {
        //         output.process(&message);
        //     });
            
        //     // debug!("{}", message);
            
        //     Ok(())
                
        // });

        // tokio::spawn(output_stream);

        Ok(())
            
    }));
    
}

// impl Pipeline {
    
//     pub fn new() -> Self {
//         Self {
//             ..Default::default()
//         }
//     }

//     pub fn from_file(path: &Path) -> Self {

//         let conf = read_to_string(path)
//             .expect("Cannot read Logstash configuration file.");
        
//         let conf = ConfigParser::parse(Rule::config, &conf)
//             .expect("Could not parse Logstash configuration file.")
//             .next()
//             .unwrap();

//         Pipeline::from_pair(conf)

//     }
    
//     fn from_pair(conf: Pair<Rule>) -> Self {

//         conf.into_inner()
//             .fold(Pipeline::new(), |mut acc, config_block| {

//                 let mut config_type = "";
                
//                 let mut config_block = config_block.into_inner()
//                     .fold(Pipeline::new(), |mut acc, config_type_plugin| {

//                         // Pest Pair is either "config_type" or "plugin"
//                         match config_type_plugin.as_rule() {
//                             Rule::config_type => {
//                                 config_type = config_type_plugin.as_str();
//                             },
//                             Rule::plugin => {

//                                 let from_pair = plugins::from_pair;
                                
//                                 match config_type {
//                                     "input" => {
//                                         acc.input.push(
//                                             from_pair(config_type, config_type_plugin));
//                                     },
//                                     "filter" => {
//                                         acc.filter.push(
//                                             from_pair(config_type, config_type_plugin));
//                                     },
//                                     "output" => {
//                                         acc.output.push(
//                                             from_pair(config_type, config_type_plugin));
//                                     },
//                                     _ => ()
//                                 }
                                
//                             },
//                             _ => ()
//                         };

//                         acc
                            
//                     });

//                 acc.input.append(&mut config_block.input);
//                 acc.filter.append(&mut config_block.filter);
//                 acc.output.append(&mut config_block.output);
//                 acc
                    
//             })
        
//     }
    
// }


// #[cfg(test)]
// mod tests {

//     #[test]
//     fn input_stdin() {
//         crate::Pipeline::from_file(std::path::Path::new("./example_configs/input/stdin.conf"));
//     }

//     #[test]
//     fn full_one() {
//         crate::Pipeline::from_file(std::path::Path::new("./example_configs/full.conf"));
//     }
    
// }
