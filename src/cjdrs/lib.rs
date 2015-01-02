#![feature(globs)]

#[cfg(test)] extern crate test;
extern crate mio;
extern crate sodiumoxide;
extern crate "rustc-serialize" as rustc_serialize;
extern crate tuntap;

pub use address::Address;
pub use config::Config;
pub use event_handler::{EventHandler, EventReceiver, Task};
pub use identity::{
	PrivateIdentity,
	PublicIdentity,
	PrivateKey,
	PublicKey};
pub use interface::NetInterface;
pub use route::Route;
pub use router::Router;
pub use util::debug;

pub mod crypto;
pub mod encoding_scheme;
pub mod interface;
pub mod packet;
pub mod util;

mod address;
mod config;
mod event_handler;
mod identity;
mod route;
mod router;
mod switch;


pub fn init() {
	sodiumoxide::init();
}
