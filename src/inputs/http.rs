#![allow(unused)]

use futures::{Async, Future, Poll, Stream};
use std::fmt;
use std::time::Duration;
use tokio::timer::Interval;

pub struct HttpInput;

impl HttpInput {
    pub fn run() {}
}
