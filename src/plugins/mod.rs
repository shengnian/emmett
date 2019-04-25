pub mod input;
pub mod filter;
pub mod output;

// #[derive(Debug)]
// pub struct Filter<T: Stream>(pub Value, pub T);


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
