pub use self::tun::Tun;
pub use self::udp::Udp;
use EventReceiver;
use Task;

mod tun;
mod udp;


pub trait NetInterface: EventReceiver {
	// fn send_message(&self, msg: &str);
	fn receive_message<'a>(&'a mut self, buffer: &'a mut [u8]) -> Option<Task>;
}
