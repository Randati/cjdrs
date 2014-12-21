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


	// TODO Move to tuntap
	pub fn read_incoming_packet(&self) {
		let mut buf = [0u8, ..BUFFER_SIZE];
		let res = self.reader.read_slice(&mut buf);

		match res {
			Ok(maybe_block_res) => match maybe_block_res {
				mio::NonBlock::Ready(len) => {
					if len <= BUFFER_SIZE {
						self.handle_incoming_packet(buf.slice_to(len));
					} else {
						panic!("Buffer too small! Buffer size {}, needed {}",
						       BUFFER_SIZE, len);
					}
				},
				mio::NonBlock::WouldBlock => println!("Would block?")
			},
			Err(e) => panic!("Error: {}", e)
		}
	}

	pub fn handle_incoming_packet(&self, bytes: &[u8]) {
		println!("Got packet!");

		if bytes.len() < TUN_HEADER_LENGTH + IPV6_HEADER_LENGTH {
			panic!("Packet too small");
		}
	 
		let bytes = bytes.slice_from(TUN_HEADER_LENGTH);
		let ipv6_header_slice = bytes.slice_to(IPV6_HEADER_LENGTH);

		let ipv6_header: &IPv6Header = unsafe { mem::transmute(ipv6_header_slice.as_ptr()) };
		let payload_length = Int::from_be(ipv6_header.payload_length_be) as uint;

		if bytes.len() != IPV6_HEADER_LENGTH + payload_length {
			panic!("Packet is lying about the length!");
		}

		let payload = bytes.slice_from(IPV6_HEADER_LENGTH);

		let source = Address::from_bytes(&ipv6_header.source_addr).unwrap();
		let destination = Address::from_bytes(&ipv6_header.destination_addr).unwrap();

		// let route = router.get_route(&destination);

		println!("Source: {}", source);
		println!("destination: {}", destination);
		println!("payload.len: {}", payload.len());

		for &x in payload.iter() {
			print!("{0:02X} ", x);
		}
		println!("")
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

const TUN_HEADER_LENGTH: uint = 4;
const IPV6_HEADER_LENGTH: uint = 40;

