#![allow(unused)]

use futures::sync::mpsc::Sender;
use futures::{try_ready, Future, Poll, Sink, Stream};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Input {
    Exec(Exec<'static>),
    Generator(Generator),
    HttpPoller(HttpPoller<'static>),
    S3(S3<'static>),
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

pub struct InputBlock(pub Vec<Input>, pub Sender<Value>);

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
mod beats;
mod cloudwatch;
mod couchdb_changes;
mod dead_letter_queue;
mod elasticsearch;
mod exec;
mod file;
mod ganglia;
mod gelf;
mod generator;
mod github;
mod google_cloud_storage;
mod google_pubsub;
mod graphite;
mod heartbeat;
mod http;
mod http_poller;
mod imap;
mod irc;
mod jdbc;
mod jms;
mod jmx;
mod kafka;
mod kinesis;
mod log4j;
mod lumberjack;
mod meetup;
mod pipe;
mod puppett_facter;
mod rabbitmq;
mod redis;
mod relp;
mod rss;
mod s3;
mod salesforce;
mod snmp;
mod snmptrap;
mod sqlite;
mod sqs;
mod stdin;
mod stomp;
mod syslog;
mod tcp;
mod twitter;
mod udp;
mod unix;
mod varnishlog;
mod websocket;
mod wmi;
mod xmpp;

pub use self::http::*;
pub use azure_event_hubs::*;
pub use beats::*;
pub use cloudwatch::*;
pub use couchdb_changes::*;
pub use dead_letter_queue::*;
pub use elasticsearch::*;
pub use exec::*;
pub use file::*;
pub use ganglia::*;
pub use gelf::*;
pub use generator::*;
pub use github::*;
pub use google_cloud_storage::*;
pub use google_pubsub::*;
pub use graphite::*;
pub use heartbeat::*;
pub use http_poller::*;
pub use imap::*;
pub use irc::*;
pub use jdbc::*;
pub use jms::*;
pub use jmx::*;
pub use kafka::*;
pub use kinesis::*;
pub use log4j::*;
pub use lumberjack::*;
pub use meetup::*;
pub use pipe::*;
pub use puppett_facter::*;
pub use rabbitmq::*;
pub use redis::*;
pub use relp::*;
pub use rss::*;
pub use s3::*;
pub use salesforce::*;
pub use snmp::*;
pub use snmptrap::*;
pub use sqlite::*;
pub use sqs::*;
pub use stdin::*;
pub use stomp::*;
pub use syslog::*;
pub use tcp::*;
pub use twitter::*;
pub use udp::*;
pub use unix::*;
pub use varnishlog::*;
pub use websocket::*;
pub use wmi::*;
pub use xmpp::*;
