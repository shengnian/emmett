#![allow(unused)]

/// https://www.elastic.co/guide/en/logstash/current/plugins-filters-clone.html

use serde_json::{json, value::Value};
use futures::{Future, Poll, Async, Sink, try_ready, Stream, sync::mpsc::{Receiver, Sender}};

impl Stream for CloneFilter {

    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {

        if let Some(ref mut receiver) = &mut self._receiver {
            
            if let Some(message) = try_ready!(receiver.poll()) {
                if let Some(sender) = self._sender.to_owned() {
                    let mut send = sender.clone().send(message.clone());
                    let mut send_again = sender.send(message.clone());
                    try_ready!(send.poll().map_err(|_| ()));
                    try_ready!(send_again.poll().map_err(|_| ()));
                }
                Ok(Async::Ready(Some(message)))
            } else {
                Ok(Async::Ready(None))
            }

        } else {
            panic!("No receiver found for GeoipFilter.");
        }
    }
    
}

#[derive(Debug)]
pub struct CloneFilter {
    clones: Vec<&'static str>,
    pub _receiver: Option<Receiver<Value>>,
    pub _sender: Option<Sender<Value>>
}

impl CloneFilter {
    pub fn new(clones: Vec<&str>) -> Self {
        Self {
            ..Default::default()
        }
    }        
}

impl Default for CloneFilter {
    fn default() -> Self {
        Self {
            clones: Vec::new(),
            _receiver: None,
            _sender: None
        }
    }
}
