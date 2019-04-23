/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-outputs-stdout.html

use serde_json::{json, value::Value};

impl<'a> Stdout<'a> {
    pub fn process(&self, message: &Value) -> Result<Value, ()> {
        println!("{:#}", message);
        let status = json!({ "status": "success" });
        Ok(status)
    }
}

#[derive(Debug)]
pub struct Stdout<'a> {
    codec: Option<&'a str>,
    enable_metric: Option<bool>,
    id: Option<&'a str>,
}

impl<'a> Stdout<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl<'a> Default for Stdout<'a> {
    fn default() -> Self {
        Self {            
            codec: None,
            enable_metric: None,
            id: None,
        }
    }
}
