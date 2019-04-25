/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-outputs-stdout.html

use futures::{Async, Poll, Future};
use serde_json::{json, value::Value};

impl<'a> Future for Stdout<'a> {

    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {

        let response = json!({ "status": "ok" });
        Ok(Async::Ready(response))
        
    }
}

// impl<'a> Stdout<'a> {
//     pub fn process(&self, message: &Value) -> StdoutMessage {
//         StdoutMessage(&self, message)
//     }
// }

// pub struct StdoutMessage<'a>(&'a Stdout<'a>, Value);

// impl<'a> Future for StdoutMessage<'a> {

//     type Item = Value;
//     type Error = ();
    
//     fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
//         Ok(Async::Ready(self.1.to_owned()))
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
pub struct Stdout<'a> {
    codec: Option<&'a str>,
    enable_metric: Option<bool>,
    id: Option<&'a str>
}

impl<'a> Stdout<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl<'a> Default for Stdout<'a> {
    fn default() -> Self {
        Self {
            codec: None,
            enable_metric: None,
            id: None
        }
    }
}
