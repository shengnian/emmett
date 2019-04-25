mod stdout;
pub use stdout::*;
mod elasticsearch;
pub use elasticsearch::*;

use futures::{Future, Poll, Stream, try_ready, sync::mpsc::Receiver};
use serde_json::Value;
use std::collections::HashMap;

pub struct OutputBlock(pub Vec<Output>, pub Receiver<Value>);

#[derive(Debug)]
pub enum Output {
    Stdout(Stdout),
}

impl OutputBlock {
    pub fn run(&mut self) {

        let (outputs, receiver) = (&mut self.0, self.1);
        
        outputs.iter_mut()
            .for_each(move |mut output| {
                match &mut output {
                    Output::Stdout(ref mut p) => p._receiver = Some(&receiver)
                };
            });

        for output in self.0 {
            tokio::spawn(output);
        }
        
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

            if let Some(message) = try_ready!(poll) {
                println!("{:#}", message);
            };            

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
