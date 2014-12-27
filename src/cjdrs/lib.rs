#![feature(globs)]

pub use address::Address;
pub use event_handler::{EventHandler, EventReceiver};
pub use identity::{
	PrivateIdentity,
	PublicIdentity,
	PrivateKey,
	PublicKey};
pub use interface::NetInterface;
pub use route::Route;
pub use router::Router;

pub mod encoding_scheme;
pub mod interface;
pub mod packet;

mod address;
mod event_handler;
mod identity;
mod route;
mod router;
mod switch;
