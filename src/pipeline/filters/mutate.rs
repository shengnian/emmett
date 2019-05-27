/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html
use futures::{
    sync::mpsc::{Receiver, Sender},
    try_ready, Async, Future, Poll, Sink, Stream,
};
use serde_json::{json, value::Value};
use std::convert::TryFrom;
use toml::value::Table;

impl Mutate {
    pub fn process(self, input: Value) -> impl Future<Item=Value, Error=()> {
        futures::future::lazy(move || {

            let mut input_copy = input.clone();
            
            if let Some(rep) = self.replace {
                for (key, value) in rep.iter() {
                    replace(&mut input_copy, key, json!(value));
                }
            }

            if let Some(cop) = self.copy {
                for (key, value) in cop.iter() {
                    let value = value.as_str().unwrap();
                    copy(&mut input_copy, key, value);
                }
            }
            
            if let Some(strip_fields) = self.strip {
                strip(&mut input_copy, strip_fields);
            }

            if let Some((split_field, split_at)) = self.split {
                split(&mut input_copy, &split_field, &split_at);
            }
            
            // split(&mut input_copy, "body", "\n");
            // capitalize(&mut input_copy, vec!["titleCopy"]);
            // join(&mut input_copy, "body", " ; ");
            
            Ok(input_copy)
                
        })
    }
}


fn copy(message: &mut Value, src: &str, dest: &str) {
    if let Some(val) = message.get(src) {
        message[dest] = val.clone();
    }
}

// fn gbsub(message: &mut Value, regex: &str) {

// }

fn join(message: &mut Value, field: &str, seperator: &str) {
    if let Some(val) = message.get_mut(field) {
        if let Some(array) = val.as_array() {

            let mut joined = String::new();
            
            for (i, item) in array.iter().enumerate() {
                if let Some(string) = item.as_str() {
                    if i < array.len() - 1 {
                        joined.push_str(string);
                        joined.push_str(seperator);
                    } else {
                        joined.push_str(string);
                    }
                }
            }
            
            *val = Value::String(joined);

        }
    }
}

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

fn strip(message: &mut Value, fields: Vec<String>) {
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

fn capitalize(message: &mut Value, fields: Vec<&str>) {
    for field in fields {
        if let Some(val) = message.get_mut(field) {
            if let Some(str_val) = val.as_str() {

                let mut capitalized = String::new();

                for (i, char) in str_val.chars().enumerate() {
                    if i == 0 {
                        capitalized.push(char.to_ascii_uppercase());
                    } else {
                        capitalized.push(char);
                    }
                }

                *val = json!(capitalized);
                
            }
        }
    }
}

#[derive(Debug, Clone)]
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
    split: Option<(String, String)>,
    strip: Option<Vec<String>>,
    update: Option<String>,
    uppercase: Option<Vec<String>>,
    capitalize: Option<Vec<String>>,
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
        
        if let Some(strip_fields) = toml.get("strip") {
            let strip_fields = strip_fields.as_array()
                .expect("Couldn't parse Mutate strip field as array.");
            let strip_fields = strip_fields.iter()
                .map(|v| v.as_str().expect("Can't parse Mutate strip fields as strings.").to_string())
                .collect();
            mutate.strip = Some(strip_fields);
        }
        
        if let Some(split_setting) = toml.get("split") {
            let split_setting = split_setting.as_table()
                .expect("Couldn't parse Mutate split field as table.");
            for (field, value) in split_setting.iter().take(1) {
                let value = value.as_str().expect("Can't parse Mutate filter split setting as string.");
                mutate.split = Some((field.to_string(), value.to_string()));
            }
        }

        Ok(mutate)
        
    }
}
