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

        if let Some(ref mut receiver) = &mut self._receiver {

            let mut process = receiver.by_ref().map(|mut input_message| {

                let json_string = input_message.get(source)
                    .unwrap()
                    .as_str()
                    .unwrap();

                let message: Value = serde_json::from_str(json_string).unwrap();

                message
                    
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

#[derive(Debug)]
pub struct JsonFilter<'a> {
    skip_on_invalid_json: Option<bool>,
    source: &'a str,
    tag_on_failure: Option<Vec<&'a str>>,
    target: Option<&'a str>,
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
