#![allow(unused)]

// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-cloudwatch.html

#[derive(Debug)]
pub struct Cloudwatch {
    pub access_key_id: Option<&'static str>,
    pub aws_credentials_file: Option<&'static str>,
    pub combined: Option<bool>,
    pub endpoint: Option<&'static str>,
    pub filters: Option<Vec<&'static str>>,
    pub interval: Option<u64>,
    pub metrics: Option<Vec<&'static str>>,
    pub namespace: Option<&'static str>,
    pub period: Option<u64>,
    pub proxy_uri: Option<&'static str>,
    pub region: Option<&'static str>,
    pub role_arn: Option<&'static str>,
    pub role_session_name: Option<&'static str>,
    pub secret_access_key: Option<&'static str>,
    pub session_token: Option<&'static str>,
    pub statistics: Option<Vec<&'static str>>,
    pub use_ssl: Option<bool>,
}

impl Default for Cloudwatch {
    fn default() -> Self {
        Self {
            access_key_id: None,
            aws_credentials_file: None,
            combined: Some(false),
            endpoint: None,
            filters: None,
            interval: Some(900),
            metrics: Some(vec![
                "CPUUtilization",
                "DiskReadOps",
                "DiskWriteOps",
                "NetworkIn",
                "NetworkOut",
            ]),
            namespace: Some("AWS/EC2"),
            period: Some(300),
            proxy_uri: None,
            region: Some("us-east-1"),
            role_arn: None,
            role_session_name: Some("logstash"),
            secret_access_key: None,
            session_token: None,
            statistics: Some(vec!["SampleCount", "Average", "Minimum", "Maximum", "Sum"]),
            use_ssl: Some(true),
        }
    }
}
