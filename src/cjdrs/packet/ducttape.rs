use std::mem;
use packet::ParseResult;

const DUCTTAPE_HEADER_LENGTH: uint = 28;


#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(packed)]
pub struct DucttapeHeader {
	pub session_layer: u32,
	pub _switch_header: u32,
	pub _ipv6_header: u32,
	pub next_hop_receive_handle: u32,
	pub receive_handle: u32,
	pub switch_label: u64
}

#[deriving(Copy, Clone, Eq, PartialEq)]
pub struct DucttapePacket<'a> {
	pub header: &'a DucttapeHeader,
	pub data: &'a [u8]
}

impl<'a> DucttapePacket<'a> {
	pub fn from_buffer(buffer: &[u8]) -> ParseResult<DucttapePacket> {
		if buffer.len() >= DUCTTAPE_HEADER_LENGTH {
			Ok(DucttapePacket {
				header: unsafe { mem::transmute(buffer.as_ptr()) },
				data:   buffer.slice_from(DUCTTAPE_HEADER_LENGTH)
			})
		} else {
			Err("Ducttape packet too short")
		}
	}
}
