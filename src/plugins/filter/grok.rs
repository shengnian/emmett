use std::collections::HashMap;

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-filters-grok.html

#[derive(Debug)]
pub struct GrokFilter {
    pub break_on_match: Option<bool>,
    pub keep_empty_captures: Option<bool>,
    pub r#match: Option<String>,
    pub named_captures_only: Option<bool>,
    pub overwrite: Option<Vec<String>>,
    pub pattern_definitions: Option<HashMap<String, String>>,
    pub patterns_dir: Option<Vec<String>>,
    pub patterns_files_glob: Option<String>,
    pub tag_on_failure: Option<Vec<String>>,
    pub tag_on_timeout: Option<String>,
    pub timeout_millis: Option<u64>
}

impl Default for GrokFilter {
    fn default() -> GrokFilter {
        GrokFilter {
            break_on_match: None,
            keep_empty_captures: None,
            r#match: None,
            named_captures_only: None,
            overwrite: None,
            pattern_definitions: None,
            patterns_dir: None,
            patterns_files_glob: None,
            tag_on_failure: None,
            tag_on_timeout: None,
            timeout_millis: None
        }
    }
}

impl GrokFilter {
    pub fn new() -> GrokFilter {
        GrokFilter {
            ..Default::default()
        }
    }
}
