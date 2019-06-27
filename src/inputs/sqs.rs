#[derive(Debug)]
/// Pull events from an Amazon Web Services Simple Queue Service (SQS) queue.
pub struct Sqs {
    access_key_id: Option<String>,
    aws_credentials_file: Option<String>,
    endpoint: Option<String>,
    id_field: Option<String>,
    md5_field: Option<String>,
    polling_frequency: Option<u64>,
    proxy_uri: Option<String>,
    queue: Option<String>,
    region: Option<String>,
    role_arn: Option<String>,
    role_session_name: Option<String>,
    secret_access_key: Option<String>,
    sent_timestamp_field: Option<String>,
    session_token: Option<String>,
    threads: Option<u64>,
}

impl Sqs {
    pub fn new() -> Self {
        Self {
            access_key_id: None,
            aws_credentials_file: None,
            endpoint: None,
            id_field: None,
            md5_field: None,
            polling_frequency: Some(20),
            proxy_uri: None,
            queue: None,
            region: Some("us-east-1".to_string()),
            role_arn: None,
            role_session_name: Some("logstash".to_string()),
            secret_access_key: None,
            sent_timestamp_field: None,
            session_token: None,
            threads: Some(1),
        }
    }
}
