#![allow(unused)]

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

use futures::sync::mpsc::Sender;
use futures::{try_ready, Future, Poll, Sink, Stream};
use serde_json::Value;
use std::collections::HashMap;

pub struct InputBlock(pub Vec<Input>, pub Sender<Value>);

#[derive(Debug)]
pub enum Input {
    Exec(Exec<'static>),
    Generator(Generator<'static>),
    HttpPoller(HttpPoller<'static>),
    S3(S3<'static>),
}

impl InputBlock {
    pub fn run(self) {
        let (inputs, sender) = (self.0, self.1);

        inputs.into_iter().for_each(|mut input| {
            match &mut input {
                Input::Exec(ref mut p) => p._sender = Some(sender.clone()),
                Input::HttpPoller(ref mut p) => p._sender = Some(sender.clone()),
                Input::S3(ref mut p) => p._sender = Some(sender.clone()),
                Input::Generator(ref mut p) => p._sender = Some(sender.clone()),
            }

            tokio::spawn(input);
        });
    }
}

impl Future for Input {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), Self::Error> {
        loop {
            let poll = match self {
                Input::Exec(p) => p.poll(),
                Input::HttpPoller(p) => p.poll(),
                Input::S3(p) => p.poll(),
                Input::Generator(p) => p.poll(),
            };

            if let Some(message) = try_ready!(poll) {
                if let Some(sender) = match self {
                    Input::Exec(p) => p._sender.clone(),
                    Input::HttpPoller(p) => p._sender.clone(),
                    Input::S3(p) => p._sender.clone(),
                    Input::Generator(p) => p._sender.clone(),
                } {
                    try_ready!(sender.send(message).poll().map_err(|_| ()));
                }
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
    r#type: Option<&'a str>,
}

impl<'a> Default for CommonOptions<'a> {
    fn default() -> Self {
        Self {
            add_field: None,
            codec: Some("plain"),
            enable_metric: Some(true),
            id: None,
            tags: None,
            r#type: None,
        }
    }
}
