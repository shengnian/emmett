/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-filters-date.html

use serde_json::Value;

impl DateFilter {
    pub fn process(&self, message: Value) -> Value {
        // if let Some(m) = message.get("date") {
        //     m.to_owned()
        // } else {
        //     serde_json::json!({"no_date": "nothing here"})
        // }
        message
    }
}

#[derive(Debug)]
pub struct DateFilter {
    locale: Option<String>,
    r#match: Option<Vec<String>>,
    tag_on_failure: Option<Vec<String>>,
    target: Option<String>,
    timezone: Option<String>
}

impl Default for DateFilter {
    fn default() -> Self {
        Self {
            locale: None,
            r#match: None,
            tag_on_failure: None,
            target: None,
            timezone: None
        }
    }
}

impl DateFilter {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
