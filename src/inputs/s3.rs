// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-s3.html
use futures::{sync::mpsc::UnboundedSender, Async, Poll, Stream};
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

/// Stream events from files from a S3 bucket.
/// Each line from each file generates an event. Files ending in .gz are handled as gzip’ed files.
/// Files that are archived to AWS Glacier will be skipped.
#[derive(Debug)]
pub struct S3 {
    /// This plugin uses the AWS SDK and supports several ways to get credentials, which will be tried in this order:

    /// 1. Static configuration, using access_key_id and secret_access_key params in logstash plugin config
    /// 2. External credentials file specified by aws_credentials_file
    /// 3. Environment variables AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY
    /// 4. Environment variables AMAZON_ACCESS_KEY_ID and AMAZON_SECRET_ACCESS_KEY
    /// 5. IAM Instance Profile (available when running inside EC2)
    pub access_key_id: Option<String>,

    /// Key-value pairs of settings and corresponding values used to parametrize the connection to s3. See full list in the AWS SDK documentation. Example:
    /// ```
    /// input {
    ///   s3 {
    ///     "access_key_id" => "1234"
    ///     "secret_access_key" => "secret"
    ///     "bucket" => "logstash-test"
    ///     "additional_settings" => {
    ///       "force_path_style" => true
    ///       "follow_redirects" => false
    ///     }
    ///   }
    /// }
    /// ```
    pub additional_settings: Option<HashMap<String, String>>,

    /// Path to YAML file containing a hash of AWS credentials. This file will only be loaded if access_key_id and secret_access_key aren’t set. The contents of the file should look like this:
    /// ```
    /// :access_key_id: "12345"
    /// :secret_access_key: "54321"
    /// ```
    pub aws_credentials_file: Option<&'static Path>,

    /// Append a prefix to the key (full path including file name in s3) after processing. If backing up to another (or the same) bucket, this effectively lets you choose a new folder to place the files in
    pub backup_add_prefix: Option<String>,

    /// Name of a S3 bucket to backup processed files to.
    pub backup_to_bucket: Option<String>,

    /// Path of a local directory to backup processed files to.
    pub backup_to_dir: Option<&'static Path>,

    /// The name of the S3 bucket.
    pub bucket: String,

    /// Whether to delete processed files from the original bucket.
    pub delete: Option<bool>,

    /// The endpoint to connect to. By default it is constructed using the value of region. This is useful when connecting to S3 compatible services, but beware that these aren’t guaranteed to work correctly with the AWS SDK.
    pub endpoint: Option<String>,

    /// Ruby style regexp of keys to exclude from the bucket
    pub exclude_pattern: Option<String>,

    /// Whether or not to include the S3 object’s properties (last_modified, content_type, metadata) into each Event at [@metadata][s3]. Regardless of this setting, [@metdata][s3][key] will always be present.
    pub include_object_properties: Option<bool>,

    /// Interval to wait between to check the file list again after a run is finished. Value is in seconds.
    pub interval: Option<u64>,

    /// If specified, the prefix of filenames in the bucket must match (not a regexp)
    pub prefix: Option<String>,

    /// URI to proxy server if required
    pub proxy_uri: Option<String>,

    /// The AWS Region
    pub region: Option<String>,

    /// The AWS IAM Role to assume, if any. This is used to generate temporary credentials, typically for cross-account access. See the [AssumeRole API documentation](https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRole.html) for more information.
    pub role_arn: Option<String>,

    /// Session name to use when assuming an IAM role.
    pub role_session_name: Option<String>,

    /// The AWS Secret Access Key
    pub secret_access_key: Option<String>,

    /// The AWS Session token for temporary credential
    pub session_token: Option<String>,

    /// Where to write the since database (keeps track of the date the last handled file was added to S3). The default will write sincedb files to in the directory {path.data}/plugins/inputs/s3/
    /// If specified, this setting must be a filename path and not just a directory.
    pub sincedb_path: Option<&'static Path>,

    /// Set the directory where logstash will store the tmp files before processing them.
    pub temporary_directory: Option<&'static Path>,

    /// Whether or not to watch for new files. Disabling this option causes the input to close itself after processing the files from a single listing.
    pub watch_for_new_files: Option<bool>,
    
    pub _sender: Option<UnboundedSender<Value>>,
}

impl Default for S3 {
    fn default() -> Self {
        Self {
            access_key_id: None,
            additional_settings: None,
            aws_credentials_file: None,
            backup_add_prefix: None,
            backup_to_bucket: None,
            backup_to_dir: None,
            bucket: String::new(),
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
