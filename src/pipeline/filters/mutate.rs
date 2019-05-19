/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html
use futures::{
    sync::mpsc::{Receiver, Sender},
    try_ready, Async, Future, Poll, Sink, Stream,
};
use serde_json::{json, value::Value};
use std::convert::TryFrom;
use toml::value::Table;

impl Stream for Mutate {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let replace_setting = &self.replace;
        let copy_setting = &self.copy;
        if let Some(ref mut receiver) = &mut self._receiver {
            
            let mut process = receiver.by_ref()
                .map(|mut input_message| {

                    if let Some(rep) = replace_setting {
                        for (key, value) in rep.iter() {
                            replace(&mut input_message, key, json!(value));
                        }
                    }

                    if let Some(cop) = copy_setting {
                        for (key, value) in cop.iter() {
                            let value = value.as_str().unwrap();
                            copy(&mut input_message, key, value);
                        }
                    }
                    
                    strip(&mut input_message, vec!["message"]);
                    split(&mut input_message, "body", "\n");
                    input_message
                });

            if let Some(message) = try_ready!(process.poll()) {
                let sender = self._sender.to_owned()
                    .expect("No sender attached to Mutate");
                let mut send = sender.send(message);
                try_ready!(send.poll().map_err(|_| ()));
            }

            Ok(Async::Ready(None))
                
        } else {
            panic!("No receiver found for GeoipFilter.");
        }
    }
}

fn copy(message: &mut Value, src: &str, dest: &str) {
    if let Some(val) = message.get(src) {
        message[dest] = val.clone();
    }
}

// fn gbsub(message: &mut Value, regex: &str) {

// }

// fn join(message: &mut Value, field: &str, seperator: &str) {
//     if let Some(val) = message.get_mut(field) {
//         if val.is_array() {
//             let array = val.as_array().unwrap();
//             let joined = array.join(seperator);
//             *val = Value::String(joined);
//         }
//     }
// }

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

fn replace(message: &mut Value, field: &str, new_val: Value) {
    if let Some(val) = message.get_mut(field) {
        *val = new_val;
    }
}

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

fn update(message: &mut Value, field: &str, new_val: &str) {
    if let Some(val) = message.get_mut(field) {
        // what about non-string values?
        *val = Value::String(new_val.to_string());
    }
}

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

// fn capitalize(message: &mut Value, fields: Vec<&str>) {
//     for field in fields {
//         if let Some(val) = message.get_mut(field) {
//             if let Some(str_val) = val.as_str() {
//             }
//         }
//     }
// }



#[derive(Debug)]
pub struct Mutate {
    convert: Option<Value>,
    copy: Option<Table>,
    gsub: Option<String>,
    join: Option<String>,
    lowercase: Option<String>,
    merge: Option<String>,
    coerce: Option<String>,
    rename: Option<String>,
    replace: Option<Table>,
    split: Option<String>,
    strip: Option<Vec<String>>,
    update: Option<String>,
    uppercase: Option<Vec<String>>,
    capitalize: Option<Vec<String>>,
    pub _receiver: Option<Receiver<Value>>,
    pub _sender: Option<Sender<Value>>,
}

impl Default for Mutate {
    fn default() -> Self {
        Self {
            convert: None,
            copy: None,
            gsub: None,
            join: None,
            lowercase: None,
            merge: None,
            coerce: None,
            rename: None,
            replace: None,
            split: None,
            strip: None,
            update: None,
            uppercase: None,
            capitalize: None,
            _receiver: None,
            _sender: None,
        }
    }
}

impl TryFrom<&toml::Value> for Mutate {
    type Error = ();
    
    fn try_from(toml: &toml::Value) -> Result<Self, Self::Error> {

        let mut mutate = Mutate {
            ..Default::default()
        };
        
        if let Some(replace) = toml.get("replace") {
            let replace = replace.as_table()
                .expect("Couldn't parse Mutate replace as table.");
            mutate.replace = Some(replace.to_owned());
        }

        if let Some(copy) = toml.get("copy") {
            let copy = copy.as_table()
                .expect("Couldn't parse Mutate copy field as table.");
            mutate.copy = Some(copy.to_owned());
        }

        Ok(mutate)
        
    }
}
