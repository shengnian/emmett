#![allow(unused)]
/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-outputs-stdout.html
use super::Run;
use std::convert::TryFrom;
use futures::{Async, Poll, Stream};
use serde_json::{json, value::Value};

impl Run for Stdout {
    fn run(&self, input: Value) {
        println!("{:#}", input)
    }
}

#[derive(Debug)]
pub struct Stdout {
    pub codec: Option<String>,
    pub enable_metric: Option<bool>,
    pub id: Option<String>,
}

impl Default for Stdout {
    fn default() -> Self {
        Self {
            codec: None,
            enable_metric: None,
            id: None,
        }
    }
}

impl TryFrom<&toml::Value> for Stdout {
    type Error = ();

    fn try_from(toml: &toml::Value) -> Result<Self, Self::Error> {
        let mut stdout = Stdout {
            ..Default::default()
        };

        // if let Some(replace) = toml.get("codec") {
        //     let codec = replace
        //         .as_table()
        //         .expect("Couldn't parse Stdout codec as table.");
        //     stdout.replace = Some(replace.to_owned());
        // }

        // if let Some(copy) = toml.get("copy") {
        //     let copy = copy
        //         .as_table()
        //         .expect("Couldn't parse Mutate copy field as table.");
        //     mutate.copy = Some(copy.to_owned());
        // }

        // if let Some(strip_fields) = toml.get("strip") {
        //     let strip_fields = strip_fields
        //         .as_array()
        //         .expect("Couldn't parse Mutate strip field as array.");
        //     let strip_fields = strip_fields
        //         .iter()
        //         .map(|v| {
        //             v.as_str()
        //                 .expect("Can't parse Mutate strip fields as strings.")
        //                 .to_string()
        //         })
        //         .collect();
        //     mutate.strip = Some(strip_fields);
        // }

        // if let Some(split_setting) = toml.get("split") {
        //     let split_setting = split_setting
        //         .as_table()
        //         .expect("Couldn't parse Mutate split field as table.");

        //     if split_setting.len() > 0 {
        //         mutate.split = Some(Vec::new());
        //     }

        //     for (field, value) in split_setting.iter() {
        //         let value = value
        //             .as_str()
        //             .expect("Can't parse Mutate filter split setting as string.");
        //         if let Some(ref mut splits) = mutate.split {
        //             splits.push((field.to_string(), value.to_string()))
        //         }
        //     }
        // }

        // if let Some(capitalize_fields) = toml.get("capitalize") {
        //     let capitalize_fields = capitalize_fields
        //         .as_array()
        //         .expect("Couldn't parse Mutate capitalize field as array.");
        //     let capitalize_fields = capitalize_fields
        //         .iter()
        //         .map(|v| {
        //             v.as_str()
        //                 .expect("Can't parse Mutate capitalize fields as strings.")
        //                 .to_string()
        //         })
        //         .collect();
        //     mutate.capitalize = Some(capitalize_fields);
        // }

        Ok(stdout)
    }
}
