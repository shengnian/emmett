#![allow(unused)]

use futures::{
    sync::mpsc::{Receiver, Sender},
    try_ready, Async, Future, Poll, Sink, Stream,
};
/// Specifiction: https://www.elastic.co/guide/en/logstash/current/plugins-filters-fingerprint.html
use serde_json::Value;
use sha1::Sha1;

impl<'a> Stream for Fingerprint<'a> {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(ref mut receiver) = &mut self._receiver {
            let mut process = receiver.by_ref().map(|mut input_message| {
                input_message["fingerprint"] = fingerprint_sha1(&input_message);
                input_message
            });

            if let Some(message) = try_ready!(process.poll()) {
                if let Some(sender) = self._sender.to_owned() {
                    let mut send = sender.send(message);
                    try_ready!(send.poll().map_err(|_| ()));
                }
            }

            Ok(Async::Ready(None))
        } else {
            panic!("No receiver found for Fingerprint.");
        }
    }
}

fn fingerprint_sha1(message: &Value) -> Value {
    let ser = serde_json::to_string(message).unwrap();
    let mut sha1 = Sha1::new();
    sha1.update(ser.as_bytes());
    Value::String(sha1.digest().to_string())
}

#[derive(Debug)]
pub struct Fingerprint<'a> {
    base64encode: Option<bool>,
    concatenate_sources: Option<bool>,
    concatenate_all_fields: Option<bool>,
    key: Option<&'a str>,
    method: Option<&'a str>,
    source: Option<Vec<&'a str>>,
    target: Option<&'a str>,
    pub _receiver: Option<Receiver<Value>>,
    pub _sender: Option<Sender<Value>>,
}

impl<'a> Fingerprint<'a> {
    pub fn new() -> Self {
        Self {
            base64encode: Some(false),
            concatenate_sources: Some(false),
            concatenate_all_fields: Some(false),
            key: None,
            method: Some("SHA1"),
            source: Some(vec!["message"]),
            target: Some("fingerprint"),
            _receiver: None,
            _sender: None,
        }
    }
}
