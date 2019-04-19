/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-output-stdin.html

#[derive(Debug)]
pub struct StdoutOutput {
    codec: Option<String>,
    enable_metric: Option<bool>,
    id: Option<String>,
}

impl Default for StdoutOutput {

    fn default() -> StdoutOutput {
        StdoutOutput {            
            codec: None,
            enable_metric: None,
            id: None,
        }
    }

}

impl StdoutOutput {

    pub fn new() -> StdoutOutput {
        StdoutOutput {
            ..Default::default()
        }
    }
    
}
