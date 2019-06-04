#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-outputs-stdout.html
use crossbeam::Receiver;
use futures::{Async, Poll, Stream};
use serde_json::{json, value::Value};

impl Stream for Stdout {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {

        let receiver = self._receiver.clone()
            .expect("No receiver found for Stdout output plugin.");

        if let Ok(message) = receiver.recv() {
            println!("{:#}", message);
        };
        
        Ok(Async::Ready(None))

    }
}

#[derive(Debug)]
pub struct Stdout {
    codec: Option<String>,
    enable_metric: Option<bool>,
    id: Option<String>,
    pub _receiver: Option<Receiver<Value>>,
}

impl Stdout {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Default for Stdout {
    fn default() -> Self {
        Self {
            codec: None,
            enable_metric: None,
            id: None,
            _receiver: None,
        }
    }
}
