mod stdin;
pub use stdin::*;
mod http_poller;
pub use http_poller::*;
mod s3;
pub use s3::*;
mod generator;
pub use generator::*;
mod exec;
pub use exec::*;

use futures::{Future, Poll, Stream, try_ready, Sink};
use futures::sync::mpsc::{Sender};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Input {
    Exec(exec::Exec<'static>, Sender<Value>),
    Generator(Generator<'static>, Sender<Value>),
    HttpPoller(HttpPoller<'static>, Sender<Value>),
    S3(S3<'static>, Sender<Value>)
}

impl Future for Input {

    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), Self::Error> {
        
        loop {

            let poll = match self {
                Input::Exec(p,_) => p.poll(),
                Input::HttpPoller(p,_) => p.poll(),
                Input::S3(p,_) => p.poll(),
                Input::Generator(p,_) => p.poll()
            };

            if let Some(message) = try_ready!(poll) {

                let send = match self {
                    Input::Exec(_,s) => s,
                    Input::HttpPoller(_,s) => s,
                    Input::S3(_,s) => s,
                    Input::Generator(_,s) => s
                };

                let send = send.to_owned()
                    .send(message)
                    .map_err(|_| ())
                    .poll();
                
                try_ready!(send);
                
            }
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
