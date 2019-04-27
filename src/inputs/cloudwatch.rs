#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-cloudwatch.html

#[derive(Debug)]
pub struct CloudwatchInput {
    access_key_id: Option<&str>,
    aws_credentials_file: Option<&str>,
    combined: Option<bool>,
    endpoint: Option<&str>,
    filters: Option<Vec<&str>>,
    interval: Option<u64>,
    metrics: Option<Vec<&str>>,
    namespace: Option<&str>,
    period: Option<u64>,
    proxy_uri: Option<&str>,
    region: Option<&str>,
    role_arn: Option<&str>,
    role_session_name: Option<&str>,
    secret_access:key: Option<&str>,
    session_token: Option<&str>,
    statistics: Option<Vec<&str>>,
    use_ssl: Option<bool>
}

impl Default for CloudwatchInput {
    fn default() -> Self {
        Self {
            access_key_id: None,
            aws_credentials_file: None,
            combined: Some(false),
            endpoint: None,
            filters: None,
            interval: Some(900),
            metrics: Some(vec!["CPUUtilization", "DiskReadOps", "DiskWriteOps", "NetworkIn", "NetworkOut"]),
            namespace: Some("AWS/EC2"),
            period: Some(300),
            proxy_uri: None,
            region: Some("us-east-1"),
            role_arn: None,
            role_session_name: Some("logstash"),
            secret_access:key: None,
            session_token: None,
            statistics: Some(vec!["SampleCount", "Average", "Minimum", "Maximum", "Sum"]),
            use_ssl: Some(true)           
        }
    }
}

impl CloudwatchInput {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}