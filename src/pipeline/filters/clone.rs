/// https://www.elastic.co/guide/en/logstash/current/plugins-filters-clone.html
use futures::{
    sync::mpsc::{Receiver, Sender},
    try_ready, Async, Future, Poll, Sink, Stream,
};
use serde_json::value::Value;

impl Stream for Clone {
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
#[allow(unused)]
pub struct Clone {
    clones: Vec<String>,
    pub _receiver: Option<Receiver<Value>>,
    pub _sender: Option<Sender<Value>>,
}

impl Default for Clone {
    fn default() -> Self {
        Self {
            clones: Vec::new(),
            _receiver: None,
            _sender: None,
        }
    }
}
