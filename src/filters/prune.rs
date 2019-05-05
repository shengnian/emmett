/// Specifiction: https://www.elastic.co/guide/en/logstash/current/plugins-filters-prune.html
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use serde_json::Value;

#[derive(Debug)]
pub struct PruneFilter<'a> {
    blacklist_names: Option<Vec<&'a str>>,
    blacklist_values: Option<HashMap<&'a str, &'a str>>,
    interpolate: bool,
    whitelist_names: Option<Vec<&'a str>>,
    whitelist_values: Option<HashMap<&'a str, &'a str>>,
    pub _receiver: Option<Receiver<Value>>,
    pub _sender: Option<Sender<Value>>,
}

impl<'a> PruneFilter<'a> {
    pub fn new() -> Self {
        Self {
            blacklist_names: Some(vec!["%{[^}]+}"]),
            blacklist_values: None,
            interpolate: false,
            whitelist_names: None,
            whitelist_values: None,
            _receiver: None,
            _sender: None
        }
    }
}
