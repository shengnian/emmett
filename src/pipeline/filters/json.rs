/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-filters-json.html
use serde_json::{json, value::Value};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::convert::TryFrom;

use futures::{
    sync::mpsc::{Receiver, Sender},
    try_ready, Async, Future, Poll, Sink, Stream,
};

impl Json {
    pub fn process(self, input: Value) -> impl Future<Item=Value, Error=()> {
        futures::future::lazy(move || {

            let mut input_mut = input.clone();

            // if field exists, get it; otherwise don't do anything
            if let Some(json_string) = input.get(self.source) {
                // if field is a string, parse it as a string; otherwise don't do anything
                if let Some(json_string) = json_string.as_str() {
                    // if field can be parsed into JSON, parse it; otherwise don't do anything
                    if let Ok(json) = serde_json::from_str(json_string) {
                        input_mut[self.target] = json;
                    }
                }
            }

            Ok(input_mut)
            
        })
    }
}

#[derive(Debug, Clone)]
pub struct Json {
    pub skip_on_invalid_json: bool,
    pub source: String,
    pub tag_on_failure: Vec<String>,
    pub target: String,
    pub _sender: Option<Sender<Value>>,
}

impl Default for Json {
    fn default() -> Self {
        Self {
            skip_on_invalid_json: false,
            source: String::new(),
            tag_on_failure: vec!["_jsonparsefailure".to_string()],
            target: "message".to_string(),
            _sender: None,
        }
    }
}


impl TryFrom<&toml::Value> for Json {
    type Error = ();
    
    fn try_from(toml: &toml::Value) -> Result<Self, Self::Error> {

        let mut json = Json {
            ..Default::default()
        };
        
        // if let Some(replace) = toml.get("replace") {
        //     let replace = replace.as_table()
        //         .expect("Couldn't parse Mutate replace as table.");
        //     mutate.replace = Some(replace.to_owned());
        // }
        
        Ok(json)
        
    }
}
