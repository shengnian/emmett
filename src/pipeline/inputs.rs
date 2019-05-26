#![allow(unused)]

use futures::sync::mpsc::{channel, Receiver, Sender};
use futures::{try_ready, Future, Poll, Sink, Stream};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Input {
    Exec(Exec, Option<Sender<Value>>),
    Generator(Generator, Option<Sender<Value>>),
    HttpPoller(HttpPoller, Option<Sender<Value>>),
    S3(S3, Option<Sender<Value>>),
}

impl Future for Input {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), Self::Error> {
        loop {
            
            debug!("Polling Input plugins.");

            let poll = match self {
                Input::Exec(p, _) => p.poll(),
                Input::HttpPoller(p, _) => p.poll(),
                Input::S3(p, _) => p.poll(),
                Input::Generator(p, _) => p.poll(),
            };


            if let Some(message) = try_ready!(poll) {

                debug!("Received message from Input plugin.");

                    if let Some(sender) = match self {
                    Input::Exec(_, s) => s,
                    Input::HttpPoller(_, s) => s,
                    Input::S3(_, s) => s,
                    Input::Generator(_, s) => s,
                } {
                    try_ready!(sender.send(message).poll().map_err(|_| ()));
                }
            }
        }
    }
}

pub struct InputBlock(pub Vec<Input>);

impl InputBlock {
    pub fn run(self) -> Receiver<Value> {

        let (input_sender, filter_receiver) = channel(1_024);
        
        self.0.into_iter().for_each(|mut input| {

            match input {
                Input::Exec(_, ref mut s) => *s = Some(input_sender.clone()),
                Input::HttpPoller(_, ref mut s) => *s = Some(input_sender.clone()),
                Input::S3(_, ref mut s) => *s = Some(input_sender.clone()),
                Input::Generator(_, ref mut s) => *s = Some(input_sender.clone()),
            }

            tokio::spawn(input);

        });
        
        filter_receiver

    }
}

#[derive(Debug)]
pub struct CommonOptions {
    add_field: Option<HashMap<String, String>>,
    codec: Option<String>,
    enable_metric: Option<bool>,
    id: Option<String>,
    tags: Option<Vec<String>>,
    r#type: Option<String>,
}

impl Default for CommonOptions {
    fn default() -> Self {
        Self {
            add_field: None,
            codec: Some("plain".to_string()),
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
