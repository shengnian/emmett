#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-filters-json.html

use std::path::Path;
use serde_json::{json, value::Value};
use std::thread::sleep;
use std::time::Duration;

use futures::{
    sync::mpsc::{Receiver, Sender},
    try_ready, Async, Future, Poll, Sink, Stream,
};

impl<'a> Stream for JsonFilter<'a> {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {

        let source = self.source;
        let skip_invalid = self.skip_on_invalid_json;
        let target = self.target;
        let tags = &self.tag_on_failure;
        
        if let Some(ref mut receiver) = &mut self._receiver {

            let mut process = receiver.by_ref().map(|mut input_message| {

                let json_string = input_message.get(source)
                    .unwrap()
                    .as_str()
                    .unwrap();

                if let Ok(json) = serde_json::from_str(json_string) {

                    if let Some(t) = target {
                        input_message[t] = json;
                        input_message
                    } else {
                        json
                    }
                    
                } else {
                    if skip_invalid == Some(true) {
                        input_message
                    } else {
                        // add tag
                        if let Some(tags) = &tags {
                            tag(&mut input_message, tags);
                        }
                        input_message
                    }
                }
                
                    
            });

            if let Some(message) = try_ready!(process.poll()) {
                if let Some(sender) = self._sender.to_owned() {
                    let mut send = sender.send(message.clone());
                    try_ready!(send.poll().map_err(|_| ()));
                }
            }

            Ok(Async::Ready(None))
                
        } else {
            panic!("No receiver found for JsonFilter.");
        }
        
    }    
}

fn tag<'a>(message: &mut Value, tags: &Vec<&'a str>) {
    message["tags"] = json!(tags);
}

#[derive(Debug)]
pub struct JsonFilter<'a> {
    pub skip_on_invalid_json: Option<bool>,
    pub source: &'a str,
    pub tag_on_failure: Option<Vec<&'a str>>,
    pub target: Option<&'a str>,
    pub _receiver: Option<Receiver<Value>>,
    pub _sender: Option<Sender<Value>>,
}

impl<'a> JsonFilter<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            skip_on_invalid_json: Some(false),
            source,
            tag_on_failure: Some(vec!["_jsonparsefailure"]),
            target: None,
            _receiver: None,
            _sender: None
        }
    }
}
