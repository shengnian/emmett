use futures::{sync::mpsc::UnboundedReceiver, Stream};
use serde_json::Value;

pub struct OutputBlock(pub Vec<Output>);

#[derive(Debug)]
pub enum Output {
    Stdout(Stdout),
}

impl OutputBlock {
    pub fn run(self, outputs_receiver: UnboundedReceiver<Value>) {
       
        let broadcast = outputs_receiver.for_each(move |message| {

            self.0.iter().for_each(move |output| {
                match output {
                    Output::Stdout(p) => p.run(message.clone()),
                };
            });
                        
            Ok(())
        });

        tokio::spawn(broadcast);

    }
}

trait Run {
    fn run(&self, input: Value);
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
