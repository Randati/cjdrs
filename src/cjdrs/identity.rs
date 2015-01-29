use std::fmt;
use rustc_serialize::hex::{FromHex, ToHex};
use sodiumoxide::crypto::scalarmult::curve25519;
use Address;
use crypto;
use util::base32;
use CjdrsResult;
use CjdrsError;


pub const PRIV_KEY_SIZE: usize = 32;
pub const PUB_KEY_SIZE: usize = 32;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PrivateKey([u8; PRIV_KEY_SIZE]);

impl PrivateKey {
	pub fn from_string(string: &str) -> CjdrsResult<PrivateKey> {
		match string.from_hex() {
			Ok(bytes) => {
				if bytes.len() != PRIV_KEY_SIZE {
					fail!(CjdrsError::InvalidPrivateKey(None));
				}

				let buffer = {
					let mut buffer = [0u8; PRIV_KEY_SIZE];
					buffer.clone_from_slice(bytes.as_slice());
					buffer
				};

				Ok(PrivateKey(buffer))
			}
			Err(e) => Err(CjdrsError::InvalidPrivateKey(Some(e)))
		}
	}

	pub fn as_slice(&self) -> &[u8; PRIV_KEY_SIZE] {
		&self.0
	}

	pub fn as_string(&self) -> String {
		self.as_slice().to_hex()
	}
}

impl fmt::Display for PrivateKey {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.as_string())
	}
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PublicKey([u8; PUB_KEY_SIZE]);

impl PublicKey {
	pub fn from_buffer(buffer: &[u8; PUB_KEY_SIZE]) -> PublicKey {
		PublicKey(*buffer)
	}

	pub fn from_slice(slice: &[u8]) -> PublicKey {
		assert_eq!(slice.len(), PUB_KEY_SIZE);

		let buffer = {
			let mut buffer = [0u8; PUB_KEY_SIZE];
			buffer.clone_from_slice(slice);
			buffer
		};

		PublicKey(buffer)
	}

	pub fn from_string(key_str: &str) -> CjdrsResult<PublicKey> {
		if key_str.len() != 52 + 2 {
			fail!(CjdrsError::InvalidPublicKey);
		} else if !key_str.ends_with(".k") {
			fail!(CjdrsError::InvalidPublicKey);
		}

		let hex_str = &key_str[..52];

		match base32::decode(hex_str) {
			Some(bytes) => {
				assert_eq!(bytes.len(), PUB_KEY_SIZE);
				Ok(PublicKey::from_slice(bytes.as_slice()))
			},
			None => Err(CjdrsError::InvalidPublicKey)
		}
	}

	pub fn as_slice(&self) -> &[u8; PUB_KEY_SIZE] {
		&self.0
	}

	pub fn as_string(&self) -> String {
		base32::encode(self.as_slice()) + ".k"
	}
}

impl fmt::Display for PublicKey {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.as_string())
	}
}



#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PrivateIdentity {
	pub private_key: PrivateKey,
	pub public_key: PublicKey,
	pub address: Address
}

impl PrivateIdentity {
	pub fn generate() -> PrivateIdentity {
		loop {
			let private_key = {
				let mut private_key_buf = [0; PRIV_KEY_SIZE];
				crypto::randombytes_into(&mut private_key_buf);
				PrivateKey(private_key_buf)
			};

			if let Some(identity) = PrivateIdentity::from_private_key(&private_key) {
				return identity;
			}
		}
	}

	pub fn from_private_key(private_key: &PrivateKey) -> Option<PrivateIdentity> {
		let input = curve25519::Scalar(*private_key.as_slice());
		let public_key_buf = curve25519::scalarmult_base(&input).0;
		let public_key = PublicKey::from_buffer(&public_key_buf);

		match Address::from_public_key(&public_key) {
			Some(address) => Some(PrivateIdentity {
				private_key: *private_key,
				public_key: public_key,
				address: address
			}),
			None => None
		}
	}
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PublicIdentity {
	pub public_key: PublicKey,
	pub address: Address
}

impl PublicIdentity {
	pub fn from_public_key(public_key: &PublicKey) -> Option<PublicIdentity> {
		match Address::from_public_key(public_key) {
			Some(address) => Some(PublicIdentity {
				public_key: *public_key,
				address: address
			}),
			None => None
		}
	}
}



#[cfg(test)]
mod tests {
	use identity::{
		PrivateKey,
		PrivateIdentity};
	use address::Address;
	use test::Bencher;


	#[test]
	fn test_private_generate() {
		let identity = PrivateIdentity::generate();
		assert_eq!(identity.address.as_slice()[0], 0xFC);
	}

	#[test]
	fn test_private_from_key() {
		let priv_key = PrivateKey([
			0x4c, 0x80, 0xb5, 0xfe, 0xe2, 0xad, 0xbd, 0x9a,
			0xeb, 0x80, 0xed, 0xe1, 0xd7, 0x5b, 0xd2, 0xba,
			0x93, 0xc2, 0xa6, 0xea, 0xbe, 0xf3, 0x8b, 0xe1,
			0x8d, 0x4b, 0x8a, 0x41, 0x8d, 0x9a, 0xa0, 0xbc]);
		let ip = Address::from_bytes(&[
			0xfc, 0x50, 0x71, 0xae, 0x09, 0xd6, 0xf7, 0x94,
			0x75, 0x54, 0x20, 0x83, 0x87, 0x3e, 0x88, 0xa9]).unwrap();
		
		let identity = PrivateIdentity::from_private_key(&priv_key).unwrap();
		assert_eq!(identity.address, ip);


		let priv_key = PrivateKey([
			0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
			0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
			0x17, 0x18, 0x19, 0x20, 0x21, 0x22, 0x23, 0x24,
			0x25, 0x26, 0x27, 0x28, 0x29, 0x30, 0x31, 0x32]);
		assert!(PrivateIdentity::from_private_key(&priv_key).is_none());
	}

	#[bench]
	fn bench_generate_identity(b: &mut Bencher) {
		b.iter(|| {
			PrivateIdentity::generate()
		})
	}
}
