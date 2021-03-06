#[macro_use] extern crate log;

pub extern crate lavalink;

extern crate evzht9h3nznqzwl as websocket;
extern crate hyper;
extern crate parking_lot;
extern crate percent_encoding;
extern crate serde;
extern crate serde_json;
extern crate serenity;

pub mod nodes;
pub mod player;

mod error;
mod listener;
mod prelude;

pub use error::{Error, Result};
pub use listener::AudioPlayerListener;