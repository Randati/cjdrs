pub use self::tun::Tun;
pub use self::udp::Udp;
use EventReceiver;

mod tun;
mod udp;


pub trait NetInterface: EventReceiver {
	fn send_message(&self, msg: &str);
	fn receive_message(&self) -> String;
}
