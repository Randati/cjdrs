use std::ffi::CString;
use std::os::unix::prelude::AsRawFd;
use mio;
use mio::net::SockAddr;
use tuntap::{TunTap, Tun};
use Address;
use CjdrsResult;
use EventReceiver;
use NetDevice;
use packet;
use Task;


#[derive(Debug)]
pub struct Tun {
	tun: TunTap,
	io_desc: mio::IoDesc
}


impl Tun {
	pub fn new(name: &str, address: &Address) -> Tun {
		let tun = TunTap::create_named(Tun, &CString::from_slice(name.as_bytes()));
		let fd = tun.file.as_raw_fd();

		tun.add_address(address.as_slice());

		Tun {
			tun: tun,
			io_desc: mio::IoDesc { fd: fd }
		}
	}

	pub fn get_name(&self) -> String {
		let name = self.tun.get_name();
		let mut name_vec = Vec::with_capacity(name.len());
		name_vec.push_all(name.as_bytes());

		match String::from_utf8(name_vec) {
			Ok(s) => s,
			Err(e) => panic!(e)
		}
	}
}

impl NetDevice for Tun {
	fn send_message(&mut self, message: &[u8], to: Option<&SockAddr>) -> CjdrsResult<()> {
		assert!(to.is_none());

		Ok(try!(self.tun.write(message)))
	}

	fn receive_message<'a>(&'a mut self, buffer: &'a mut [u8]) -> Option<Task> {
		let data_slice = self.tun.read(buffer).ok().expect("Reading did not succeed");
		let packet = packet::Tun::from_buffer(data_slice);

		match packet {
			Ok(tun_packet) => {
				let ipv6_packet = tun_packet.get_data();
				Some(Task::HandleOutgoingPacket(*ipv6_packet))
			},
			Err(e) => {
				println!("Received an invalid packet from tun device: {}", e);
				None
			}
		}
	}
}

impl EventReceiver for Tun {
	fn register(&self, event_loop: &mut mio::EventLoop<usize, ()>, token: mio::Token)
	           -> mio::MioResult<()> {
		event_loop.register(self, token)
	}

	fn receive<'a>(&'a mut self, buffer: &'a mut [u8]) -> Option<Task> {
		self.receive_message(buffer)
	}
}

impl mio::IoHandle for Tun {
	fn desc(&self) -> &mio::IoDesc {
		&self.io_desc
	}
}
