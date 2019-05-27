/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-exec.html
use futures::{sync::mpsc::Sender, try_ready, Async, Future, Poll, Stream};
use serde_json::{json, value::Value};
use std::process::Command;
use std::time::Duration;
use tokio::timer::Interval;
use tokio_process::CommandExt;

impl Stream for Exec {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {

        // self.schedule
        std::thread::sleep(Duration::from_millis(1000));
        // try_ready!(self.interval.poll().map_err(|_| ()));

        let mut message = Command::new(&self.command)
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
pub struct Exec {
    command: String,
    interval: Interval,
    schedule: Option<String>,
    pub _sender: Option<Sender<Value>>,
}

impl Default for Exec {
    fn default() -> Self {
        Self {
            command: "".to_string(),
            interval: Interval::new_interval(Duration::from_secs(5)),
            schedule: Some("test".to_string()),
            _sender: None,
        }
    }
}

impl Exec {
    pub fn new(command: String) -> Self {
        Self {
            command,
            ..Default::default()
        }
    }
}
