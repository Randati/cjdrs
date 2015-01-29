pub use self::ipv6::IPv6;
pub use self::tun::Tun;
pub use self::cryptoauth::CryptoAuth;

use std::mem;

mod ipv6;
mod cryptoauth;
mod tun;

pub type ParseResult<P> = Result<P, &'static str>;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Packet<'a, H: 'a, D> {
	pub slice: &'a [u8],
	pub header: &'a H,
	pub data: D
}


fn buffer_to_type<S>(buffer: &[u8]) -> ParseResult<&S> {
	if buffer.len() < mem::size_of::<S>() {
		Err("Buffer too short for conversion to type")
	} else {
		Ok(unsafe { mem::transmute(buffer.as_ptr()) })
	}
}
