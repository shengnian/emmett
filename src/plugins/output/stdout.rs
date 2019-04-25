/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-outputs-stdout.html

use futures::{Async, Poll, Stream};
use serde_json::{json, value::Value};
use futures::sync::mpsc::Receiver;

impl Stream for Stdout {

    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let response = json!({ "status": "ok" });
        Ok(Async::Ready(Some(response)))
    }
}

// impl<'a> Stdout<'a> {
//     pub fn process(&self, message: &Value) -> StdoutMessage {
//         StdoutMessage(&self, message)
//     }
// }

// impl<'a> Stdout<'a> {
//     pub fn process(&self, message: Value) -> Result<Value, ()> {
//         println!("{:#}", message);
//         let status = json!({ "status": "success" });
//         Ok(status)
//     }
// }

#[derive(Debug)]
pub struct Stdout {
    codec: Option<&'static str>,
    enable_metric: Option<bool>,
    id: Option<&'static str>,
    pub _receiver: Option<&'static Receiver<Value>>
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
