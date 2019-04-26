/// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-filters-mutate.html

use serde_json::{json, value::Value};
use futures::{Future, Poll, Async, Sink, try_ready, Stream, sync::mpsc::{Receiver, Sender}};

impl Stream for MutateFilter {

    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {

        if let Some(ref mut receiver) = &mut self._receiver {

            let mut process = receiver.by_ref().map(|mut input_message| {
                
                if let Some(source) = input_message.get_mut("id") {
                    *source = json!("yo dawg");
                    input_message
                } else {
                    input_message
                }
                
            });
            
            if let Some(message) = try_ready!(process.poll()) {
                if let Some(sender) = self._sender.to_owned() {
                    let mut send = sender.send(message.clone());
                    try_ready!(send.poll().map_err(|_| ()));
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
pub struct MutateFilter {
    pub _receiver: Option<Receiver<Value>>,
    pub _sender: Option<Sender<Value>>
}

impl MutateFilter {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }        
}

impl Default for MutateFilter {
    fn default() -> Self {
        Self {
            _receiver: None,
            _sender: None
        }
    }
}
