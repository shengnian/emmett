// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-exec.html
use futures::{sync::mpsc::UnboundedSender, Async, Poll, Stream};
use serde_json::{json, value::Value};
use std::process::Command;
use std::time::Duration;
use tokio::timer::Interval;

impl Stream for Exec {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // self.schedule
        std::thread::sleep(Duration::from_millis(1000));
        // try_ready!(self.interval.poll().map_err(|_| ()));

        let message = Command::new(&self.command)
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
/// Periodically run a shell command and capture the whole output as an event.
pub struct Exec {
    pub command: String,
    pub interval: Interval,
    pub schedule: Option<String>,
    pub _sender: Option<UnboundedSender<Value>>,
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
