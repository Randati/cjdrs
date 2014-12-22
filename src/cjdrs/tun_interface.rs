extern crate tuntap;
extern crate libc;
extern crate mio;

use std::mem;
use std::num::Int;
use std::os::unix::prelude::AsRawFd;
use self::mio::IoReader;
use self::tuntap::{TunTap, Tun};
use Address;


const BUFFER_SIZE: uint = 1500;


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

	pub fn handle_incoming_packet(&mut self) {
		let mut buffer = [0u8, ..1500];
		let tun_packet = self.tun.read(&mut buffer).ok().expect("Reading did not succeed");

		// TODO Check correct header

		// TUN header
		if tun_packet.len() < TUN_HEADER_LENGTH + IPV6_HEADER_LENGTH {
			panic!("Packet too small");
		}


		// IPv6 header
		let ipv6_packet = tun_packet.slice_from(TUN_HEADER_LENGTH);
		let ipv6_header: &IPv6Header = unsafe {
			mem::transmute(ipv6_packet.slice_to(IPV6_HEADER_LENGTH).as_ptr())
		};
		
		let source = Address::from_bytes(&ipv6_header.source_addr).unwrap();
		let destination = Address::from_bytes(&ipv6_header.destination_addr).unwrap();
		let payload_length = Int::from_be(ipv6_header.payload_length_be) as uint;


		// Packet payload
		if ipv6_packet.len() != IPV6_HEADER_LENGTH + payload_length {
			panic!("Packet is lying about the length!");
		}
		let payload = ipv6_packet.slice_from(IPV6_HEADER_LENGTH);


		println!("{} -> {}", source, destination);

		// route = get_route()
		// next_node = get_next_node(route)
		// if next_node == destination
		//		send_to_router()
		// else
		//		crypto auth stuff
		//		interface.send_message()
	}
}



#[repr(packed)]
struct IPv6Header {
	version_class_flow: u32,
	payload_length_be: u16,
	next_header: u8,
	hop_limit: u8,
	source_addr: [u8, ..16],
	destination_addr: [u8, ..16]
}

#[repr(packed)]
struct DucttapeHeader {
	session_layer: u32,
	_switch_header: u32,
	_ipv6_header: u32,
	next_hop_receive_handle: u32,
	receive_handle: u32,
	switch_label: u64
}

const TUN_HEADER_LENGTH: uint = 4;
const IPV6_HEADER_LENGTH: uint = 40;

