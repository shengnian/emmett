mod stdout;
pub use stdout::*;
mod elasticsearch;
pub use elasticsearch::*;

// use futures::{Future, Poll, Stream, try_ready};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Output<'a> {
    Stdout(Stdout<'a>),
}

// impl Future for Output {

//     type Item = ();
//     type Error = ();

//     fn poll(&mut self) -> Poll<(), Self::Error> {
        
//         loop {

//             let poll = match self {
//                 Output::Stdout(p) => p.poll(),
//             };

//             try_ready!(poll);

//         }

//     }

// }

impl<'a> Output<'a> {
    pub fn process(&self, message: &Value) -> Result<Value, ()> {
        match self {
            Output::Stdout(p) => p.process(&message),
        }
    }
}

#[derive(Debug)]
pub struct CommonOptions<'a> {
    add_field: Option<HashMap<&'a str, &'a str>>,
    codec: Option<&'a str>,
    enable_metric: Option<bool>,
    id: Option<&'a str>,
    tags: Option<Vec<&'a str>>,
    r#type: Option<&'a str>
}

impl<'a> Default for CommonOptions<'a> {
    fn default() -> Self {
        Self {
            add_field: None,
            codec: Some("plain"),
            enable_metric: Some(true),
            id: None,
            tags: None,
            r#type: None    
        }
    }
}
