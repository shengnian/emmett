#[derive(Debug)]
/// Read events from standard input.
/// By default, each event is assumed to be one line. If you want to join lines, you’ll want to use the multiline codec.
pub struct Stdin {
    pub add_field: Option<String>,
    pub codec: Option<String>,
    pub enable_metric: Option<bool>,
    pub id: Option<String>,
    pub tags: Option<Vec<String>>,
    pub r#type: Option<String>,
}

impl Default for Stdin {
    fn default() -> Self {
        Self {
            add_field: None,
            codec: None,
            enable_metric: None,
            id: None,
            tags: None,
            r#type: None,
        }
    }
}

impl Stdin {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
