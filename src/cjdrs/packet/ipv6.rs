use std::mem;
use std::num::Int;
use packet::{ParseResult, PacketData};
use Address;

const IPV6_HEADER_LENGTH: uint = 40;


#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(packed)]
struct IPv6Header {
	version_class_flow: u16,
	flow_label_low: u16,
	payload_length_be: u16,
	next_header: u8,
	hop_limit: u8,
	source_addr: [u8, ..16],
	destination_addr: [u8, ..16]
}

impl IPv6Header {
	fn get_version(&self) -> uint {
		((Int::from_be(self.version_class_flow) & 0xF000) >> 12) as uint
	}
}

#[deriving(Copy, Clone, Eq, PartialEq)]
pub struct IPv6Packet<'a, D: PacketData<'a>> {
	slice: &'a [u8],
	header: &'a IPv6Header,
	data: D
}

impl<'a, D: PacketData<'a>> IPv6Packet<'a, D> {
	pub fn get_data(&self) -> &D {
		&self.data
	}

	pub fn get_destination(&self) -> Address {
		Address::from_slice(&self.header.destination_addr).unwrap()
	}
}

impl<'a, D: PacketData<'a>> PacketData<'a> for IPv6Packet<'a, D> {
	fn from_buffer(buffer: &'a [u8]) -> ParseResult<IPv6Packet<'a, D>> {
		if buffer.len() < IPV6_HEADER_LENGTH {
			return Err("IPv6 packet too short")
		}

		let header: &IPv6Header = unsafe { mem::transmute(buffer.as_ptr()) };

		if header.get_version() != 6 {
			return Err("Packet not IPv6")
		}
		if !Address::is_valid(&header.source_addr) {
			return Err("Source address not valid")
		}
		if !Address::is_valid(&header.destination_addr) {
			return Err("Destination address not valid")
		}

		let data = try!(PacketData::from_buffer(buffer.slice_from(IPV6_HEADER_LENGTH)));

		Ok(IPv6Packet {
			slice: buffer,
			header: header,
			data: data
		})
	}

	fn as_slice(&self) -> &'a [u8] {
		self.slice
	}
}
