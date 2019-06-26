// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-generator.html
use futures::{sync::mpsc::UnboundedSender, try_ready, Async, Poll, Stream};
use serde_json::{json, value::Value};
use std::convert::TryFrom;
use std::thread::sleep;
use std::time::Duration;
use tokio::timer::Interval;

impl Stream for Generator {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        try_ready!(self
            ._interval
            .poll()
            .map_err(|e| panic!("Generator timer failed: {:#?}", e)));

        let message = json!({
            "ip": "108.55.13.247",
            "jsonString": "{\n  \"userId\": 1,\n  \"id\": 1,\n  \"title\": \"delectus aut autem\",\n  \"completed\": false\n}"
        });

        Ok(Async::Ready(Some(message)))
    }
}

#[derive(Debug)]
/// Generate random log events.
/// The general intention of this is to test performance of plugins.
/// An event is generated first
pub struct Generator {
    /// Set how many messages should be generated.
    /// The default, 0, means generate an unlimited number of events.
    pub count: Option<u64>,

    /// The lines to emit, in order. This option cannot be used with the message setting.
    ///
    /// Example:
    ///
    /// ```
    /// input {
    ///   generator {
    ///     lines => [
    ///       "line 1",
    ///       "line 2",
    ///       "line 3"
    ///     ]
    ///     # Emit all lines 3 times.
    ///     count => 3
    ///   }
    /// }
    /// ```
    ///
    /// The above will emit line 1 then line 2 then line, then line 1, etc…
    pub lines: Option<Vec<String>>,
    pub message: Option<String>,
    pub threads: Option<u32>,
    pub _sender: Option<UnboundedSender<Value>>,
    _interval: Interval,
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
            count: Some(0),
            lines: None,
            message: Some("Hello world!".to_string()),
            threads: Some(1),
            _sender: None,
            _interval: Interval::new_interval(Duration::from_millis(1500)),
        }
    }
}

impl TryFrom<&toml::Value> for Generator {
    type Error = ();

    fn try_from(toml: &toml::Value) -> Result<Self, Self::Error> {
        let mut generator = Generator {
            ..Default::default()
        };

        if let Some(count) = toml.get("count") {
            let count = count
                .as_integer()
                .expect("Couldn't parse Generator count field as integer.");
            generator.count = Some(count as u64);
        }

        if let Some(lines) = toml.get("lines") {
            let lines = lines
                .as_array()
                .expect("Couldn't parse Generator message field as array.")
                .iter()
                .map(|x| x.as_str().unwrap().to_owned())
                .collect();

            generator.lines = Some(lines);
        }

        if let Some(message) = toml.get("message") {
            let message = message
                .as_str()
                .expect("Couldn't parse Generator message field as string.");
            generator.message = Some(message.to_owned());
        }

        if let Some(threads) = toml.get("threads") {
            let threads = threads
                .as_integer()
                .expect("Couldn't parse Generator threads field as integer.");
            generator.threads = Some(threads as u32);
        }

        Ok(generator)
    }
}
