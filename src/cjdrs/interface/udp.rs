use mio;
use mio::net::SockAddr;
use mio::net::udp::UdpSocket;
use mio::{event, IoReader};
use EventReceiver;
use NetInterface;
use Task;
use packet;


pub struct Udp {
	send_sock: UdpSocket,
	recv_sock: UdpSocket
}

impl Udp {
	pub fn create() -> Udp {
		let send_sock = UdpSocket::v4().unwrap();
		let recv_sock = UdpSocket::v4().unwrap();

		let bind_addr = SockAddr::parse("0.0.0.0:3300".as_slice())
			.expect("could not parse InetAddr for localhost");

		recv_sock.bind(&bind_addr).unwrap();


		Udp {
			send_sock: send_sock,
			recv_sock: recv_sock
		}
	}
}

impl NetInterface for Udp {
	fn send_message(&self, msg: &str) {
		unimplemented!();
	}

	fn receive_message(&self) -> String {
		unimplemented!();
	}
}


impl EventReceiver for Udp {
	fn register(&self, event_loop: &mut mio::EventLoop<uint, ()>, token: mio::Token)
	           -> mio::MioResult<()> {
		event_loop.register_opt(&self.recv_sock, token, event::READABLE, event::EDGE)
	}

	fn receive<'a>(&'a mut self, buffer: &'a mut [u8]) -> Option<Task> {
		let len = self.recv_sock.read_slice(buffer).unwrap().unwrap();
		let data = buffer.slice_to(len);

		match packet::CryptoAuth::from_buffer(data) {
			Ok(ca_packet) => {
				Some(Task::HandleIncomingPacket(ca_packet))
			},
			Err(e) => {
				println!("Received an invalid packet from udp interface: {}", e);
				None
			}
		}
	}
}
