// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-http.html

/// Using this input you can receive single or multiline events over http(s). Applications can send an HTTP request to the endpoint started by this input and Logstash will convert it into an event for subsequent processing. Users can pass plain text, JSON, or any formatted data and use a corresponding codec with this input. For Content-Type application/json the json codec is used, but for all other data formats, plain codec is used.
/// 
/// This input can also be used to receive webhook requests to integrate with other services and applications. By taking advantage of the vast plugin ecosystem available in Logstash you can trigger actionable events right from your application.
pub struct Http {
    
}
