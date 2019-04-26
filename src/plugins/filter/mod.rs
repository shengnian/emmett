mod grok;
pub use grok::*;
mod date;
pub use date::*;
mod geoip;
pub use geoip::*;
mod mutate;
pub use mutate::*;

use futures::{Poll, Future, Stream, try_ready};
use futures::sync::mpsc::{channel, Sender, Receiver};
use serde_json::Value;

pub struct FilterBlock(pub Vec<Filter>, pub Receiver<Value>, pub Sender<Value>);

#[derive(Debug)]
pub enum Filter {
    Geoip(geoip::GeoipFilter<'static>),
    MutateFilter(mutate::MutateFilter)
}

impl FilterBlock {
    pub fn run(self) {

        let (filters, receiver, sender) = (self.0, self.1, self.2);
        
        let mut filters = filters.into_iter()
            .fold(Vec::new(), |mut acc, mut filter| {

                let (tx, rx) = channel(1_024);

                match &mut filter {
                    Filter::Geoip(ref mut p) => {
                        p._receiver = Some(rx);
                        p._sender = Some(tx);
                    },
                    Filter::MutateFilter(ref mut p) => {
                        p._receiver = Some(rx);
                        p._sender = Some(tx);
                    },
                };
                
                acc.push(filter);
                acc
                    
            });

        
        if let Some(filter) = filters.iter_mut().nth(0) {
            match filter {
                Filter::Geoip(ref mut p) => p._receiver = Some(receiver),
                Filter::MutateFilter(ref mut p) => p._receiver = Some(receiver)
            };
        }

        if let Some(filter) = filters.iter_mut().last() {
            match filter {
                Filter::Geoip(ref mut p) => p._sender = Some(sender),
                Filter::MutateFilter(ref mut p) => p._sender = Some(sender)
            };
        }
        
        for filter in filters {
            tokio::spawn(filter);
        }
        
    }
}

impl Future for Filter {

    type Item = ();
    type Error = ();
    
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {

        loop {

            let poll = match self {
                Filter::Geoip(p) => p.poll(),
                Filter::MutateFilter(p) => p.poll(),
            };

            // if let Some(message) = try_ready!(poll) {
            //     println!("{:#}", message);
            // };

        }

    }
    
}
