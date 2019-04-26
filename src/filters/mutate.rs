#![allow(unused)]

/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html
use futures::{
    sync::mpsc::{Receiver, Sender},
    try_ready, Async, Future, Poll, Sink, Stream,
};
use serde_json::{json, value::Value};

impl Stream for MutateFilter {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(ref mut receiver) = &mut self._receiver {
            let mut process = receiver.by_ref().map(|mut input_message| {
                replace(&mut input_message, "ip", json!("yo dawg"));
                strip(&mut input_message, vec!["message"]);
                split(&mut input_message, "message", "\n");
                input_message
            });

            if let Some(message) = try_ready!(process.poll()) {
                if let Some(sender) = self._sender.to_owned() {
                    let mut send = sender.send(message.clone());
                    try_ready!(send.poll().map_err(|_| ()));
                }
                Ok(Async::Ready(Some(message)))
            } else {
                Ok(Async::Ready(None))
            }
        } else {
            panic!("No receiver found for GeoipFilter.");
        }
    }
}

/// https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html#plugins-filters-mutate-copy
// fn copy(message: &mut Value, src: &str, dest: &str) {

// }

/// https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html#plugins-filters-mutate-gsub
// fn gbsub(message: &mut Value, regex: &str) {

// }

/// https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html#plugins-filters-mutate-join
// fn join(message: &mut Value, field: &str, seperator: &str) {
//     if let Some(val) = message.get_mut(field) {
//         if val.is_array() {
//             let array = val.as_array().unwrap();
//             let joined = array.join(seperator);
//             *val = Value::String(joined);
//         }
//     }
// }

/// https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html#plugins-filters-mutate-replace
fn lowercase(message: &mut Value, fields: Vec<&str>) {
    for field in fields {
        if let Some(val) = message.get_mut(field) {
            if val.is_string() {
                let lowercase = val
                    .as_str()
                    .expect("Mutate filter couldn't convert string to lowercase. ")
                    .to_lowercase();
                *val = Value::String(lowercase);
            }
        }
    }
}

/// https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html#plugins-filters-mutate-coerce
fn coerce(message: &mut Value, field: &str, new_val: Value) {
    if let Some(val) = message.get_mut(field) {
        if val.is_null() {
            *val = new_val;
        }
    }
}

// // what happens if other field already exists?
// fn rename(message: &mut Value, field: &str, new_name: &str) {
//     if let Some(obj) = message[field].as_object_mut() {
//         obj.
//     };
// }

/// https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html#plugins-filters-mutate-replace
fn replace(message: &mut Value, field: &str, new_val: Value) {
    if let Some(val) = message.get_mut(field) {
        *val = new_val;
    }
}

/// https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html#plugins-filters-mutate-split
fn split(message: &mut Value, field: &str, separator: &str) {
    if let Some(val) = message.get_mut(field) {
        if let Some(str_val) = val.as_str() {
            let array: Vec<Value> = str_val
                .split(separator)
                .map(|v| Value::String(v.to_string()))
                .collect();
            *val = Value::Array(array);
        }
    }
}

/// https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html#plugins-filters-mutate-strip
fn strip(message: &mut Value, fields: Vec<&str>) {
    for field in fields {
        if let Some(val) = message.get_mut(field) {
            if let Some(str_val) = val.as_str() {
                let stripped = str_val.trim().to_string();
                *val = Value::String(stripped);
            }
        }
    }
}

/// https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html#plugins-filters-mutate-update
fn update(message: &mut Value, field: &str, new_val: &str) {
    if let Some(val) = message.get_mut(field) {
        // what about non-string values?
        *val = Value::String(new_val.to_string());
    }
}

/// https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html#plugins-filters-mutate-uppercase
fn uppercase(message: &mut Value, fields: Vec<&str>) {
    for field in fields {
        if let Some(val) = message.get_mut(field) {
            if let Some(str_val) = val.as_str() {
                let uppercase = str_val.to_uppercase();
                *val = Value::String(uppercase);
            }
        }
    }
}

/// https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html#plugins-filters-mutate-capitalize
// fn capitalize(message: &mut Value, fields: Vec<&str>) {
//     for field in fields {
//         if let Some(val) = message.get_mut(field) {
//             if let Some(str_val) = val.as_str() {
//             }
//         }
//     }
// }

#[derive(Debug)]
pub struct MutateFilter {
    pub _receiver: Option<Receiver<Value>>,
    pub _sender: Option<Sender<Value>>,
}

impl MutateFilter {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Default for MutateFilter {
    fn default() -> Self {
        Self {
            _receiver: None,
            _sender: None,
        }
    }
}
