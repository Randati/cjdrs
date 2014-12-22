extern crate crypto;

use std::rand::Rng;
use self::crypto::curve25519::curve25519_base;
use Address;


const PRIV_KEY_SIZE: uint = 32;
const PUB_KEY_SIZE: uint = 32;


#[deriving(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PrivateKey([u8, ..PRIV_KEY_SIZE]);

impl PrivateKey {
	pub fn as_slice(&self) -> &[u8] {
		let &PrivateKey(ref slice) = self;
		slice
	}
}


#[deriving(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PublicKey([u8, ..PUB_KEY_SIZE]);

impl PublicKey {
	pub fn as_slice(&self) -> &[u8] {
		let &PublicKey(ref slice) = self;
		slice
	}
}


#[deriving(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PrivateIdentity {
	pub private_key: PrivateKey,
	pub public_key: PublicKey,
	pub address: Address
}

impl PrivateIdentity {
	pub fn generate<R: Rng>(rng: &mut R) -> PrivateIdentity {
		loop {
			let mut private_key_buf = [0, ..PRIV_KEY_SIZE];
			rng.fill_bytes(private_key_buf.as_mut_slice());
			let private_key = PrivateKey(private_key_buf);

			if let Some(public_identity) = PublicIdentity::from_private_key(&private_key) {
				return PrivateIdentity {
					private_key: private_key,
					public_key: public_identity.public_key,
					address: public_identity.address
				}
			}
		}
	}
}


#[deriving(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PublicIdentity {
	pub public_key: PublicKey,
	pub address: Address
}

impl PublicIdentity {
	pub fn from_private_key(private_key: &PrivateKey) -> Option<PublicIdentity> {
		let &PrivateKey(ref private_key_slice) = private_key;
		let public_key_buf = curve25519_base(private_key_slice);
		let public_key = PublicKey(public_key_buf);

		match Address::from_public_key(&public_key) {
			Some(address) => Some(PublicIdentity {
				public_key: public_key,
				address: address
			}),
			None => None
		}
	}
}



#[cfg(test)]
mod tests {
	use std::rand::OsRng;
	use identity::{
		PrivateKey,
		PrivateIdentity,
		PublicIdentity};
	use address::Address;


	#[test]
	fn test_generate_private() {
		let identity = PrivateIdentity::generate(&mut OsRng::new().unwrap());
		assert_eq!(identity.address.as_slice()[0], 0xFC);
	}

	#[test]
	fn test_get_public() {
		let priv_key = PrivateKey([
			0x4c, 0x80, 0xb5, 0xfe, 0xe2, 0xad, 0xbd, 0x9a,
			0xeb, 0x80, 0xed, 0xe1, 0xd7, 0x5b, 0xd2, 0xba,
			0x93, 0xc2, 0xa6, 0xea, 0xbe, 0xf3, 0x8b, 0xe1,
			0x8d, 0x4b, 0x8a, 0x41, 0x8d, 0x9a, 0xa0, 0xbc]);
		let ip = Address::from_bytes(&[
			0xfc, 0x50, 0x71, 0xae, 0x09, 0xd6, 0xf7, 0x94,
			0x75, 0x54, 0x20, 0x83, 0x87, 0x3e, 0x88, 0xa9]).unwrap();
		
		let identity = PublicIdentity::from_private_key(&priv_key).unwrap();
		assert_eq!(identity.address, ip);


		let priv_key = PrivateKey([
			0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
			0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
			0x17, 0x18, 0x19, 0x20, 0x21, 0x22, 0x23, 0x24,
			0x25, 0x26, 0x27, 0x28, 0x29, 0x30, 0x31, 0x32]);
		assert!(PublicIdentity::from_private_key(&priv_key).is_none());
	}
}
