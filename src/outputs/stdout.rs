#![allow(unused)]

use super::Run;
/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-outputs-stdout.html
use futures::{Async, Poll, Stream};
use serde_json::{json, value::Value};

impl Run for Stdout {
    fn run(&self, input: Value) {
        println!("{:#}", input)
    }
}

#[derive(Debug)]
pub struct Stdout {
    pub codec: Option<String>,
    pub enable_metric: Option<bool>,
    pub id: Option<String>,
}

impl Default for Stdout {
    fn default() -> Self {
        Self {
            codec: None,
            enable_metric: None,
            id: None,
        }
    }
}
