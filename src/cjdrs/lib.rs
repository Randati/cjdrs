#![feature(collections, core, hash, io, std_misc)]

#[cfg(test)] extern crate test;
extern crate mio;
extern crate sodiumoxide;
extern crate "rustc-serialize" as rustc_serialize;
extern crate tuntap;

pub use address::Address;
pub use config::Config;
pub use error::{CjdrsError, CjdrsResult};
pub use event_handler::{EventHandler, EventReceiver, Task};
pub use identity::{
	PrivateIdentity,
	PublicIdentity,
	PrivateKey,
	PublicKey};
pub use device::NetDevice;
pub use route::Route;
pub use router::Router;
pub use util::debug;

mod macros;

pub mod crypto;
pub mod encoding_scheme;
pub mod device;
pub mod packet;
pub mod util;

mod address;
mod config;
mod error;
mod event_handler;
mod identity;
mod route;
mod router;


pub fn init() {
	sodiumoxide::init();
}
