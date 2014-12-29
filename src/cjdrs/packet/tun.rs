use std::mem;
use packet::{ParseResult, Packet, buffer_to_type};
use util::BigEndian;

pub const TUN_HEADER_LENGTH: uint = 4;


#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(u16)]
enum TunProtocolType {
	IPv4 = 0x0800,
	IPv6 = 0x86DD,
}


#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(packed)]
pub struct TunHeader {
	_unused: BigEndian<u16>,
	protocol_type: BigEndian<u16>
}

impl TunHeader {
	fn is_ipv6(&self) -> bool {
		self.protocol_type.val() == TunProtocolType::IPv6 as u16
	}
}


#[deriving(Copy, Clone, Eq, PartialEq)]
pub struct Tun<'a, D: Packet<'a>> {
	slice: &'a [u8],
	header: &'a TunHeader,
	data: D
}

impl<'a, D: Packet<'a>> Tun<'a, D> {
	pub fn get_data(&self) -> &D {
		&self.data
	}
}

impl<'a, D: Packet<'a>> Packet<'a> for Tun<'a, D> {
	fn from_buffer(buffer: &'a [u8]) -> ParseResult<Tun<'a, D>> {
		let header: &TunHeader = try!(buffer_to_type(buffer));

		if !header.is_ipv6() {
			return Err("Tun packet not IPv6");
		}

		let data = try!(Packet::from_buffer(buffer.slice_from(TUN_HEADER_LENGTH)));

		Ok(Tun {
			slice: buffer,
			header: header,
			data: data
		})
	}

	fn as_slice(&self) -> &'a [u8] {
		self.slice
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::mem::size_of;
	
	#[test]
	fn test_sizeof() {
		assert_eq!(size_of::<TunHeader>(), TUN_HEADER_LENGTH);
	}
}
