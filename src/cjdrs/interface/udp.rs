extern crate mio;

use std::str;
use self::mio::net::SockAddr;
use self::mio::net::udp::UdpSocket;
use self::mio::{event, IoReader};
use EventReceiver;
use NetInterface;


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

	fn receive(&mut self) {
		let mut buffer = [0u8, ..1500];

		match self.recv_sock.read_slice(&mut buffer).unwrap() {
			mio::NonBlock::Ready(len) => {
				let data = buffer.slice_to(len);
				let s = str::from_utf8(data).unwrap();
				println!("Got: '{}'", s);
			},
			mio::NonBlock::WouldBlock => panic!()
		};
	}
}
