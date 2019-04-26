mod grok;
pub use grok::*;
mod date;
pub use date::*;
mod geoip;
pub use geoip::*;
mod mutate;
pub use mutate::*;
mod clone;
pub use clone::*;

use futures::{
    sync::mpsc::{channel, Receiver, Sender},
    Future, Poll, Sink, Stream,
};
use serde_json::Value;

pub struct FilterBlock(pub Vec<Filter>, pub Receiver<Value>, pub Sender<Value>);

#[derive(Debug)]
pub enum Filter {
    Geoip(geoip::GeoipFilter<'static>),
    MutateFilter(mutate::MutateFilter),
    CloneFilter(clone::CloneFilter),
}

impl FilterBlock {
    pub fn run(self) {
        let (mut filters, receiver, sender) = (self.0, self.1, self.2);
        let count = filters.len();

        // if there are no filters, connect the filter_receiver to the filter_sender

        // let no_filter_channel = receiver.for_each(|message| {
        //     sender.clone().send(message);
        //     Ok(())
        // });

        filters
            .iter_mut()
            .enumerate()
            .fold(channel(1_024), |mut acc, (i, mut filter)| {
                let (tx, rx) = channel(1_024);

                if i == 0 {
                    match &mut filter {
                        Filter::Geoip(ref mut p) => {
                            p._receiver = None;
                            p._sender = Some(tx);
                        }
                        Filter::MutateFilter(ref mut p) => {
                            p._receiver = None;
                            p._sender = Some(tx);
                        }
                        Filter::CloneFilter(ref mut p) => {
                            p._receiver = None;
                            p._sender = Some(tx);
                        }
                    };
                    acc.1 = rx;
                    acc
                } else if (i > 0) && (i < (count - 1)) {
                    match &mut filter {
                        Filter::Geoip(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = Some(tx);
                        }
                        Filter::MutateFilter(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = Some(tx);
                        }
                        Filter::CloneFilter(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = Some(tx);
                        }
                    };
                    acc.1 = rx;
                    acc
                } else {
                    match &mut filter {
                        Filter::Geoip(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = None;
                        }
                        Filter::MutateFilter(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = None;
                        }
                        Filter::CloneFilter(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = None;
                        }
                    };
                    channel(1_024)
                }
            });

        if let Some(filter) = filters.get_mut(0) {
            match filter {
                Filter::Geoip(ref mut p) => p._receiver = Some(receiver),
                Filter::MutateFilter(ref mut p) => p._receiver = Some(receiver),
                Filter::CloneFilter(ref mut p) => p._receiver = Some(receiver),
            };
        }

        if let Some(filter) = filters.iter_mut().last() {
            match filter {
                Filter::Geoip(ref mut p) => p._sender = Some(sender),
                Filter::MutateFilter(ref mut p) => p._sender = Some(sender),
                Filter::CloneFilter(ref mut p) => p._sender = Some(sender),
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
            let _poll = match self {
                Filter::Geoip(p) => p.poll(),
                Filter::MutateFilter(p) => p.poll(),
                Filter::CloneFilter(p) => p.poll(),
            };

            // if let Some(message) = try_ready!(_poll) {
            //     println!("{:#}", message);
            // };
        }
    }
}
