#![allow(unused)]

mod stdout;
pub use stdout::*;
mod elasticsearch;
pub use elasticsearch::*;
mod datadog;
pub use datadog::*;
mod csv;
pub use csv::*;
    
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
        let (s, r) = unbounded();

        self.0.iter_mut().for_each(|output| {
            match output {
                Output::Stdout(ref mut p) => p._receiver = Some(r.clone()),
            };
        });

        for output in self.0 { tokio::spawn(output); }

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
