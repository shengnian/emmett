/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-generator.html

use std::time::Duration;
use futures::{Stream, Poll, Async};
use std::thread::sleep;
use serde_json::{json, value::Value};

impl<'a> Stream for Generator<'a> {

    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        sleep(Duration::from_millis(3000));
        let message = json!({ "ip": "8.8.8.8" });
        Ok(Async::Ready(Some(message)))
    }
    
}
    
#[derive(Debug)]
pub struct Generator<'a> {
    count: Option<u64>,
    lines: Option<Vec<&'a str>>,
    message: Option<&'a str>,
    threads: Option<u32>
}

impl<'a> Generator<'a> {
    pub fn new() -> Self {
        Self {
           ..Default::default()
        }
    }        
}

impl<'a> Default for Generator<'a> {
    fn default() -> Self {
        Self {
            count: Some(0),
            lines: None,
            message: Some("Hello world!"),
            threads: Some(1)
        }
    }        
}
