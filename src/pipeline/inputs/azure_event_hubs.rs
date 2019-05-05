#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-azure_event_hubs.html

#[derive(Debug)]
pub struct AzureEventHubsInput {
    config_mode: Option<String>,
    event_hubs: Option<Vec<&'static str>>,
    event_hub_connections: Option<Vec<&'static str>>,
    event_hub_connection: Option<String>,
    checkpoint_interval: Option<u64>,
    consumer_group: Option<String>,
    decorate_events: Option<bool>,
    initial_position: Option<String>,
    initial_position_look_back: Option<u64>,
    max_batch_size: Option<u64>,
    storage_connection: Option<String>,
    storage_container: Option<String>,
    threads: Option<u64>,
}

impl Default for AzureEventHubsInput {
    fn default() -> Self {
        Self {
            config_mode: None,
            event_hubs: None,
            event_hub_connections: None,
            event_hub_connection: None,
            checkpoint_interval: None,
            consumer_group: None,
            decorate_events: None,
            initial_position: None,
            initial_position_look_back: None,
            max_batch_size: None,
            storage_connection: None,
            storage_container: None,
            threads: None,
        }
    }
}

impl AzureEventHubsInput {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
