extern crate tuntap;
extern crate libc;
extern crate mio;

use std::os::unix::prelude::AsRawFd;
use self::mio::IoReader;
use self::tuntap::{TunTap, Tun};
use Address;
use packet::{ TunPacket, IPv6Packet, DucttapePacket, ParseResult };


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

	pub fn read_incoming_packet<'a>(&'a mut self, empty_buffer: &'a mut [u8]) -> ParseResult<DucttapePacket> {
		let tun_data = self.tun.read(empty_buffer).ok().expect("Reading did not succeed");

		// TODO Check correct header

		let tun_packet      = try!(TunPacket::from_buffer(tun_data));
		let ipv6_packet     = try!(IPv6Packet::from_buffer(tun_packet.data));
		let ducttape_packet = try!(DucttapePacket::from_buffer(ipv6_packet.data));


		let source = Address::from_bytes(&ipv6_packet.header.source_addr).unwrap();
		let destination = Address::from_bytes(&ipv6_packet.header.destination_addr).unwrap();
		
		println!("{} -> {}", source, destination);
		

		// route = get_route()
		// next_node = get_next_node(route)
		// if next_node == destination
		//		send_to_router()
		// else
		//		crypto auth stuff
		//		interface.send_message()

		Ok(ducttape_packet)
	}
}
