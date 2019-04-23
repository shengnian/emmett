mod grok;
pub use grok::*;
mod date;
pub use date::*;
mod geoip;
pub use geoip::*;

#[derive(Debug)]
pub enum Filter {
    Geoip(geoip::GeoipFilter<'static>),
    Date(date::DateFilter)
}

use serde_json::Value;

impl Filter {
    pub fn process(&self, message: Value) -> Value {
        match self {
            Filter::Geoip(p) => p.process(message),
            Filter::Date(p) => p.process(message)
        }
    }
}

// trait Message {
//     fn tag(&mut self, tag: &str) -> &Self {
//         self
//     }
// }

// impl Message for Value {
//     fn tag(&mut self, tag: &str) -> &Self {
//         self
//     }
// }
