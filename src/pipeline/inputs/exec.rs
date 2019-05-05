#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-exec.html
use futures::{sync::mpsc::Sender, try_ready, Async, Future, Poll, Stream};
use serde_json::{json, value::Value};
use std::process::Command;
use std::time::Duration;
use tokio::timer::Interval;
use tokio_process::CommandExt;

impl<'a> Stream for Exec<'a> {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // self.schedule
        std::thread::sleep(Duration::from_millis(1000));
        // try_ready!(self.interval.poll().map_err(|_| ()));

        let mut message = Command::new(self.command)
            .output()
            .expect("Couldn't get Exec command output.");

        // let mut message = Command::new(self.command)
        //     .output_async();

        // let message = try_ready!(message.poll().map_err(|_| ()));

        let message = String::from_utf8(message.stdout).unwrap();
        let message = json!({ "message": message });

        Ok(Async::Ready(Some(message)))
    }
}

#[derive(Debug)]
pub struct Exec<'a> {
    command: &'a str,
    interval: Interval,
    schedule: Option<&'a str>,
    pub _sender: Option<Sender<Value>>,
}

impl<'a> Default for Exec<'a> {
    fn default() -> Self {
        Self {
            command: "",
            interval: Interval::new_interval(Duration::from_secs(5)),
            schedule: Some("test"),
            _sender: None,
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
