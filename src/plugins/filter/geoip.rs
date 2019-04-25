/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-filters-geoip.html

use serde_json::{json, value::Value};
use std::path::Path;
use reqwest::{Client, RedirectPolicy};
use futures::{Poll, Async, Stream, sync::mpsc::{Receiver, Sender}};

impl<'a> Stream for GeoipFilter<'a> {

    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {

        if let Some(ref receiver) = self._receiver {
            receiver.for_each(|input_message| {

                let source = input_message.get(self.source)
                    .unwrap()
                    .as_str()
                    .expect("Couldn't parse Geoip source as string.");

                let value = match ip_api(source) {
                    Ok(v) => v,
                    Err(e) => panic!("{}", e)
                };

                let _output_message = json!({ self.target.unwrap(): value });

                Ok(())
                    
            });

            // tokio::spawn(queue);

            Ok(Async::Ready(Some(json!({"message": "success!"}))))            

        } else {
            panic!("ljkhsdkjhsdff");
        }
    }
    
}

// impl<'a> GeoipFilter<'a> {
//     pub fn process(&self, message: Value) -> Value {
        
//         if let Some(source) = message.get(self.source) {

//             let source = source.as_str()
//                 .expect("Couldn't parse Geoip source as string.");

//             let value = match ip_api(source) {
//                 Ok(v) => v,
//                 Err(e) => panic!("{}", e)
//             };

//             json!({ self.target.unwrap(): value })

//         } else {
//             message
//         }

//     }
// }

#[derive(Debug)]
pub struct GeoipFilter<'a> {
    cache_size: Option<u64>,
    database: Option<&'a Path>,
    default_database_type: Option<&'a str>,
    fields: Option<Vec<&'a str>>,
    source: &'a str,
    tag_on_failure: Option<Vec<&'a str>>,
    target: Option<&'a str>,
    pub _receiver: Option<Receiver<Value>>,
    pub _sender: Option<Sender<Value>>
}

impl<'a> GeoipFilter<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            ..Default::default()
        }
    }        
}

impl<'a> Default for GeoipFilter<'a> {
    fn default() -> Self {
        Self {
            cache_size: Some(1000),
            database: None,
            default_database_type: Some("City"),
            fields: None,
            source: "",
            tag_on_failure: Some(vec!["_geoip_lookup_failure"]),
            target: Some("geoip"),
            _receiver: None,
            _sender: None
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
