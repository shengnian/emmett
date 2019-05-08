/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-s3.html
use futures::{sync::mpsc::Sender, Async, Poll, Stream};
use serde_json::{json, value::Value};
use std::collections::HashMap;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use rusoto_core::{request::HttpClient, Region};
use rusoto_credential::StaticProvider;
use rusoto_s3::S3Client;

impl Stream for S3 {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let creds = StaticProvider::new("key".to_string(), "secret_key".to_string(), None, None);
        let _client = S3Client::new_with(HttpClient::new().unwrap(), creds, Region::UsEast1);

        sleep(Duration::from_millis(3000));

        let message = json!({ "ip": "108.55.13.247" });

        Ok(Async::Ready(Some(message)))
    }
}

#[derive(Debug)]
pub struct S3 {
    access_key_id: Option<String>,
    additional_settings: Option<HashMap<String, String>>,
    aws_credentials_file: Option<&'static Path>,
    backup_add_prefix: Option<String>,
    backup_to_bucket: Option<String>,
    backup_to_dir: Option<&'static Path>,
    bucket: String,
    delete: Option<bool>,
    endpoint: Option<String>,
    exclude_pattern: Option<String>,
    include_object_properties: Option<bool>,
    interval: Option<u64>,
    prefix: Option<String>,
    proxy_uri: Option<String>,
    region: Option<String>,
    role_arn: Option<String>,
    role_session_name: Option<String>,
    secret_access_key: Option<String>,
    session_token: Option<String>,
    sincedb_path: Option<&'static Path>,
    temporary_directory: Option<&'static Path>,
    watch_for_new_files: Option<bool>,
    pub _sender: Option<Sender<Value>>,
}

impl S3 {
    pub fn new(bucket: String) -> Self {
        Self {
            access_key_id: None,
            additional_settings: None,
            aws_credentials_file: None,
            backup_add_prefix: None,
            backup_to_bucket: None,
            backup_to_dir: None,
            bucket,
            delete: Some(false),
            endpoint: None,
            exclude_pattern: None,
            include_object_properties: Some(false),
            interval: Some(60),
            prefix: None,
            proxy_uri: None,
            region: Some("us-east-1".to_string()),
            role_arn: None,
            role_session_name: Some("logstash".to_string()),
            secret_access_key: None,
            session_token: None,
            sincedb_path: None,
            temporary_directory: Some(Path::new("/tmp/logstash")),
            watch_for_new_files: Some(true),
            _sender: None,
        }
    }
}
