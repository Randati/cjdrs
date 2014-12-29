use Address;
use packet::{ParseResult, Packet, buffer_to_type};
use util::BigEndian;

pub const IPV6_HEADER_LENGTH: uint = 40;


#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(packed)]
pub struct IPv6Header {
	version_class_flow: BigEndian<u16>,
	flow_label_low: BigEndian<u16>,
	payload_length: BigEndian<u16>,
	next_header: u8,
	hop_limit: u8,
	source_addr: [u8, ..16],
	destination_addr: [u8, ..16]
}

impl IPv6Header {
	fn get_version(&self) -> uint {
		((self.version_class_flow.val() & 0xF000) >> 12) as uint
	}
}

#[deriving(Copy, Clone, Eq, PartialEq)]
pub struct IPv6<'a, D: Packet<'a>> {
	slice: &'a [u8],
	header: &'a IPv6Header,
	data: D
}

impl<'a, D: Packet<'a>> IPv6<'a, D> {
	pub fn get_data(&self) -> &D {
		&self.data
	}

	pub fn get_destination(&self) -> Address {
		Address::from_slice(&self.header.destination_addr).unwrap()
	}
}

impl<'a, D: Packet<'a>> Packet<'a> for IPv6<'a, D> {
	fn from_buffer(buffer: &'a [u8]) -> ParseResult<IPv6<'a, D>> {
		let header: &IPv6Header = try!(buffer_to_type(buffer));

		if header.get_version() != 6 {
			return Err("Packet not IPv6")
		}
		if !Address::is_valid(&header.source_addr) {
			return Err("Source address not valid")
		}
		if !Address::is_valid(&header.destination_addr) {
			return Err("Destination address not valid")
		}

		let data = try!(Packet::from_buffer(buffer.slice_from(IPV6_HEADER_LENGTH)));

		Ok(IPv6 {
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
		assert_eq!(size_of::<IPv6Header>(), IPV6_HEADER_LENGTH);
	}
}
