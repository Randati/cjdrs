use sodiumoxide::crypto::asymmetricbox::curve25519xsalsa20poly1305 as crypto_box;
use sodiumoxide::crypto::scalarmult::curve25519;
use sodiumoxide::crypto::hash::sha256;
use PrivateKey;
use PublicKey;

pub use sodiumoxide::randombytes::{randombytes, randombytes_into};



#[derive(Copy, Eq, PartialEq)]
pub struct PasswordHash([u8; 32]);

impl PasswordHash {
	pub fn from_password(password: &str) -> PasswordHash {
		let sha256::Digest(hash) = sha256::hash(password.as_bytes());
		PasswordHash(hash)
	}

	fn as_slice(&self) -> &[u8] {
		let &PasswordHash(ref hash) = self;
		hash
	}
}



pub struct SharedSecret(crypto_box::PrecomputedKey);

impl SharedSecret {
	pub fn without_password(my_private_key: &PrivateKey,
	                        her_public_key: &PublicKey)
	                        -> SharedSecret {
		let public = crypto_box::PublicKey::from_slice(her_public_key.as_slice()).unwrap();
		let secret = crypto_box::SecretKey::from_slice(my_private_key.as_slice()).unwrap();
		let key = crypto_box::precompute(&public, &secret);
		SharedSecret(key)
	}

	pub fn with_password(my_private_key: &PrivateKey,
	                     her_public_key: &PublicKey,
	                     password_hash: &PasswordHash)
	                     -> SharedSecret {
		let curve25519::GroupElement(mult_res) = curve25519::scalarmult(
				&curve25519::Scalar::from_slice(my_private_key.as_slice()).unwrap(),
				&curve25519::GroupElement::from_slice(her_public_key.as_slice()).unwrap());
		assert_eq!(mult_res.len(), 32);
		
		let mut hash_input_buffer = Vec::with_capacity(64);
		hash_input_buffer.push_all(&mult_res);
		hash_input_buffer.push_all(password_hash.as_slice());
		assert_eq!(hash_input_buffer.len(), 64);

		let sha256::Digest(hash) = sha256::hash(hash_input_buffer.as_slice());
		let precomputed_key = crypto_box::PrecomputedKey::from_slice(hash.as_slice()).unwrap();
		SharedSecret(precomputed_key)
	}

	fn get_key(&self) -> &crypto_box::PrecomputedKey {
		let &SharedSecret(ref key) = self;
		key
	}
}



#[derive(Copy, Eq, PartialEq)]
pub enum Nonce {
	Mine([u8; 24]),
	Hers([u8; 24])
}

impl Nonce {
	fn get_bytes(&self) -> &[u8; 24] {
		match *self {
			Nonce::Mine(ref bytes) => bytes,
			Nonce::Hers(ref bytes) => bytes
		}
	}
}



#[derive(Copy)]
pub struct CryptoBox;

impl CryptoBox {
	pub fn encrypt(message: &[u8],
	               nonce: &Nonce,
	               shared_secret: &SharedSecret) -> Vec<u8> {
		crypto_box::seal_precomputed(
			message,
			&crypto_box::Nonce(*nonce.get_bytes()),
			shared_secret.get_key())
	}
	
	pub fn decrypt(message: &[u8],
	               nonce: &Nonce,
	               shared_secret: &SharedSecret) -> Option<Vec<u8>> {
		crypto_box::open_precomputed(
			message,
			&crypto_box::Nonce(*nonce.get_bytes()),
			shared_secret.get_key())
	}
}
