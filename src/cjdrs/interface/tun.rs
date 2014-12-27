extern crate tuntap;
extern crate mio;

use std::os::unix::prelude::AsRawFd;
use self::mio::{IoHandle, IoDesc};
use self::tuntap::{TunTap, Tun};
use Address;
use EventReceiver;
use packet::{TunPacket, IPv6Packet, RawPacket, ParseResult, PacketData};


pub struct Tun {
	tun: TunTap,
	io_desc: mio::IoDesc
}


impl Tun {
	pub fn new(address: &Address) -> Tun {
		let tun = TunTap::create(Tun);
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

	fn receive(&mut self) {
		let mut buffer = [0u8, ..1500];
		let data_slice = self.tun.read(&mut buffer).ok().expect("Reading did not succeed");

		let packet: ParseResult<TunPacket<IPv6Packet<RawPacket>>> =
			PacketData::from_buffer(data_slice);

		match packet {
			Ok(tun_packet) => {
				let ipv6_packet = tun_packet.get_data();
				let destination = ipv6_packet.get_destination();
				println!("Tun -> {}", destination);
			},
			Err(e) => println!("Couldn't parse packet: {}", e)
		}
	}
}

impl mio::IoHandle for Tun {
	fn desc(&self) -> &IoDesc {
		&self.io_desc
	}
}
