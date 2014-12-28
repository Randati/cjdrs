use std::mem;
use packet::{ParseResult, Packet};

pub const DUCTTAPE_HEADER_LENGTH: uint = 28;


#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(packed)]
pub struct DucttapeHeader {
	session_layer: u32,
	_switch_header: u32,
	_ipv6_header: u32,
	next_hop_receive_handle: u32,
	receive_handle: u32,
	switch_label: u64
}

#[deriving(Copy, Clone, Eq, PartialEq)]
pub struct Ducttape<'a> {
	slice: &'a [u8],
	header: &'a DucttapeHeader,
	data: &'a [u8]
}

impl<'a> Ducttape<'a> {
	pub fn get_data(&self) -> &'a [u8] {
		self.data
	}
}

impl<'a> Packet<'a> for Ducttape<'a> {
	fn from_buffer(buffer: &'a [u8]) -> ParseResult<Ducttape<'a>> {
		if buffer.len() < DUCTTAPE_HEADER_LENGTH {
			return Err("Ducttape packet too short")
		}

		Ok(Ducttape {
			slice: buffer,
			header: unsafe { mem::transmute(buffer.as_ptr()) },
			data: buffer.slice_from(DUCTTAPE_HEADER_LENGTH)
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
		assert_eq!(size_of::<DucttapeHeader>(), DUCTTAPE_HEADER_LENGTH);
	}
}
