use log::debug;
use env_logger::{Builder, Env};
use structopt::{self, StructOpt};
use pest_derive::Parser;
use futures::{stream::Stream, future::lazy, sync::mpsc, Future, sink::Sink};

pub mod plugins;
use plugins::{input, filter, output, input::Input, filter::Filter, output::Output};

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
        Builder::from_env(Env::default().default_filter_or("debug")).init();
    }

    let (input_sender, filter_receiver) = mpsc::channel(1_024);

    // create some input instances
    // let generator = Input::Generator(input::Generator::new(), input_sender.clone());
    // let generator_2 = Input::Generator(input::Generator::new(), input_sender.clone());

    let poller = Input::HttpPoller(
        input::HttpPoller::new(5000, vec!["https://jsonplaceholder.typicode.com/todos/1"]),
        input_sender.clone()
    );
    
    let exec = Input::Exec(input::Exec::new("ls"), input_sender.clone());

    // create some filters
    let geoip = Filter::Geoip(filter::GeoipFilter::new("ip"));

    // create some outputs
    let stdout = Output::Stdout(output::Stdout::new());

    // config blocks
    let inputs: Vec<Input> = vec![poller];
    let filters: Vec<Filter> = vec![geoip];
    // let outputs: Vec<Output> = vec![stdout];
    
    tokio::run(lazy(move || {

        for input in inputs { tokio::spawn(input); }

        let (filter_sender, output_receiver) = mpsc::channel(1_024);

        // send every message through the filter pipeline
        let filter = filter_receiver.for_each(move |message| {

            let message = filters.iter()
                .fold(message, |acc, x| x.process(acc));
            
            filter_sender.clone()
                .send(message)
                .poll()
                .unwrap();
            
            Ok(())
                
        });

        tokio::spawn(filter);

        // send every message to every output
        let output = output_receiver.for_each(|message| {
            
            let stdout = Output::Stdout(output::Stdout::new());
            stdout.process(&message).unwrap();
            
            debug!("{}", message);

            Ok(())
                
        });

        tokio::spawn(output);

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
