use pest_derive::Parser;
use futures::{stream::Stream, future::lazy, sync::mpsc, Future, sink::Sink};

pub mod plugins;
use plugins::{input, filter, input::Input, filter::Filter};

#[derive(Parser)]
#[grammar = "logstash.pest"]
pub struct ConfigParser;

fn main() {

    let logo = r#"
(=^•ω•^=) Emmett v0.1.0
"#;

    println!("{}", logo);

    let (input_sender, filter_receiver) = mpsc::channel(1_024);

    // create some input instances
    let http_poller = input::HttpPollerInput::new(
        5000,
        vec!["http://date.jsontest.com/"]
    );
    let s3_poller = input::S3Input::new("test");

    // add inputs to enum variant wrappers
    let poller = Input::HttpPoller(http_poller, input_sender.clone());
    let s3_plugin = Input::S3(s3_poller, input_sender.clone());

    // create some filters
    let geoip = Filter::Geoip(filter::GeoipFilter::new("test"));
    let date = Filter::Date(filter::DateFilter::new());

    // config blocks
    let inputs = vec![poller, s3_plugin];
    let filters = vec![geoip, date];
    
    tokio::run(lazy(move || {

        for input in inputs { tokio::spawn(input); }

        let (filter_sender, output_receiver) = mpsc::channel(1_024);
        
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

        let output = output_receiver.for_each(|message| {
            dbg!(message);
            Ok(())
        });

        tokio::spawn(output);

        Ok(())
            
    }));
    
}

// fn test(input: &str) -> Box<dyn Run> {
//     if input == "one" {
//         Box::new(HttpPollerInput::new(
//             "test".to_string(),
//             vec!["http://httpbin.org/ip".to_string(), "http://google.com".to_string()]
//         ))
//     } else {
//         Box::new(plugins::StdinInput::new())
//     }
// }


// #[derive(Debug)]
// pub struct Pipeline {
//     pub input: Vec<Box<dyn plugins::Plugin>>,
//     pub filter: Vec<Box<dyn plugins::Plugin>>,
//     pub output: Vec<Box<dyn plugins::Plugin>>
// }

// impl Default for Pipeline {
//     fn default() -> Self {
//         Self {
//             input: Vec::new(),
//             filter: Vec::new(),
//             output: Vec::new()
//         }
//     }
// }

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
