use serde_json::{json, value::Value};
/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-filters-json.html
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use futures::{
    sync::mpsc::{Receiver, Sender},
    try_ready, Async, Future, Poll, Sink, Stream,
};

impl<'a> Stream for Json<'a> {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let source = self.source;
        let skip_invalid = self.skip_on_invalid_json;
        let target = self.target;
        let tags = &self.tag_on_failure;

        if let Some(ref mut receiver) = &mut self._receiver {
            let mut process = receiver.by_ref().map(|mut input_message| {
                let json_string = input_message.get(source).unwrap().as_str().unwrap();

                if let Ok(json) = serde_json::from_str(json_string) {
                        input_message[target] = json;
                        input_message
                } else {
                    if skip_invalid {
                        input_message
                    } else {
                        // add tags
                        tag(&mut input_message, tags);
                        input_message
                    }
                }
            });

            if let Some(message) = try_ready!(process.poll()) {
                if let Some(sender) = self._sender.to_owned() {
                    let mut send = sender.send(message);
                    try_ready!(send.poll().map_err(|_| ()));
                }
            }

            Ok(Async::Ready(None))
        } else {
            panic!("No receiver found for Json.");
        }
    }
}

fn tag<'a>(message: &mut Value, tags: &Vec<&'a str>) {
    message["tags"] = json!(tags);
}

#[derive(Debug)]
pub struct Json<'a> {
    pub skip_on_invalid_json: bool,
    pub source: &'a str,
    pub tag_on_failure: Vec<&'a str>,
    pub target: &'a str,
    pub _receiver: Option<Receiver<Value>>,
    pub _sender: Option<Sender<Value>>,
}

impl<'a> Json<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            skip_on_invalid_json: false,
            source,
            tag_on_failure: vec!["_jsonparsefailure"],
            target: "message",
            _receiver: None,
            _sender: None,
        }
    }
}
