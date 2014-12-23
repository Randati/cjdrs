pub use self::ducttape::DucttapePacket;
pub use self::ipv6::IPv6Packet;
pub use self::tun::TunPacket;

mod ducttape;
mod ipv6;
mod tun;

pub type ParseResult<P> = Result<P, &'static str>;

pub trait PacketData<'a> {
	fn from_buffer(buffer: &'a [u8]) -> ParseResult<Self>;
	fn as_slice(&self) -> &'a [u8];
}
