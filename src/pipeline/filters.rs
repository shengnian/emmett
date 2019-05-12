use futures::{
    sync::mpsc::{channel, Receiver, Sender},
    Future, Poll, Sink, Stream,
};
use serde_json::Value;

pub struct FilterBlock(pub Vec<Filter>);

#[derive(Debug)]
pub enum Filter {
    Geoip(geoip::Geoip<'static>),
    Json(json::Json<'static>),
    Mutate(mutate::Mutate),
    Clone(clone::Clone),
    Fingerprint(fingerprint::Fingerprint<'static>),
}

impl FilterBlock {
    pub fn run(self, filter_receiver: Receiver<Value>) -> Receiver<Value> {
        let mut filters = self.0;
        
        let (filter_sender, outputs_receiver) = channel(1_024);
        
        let last = filters.len() -1;

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
                        Filter::Clone(ref mut p) => {
                            p._receiver = None;
                            p._sender = Some(tx);
                        }
                        Filter::Fingerprint(ref mut p) => {
                            p._receiver = None;
                            p._sender = Some(tx);
                        }
                        Filter::Geoip(ref mut p) => {
                            p._receiver = None;
                            p._sender = Some(tx);
                        }
                        Filter::Json(ref mut p) => {
                            p._receiver = None;
                            p._sender = Some(tx);
                        }
                        Filter::Mutate(ref mut p) => {
                            p._receiver = None;
                            p._sender = Some(tx);
                        }
                    };
                    acc.1 = rx;
                    acc
                } else if (i > 0) && (i < last) {
                    match &mut filter {
                        Filter::Clone(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = Some(tx);
                        }
                        Filter::Fingerprint(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = Some(tx);
                        }
                        Filter::Geoip(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = Some(tx);
                        }
                        Filter::Json(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = Some(tx);
                        }
                        Filter::Mutate(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = Some(tx);
                        }
                    };

                    acc.1 = rx;
                    acc
                } else {
                    match &mut filter {
                        Filter::Clone(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = None;
                        }
                        Filter::Fingerprint(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = None;
                        }
                        Filter::Geoip(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = None;
                        }
                        Filter::Json(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = None;
                        }
                        Filter::Mutate(ref mut p) => {
                            p._receiver = Some(acc.1);
                            p._sender = None;
                        }
                    };

                    channel(1_024)
                }
            });

        if let Some(filter) = filters.get_mut(0) {
            match filter {
                Filter::Clone(ref mut p) => p._receiver = Some(filter_receiver),
                Filter::Fingerprint(ref mut p) => p._receiver = Some(filter_receiver),
                Filter::Geoip(ref mut p) => p._receiver = Some(filter_receiver),
                Filter::Json(ref mut p) => p._receiver = Some(filter_receiver),
                Filter::Mutate(ref mut p) => p._receiver = Some(filter_receiver),
            };
        }

        if let Some(filter) = filters.iter_mut().last() {
            match filter {
                Filter::Clone(ref mut p) => p._sender = Some(filter_sender),
                Filter::Fingerprint(ref mut p) => p._sender = Some(filter_sender),
                Filter::Geoip(ref mut p) => p._sender = Some(filter_sender),
                Filter::Json(ref mut p) => p._sender = Some(filter_sender),
                Filter::Mutate(ref mut p) => p._sender = Some(filter_sender),
            };
        }

        for filter in filters {
            tokio::spawn(filter);
        }

        outputs_receiver
        
    }
}

impl Future for Filter {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let _poll = match self {
                Filter::Clone(p) => p.poll(),
                Filter::Fingerprint(p) => p.poll(),
                Filter::Geoip(p) => p.poll(),
                Filter::Json(p) => p.poll(),
                Filter::Mutate(p) => p.poll(),
            };

            // if let Some(message) = try_ready!(_poll) {
            //     println!("{:#}", message);
            // };
        }
    }
}

mod aggregate;
mod alter;
mod bytes;
mod cidr;
mod cipher;
mod clone;
mod csv;
mod date;
mod de_dot;
mod dissect;
mod dns;
mod drop;
mod elapsed;
mod elasticsearch;
mod environment;
mod extractnumbers;
mod fingerprint;
mod geoip;
mod grok;
mod http;
mod i18n;
mod jdbc_static;
mod jdbc_streaming;
mod json;
mod json_encode;
mod kv;
mod memcached;
mod metricize;
mod metrics;
mod mutate;
mod prune;
mod range;
mod ruby;
mod sleep;
mod split;
mod syslog_pri;
mod throttle;
mod tld;
mod translate;
mod truncate;
mod urldecode;
mod useragent;
mod uuid;
mod xml;

pub use self::http::*;
pub use aggregate::*;
pub use alter::*;
pub use bytes::*;
pub use cidr::*;
pub use cipher::*;
pub use clone::*;
pub use csv::*;
pub use date::*;
pub use de_dot::*;
pub use dissect::*;
pub use dns::*;
pub use drop::*;
pub use elapsed::*;
pub use elasticsearch::*;
pub use environment::*;
pub use extractnumbers::*;
pub use fingerprint::*;
pub use geoip::*;
pub use grok::*;
pub use i18n::*;
pub use jdbc_static::*;
pub use jdbc_streaming::*;
pub use json::*;
pub use json_encode::*;
pub use kv::*;
pub use memcached::*;
pub use metricize::*;
pub use metrics::*;
pub use mutate::*;
pub use prune::*;
pub use range::*;
pub use ruby::*;
pub use sleep::*;
pub use split::*;
pub use syslog_pri::*;
pub use throttle::*;
pub use tld::*;
pub use translate::*;
pub use truncate::*;
pub use urldecode::*;
pub use useragent::*;
pub use uuid::*;
pub use xml::*;