use std::mem::size_of;
use packet::{ParseResult, Packet, buffer_to_type};
use packet;
use util::BigEndian;

#[cfg(test)] pub const TUN_HEADER_LENGTH: usize = 4;



#[derive(Show, Copy, Clone, Eq, PartialEq)]
#[repr(packed)]
pub struct TunHeader {
	_unused: BigEndian<u16>,
	protocol_type: BigEndian<u16>
}

impl TunHeader {
	fn is_ipv6(&self) -> bool {
		self.protocol_type.val() == 0x86DD
	}
}



pub type Tun<'a> = Packet<'a, TunHeader, packet::IPv6<'a>>;

impl<'a> Tun<'a> {
	pub fn from_buffer(buffer: &[u8]) -> ParseResult<Tun> {
		let header: &TunHeader = try!(buffer_to_type(buffer));

		if !header.is_ipv6() {
			return Err("Tun packet not IPv6");
		}

		let payload = {
			let data_slice = &buffer[size_of::<TunHeader>()..];
			try!(packet::IPv6::from_buffer(data_slice))
		};

		Ok(Tun {
			slice: buffer,
			header: header,
			data: payload
		})
	}

	pub fn get_data(&self) -> &packet::IPv6<'a> {
		&self.data
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
