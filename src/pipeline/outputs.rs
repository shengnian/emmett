use crossbeam::unbounded;
use futures::{sync::mpsc::UnboundedReceiver, Future, Poll, Stream, Async, try_ready, lazy, Sink};
use serde_json::Value;

pub struct OutputBlock(pub Vec<Output>);

#[derive(Debug)]
pub enum Output {
    Stdout(Stdout),
}

impl OutputBlock {
    pub fn run(mut self, outputs_receiver: UnboundedReceiver<Value>) {
        // use crossbeam_channel to account for spmc instead of mpsc
        let (s, r) = unbounded();

        // attach a receiver to each output plugin
        self.0.iter_mut().for_each(|output| {
            match output {
                Output::Stdout(ref mut p) => p._receiver = Some(r.clone()),
            };
        });

        // run each output plugin
        for output in self.0 {
            tokio::spawn(output);
        }

        // for every message sent to the `output` block, send to each output separately
        let broadcast = outputs_receiver.for_each(move |message| {

            // if let Err(e) = s.send(message) {
            //     eprintln!("{}", e);
            // }
            
            println!("{:#}", message);
            
            Ok(())
        });

        tokio::spawn(broadcast);

    }
}

impl Future for Output {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {

            let poll = match self {
                Output::Stdout(p) => p.poll(),
            };

            if try_ready!(poll.map_err(|_| ())) == None {
                break
            }
            
        }
        
        Ok(Async::Ready(()))
            
    }
}

mod boundary;
mod circonus;
mod cloudwatch;
mod csv;
mod datadog;
mod datadog_metrics;
mod elastic_app_search;
mod elasticsearch;
mod email;
mod exec;
mod file;
mod ganglia;
mod gelf;
mod google_bigquery;
mod google_pubsub;
mod graphite;
mod http;
mod influxdb;
mod irc;
mod juggernaut;
mod kafka;
mod librato;
mod loggly;
mod lumberjack;
mod metriccatcher;
mod mongodb;
mod nagios;
mod nagios_nsca;
mod opentsdb;
mod pagerduty;
mod pipe;
mod rabbitmq;
mod redis;
mod redmine;
mod riak;
mod riemann;
mod s3;
mod sns;
mod solr_http;
mod sqs;
mod statsd;
mod stdout;
mod stomp;
mod syslog;
mod tcp;
mod timber;
mod udp;
mod webhdfs;
mod websocket;
mod xmpp;
mod zabbix;

pub use self::http::*;
pub use boundary::*;
pub use circonus::*;
pub use cloudwatch::*;
pub use csv::*;
pub use datadog::*;
pub use datadog_metrics::*;
pub use elastic_app_search::*;
pub use elasticsearch::*;
pub use email::*;
pub use exec::*;
pub use file::*;
pub use ganglia::*;
pub use gelf::*;
pub use google_bigquery::*;
pub use google_pubsub::*;
pub use graphite::*;
pub use influxdb::*;
pub use irc::*;
pub use juggernaut::*;
pub use kafka::*;
pub use librato::*;
pub use loggly::*;
pub use lumberjack::*;
pub use metriccatcher::*;
pub use mongodb::*;
pub use nagios::*;
pub use nagios_nsca::*;
pub use opentsdb::*;
pub use pagerduty::*;
pub use pipe::*;
pub use rabbitmq::*;
pub use redis::*;
pub use redmine::*;
pub use riak::*;
pub use riemann::*;
pub use s3::*;
pub use sns::*;
pub use solr_http::*;
pub use sqs::*;
pub use statsd::*;
pub use stdout::*;
pub use stomp::*;
pub use syslog::*;
pub use tcp::*;
pub use timber::*;
pub use udp::*;
pub use webhdfs::*;
pub use websocket::*;
pub use xmpp::*;
pub use zabbix::*;
