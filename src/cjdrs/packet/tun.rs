use std::mem;
use std::num::Int;
use packet::{ParseResult, PacketData};

const TUN_HEADER_LENGTH: uint = 4;


#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(u16)]
enum TunProtocolType {
	IPv4 = 0x0800,
	IPv6 = 0x86DD,
}


#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(packed)]
struct TunHeader {
	_unused: u16,
	protocol_type: u16
}

impl TunHeader {
	fn is_ipv6(&self) -> bool {
		Int::from_be(self.protocol_type) == TunProtocolType::IPv6 as u16
	}
}


#[deriving(Copy, Clone, Eq, PartialEq)]
pub struct TunPacket<'a, D: PacketData<'a>> {
	slice: &'a [u8],
	header: &'a TunHeader,
	data: D
}

impl<'a, D: PacketData<'a>> TunPacket<'a, D> {
	pub fn get_data(&self) -> &D {
		&self.data
	}
}

impl<'a, D: PacketData<'a>> PacketData<'a> for TunPacket<'a, D> {
	fn from_buffer(buffer: &'a [u8]) -> ParseResult<TunPacket<'a, D>> {
		if buffer.len() < TUN_HEADER_LENGTH {
			return Err("Tun packet too short");
		}

		let header: &TunHeader = unsafe { mem::transmute(buffer.as_ptr()) };

		if !header.is_ipv6() {
			return Err("Tun packet not IPv6");
		}

		let data = try!(PacketData::from_buffer(buffer.slice_from(TUN_HEADER_LENGTH)));

		Ok(TunPacket {
			slice: buffer,
			header: header,
			data: data
		})
	}

	fn as_slice(&self) -> &'a [u8] {
		self.slice
	}
}
