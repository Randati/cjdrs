pub use self::ducttape::Ducttape;
pub use self::ipv6::IPv6;
pub use self::tun::Tun;

mod ducttape;
mod ipv6;
mod tun;

pub type ParseResult<P> = Result<P, &'static str>;

pub trait Packet<'a> {
	fn from_buffer(buffer: &'a [u8]) -> ParseResult<Self>;
	fn as_slice(&self) -> &'a [u8];
}


pub struct Raw<'a>(&'a [u8]);

impl<'a> Packet<'a> for Raw<'a> {
	fn from_buffer(buffer: &'a [u8]) -> ParseResult<Raw> {
		Ok(Raw(buffer))
	}

	fn as_slice(&self) -> &'a [u8] {
		let &Raw(slice) = self;
		slice
	}
}
