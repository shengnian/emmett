mod stdout;
pub use stdout::*;
mod elasticsearch;
pub use elasticsearch::*;

use futures::{Future, Poll, Stream, sync::mpsc::Receiver};
use serde_json::Value;
use std::collections::HashMap;
use crossbeam_channel::unbounded;

pub struct OutputBlock(pub Vec<Output>, pub Receiver<Value>);

#[derive(Debug)]
pub enum Output {
    Stdout(Stdout),
}

impl OutputBlock {
    pub fn run(mut self) {

        let (s, r) = unbounded();
        
        &mut self.0.iter_mut()
            .for_each(|output| {
                match output {
                    Output::Stdout(ref mut p) => p._receiver = Some(r.clone())
                };
            });

        for output in self.0 {
            tokio::spawn(output);
        }

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
