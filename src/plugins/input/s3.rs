/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-s3.html

use std::time::Duration;
use futures::{stream::iter_ok, Stream, Poll, Async, try_ready};
use std::thread::sleep;
use reqwest::{ClientBuilder, RedirectPolicy};
use serde_json::value::Value;
use std::collections::HashMap;
use std::path::Path;

use rusoto_credential::StaticProvider;
use rusoto_core::{Region, request::HttpClient};
use rusoto_s3::S3Client;


impl<'a> Stream for S3Input<'a> {

    type Item = String;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {

        let creds = StaticProvider::new("key".to_string(), "secret_key".to_string(), None, None);
        let client = S3Client::new_with(HttpClient::new().unwrap(), creds, Region::UsEast1);

        sleep(Duration::from_millis(1000));
        
        Ok(Async::Ready(Some("s3 message".to_string())))
            
    }
    
}
    
#[derive(Debug)]
pub struct S3Input<'a> {
    access_key_id: Option<&'a str>,
    additional_settings: Option<HashMap<&'a str, &'a str>>,
    aws_credentials_file: Option<&'a Path>,
    backup_add_prefix: Option<&'a str>,
    backup_to_bucket: Option<&'a str>,
    backup_to_dir: Option<&'a Path>,
    bucket: &'a str,
    delete: Option<bool>,
    endpoint: Option<&'a str>,
    exclude_pattern: Option<&'a str>,
    include_object_properties: Option<bool>,
    interval: Option<u64>,
    prefix: Option<&'a str>,
    proxy_uri: Option<&'a str>,
    region: Option<&'a str>,
    role_arn: Option<&'a str>,
    role_session_name: Option<&'a str>,
    secret_access_key: Option<&'a str>,
    session_token: Option<&'a str>,
    sincedb_path: Option<&'a Path>,
    temporary_directory: Option<&'a Path>,
    watch_for_new_files: Option<bool>
}

impl<'a> S3Input<'a> {
    pub fn new(bucket: &'a str) -> Self {
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
            region: Some("us-east-1"),
            role_arn: None,
            role_session_name: Some("logstash"),
            secret_access_key: None,
            session_token: None,
            sincedb_path: None,
            temporary_directory: Some(Path::new("/tmp/logstash")),
            watch_for_new_files: Some(true)
        }        
    }
}
