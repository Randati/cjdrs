use std::mem;
use packet::{ParseResult, PacketData};

const DUCTTAPE_HEADER_LENGTH: uint = 28;


#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(packed)]
struct DucttapeHeader {
	session_layer: u32,
	_switch_header: u32,
	_ipv6_header: u32,
	next_hop_receive_handle: u32,
	receive_handle: u32,
	switch_label: u64
}

#[deriving(Copy, Clone, Eq, PartialEq)]
pub struct DucttapePacket<'a> {
	slice: &'a [u8],
	header: &'a DucttapeHeader,
	data: &'a [u8]
}

impl<'a> DucttapePacket<'a> {
	pub fn get_data(&self) -> &'a [u8] {
		self.data
	}
}

impl<'a> PacketData<'a> for DucttapePacket<'a> {
	fn from_buffer(buffer: &'a [u8]) -> ParseResult<DucttapePacket<'a>> {
		if buffer.len() < DUCTTAPE_HEADER_LENGTH {
			return Err("Ducttape packet too short")
		}

		Ok(DucttapePacket {
			slice: buffer,
			header: unsafe { mem::transmute(buffer.as_ptr()) },
			data: buffer.slice_from(DUCTTAPE_HEADER_LENGTH)
		})
	}

	fn as_slice(&self) -> &'a [u8] {
		self.slice
	}
}
