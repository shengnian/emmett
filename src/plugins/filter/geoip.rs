/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-filters-geoip.html

use futures::{Future, Poll, Async};
use serde_json::value::Value;
use std::path::Path;

impl<'a> GeoipFilter<'a> {
    pub fn process(&self, message: Value) -> GeoipFilterOutput {
        GeoipFilterOutput(message)
    }
}

pub struct GeoipFilterOutput(Value);

impl Future for GeoipFilterOutput {

    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready(self.0.to_owned()))
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
