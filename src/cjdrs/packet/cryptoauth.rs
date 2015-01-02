use std::mem::size_of;
use packet::{ParseResult, Packet, buffer_to_type};
use identity::{PublicKey, PUB_KEY_SIZE};
use util::BigEndian;

pub const CRYPTOAUTH_HEADER_LENGTH: uint = 120;



#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(packed)]
pub struct Challenge {
	challenge_type: u8,
	lookup: [u8; 7],
	require_auth_and_derivation_count: BigEndian<u16>,
	additional: BigEndian<u16>
}

impl Challenge {
	pub fn require_auth(&self) -> bool {
		self.require_auth_and_derivation_count.val() >> 15 != 0
	}
}

#[deriving(Copy, Clone, Eq, PartialEq)]
#[repr(packed)]
pub struct CryptoAuthHeader {
	stage: BigEndian<u32>,
	auth_challenge: Challenge,
	nonce: [u8; 24],
	public_key: [u8; PUB_KEY_SIZE],
	authenticator: [u8; 16],
	encrypted_temp_key: [u8; 32]
}



pub type CryptoAuth<'a> = Packet<'a, CryptoAuthHeader, &'a [u8]>;

impl<'a> CryptoAuth<'a> {
	pub fn from_buffer(buffer: &[u8]) -> ParseResult<CryptoAuth> {
		let stage_or_nonce: &BigEndian<u32> = try!(buffer_to_type(buffer));
		
		match stage_or_nonce.val() {
			0 | 1 => {
				let header: &CryptoAuthHeader = try!(buffer_to_type(buffer));
				println!("Auth challenge");
				println!("    Challenge type: {}", header.auth_challenge.challenge_type);
				println!("    Lookup:         {}", as_hex(&header.auth_challenge.lookup));
				println!("    Require auth:   {}", header.auth_challenge.require_auth());
				println!("Nonce:                  {}", as_hex(&header.nonce));
				println!("Perm public key:        {}", PublicKey::from_slice(&header.public_key).as_string());
				println!("Poly1305 authenticator: {}", as_hex(&header.authenticator));
				println!("Temporary public key:   {}", as_hex(&header.encrypted_temp_key));
				
				let data = buffer.slice_from(size_of::<CryptoAuthHeader>());
				Ok(CryptoAuth {
					slice: buffer,
					header: header,
					data: data
				})
			},
			n => {
				println!("Unknown type {}", n);
				unimplemented!();
			}
		}
	}
}


fn as_hex(slice: &[u8]) -> String {
	let mut ret = "".to_string();
	ret.push_str(format!("({})[", slice.len()).as_slice());
	for &b in slice.iter() {
		let s = format!("{:02X} ", b);
		ret.push_str(s.as_slice());
	}
	ret + "]"
}



#[cfg(test)]
mod tests {
	use super::*;
	use std::mem::size_of;
	
	#[test]
	fn test_sizeof() {
		assert_eq!(size_of::<CryptoAuthHeader>(), CRYPTOAUTH_HEADER_LENGTH);
	}
}
