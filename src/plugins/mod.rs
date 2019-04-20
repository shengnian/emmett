pub mod input;
pub mod filter;
pub mod output;

use futures::{Future, Poll, Stream, try_ready};
use std::collections::HashMap;
    
pub struct Plugin<T: Stream>(pub T);

#[allow(unused)]
struct CommonOptions<'a> {
    add_field: Option<HashMap<&'a str, &'a str>>,
    codec: Option<&'a str>,
    enable_metric: Option<bool>,
    id: Option<&'a str>,
    tags: Option<Vec<&'a str>>,
    r#type: Option<&'a str>
}

impl<'a> Future for Plugin<input::HttpPollerInput<'a>> {

    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), Self::Error> {

        loop {
            if let Some(message) = try_ready!(self.0.poll()) {
                dbg!(message);
                // process message here
            }
        }

    }

}

impl<'a> Future for Plugin<input::S3Input<'a>> {

    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), Self::Error> {

        loop {
            if let Some(message) = try_ready!(self.0.poll()) {
                dbg!(message);
                // process message here
            }
        }

    }

}

// pub fn from_pair<T>(config_block: &str, plugin: Pair<Rule>) -> Box<dyn Plugin> {

//     let name = plugin.into_inner().next().unwrap().as_str();
//     let plugin_str = format!("{}_{}", config_block, name);
//     self::from_str(plugin_str)

// }

// pub fn from_str<T>(p: String) -> Box<dyn Plugin> {

//     match p.as_str() {
//         "input_stdin" => Box::new(StdinInput::new()),
//         // "input_http_poller" => Plugin::HttpPoller(HttpPollerInput::new("test".to_string(), Url::parse("https://goofle.com"))),
//         "filter_date" => Box::new(DateFilter::new()),
//         // "filter_grok" => Box::new(GrokFilter::new()),
//         // "output_elasticsearch" => Box::new(ElasticsearchOutput::new()),
//         // "output_stdout" => Box::new(StdoutOutput::new()),
//         x => panic!("{} is not a valid plugin.", x)
//     }
    
// }
