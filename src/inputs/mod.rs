#![allow(unused)]

use futures::sync::mpsc::Sender;
use futures::{try_ready, Future, Poll, Sink, Stream};
use serde_json::Value;
use std::collections::HashMap;

pub struct InputBlock(pub Vec<Input>, pub Sender<Value>);

#[derive(Debug)]
pub enum Input {
    Exec(Exec<'static>),
    Generator(Generator<'static>),
    HttpPoller(HttpPoller<'static>),
    S3(S3<'static>),
}

impl InputBlock {
    pub fn run(self) {
        let (inputs, sender) = (self.0, self.1);

        inputs.into_iter().for_each(|mut input| {
            match &mut input {
                Input::Exec(ref mut p) => p._sender = Some(sender.clone()),
                Input::HttpPoller(ref mut p) => p._sender = Some(sender.clone()),
                Input::S3(ref mut p) => p._sender = Some(sender.clone()),
                Input::Generator(ref mut p) => p._sender = Some(sender.clone()),
            }

            tokio::spawn(input);
        });
    }
}

impl Future for Input {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), Self::Error> {
        loop {
            let poll = match self {
                Input::Exec(p) => p.poll(),
                Input::HttpPoller(p) => p.poll(),
                Input::S3(p) => p.poll(),
                Input::Generator(p) => p.poll(),
            };

            if let Some(message) = try_ready!(poll) {
                if let Some(sender) = match self {
                    Input::Exec(p) => p._sender.to_owned(),
                    Input::HttpPoller(p) => p._sender.to_owned(),
                    Input::S3(p) => p._sender.to_owned(),
                    Input::Generator(p) => p._sender.to_owned(),
                } {
                    try_ready!(sender.send(message).poll().map_err(|_| ()));
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct CommonOptions<'a> {
    add_field: Option<HashMap<&'a str, &'a str>>,
    codec: Option<&'a str>,
    enable_metric: Option<bool>,
    id: Option<&'a str>,
    tags: Option<Vec<&'a str>>,
    r#type: Option<&'a str>,
}

impl<'a> Default for CommonOptions<'a> {
    fn default() -> Self {
        Self {
            add_field: None,
            codec: Some("plain"),
            enable_metric: Some(true),
            id: None,
            tags: None,
            r#type: None,
        }
    }
}

mod azure_event_hubs;
pub use azure_event_hubs::*;
mod beats;
pub use beats;
mod cloudwatch;
pub use cloudwatch::*;
mod couchdb_changes;
pub use couchdb_changes::*;
mod dead_letter_queue;
pub use dead_letter_queue::*;
mod elasticsearch;
pub use elasticsearch::*;
mod exec;
pub use exec::*;
mod file;
pub use file::*;
mod ganglia;
pub use ganglia::*;
mod gelf;
pub use gelf::*;
mod generator;
pub use generator::*;
mod github;
pub use github::*;
mod google_cloud_storage;
pub use google_cloud_storage::*;
mod google_pubsub;
pub use google_pubsub::*;
mod graphite;
pub use graphite::*;
mod heartbeat;
pub use heartbeat::*;
mod http;
pub use http::*;
mod http_poller;
pub use http_poller::*;
mod imap;
pub use imap::*;
mod irc;
pub use irc::*;
mod jdbc;
pub use jdbc::*;
mod jms;
pub use jms::*;
mod jmx;
pub use jmx::*;
mod kafka;
pub use kafka::*;
mod kinesis;
pub use kinesis::*;
mod log4j;
pub use log4j::*;
mod lumberjack;
pub use lumberjack::*;
mod meetup;
pub use meetup::*;
mod pipe;
pub use pipe::*;
mod puppett_facter;
pub use puppett_facter::*;
mod rabbitmq;
pub use rabbitmq::*;
mod redis;
pub use redis::*;
mod relp;
pub use relp::*;
mod rss;
pub use rss::*;
mod s3;
pub use s3::*;
mod salesforce;
pub use salesforce::*;
mod snmp;
pub use snmp::*;
mod snmptrap;
pub use snmptrap::*;
mod sqlite;
pub use sqlite::*;
mod sqs;
pub use sqs::*;
mod stdin;
pub use stdin::*;
mod stomp;
pub use stomp::*;
mod syslog;
pub use syslog::*;
mod tcp;
pub use tcp::*;
mod twitter;
pub use twitter::*;
mod udp;
pub use udp::*;
mod unix;
pub use unix::*;
mod varnishlog;
pub use varnishlog::*;
mod websocket;
pub use websocket::*;
mod wmi;
pub use wmi::*;
mod xmpp;
pub use xmpp::*;
