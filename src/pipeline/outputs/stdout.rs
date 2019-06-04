#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-outputs-stdout.html
use crossbeam::Receiver;
use futures::{Async, Poll, Stream};
use serde_json::{json, value::Value};

impl Stream for Stdout {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(receiver) = &self._receiver {
            if let Ok(message) = receiver.recv() {
                debug!("Stdout output plugin received a message.");
                println!("{:#}", message);
            }
            Ok(Async::Ready(None))
        } else {
            panic!("No receiver found for Stdout output.")
        }
    }
}

#[derive(Debug)]
pub struct Stdout {
    codec: Option<&'static str>,
    enable_metric: Option<bool>,
    id: Option<&'static str>,
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
