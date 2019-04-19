/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-http_poller.html

use std::time::Duration;
use futures::{future::lazy, stream::iter_ok, Stream, Poll, Async, try_ready};
use std::thread::sleep;
use reqwest::get;

impl Stream for HttpPollerInput {

    type Item = String;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        
        sleep(Duration::from_millis(self.schedule));

        let mut uri_stream = iter_ok(self.urls.to_owned())
            .and_then(|uri| {
                                
                let res = get(&uri)
                    .unwrap()
                    .text()
                    .unwrap();

                Ok(res)

            });

        let res = try_ready!(uri_stream.poll());        

        Ok(Async::Ready(res))
            
    }
    
}
    
#[derive(Debug)]
pub struct HttpPollerInput {
    user: Option<String>,
    password: Option<String>,
    automatic_retries: Option<u64>,
    cacert: Option<String>,
    client_cert: Option<String>,
    client_key: Option<String>,
    connect_timeout: Option<u64>,
    cookies: Option<bool>,
    follow_redirects: Option<bool>,
    keystore: Option<String>,
    keystore_password: Option<String>,
    keystore_type: Option<String>,
    metadata_target: Option<String>,
    pool_max: Option<u64>,
    pool_max_per_route: Option<u64>,
    proxy: Option<String>,
    request_timeout: Option<u64>,
    retry_non_idempotent: Option<bool>,
    schedule: u64,
    socket_timeout: Option<u64>,
    target: Option<String>,
    truststore: Option<String>,
    truststore_password: Option<String>,
    truststore_type: Option<String>,
    urls: Vec<String>,
    validate_after_inactivity: Option<u64>
}

impl HttpPollerInput {
    pub fn new(schedule: u64, urls: Vec<String>) -> Self {
        Self {
            user: None,
            password: None,
            automatic_retries: None,
            cacert: None,
            client_cert: None,
            client_key: None,
            connect_timeout: None,
            cookies: None,
            follow_redirects: None,
            keystore: None,
            keystore_password: None,
            keystore_type: None,
            metadata_target: None,
            pool_max: None,
            pool_max_per_route: None,
            proxy: None,
            request_timeout: None,
            retry_non_idempotent: None,
            schedule,
            socket_timeout: None,
            target: None,
            truststore: None,
            truststore_password: None,
            truststore_type: None,
            urls,
            validate_after_inactivity: None,
        }
    }        
}
