mod stdout;
pub use stdout::*;
mod elasticsearch;
pub use elasticsearch::*;

use futures::{Future, Poll, sync::mpsc::Receiver};
use serde_json::Value;
use std::collections::HashMap;

pub struct OutputBlock<'a>(pub Vec<Output<'a>>, pub Receiver<Value>);

#[derive(Debug)]
pub enum Output<'a> {
    Stdout(Stdout<'a>),
}

impl<'a> Future for Output<'a> {

    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), Self::Error> {
        
        loop {

            // let poll = match self {
            //     Output::Stdout(p,_) => p.poll(),
            // };

            // if let Some(message) = try_ready!(poll) {

            //     let send = match self {
            //         Output::Stdout(_,s) => s,
            //     };

            //     let send = send.to_owned()
            //         .send(message)
            //         .map_err(|_| ())
            //         .poll();
                
            //     try_ready!(send);

            // }


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
