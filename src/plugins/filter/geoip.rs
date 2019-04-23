/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-filters-geoip.html

use serde_json::{json, value::Value};
use std::path::Path;
use reqwest::{ Client, RedirectPolicy};

impl<'a> GeoipFilter<'a> {
    pub fn process(&self, message: Value) -> Value {
        
        if let Some(m) = message.get(self.source) {
            let value = ip_api(m.as_str().unwrap());
            json!({ self.target.unwrap(): value })
        } else {
            message
        }

    }
}

#[derive(Debug)]
pub struct GeoipFilter<'a> {
    cache_size: Option<u64>,
    database: Option<&'a Path>,
    default_database_type: Option<&'a str>,
    fields: Option<Vec<&'a str>>,
    source: &'a str,
    tag_on_failure: Option<Vec<&'a str>>,
    target: Option<&'a str>
}

impl<'a> GeoipFilter<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            cache_size: Some(1000),
            database: None,
            default_database_type: Some("City"),
            fields: None,
            source,
            tag_on_failure: Some(vec!["_geoip_lookup_failure"]),
            target: Some("geoip")
        }
    }        
}

fn ip_api(ip: &str) -> Value {
    
    let client = Client::builder()
        .redirect(RedirectPolicy::limited(10))
        .build()
        .unwrap();

    let uri = format!("http://ip-api.com/json/{}", ip);
    client.get(&uri)
        .send()
        .unwrap()
        .json()
        .unwrap()
}
