/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-outputs-stdout.html

use futures::{Async, Poll, Stream};
use serde_json::{json, value::Value};
use crossbeam_channel::Receiver;

impl Stream for Stdout {

    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(receiver) = &self._receiver {
            if let Some(message) = receiver.recv().ok() {
                println!("{:#}", message);
                Ok(Async::Ready(Some(message)))
            } else {
                Ok(Async::Ready(None))
            }
        } else {
            panic!("kjhsdkfjhskjhdsf")
        }
    }
}

#[derive(Debug)]
pub struct Stdout {
    codec: Option<&'static str>,
    enable_metric: Option<bool>,
    id: Option<&'static str>,
    pub _receiver: Option<Receiver<Value>>
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
            _receiver: None
        }
    }
}
