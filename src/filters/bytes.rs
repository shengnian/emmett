use serde_json::Value;
/// Specifiction: https://www.elastic.co/guide/en/logstash/current/plugins-filters-bytes.html
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub struct BytesFilter<'a> {
    source: &'a str,
    target: Option<&'a str>,
    conversion_method: &'a str,
    separator: &'a str, // docs seem to have a typo
    pub _receiver: Option<Receiver<Value>>,
    pub _sender: Option<Sender<Value>>,
}

impl<'a> BytesFilter<'a> {
    pub fn new() -> Self {
        Self {
            source: "message",
            target: None,
            conversion_method: "binary",
            separator: ".",
            _receiver: None,
            _sender: None,
        }
    }
}
