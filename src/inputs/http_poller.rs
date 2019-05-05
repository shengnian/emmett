/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-http_poller.html
use futures::{stream::iter_ok, sync::mpsc::Sender, try_ready, Async, Poll, Stream};
use reqwest::{Certificate, Client, Proxy, RedirectPolicy};
use serde_json::value::Value;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;
use tokio::timer::Interval;

impl<'a> Stream for HttpPoller<'a> {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut client = Client::builder();

        // automatic_retries
        // not implemented in Reqwest yet

        // cacert
        if let Some(cacert_path) = self.cacert {
            let mut buf = Vec::new();
            File::open(cacert_path)
                .expect("Couldn't find CA file.")
                .read_to_end(&mut buf)
                .expect("Couldn't read CA file.");

            let cert =
                Certificate::from_der(&buf).expect("Certificate cannot be created from file.");

            client = client.add_root_certificate(cert);
        }

        // client_cert

        // client_key

        // connect_timeout
        if let Some(timeout) = self.connect_timeout {
            client = client.timeout(timeout);
        }

        // cookies
        client = client.cookie_store(self.cookies);

        // follow_redirects
        if self.follow_redirects == Some(true) {
            client = client.redirect(RedirectPolicy::default());
        }

        // keepalive

        // keystore and keystore_password

        // keystore_type

        // pool_max
        client = client.max_idle_per_host(self.pool_max);

        // pool_max_per_route
        // careful about this and pool_max

        // proxy
        if let Some(proxy) = self.proxy.to_owned() {
            client = client.proxy(proxy);
        }

        // request_timeout
        if let Some(timeout) = self.request_timeout {
            client = client.timeout(Duration::from_secs(timeout));
        }

        // retry_non_idempotent

        let client = client.build().expect("Couldn't build Reqwest client.");

        // schedule
        try_ready!(self.schedule.poll().map_err(|_| ()));

        // urls
        let mut response_stream = iter_ok(self.urls.to_owned()).and_then(|uri| {
            let url = url::Url::parse(uri).unwrap();
            let mut req = client.request(http::Method::GET, url);

            // user and password
            if let Some(user) = self.user {
                req = req.basic_auth::<&str, &str>(user, self.password);
            }

            let res = req
                .send()
                .expect("Couldn't send HttpPoller request.")
                .json()
                .expect("Couldn't parse HttpPoller response as JSON.");

            // metadata_target

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
    connect_timeout: Option<Duration>,
    cookies: bool,
    follow_redirects: Option<bool>,
    keepalive: Option<bool>,
    keystore: Option<&'a Path>,
    keystore_password: Option<&'a str>,
    keystore_type: Option<&'a str>,
    metadata_target: &'a str,
    pool_max: usize,
    pool_max_per_route: Option<u64>,
    proxy: Option<Proxy>,
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
            connect_timeout: Some(Duration::from_secs(10)),
            cookies: true,
            follow_redirects: Some(true),
            keepalive: Some(true),
            keystore: None,
            keystore_password: None,
            keystore_type: Some("JKS"),
            metadata_target: "@metadata",
            pool_max: 50,
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
            schedule: Interval::new_interval(Duration::from_millis(schedule)),
            urls,
            ..Default::default()
        }
    }
}
