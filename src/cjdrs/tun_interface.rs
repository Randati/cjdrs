extern crate tuntap;
extern crate libc;
extern crate mio;

use std::os::unix::prelude::AsRawFd;
use self::mio::IoReader;
use self::tuntap::{TunTap, Tun};
use Address;
use packet::{TunPacket, IPv6Packet, DucttapePacket, ParseResult, PacketData};


pub struct TunInterface {
	tun: TunTap,
	pub reader: mio::io::PipeReader,
	pub writer: mio::io::PipeWriter
}


impl TunInterface {
	pub fn new(address: &Address) -> TunInterface {
		let tun = TunTap::create(Tun);
		tun.add_address(address.as_slice());

		let fd = tun.file.as_raw_fd();
		TunInterface {
			tun: tun,
			reader: mio::io::PipeReader { desc: mio::os::IoDesc { fd: fd } },
			writer: mio::io::PipeWriter { desc: mio::os::IoDesc { fd: fd } }
		}
	}

	pub fn get_name(&self) -> String {
		self.tun.get_name()
	}

	pub fn read_incoming_packet<'a>(&'a mut self, empty_buffer: &'a mut [u8])
	       -> ParseResult<TunPacket<IPv6Packet<DucttapePacket>>> {
		let tun_data = self.tun.read(empty_buffer).ok().expect("Reading did not succeed");
		PacketData::from_buffer(tun_data)
	}
}
