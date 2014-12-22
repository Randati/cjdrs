use std::mem;
use std::num::Int;
use packet::ParseResult;

const TUN_HEADER_LENGTH: uint = 4;


#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(u16)]
enum TunProtocolType {
	IPv4  = 0x0800,
	IPv6  = 0x08DD,
}


#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(packed)]
pub struct TunHeader {
	pub _unused: u16,
	pub protocol_type: u16
}

// TODO Reduce duplicated code
impl TunHeader {
	pub fn is_ipv4(&self) -> bool {
		Int::from_be(self.protocol_type) == TunProtocolType::IPv4 as u16
	}

	pub fn is_ipv6(&self) -> bool {
		Int::from_be(self.protocol_type) == TunProtocolType::IPv6 as u16
	}
}


#[deriving(Copy, Clone, Eq, PartialEq)]
pub struct TunPacket<'a> {
	pub header: &'a TunHeader,
	pub data: &'a [u8]
}

impl<'a> TunPacket<'a> {
	pub fn from_buffer(buffer: &[u8]) -> ParseResult<TunPacket> {
		if buffer.len() >= TUN_HEADER_LENGTH {
			Ok(TunPacket {
				header: unsafe { mem::transmute(buffer.as_ptr()) },
				data:   buffer.slice_from(TUN_HEADER_LENGTH)
			})
		} else {
			Err("Tun packet too short")
		}
	}
}
