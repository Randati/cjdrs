extern crate tuntap;
extern crate mio;

use std::os::unix::prelude::AsRawFd;
use self::mio::{IoHandle, IoDesc};
use self::tuntap::{TunTap, Tun};
use Address;
use packet::{TunPacket, IPv6Packet, RawPacket, ParseResult, PacketData};


pub struct TunInterface {
	tun: TunTap,
	io_desc: mio::IoDesc
}


impl TunInterface {
	pub fn new(address: &Address) -> TunInterface {
		let tun = TunTap::create(Tun);
		let fd = tun.file.as_raw_fd();

		tun.add_address(address.as_slice());

		TunInterface {
			tun: tun,
			io_desc: mio::IoDesc { fd: fd }
		}
	}

	pub fn get_name(&self) -> String {
		self.tun.get_name()
	}

	pub fn read_incoming_packet<'a>(&'a mut self, empty_buffer: &'a mut [u8])
	       -> ParseResult<TunPacket<IPv6Packet<RawPacket>>> {
		let tun_data = self.tun.read(empty_buffer).ok().expect("Reading did not succeed");
		PacketData::from_buffer(tun_data)
	}
}

impl mio::IoHandle for TunInterface {
	fn desc(&self) -> &IoDesc {
		&self.io_desc
	}
}
