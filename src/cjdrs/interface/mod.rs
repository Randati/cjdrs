pub use self::tun::Tun;
pub use self::udp::Udp;
use EventReceiver;
use Task;

mod tun;
mod udp;


// TODO Show blocked by https://github.com/rust-lang/rust/issues/20676
pub trait NetInterface: EventReceiver {
	// fn send_message(&self, msg: &str);
	fn receive_message<'a>(&'a mut self, buffer: &'a mut [u8]) -> Option<Task>;
}
