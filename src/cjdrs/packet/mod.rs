pub use self::ducttape::{DucttapeHeader, DucttapePacket};
pub use self::ipv6::{IPv6Header, IPv6Packet};
pub use self::tun::{TunHeader, TunPacket};

mod ducttape;
mod ipv6;
mod tun;

pub type ParseResult<P> = Result<P, &'static str>;
