#![crate_type = "lib"]
#![feature(fnbox)]

#[macro_use]
extern crate log;
#[cfg(test)]
extern crate env_logger;
#[macro_use]
extern crate quick_error;
extern crate mio;

mod errors;

pub use mio::EventLoop;
pub use errors::{Result, Error};
