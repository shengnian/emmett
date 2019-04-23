/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-filters-geoip.html

use serde_json::{json, value::Value};
use std::path::Path;
use reqwest::{ Client, RedirectPolicy};

impl<'a> GeoipFilter<'a> {
    pub fn process(&self, message: Value) -> Value {
        
        if let Some(source) = message.get(self.source) {

            let source = source.as_str()
                .expect("Couldn't parse Geoip source as string.");

            let value = match ip_api(source) {
                Ok(v) => v,
                Err(e) => panic!("{}", e)
            };

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

fn ip_api(ip: &str) -> Result<Value, reqwest::Error> {
    
    let client = Client::builder()
        .redirect(RedirectPolicy::limited(10))
        .build()
        .expect("Couldn't build Reqwest client.");

    let uri = format!("http://ip-api.com/json/{}", ip);

    let res = client.get(&uri)
        .send();

    res?.json()
        
}
