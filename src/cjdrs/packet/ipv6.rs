use std::mem;
use std::num::Int;
use packet::ParseResult;
use Address;

const IPV6_HEADER_LENGTH: uint = 40;


#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(packed)]
pub struct IPv6Header {
	pub version_class_flow: u16,
	pub flow_label_low: u16,
	pub payload_length_be: u16,
	pub next_header: u8,
	pub hop_limit: u8,
	pub source_addr: [u8, ..16],
	pub destination_addr: [u8, ..16]
}

impl IPv6Header {
	pub fn get_version(&self) -> uint {
		((Int::from_be(self.version_class_flow) & 0xF000) >> 12) as uint
	}
}

#[deriving(Copy, Clone, Eq, PartialEq)]
pub struct IPv6Packet<'a> {
	pub header: &'a IPv6Header,
	pub data: &'a [u8]
}

impl<'a> IPv6Packet<'a> {
	pub fn from_buffer(buffer: &[u8]) -> ParseResult<IPv6Packet> {
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

		Ok(IPv6Packet {
			header: header,
			data:   buffer.slice_from(IPV6_HEADER_LENGTH)
		})
	}
}
