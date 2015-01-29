pub use self::tun::Tun;
pub use self::udp::Udp;

use std::fmt;
use mio::net::SockAddr;
use CjdrsResult;
use EventReceiver;
use Task;

mod tun;
mod udp;


pub trait NetDevice: EventReceiver + fmt::Debug {
	fn send_message(&mut self, message: &[u8], to: Option<&SockAddr>) -> CjdrsResult<()>;
	fn receive_message<'a>(&'a mut self, buffer: &'a mut [u8]) -> Option<Task>;
}
