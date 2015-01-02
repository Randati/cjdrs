use std::os::unix::prelude::AsRawFd;
use mio;
use tuntap::{TunTap, Tun};
use Address;
use EventReceiver;
use packet;
use Task;


pub struct Tun {
	tun: TunTap,
	io_desc: mio::IoDesc
}


impl Tun {
	pub fn new(name: &str, address: &Address) -> Tun {
		let tun = TunTap::create_named(Tun, name);
		let fd = tun.file.as_raw_fd();

		tun.add_address(address.as_slice());

		Tun {
			tun: tun,
			io_desc: mio::IoDesc { fd: fd }
		}
	}

	pub fn get_name(&self) -> String {
		self.tun.get_name()
	}
}


impl EventReceiver for Tun {
	fn register(&self, event_loop: &mut mio::EventLoop<uint, ()>, token: mio::Token)
	           -> mio::MioResult<()> {
		event_loop.register(self, token)
	}

	fn receive<'a>(&'a mut self, buffer: &'a mut [u8]) -> Option<Task> {
		let data_slice = self.tun.read(buffer).ok().expect("Reading did not succeed");
		let packet = packet::Tun::from_buffer(data_slice);

		match packet {
			Ok(tun_packet) => {
				let ipv6_packet = tun_packet.get_data();
				Some(Task::HandleOutgoingPacket(*ipv6_packet))
			},
			Err(e) => {
				println!("Received an invalid packet from tun interface: {}", e);
				None
			}
		}
	}
}

impl mio::IoHandle for Tun {
	fn desc(&self) -> &mio::IoDesc {
		&self.io_desc
	}
}
