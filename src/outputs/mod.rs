#![allow(unused)]

use crossbeam_channel::unbounded;
use futures::{sync::mpsc::Receiver, Future, Poll, Stream};
use serde_json::Value;

pub struct OutputBlock(pub Vec<Output>, pub Receiver<Value>);

#[derive(Debug)]
pub enum Output {
    Stdout(Stdout),
}

impl OutputBlock {
    pub fn run(mut self) {

        // use crossbeam_channel to account for spmc instead of mpsc
        let (s, r) = unbounded();

        // attach a receiver to each output plugin
        self.0.iter_mut().for_each(|output| {
            match output {
                Output::Stdout(ref mut p) => p._receiver = Some(r.clone()),
            };
        });

        // run each output plugin
        for output in self.0 { tokio::spawn(output); }

        // for every message sent to the `output` block, send to each output separately
        let broadcast = self.1.for_each(move |message| {
            s.send(message).unwrap();
            Ok(())
        });

        tokio::spawn(broadcast);

    }
}

impl Future for Output {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), Self::Error> {
        loop {
            let poll = match self {
                Output::Stdout(p) => p.poll(),
            };

            poll.expect("Something went wrong polling an output plugin.");
        }
    }
}

#[derive(Debug)]
pub struct CommonOptions<'a> {
    codec: Option<&'a str>,
    enable_metric: Option<bool>,
    id: Option<&'a str>,
}

impl<'a> Default for CommonOptions<'a> {
    fn default() -> Self {
        Self {
            codec: Some("plain"),
            enable_metric: Some(true),
            id: None,
        }
    }
}

mod csv;
pub use csv::*;
mod datadog;
pub use datadog::*;
mod elasticsearch;
pub use elasticsearch::*;
mod stdout;
pub use stdout::*;
