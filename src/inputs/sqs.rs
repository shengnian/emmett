#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-sqs.html

use std::path::Path;

#[derive(Debug)]
pub struct SqsInput<'a> {
    access_key_id: Option<&'a str>,
    aws_credentials_file: Option<&'a str>,
    endpoint: Option<&'a str>,
    id_field: Option<&'a str>,
    md5_field: Option<&'a str>,
    polling_frequency: Option<u64>,
    proxy_uri: Option<&'a str>,
    queue: Option<&'a str>,
    region: Option<&'a str>,
    role_arn: Option<&'a str>,
    role_session_name: Option<&'a str>,
    secret_access_key: Option<&'a str>,
    sent_timestamp_field: Option<&'a str>,
    session_token: Option<&'a str>,
    threads: Option<u64>
}

impl<'a> SqsInput<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self {
            access_key_id: None,
            aws_credentials_file: None,
            endpoint: None,
            id_field: None,
            md5_field: None,
            polling_frequency: Some(20),
            proxy_uri: None,
            queue: None,
            region: Some("us-east-1"),
            role_arn: None,
            role_session_name: Some("logstash"),
            secret_access_key: None,
            sent_timestamp_field: None,
            session_token: None,
            threads: Some(1)
        }
    }
}
