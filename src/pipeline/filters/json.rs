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
    pub fn process(self, input: Value) -> Result<Value, ()> {

        let mut input_mut = input.clone();

        // if field exists, get it; otherwise don't do anything
        if let Some(json_string) = input.get(self.source.unwrap()) {
            // if field is a string, parse it as a string; otherwise don't do anything
            if let Some(json_string) = json_string.as_str() {
                // try parsing field as JSON, otherwise don't do anything
                if let Ok(json) = serde_json::from_str(json_string) {
                    input_mut[self.target.unwrap()] = json;
                }
            }
        }

        Ok(input_mut)
            
    }
}

#[derive(Debug, Clone)]
pub struct Json {
    pub skip_on_invalid_json: bool,
    pub source: Option<String>,
    pub tag_on_failure: Vec<String>,
    pub target: Option<String>,
    pub _sender: Option<Sender<Value>>,
}

impl Default for Json {
    fn default() -> Self {
        Self {
            skip_on_invalid_json: false,
            source: None,
            tag_on_failure: vec!["_jsonparsefailure".to_string()],
            target: None,
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
        
        if let Some(source) = toml.get("source") {
            let source = source.as_str()
                .expect("Couldn't parse Json filter source as string.");
            json.source = Some(source.to_owned());
        }

        if let Some(target) = toml.get("target") {
            let target = target.as_str()
                .expect("Couldn't parse Json filter target as string.");
            json.target = Some(target.to_owned());
        }
        
        Ok(json)
        
    }
}
