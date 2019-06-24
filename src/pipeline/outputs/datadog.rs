#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-outputs-datadog.html
#[derive(Debug)]
pub struct DatadogOutput {
    alert_type: Option<AlertType>,
    api_key: &'static str,
    date_happened: Option<&'static str>,
    dd_tags: Option<Vec<&'static str>>,
    priority: Option<Priority>,
    source_type_name: Option<SourceTypeName>,
    text: Option<&'static str>,
    title: Option<&'static str>,
}

impl Default for DatadogOutput {
    fn default() -> Self {
        Self {
            alert_type: None,
            api_key: "",
            date_happened: None,
            dd_tags: None,
            priority: None,
            source_type_name: Some(SourceTypeName::MyApps),
            text: Some("message"),
            title: Some("Logstash event for host"),
        }
    }
}

impl DatadogOutput {
    fn new(api_key: &'static str) -> Self {
        Self {
            api_key,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
enum AlertType {
    Info,
    Error,
    Warning,
    Success,
}

#[derive(Debug)]
enum Priority {
    Normal,
    Low,
}

#[derive(Debug)]
enum SourceTypeName {
    Nagios,
    Hudson,
    Jenkins,
    User,
    MyApps,
    Feed,
    Chef,
    Puppet,
    Git,
    BitBucket,
    Fabric,
    Capistrano,
}
