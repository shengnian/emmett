pub mod stdin;
pub use stdin::*;
pub mod http_poller;
pub use http_poller::*;
pub mod s3;
pub use s3::*;

use futures::{Future, Poll, Stream, try_ready, Sink};
use futures::sync::mpsc::{Sender};
use serde_json::Value;

#[derive(Debug)]
pub enum Input {
    HttpPoller(HttpPollerInput<'static>, Sender<Value>),
    S3(S3Input<'static>, Sender<Value>)
}

impl Future for Input {

    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), Self::Error> {
        
        loop {

            let poll = match self {
                Input::HttpPoller(p,_) => p.poll(),
                Input::S3(p,_) => p.poll()
            };

            if let Some(message) = try_ready!(poll) {

                let send = match self {
                    Input::HttpPoller(_,s) => s,
                    Input::S3(_,s) => s
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

// struct CommonOptions<'a> {
//     add_field: Option<HashMap<&'a str, &'a str>>,
//     codec: Option<&'a str>,
//     enable_metric: Option<bool>,
//     id: Option<&'a str>,
//     tags: Option<Vec<&'a str>>,
//     r#type: Option<&'a str>
// }
