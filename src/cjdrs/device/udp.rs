use mio;
use mio::net::SockAddr;
use mio::net::udp::UdpSocket;
use mio::{event, IoReader};
use mio::buf::SliceBuf;
use mio::net::UnconnectedSocket;
use CjdrsResult;
use CjdrsError;
use EventReceiver;
use NetDevice;
use Task;
use packet;


#[derive(Debug)]
pub struct Udp {
	send_sock: UdpSocket,
	recv_sock: UdpSocket
}

impl Udp {
	pub fn create(bind: &str) -> CjdrsResult<Udp> {
		let send_sock = try!(UdpSocket::v4());
		let recv_sock = try!(UdpSocket::v4());

		let bind_addr = match SockAddr::parse(bind) {
			Some(a) => a,
			None => fail!(CjdrsError::InvalidBindAddress(bind.to_string()))
		};

		try!(recv_sock.bind(&bind_addr));

		Ok(Udp {
			send_sock: send_sock,
			recv_sock: recv_sock
		})
	}
}

impl NetDevice for Udp {
	fn send_message(&mut self, message: &[u8], to: Option<&SockAddr>) -> CjdrsResult<()> {
		let address = match to {
			Some(a) => a,
			None => unreachable!()
		};

		let mut buf = SliceBuf::wrap(message);
		try!(self.send_sock.send_to(&mut buf, address));
		Ok(())
	}

	fn receive_message<'a>(&'a mut self, buffer: &'a mut [u8]) -> Option<Task> {
		let len = self.recv_sock.read_slice(buffer).unwrap().unwrap();
		let data = &buffer[..len];

		match packet::CryptoAuth::from_buffer(data) {
			Ok(ca_packet) => {
				Some(Task::HandleIncomingPacket(ca_packet))
			},
			Err(e) => {
				println!("Received an invalid packet from udp device: {}", e);
				None
			}
		}
	}
}


impl EventReceiver for Udp {
	fn register(&self, event_loop: &mut mio::EventLoop<usize, ()>, token: mio::Token)
	           -> mio::MioResult<()> {
		event_loop.register_opt(&self.recv_sock, token, event::READABLE, event::EDGE)
	}

	fn receive<'a>(&'a mut self, buffer: &'a mut [u8]) -> Option<Task> {
		self.receive_message(buffer)
	}
}
