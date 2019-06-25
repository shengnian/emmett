use serde_json::Value;
/// Specifiction: https://www.elastic.co/guide/en/logstash/current/plugins-filters-bytes.html
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
#[allow(unused)]
pub struct BytesFilter {
    source: String,
    target: Option<String>,
    conversion_method: String,
    separator: String, // docs seem to have a typo
    pub _receiver: Option<Receiver<Value>>,
    pub _sender: Option<Sender<Value>>,
}

impl Default for BytesFilter {
    fn default() -> Self {
        Self {
            source: "message".to_string(),
            target: None,
            conversion_method: "binary".to_string(),
            separator: ".".to_string(),
            _receiver: None,
            _sender: None,
        }
    }
}
