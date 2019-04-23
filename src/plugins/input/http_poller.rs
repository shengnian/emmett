/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-http_poller.html

use std::time::Duration;
use futures::{stream::iter_ok, Stream, Poll, Async, try_ready};
use std::thread::sleep;
use reqwest::{ClientBuilder, RedirectPolicy};
use serde_json::value::Value;
use std::path::Path;

impl<'a> Stream for HttpPoller<'a> {

    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {

        let client = ClientBuilder::new()
            .redirect(RedirectPolicy::default())
            .build()
            .expect("Couldn't build Reqwest client.");
        
        sleep(Duration::from_millis(self.schedule));

        let mut response_stream = iter_ok(self.urls.to_owned())
            .and_then(|uri| {

                let res = client.get(uri)
                    .send()
                    .unwrap()
                    .json()
                    .unwrap();

                Ok(res)
                    
                // if let Ok(mut res) = res {
                //     res.json().map_err(|_| ())
                // } else {
                //     Err(())
                // }

            });

        let res = try_ready!(response_stream.poll());        

        Ok(Async::Ready(res))
            
    }
    
}
    
#[derive(Debug)]
pub struct HttpPoller<'a> {
    user: Option<&'a str>,
    password: Option<&'a str>,
    automatic_retries: Option<u64>,
    cacert: Option<&'a Path>,
    client_cert: Option<&'a Path>,
    client_key: Option<&'a Path>,
    connect_timeout: Option<u64>,
    cookies: Option<bool>,
    follow_redirects: Option<bool>,
    keepalive: Option<bool>,
    keystore: Option<&'a Path>,
    keystore_password: Option<&'a str>,
    keystore_type: Option<&'a str>,
    metadata_target: Option<&'a str>,
    pool_max: Option<u64>,
    pool_max_per_route: Option<u64>,
    proxy: Option<&'a str>,
    request_timeout: Option<u64>,
    retry_non_idempotent: Option<bool>,
    schedule: u64,
    socket_timeout: Option<u64>,
    target: Option<&'a str>,
    truststore: Option<&'a Path>,
    truststore_password: Option<&'a str>,
    truststore_type: Option<&'a str>,
    urls: Vec<&'a str>,
    validate_after_inactivity: Option<u64>
}

impl<'a> HttpPoller<'a> {
    pub fn new(schedule: u64, urls: Vec<&'a str>) -> Self {
        Self {
            user: None,
            password: None,
            automatic_retries: Some(1),
            cacert: None,
            client_cert: None,
            client_key: None,
            connect_timeout: Some(10),
            cookies: Some(true),
            follow_redirects: Some(true),
            keepalive: Some(true),
            keystore: None,
            keystore_password: None,
            keystore_type: Some("JKS"),
            metadata_target: Some("@metadata"),
            pool_max: Some(50),
            pool_max_per_route: Some(25),
            proxy: None,
            request_timeout: Some(60),
            retry_non_idempotent: Some(false),
            schedule,
            socket_timeout: Some(10),
            target: None,
            truststore: None,
            truststore_password: None,
            truststore_type: Some("JKS"),
            urls,
            validate_after_inactivity: Some(200),
        }
    }        
}
