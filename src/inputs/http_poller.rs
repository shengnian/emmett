/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-http_poller.html
use futures::{sync::mpsc::UnboundedSender, try_ready, Async, Poll, Stream};
use reqwest::{Certificate, Client, Proxy, RedirectPolicy};
use serde_json::value::Value;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;
use tokio::timer::Interval;
use url::Url;

impl Stream for HttpPoller {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // schedule
        try_ready!(self
            .schedule
            .poll()
            .map_err(|e| panic!("HttpPoller timer failed: {:#?}", e)));

        let client = self
            ._client
            .as_ref()
            .expect("Couldn't access http client for HttpPoller input.");

        // urls - only use first url for now
        let url = &self.urls[0];

        let mut req = client.request(http::Method::GET, url.to_owned());

        // user and password
        if let (Some(user), pass) = (self.user.to_owned(), self.password.to_owned()) {
            req = req.basic_auth(user, pass);
        }

        let res = req
            .send()
            .expect("Couldn't send HttpPoller input request.")
            .json()
            .expect("Couldn't parse HttpPoller input response as JSON.");

        // metadata_target

        Ok(Async::Ready(res))
    }
}

#[derive(Debug)]
pub struct HttpPoller {
    user: Option<String>,
    password: Option<String>,
    automatic_retries: Option<u64>,
    cacert: Option<&'static Path>,
    client_cert: Option<&'static Path>,
    client_key: Option<&'static Path>,
    connect_timeout: Option<Duration>,
    cookies: bool,
    follow_redirects: Option<bool>,
    keepalive: Option<bool>,
    keystore: Option<&'static Path>,
    keystore_password: Option<String>,
    keystore_type: String,
    metadata_target: String,
    pool_max: usize,
    pool_max_per_route: Option<u64>,
    proxy: Option<Proxy>,
    request_timeout: Option<u64>,
    retry_non_idempotent: Option<bool>,
    schedule: Interval,
    socket_timeout: Option<u64>,
    target: Option<String>,
    truststore: Option<&'static Path>,
    truststore_password: Option<String>,
    truststore_type: String,
    urls: Vec<Url>,
    validate_after_inactivity: Option<u64>,
    _client: Option<Client>,
    pub _sender: Option<UnboundedSender<Value>>,
}

impl Default for HttpPoller {
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
            keystore_type: "JKS".to_string(),
            metadata_target: "@metadata".to_string(),
            pool_max: 50,
            pool_max_per_route: Some(25),
            proxy: None,
            request_timeout: Some(60),
            retry_non_idempotent: Some(false),
            schedule: Interval::new_interval(Duration::from_millis(2000)),
            socket_timeout: Some(10),
            target: None,
            truststore: None,
            truststore_password: None,
            truststore_type: "JKS".to_string(),
            urls: Vec::new(),
            validate_after_inactivity: Some(200),
            _client: None,
            _sender: None,
        }
    }
}

impl TryFrom<&toml::Value> for HttpPoller {
    type Error = ();

    fn try_from(toml: &toml::Value) -> Result<Self, Self::Error> {
        let mut poller = HttpPoller {
            ..Default::default()
        };

        if let Some(urls) = toml.get("urls") {
            // if more than one
            if let Some(urls) = urls.as_array() {
                urls.iter().for_each(|url| {
                    if let Some(url) = url.as_table() {
                        for (_key, value) in url.iter() {
                            if let Some(url) = value.as_str() {
                                poller.urls.push(
                                    Url::parse(url)
                                        .expect("Can't parse HttpPoller input config URL."),
                                );
                            }
                            if let Some(url) = value.as_table() {
                                let url = url
                                    .get("url")
                                    .expect("Missing required URL field.")
                                    .as_str()
                                    .expect("Couldn't parse HttpPoller url as string.");
                                poller.urls.push(
                                    Url::parse(url)
                                        .expect("Can't parse HttpPoller input config URL."),
                                );
                            }
                        }
                    }
                });
            }
        }

        // build client

        let mut client = Client::builder();

        // automatic_retries
        // not implemented in Reqwest yet

        // cacert
        if let Some(cacert_path) = poller.cacert {
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
        if let Some(timeout) = poller.connect_timeout {
            client = client.timeout(timeout);
        }

        // cookies
        client = client.cookie_store(poller.cookies);

        // follow_redirects
        if poller.follow_redirects == Some(true) {
            client = client.redirect(RedirectPolicy::default());
        }

        // keepalive

        // keystore and keystore_password

        // keystore_type

        // pool_max
        client = client.max_idle_per_host(poller.pool_max);

        // pool_max_per_route
        // careful about this and pool_max

        // proxy
        if let Some(proxy) = poller.proxy.take() {
            client = client.proxy(proxy);
        }

        // request_timeout
        if let Some(timeout) = poller.request_timeout {
            client = client.timeout(Duration::from_secs(timeout));
        }

        // retry_non_idempotent

        let client = client.build().expect("Couldn't build Reqwest client.");
        poller._client = Some(client);

        Ok(poller)
    }
}
