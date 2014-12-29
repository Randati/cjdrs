#![feature(globs)]

extern crate mio;
extern crate sodiumoxide;
extern crate "rustc-serialize" as serialize;
extern crate tuntap;

pub use address::Address;
pub use event_handler::{EventHandler, EventReceiver};
pub use identity::{
	PrivateIdentity,
	PublicIdentity,
	PrivateKey,
	PublicKey};
pub use interface::NetInterface;
pub use packet::Packet;
pub use route::Route;
pub use router::Router;

pub mod encoding_scheme;
pub mod interface;
pub mod packet;
pub mod util;

mod address;
mod event_handler;
mod identity;
mod route;
mod router;
mod switch;
