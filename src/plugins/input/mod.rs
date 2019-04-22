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

            let message = match self {
                Input::HttpPoller(p,_) => p.poll().map_err(|_| ()),
                Input::S3(p,_) => p.poll().map_err(|_| ())
            };
            
            if let Some(message) = try_ready!(message) {

                let send = match self {
                    Input::HttpPoller(_,s) => s,
                    Input::S3(_,s) => s
                };

                try_ready!(send.to_owned().send(message).map_err(|_| ()).poll());
                
            }
        }

    }

}

// #[derive(Debug)]
// pub struct Input<T: Stream, M>(pub T, pub M);

// impl<T> Future for Input<T, Sender<T::Item>>
// where
//     T: Stream,
//     T::Item: Display,
//     T::Item: Debug
// {

//     type Item = ();
//     type Error = ();

//     fn poll(&mut self) -> Poll<(), Self::Error> {
        
//         loop {

//             let message = self.0.poll().map_err(|_| ());
            
//             if let Some(message) = try_ready!(message) {

//                 let mut send = self.1.to_owned()
//                     .send(message)
//                     .map_err(|_| ());

//                 try_ready!(send.poll());
                
//             }
//         }

//     }

// }


// #[allow(unused)]
// struct CommonOptions<'a> {
//     add_field: Option<HashMap<&'a str, &'a str>>,
//     codec: Option<&'a str>,
//     enable_metric: Option<bool>,
//     id: Option<&'a str>,
//     tags: Option<Vec<&'a str>>,
//     r#type: Option<&'a str>
// }
