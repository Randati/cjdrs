pub use self::tun::Tun;
pub use self::udp::Udp;

use mio::net::SockAddr;
use CjdrsResult;
use EventReceiver;
use Task;

mod tun;
mod udp;


// TODO Show blocked by https://github.com/rust-lang/rust/issues/20676
pub trait NetDevice: EventReceiver {
	fn send_message(&mut self, message: &[u8], to: Option<&SockAddr>) -> CjdrsResult<()>;
	fn receive_message<'a>(&'a mut self, buffer: &'a mut [u8]) -> Option<Task>;
}
