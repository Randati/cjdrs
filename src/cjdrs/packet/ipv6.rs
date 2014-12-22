use std::mem;
use packet::ParseResult;

const IPV6_HEADER_LENGTH: uint = 40;


#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(packed)]
pub struct IPv6Header {
	pub version_class_flow: u32,
	pub payload_length_be: u16,
	pub next_header: u8,
	pub hop_limit: u8,
	pub source_addr: [u8, ..16],
	pub destination_addr: [u8, ..16]
}


#[deriving(Copy, Clone, Eq, PartialEq)]
pub struct IPv6Packet<'a> {
	pub header: &'a IPv6Header,
	pub data: &'a [u8]
}

impl<'a> IPv6Packet<'a> {
	pub fn from_buffer(buffer: &[u8]) -> ParseResult<IPv6Packet> {
		if buffer.len() >= IPV6_HEADER_LENGTH {
			Ok(IPv6Packet {
				header: unsafe { mem::transmute(buffer.as_ptr()) },
				data:   buffer.slice_from(IPV6_HEADER_LENGTH)
			})
		} else {
			Err("IPv6 packet too short")
		}
	}
}
