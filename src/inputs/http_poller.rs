// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-http_poller.html
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
/// This Logstash input plugin allows you to call an HTTP API, decode the output of it into event(s), and send them on their merry way.
pub struct HttpPoller {
    /// Username to use with HTTP authentication for ALL requests. Note that you can also set this per-URL. If you set this you must also set the password option.
    pub user: Option<String>,

    /// Password to be used in conjunction with the username for HTTP authentication.
    pub password: Option<String>,

    /// How many times should the client retry a failing URL. We highly recommend NOT setting this value to zero if keepalive is enabled. Some servers incorrectly end keepalives early requiring a retry! Note: if retry_non_idempotent is set only GET, HEAD, PUT, DELETE, OPTIONS, and TRACE requests will be retried.
    pub automatic_retries: Option<u64>,

    /// If you need to use a custom X.509 CA (.pem certs) specify the path to that here
    pub cacert: Option<&'static Path>,

    /// If you’d like to use a client certificate (note, most people don’t want this) set the path to the x509 cert here.
    pub client_cert: Option<&'static Path>,

    /// If you’re using a client certificate specify the path to the encryption key here.
    pub client_key: Option<&'static Path>,

    /// Timeout (in seconds) to wait for a connection to be established. Default is 10s.
    pub connect_timeout: Option<Duration>,

    /// Enable cookie support. With this enabled the client will persist cookies across requests as a normal web browser would. Enabled by default.
    pub cookies: bool,

    /// Should redirects be followed?
    pub follow_redirects: Option<bool>,

    /// Turn this on to enable HTTP keepalive support. We highly recommend setting automatic_retries to at least one with this to fix interactions with broken keepalive implementations.
    pub keepalive: Option<bool>,

    /// If you need to use a custom keystore (.jks) specify that here. This does not work with .pem keys!
    pub keystore: Option<&'static Path>,

    /// Specify the keystore password here. Note, most .jks files created with keytool require a password!
    pub keystore_password: Option<String>,

    /// Specify the keystore type here. One of JKS or PKCS12.
    pub keystore_type: String,

    /// If you’d like to work with the request/response metadata. Set this value to the name of the field you’d like to store a nested hash of metadata.
    pub metadata_target: String,

    /// Max number of concurrent connections.
    pub pool_max: usize,

    /// Max number of concurrent connections to a single host.
    pub pool_max_per_route: Option<u64>,

    /// If you’d like to use an HTTP proxy.
    pub proxy: Option<Proxy>,

    /// Timeout (in seconds) for the entire request.
    pub request_timeout: Option<u64>,

    /// If automatic_retries is enabled this will cause non-idempotent HTTP verbs (such as POST) to be retried.
    pub retry_non_idempotent: Option<bool>,

    /// Schedule of when to periodically poll from the urls Format: A hash with + key: "cron" | "every" | "in" | "at" + value: string Examples: a) { "every" ⇒ "1h" } b) { "cron" ⇒ "* * * * * UTC" } See: rufus/scheduler for details about different schedule options and value string format.
    pub schedule: Interval,

    /// Timeout (in seconds) to wait for data on the socket.
    pub socket_timeout: Option<u64>,

    /// Define the target field for placing the received data. If this setting is omitted, the data will be stored at the root (top level) of the event.
    pub target: Option<String>,

    /// If you need to use a custom truststore (.jks) specify that here. This does not work with .pem certs!
    pub truststore: Option<&'static Path>,

    /// Specify the truststore password here. Note, most .jks files created with keytool require a password!
    pub truststore_password: Option<String>,

    /// Specify the truststore type here. One of JKS or PKCS12. Default is JKS
    pub truststore_type: String,

    /// A Hash of urls in this format : "name" => "url". The name and the url will be passed in the outputed event.
    pub urls: Vec<Url>,

    /// How long to wait before checking for a stale connection to determine if a keepalive request is needed. Consider setting this value lower than the default, possibly to 0, if you get connection errors regularly.
/// This client is based on Apache Commons' HTTP implementation. Here’s how the Apache Commons documentation describes this option: "Defines period of inactivity in milliseconds after which persistent connections must be re-validated prior to being leased to the consumer. Non-positive value passed to this method disables connection validation. This check helps detect connections that have become stale (half-closed) while kept inactive in the pool."
    pub validate_after_inactivity: Option<u64>,
    
    pub _client: Option<Client>,
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
