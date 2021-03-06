/*! The main Fastnet module.

This is the low-level API.  If your goal is extremely high-performance usage, this is the API you want.  See the blocking module for a simpler API which is less annoying for common use cases.*/
#![allow(warnings)]

extern crate byteorder;
extern crate mio;
extern crate crc;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate time;

mod constants;
mod packets;
mod server;
mod status_translator;
mod async;
mod frame;

pub use async::*;
