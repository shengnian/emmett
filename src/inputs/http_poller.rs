use futures::{stream::iter_ok, sync::mpsc::Sender, try_ready, Async, Poll, Stream};
use reqwest::{ClientBuilder, RedirectPolicy};
use serde_json::value::Value;
use std::path::Path;
/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-http_poller.html
use std::time::Duration;
use tokio::timer::Interval;

impl<'a> Stream for HttpPoller<'a> {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut client = ClientBuilder::new();

        // self.follow_redirects
        if self.follow_redirects == Some(true) {
            client = client.redirect(RedirectPolicy::default());
        }

        // self.request_timeout
        if let Some(timeout) = self.request_timeout {
            client = client.timeout(Duration::from_secs(timeout));
        }

        // self.cookies
        if self.cookies == Some(true) {
            client = client.cookie_store(true);
        }

        let client = client.build().expect("Couldn't build Reqwest client.");

        // self.schedule
        try_ready!(self.schedule.poll().map_err(|_| ()));

        // self.urls
        let mut response_stream = iter_ok(self.urls.to_owned()).and_then(|uri| {
            let url = url::Url::parse(uri).unwrap();
            let mut req = client.request(http::Method::GET, url);

            // self.user and self.password
            if let Some(user) = self.user {
                req = req.basic_auth::<&str, &str>(user, self.password);
            }

            let res = req
                .send()
                .expect("Couldn't send HttpPoller request.")
                .json()
                .expect("Couldn't parse HttpPoller response as JSON.");

            Ok(res)
        });

        let message = try_ready!(response_stream.poll());

        Ok(Async::Ready(message))
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
    schedule: Interval,
    socket_timeout: Option<u64>,
    target: Option<&'a str>,
    truststore: Option<&'a Path>,
    truststore_password: Option<&'a str>,
    truststore_type: Option<&'a str>,
    urls: Vec<&'a str>,
    validate_after_inactivity: Option<u64>,
    pub _sender: Option<Sender<Value>>,
}

impl<'a> Default for HttpPoller<'a> {
    fn default() -> Self {
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
            schedule: Interval::new_interval(Duration::from_secs(5)),
            socket_timeout: Some(10),
            target: None,
            truststore: None,
            truststore_password: None,
            truststore_type: Some("JKS"),
            urls: vec![],
            validate_after_inactivity: Some(200),
            _sender: None,
        }
    }
}

impl<'a> HttpPoller<'a> {
    pub fn new(schedule: u64, urls: Vec<&'a str>) -> Self {
        Self {
            schedule: Interval::new_interval(Duration::from_secs(schedule)),
            urls,
            ..Default::default()
        }
    }
}
