/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-filters-geoip.html
use reqwest::{Client, RedirectPolicy};
use serde_json::{json, value::Value};
use std::path::Path;

#[allow(unused)]
impl<'a> Geoip<'a> {
    pub fn process(self, input: Value) -> Result<Value, ()> {
        if let Some(source) = input.get(&self.source) {
            let source = source
                .as_str()
                .expect("Couldn't parse Geoip source as string.");

            let value = match ip_api(source) {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            };

            let output_message = json!({ self.target: value });
            Ok(output_message)
        } else {
            Ok(input)
        }
    }
}

#[derive(Debug)]

#[allow(unused)]
pub struct Geoip<'a> {
    cache_size: Option<u64>,
    database: Option<&'a Path>,
    default_database_type: Option<String>,
    fields: Option<Vec<String>>,
    source: String,
    tag_on_failure: Option<Vec<String>>,
    target: String,
}

impl<'a> Default for Geoip<'a> {
    fn default() -> Self {
        Self {
            cache_size: Some(1000),
            database: None,
            default_database_type: Some("City".to_string()),
            fields: None,
            source: "".to_string(),
            tag_on_failure: Some(vec!["_geoip_lookup_failure".to_string()]),
            target: "geoip".to_string(),
        }
    }
}


#[allow(unused)]
fn ip_api(ip: &str) -> Result<Value, reqwest::Error> {
    let client = Client::builder()
        .redirect(RedirectPolicy::limited(10))
        .build()
        .expect("Couldn't build Reqwest client.");

    let uri = format!("http://ip-api.com/json/{}", ip);

    let res = client.get(&uri).send();

    res?.json()
}
