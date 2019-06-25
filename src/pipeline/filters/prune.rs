/// Specifiction: https://www.elastic.co/guide/en/logstash/current/plugins-filters-prune.html
use serde_json::Value;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
#[allow(unused)]
pub struct PruneFilter {
    blacklist_names: Option<Vec<String>>,
    blacklist_values: Option<HashMap<String, String>>,
    interpolate: bool,
    whitelist_names: Option<Vec<String>>,
    whitelist_values: Option<HashMap<String, String>>,
    pub _receiver: Option<Receiver<Value>>,
    pub _sender: Option<Sender<Value>>,
}

impl Default for PruneFilter {
    fn default() -> Self {
        Self {
            blacklist_names: Some(vec!["%{[^}]+}".to_string()]),
            blacklist_values: None,
            interpolate: false,
            whitelist_names: None,
            whitelist_values: None,
            _receiver: None,
            _sender: None,
        }
    }
}
