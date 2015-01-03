use std::mem::{size_of, transmute};
use std::raw::Slice;
use crypto::{SharedSecret, Nonce, CryptoBox};
use packet::{ParseResult, Packet, buffer_to_type};
use identity::{PublicKey, PUB_KEY_SIZE};
use util::BigEndian;
use debug::as_hex;

#[cfg(test)] pub const CRYPTOAUTH_HEADER_LENGTH: uint = 120;



#[derive(Copy, Clone, Eq, PartialEq)]
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

	pub fn derivations(&self) -> u16 {
		self.require_auth_and_derivation_count.val() & (!0 >> 1)
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
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
				let data = buffer.slice_from(size_of::<CryptoAuthHeader>());

				println!("");
				println!("Stage:                  0x{:08X}", header.stage.val());
				println!("Auth challenge");
				println!("    Challenge type:     {}", header.auth_challenge.challenge_type);
				println!("    Lookup:             {}", as_hex(&header.auth_challenge.lookup));
				println!("    Require auth:       {}", header.auth_challenge.require_auth());
				println!("    Derivations:        {}", header.auth_challenge.derivations());
				println!("    Additional:         0x{:04X}", header.auth_challenge.additional.val());
				println!("Nonce:                  {}", as_hex(&header.nonce));
				println!("Perm public key:        {}", PublicKey::from_slice(&header.public_key).as_string());
				println!("Poly1305 authenticator: {}", as_hex(&header.authenticator));
				println!("Temporary public key:   {}", as_hex(&header.encrypted_temp_key));
				println!("Data:                   {}", as_hex(data));
				println!("");
				


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

	pub fn challenge(&self) -> &Challenge {
		&self.header.auth_challenge
	}

	pub fn public_key(&self) -> PublicKey {
		PublicKey(self.header.public_key)
	}

	pub fn decrypt(&mut self, shared_secret: &SharedSecret) -> Option<Vec<u8>> {
		let encrypted_part: &[u8] = unsafe {
			transmute(Slice {
				data: &self.header.authenticator,
				len: 16 + 32 + self.data.len()
			})
		};

		CryptoBox::decrypt(
			encrypted_part,
			&Nonce::Hers(self.header.nonce),
			shared_secret)
	}
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
