// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-azure_event_hubs.html

#[derive(Debug)]
/// This plugin consumes events from Azure Event Hubs, a highly scalable data streaming platform and event ingestion service. Event producers send events to the Azure Event Hub, and this plugin consumes those events for use with Logstash.
pub struct AzureEventHubs {
    pub config_mode: Option<String>,
    pub event_hubs: Option<Vec<&'static str>>,
    pub event_hub_connections: Option<Vec<&'static str>>,
    pub event_hub_connection: Option<String>,
    pub checkpoint_interval: Option<u64>,
    pub consumer_group: Option<String>,
    pub decorate_events: Option<bool>,
    pub initial_position: Option<String>,
    pub initial_position_look_back: Option<u64>,
    pub max_batch_size: Option<u64>,
    pub storage_connection: Option<String>,
    pub storage_container: Option<String>,
    pub threads: Option<u64>,
}

impl Default for AzureEventHubs {
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
