#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-generator.html
use futures::{sync::mpsc::Sender, Async, Poll, Stream};
use serde_json::{json, value::Value};
use std::thread::sleep;
use std::time::Duration;
use std::convert::TryFrom;

impl Stream for Generator {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        sleep(Duration::from_millis(200));

        // let message = json!({
        //     "ip": "108.55.13.247",
        //     "jsonString": "{\n  \"userId\": 1,\n  \"id\": 1,\n  \"title\": \"delectus aut autem\",\n  \"completed\": false\n}"
        // });

        let message = json!({
            "message": self.message
        });
        
        Ok(Async::Ready(Some(message)))
    }
}

#[derive(Debug)]
pub struct Generator {
    count: u64,
    lines: Option<Vec<String>>,
    message: String,
    threads: u32,
    pub _sender: Option<Sender<Value>>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            count: 0,
            lines: None,
            message: "Hello world!".to_string(),
            threads: 1,
            _sender: None,
        }
    }
}

impl TryFrom<toml::Value> for Generator {
    type Error = ();
    
    fn try_from(toml: toml::Value) -> Result<Self, Self::Error> {

        let mut generator = Generator {
            ..Default::default()
        };
        
        if let Some(count) = toml.get("count") {
            let count = count.as_integer()
                .expect("Couldn't parse Generator count field as integer.");
            generator.count = count as u64;
        }

        if let Some(lines) = toml.get("lines") {
            let lines = lines.as_array()
                .expect("Couldn't parse Generator message field as array.")
                .into_iter()
                .map(|x| x.as_str().unwrap().to_owned())
                .collect();

            generator.lines = Some(lines);
        }

        if let Some(message) = toml.get("message") {
            let message = message.as_str()
                .expect("Couldn't parse Generator message field as string.");
            generator.message = message.to_owned();
        }
        
        if let Some(threads) = toml.get("threads") {
            let threads = threads.as_integer()
                .expect("Couldn't parse Generator threads field as integer.");
            generator.threads = threads as u32;
        }
            
        Ok(generator)
        
    }
}
