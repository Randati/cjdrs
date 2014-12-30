use sodiumoxide::crypto::asymmetricbox::curve25519xsalsa20poly1305 as crypto_box;
use PrivateKey;
use PublicKey;


pub fn precompute(my_private_key: &PrivateKey,
                  her_public_key: &PublicKey) -> crypto_box::PrecomputedKey {
	let public = crypto_box::PublicKey::from_slice(her_public_key.as_slice()).unwrap();
	let secret = crypto_box::SecretKey::from_slice(my_private_key.as_slice()).unwrap();
	crypto_box::precompute(&public, &secret)
}

pub fn encrypt(message: &[u8],
               nonce: &crypto_box::Nonce,
               precomputed_key: &crypto_box::PrecomputedKey) -> Vec<u8> {
	crypto_box::seal_precomputed(
		message,
		nonce,
		precomputed_key)
}

pub fn decrypt(message: &[u8],
               nonce: &crypto_box::Nonce,
               precomputed_key: &crypto_box::PrecomputedKey) -> Option<Vec<u8>> {
	crypto_box::open_precomputed(
		message,
		nonce,
		precomputed_key)
}
