/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-exec.html

use std::time::Duration;
use futures::{Stream, Poll, Async, sync::mpsc::Sender};
use std::thread::sleep;
use serde_json::{json, value::Value};
use std::process::Command;

impl<'a> Stream for Exec<'a> {

    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        
        sleep(Duration::from_secs(self.interval.unwrap()));

        let message = Command::new(self.command)
            .output()
            .expect("Couldn't get Exec command output.");

        let message = String::from_utf8(message.stdout).unwrap();
        let message = json!({ "message": message });
        
        Ok(Async::Ready(Some(message)))
            
    }
    
}
    
#[derive(Debug)]
pub struct Exec<'a> {
    command: &'a str,
    interval: Option<u64>,
    schedule: Option<&'a str>,
    pub _sender: Option<Sender<Value>>
}

impl<'a> Default for Exec<'a> {
    fn default() -> Self {
        Self {
            command: "",
            interval: Some(5),
            schedule: Some("test"),
            _sender: None
        }
    }
}

impl<'a> Exec<'a> {
    pub fn new(command: &'a str) -> Self {
        Self {
            command,
            ..Default::default()
        }
    }        
}
