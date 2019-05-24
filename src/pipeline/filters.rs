use futures::{
    sync::mpsc::{channel, Receiver, Sender},
    Future, Poll, Sink, Stream, Async
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
    pub fn run(&self, receiver: Receiver<Value>) -> Receiver<Value> {

        let (filter_sender, outputs_receiver) = channel(1_024);
        let filters = &self.0;
        
        receiver.for_each(|message| {
            for filter in filters {
                
                let mut future = match filter {
                    Filter::Mutate(mut p) => p.process(message),
                    _ => panic!("kjhsdkjhsf")
                };
                dbg!(future.poll());

            }
            Ok(())
        });

        
        outputs_receiver
            
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
