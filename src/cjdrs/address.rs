use std::fmt;
use std::mem;
use std::num::Int;
use std::cmp::Ordering;
use std::slice::bytes::copy_memory;
use sodiumoxide::crypto::hash::sha512;
use PublicKey;

const ADDRESS_SIZE: usize = 16;


#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Address {
	bytes: [u8; ADDRESS_SIZE]
}

impl Address {
	pub fn is_valid(slice: &[u8]) -> bool {
		assert_eq!(slice.len(), ADDRESS_SIZE);
		slice[0] == 0xFC
	}

	pub fn from_bytes(bytes: &[u8; ADDRESS_SIZE]) -> Option<Address> {
		if Address::is_valid(bytes.as_slice()) {
			Some(Address { bytes: *bytes })
		} else {
			None
		}
	}

	pub fn from_slice(slice: &[u8]) -> Option<Address> {
		if Address::is_valid(slice) {
			let mut bytes = [0u8; ADDRESS_SIZE];
			copy_memory(&mut bytes, slice);
			Some(Address { bytes: bytes })
		} else {
			None
		}
	}

	pub fn from_public_key(public_key: &PublicKey) -> Option<Address> {
		let first_sha = sha512::hash(public_key.as_slice());
		let second_sha = sha512::hash(first_sha.as_slice());
		let raw_address = &second_sha.as_slice()[..ADDRESS_SIZE];
		Address::from_slice(raw_address)
	}

	pub fn as_slice(&self) -> &[u8] {
		self.bytes.as_slice()
	}


	#[inline]
	pub fn as_u64_be(&self) -> [u64; 2] {
		[
			unsafe { *mem::transmute::<*const u8, *const u64>((&self.bytes[0..]).as_ptr()) }.to_be(),
			unsafe { *mem::transmute::<*const u8, *const u64>((&self.bytes[8..]).as_ptr()) }.to_be(),
		]
	}

	#[inline]
	pub fn as_u32_be(&self) -> [u32; 4] {
		[
			unsafe { *mem::transmute::<*const u8, *const u32>((&self.bytes[ 0..]).as_ptr()) }.to_be(),
			unsafe { *mem::transmute::<*const u8, *const u32>((&self.bytes[ 4..]).as_ptr()) }.to_be(),
			unsafe { *mem::transmute::<*const u8, *const u32>((&self.bytes[ 8..]).as_ptr()) }.to_be(),
			unsafe { *mem::transmute::<*const u8, *const u32>((&self.bytes[12..]).as_ptr()) }.to_be(),
		]
	}

	#[inline]
	pub fn as_u16_be(&self) -> [u16; 8] {
		[
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[ 0..]).as_ptr()) }.to_be(),
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[ 2..]).as_ptr()) }.to_be(),
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[ 4..]).as_ptr()) }.to_be(),
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[ 6..]).as_ptr()) }.to_be(),
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[ 8..]).as_ptr()) }.to_be(),
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[10..]).as_ptr()) }.to_be(),
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[12..]).as_ptr()) }.to_be(),
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[14..]).as_ptr()) }.to_be(),
		]
	}

	#[inline]
	pub fn as_u64_le(&self) -> [u64; 2] {
		[
			unsafe { *mem::transmute::<*const u8, *const u64>((&self.bytes[0..]).as_ptr()) }.to_le(),
			unsafe { *mem::transmute::<*const u8, *const u64>((&self.bytes[8..]).as_ptr()) }.to_le(),
		]
	}

	#[inline]
	pub fn as_u32_le(&self) -> [u32; 4] {
		[
			unsafe { *mem::transmute::<*const u8, *const u32>((&self.bytes[ 0..]).as_ptr()) }.to_le(),
			unsafe { *mem::transmute::<*const u8, *const u32>((&self.bytes[ 4..]).as_ptr()) }.to_le(),
			unsafe { *mem::transmute::<*const u8, *const u32>((&self.bytes[ 8..]).as_ptr()) }.to_le(),
			unsafe { *mem::transmute::<*const u8, *const u32>((&self.bytes[12..]).as_ptr()) }.to_le(),
		]
	}

	#[inline]
	pub fn as_u16_le(&self) -> [u16; 8] {
		[
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[ 0..]).as_ptr()) }.to_le(),
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[ 2..]).as_ptr()) }.to_le(),
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[ 4..]).as_ptr()) }.to_le(),
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[ 6..]).as_ptr()) }.to_le(),
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[ 8..]).as_ptr()) }.to_le(),
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[10..]).as_ptr()) }.to_le(),
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[12..]).as_ptr()) }.to_le(),
			unsafe { *mem::transmute::<*const u8, *const u16>((&self.bytes[14..]).as_ptr()) }.to_le(),
		]
	}

	pub fn xor_distance(&self, other: &Address) -> (u64, u64) {
		let a = self.as_u64_be();
		let b = other.as_u64_be();
		(a[1] ^ b[1], a[0] ^ b[0])
	}

	pub fn xor_compare(ab: (&Address, &Address), cd: (&Address, &Address)) -> Ordering {
		let a = ab.0.as_u64_be();
		let b = ab.1.as_u64_be();
		let c = cd.0.as_u64_be();
		let d = cd.1.as_u64_be();

		match (a[1] ^ b[1]).cmp(&(c[1] ^ d[1])) {
			Ordering::Less    => return Ordering::Less,
			Ordering::Greater => return Ordering::Greater,
			Ordering::Equal   => return (a[0] ^ b[0]).cmp(&(c[0] ^ d[0]))
		}
	}
}

impl fmt::Display for Address {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let b = &self.bytes;
		write!(f,
			"{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}",
			b[ 0], b[ 1], b[ 2], b[ 3], b[ 4], b[ 5], b[ 6], b[ 7],
			b[ 8], b[ 9], b[10], b[11], b[12], b[13], b[14], b[15])
	}
}

impl fmt::Debug for Address {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self)
	}
}


#[cfg(test)]
mod tests {
	use address::Address;
	use std::cmp::Ordering;
	
	#[test]
	fn test_as_u64_be() {
		let address = Address::from_bytes(&[
			0xfc, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
			0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15]).unwrap();
		let address_64 = [0xfc01020304050607, 0x0809101112131415];
		assert_eq!(address.as_u64_be(), address_64);
	}

	#[test]
	fn test_as_u32_be() {
		let address = Address::from_bytes(&[
			0xfc, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
			0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15]).unwrap();
		let address_32 = [0xfc010203, 0x04050607, 0x08091011, 0x12131415];
		assert_eq!(address.as_u32_be(), address_32);
	}

	#[test]
	fn test_as_u16_be() {
		let address = Address::from_bytes(&[
			0xfc, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
			0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15]).unwrap();
		let address_16 = [0xfc01, 0x0203, 0x0405, 0x0607, 0x0809, 0x1011, 0x1213, 0x1415];
		assert_eq!(address.as_u16_be(), address_16);
	}

	#[test]
	fn test_as_u64_le() {
		let address = Address::from_bytes(&[
			0xfc, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
			0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15]).unwrap();
		let address_64 = [0x07060504030201fc, 0x1514131211100908];
		assert_eq!(address.as_u64_le(), address_64);
	}

	#[test]
	fn test_as_u32_le() {
		let address = Address::from_bytes(&[
			0xfc, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
			0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15]).unwrap();
		let address_32 = [0x030201fc, 0x07060504, 0x11100908, 0x15141312];
		assert_eq!(address.as_u32_le(), address_32);
	}

	#[test]
	fn test_as_u16_le() {
		let address = Address::from_bytes(&[
			0xfc, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
			0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15]).unwrap();
		let address_16 = [0x01fc, 0x0302, 0x0504, 0x0706, 0x0908, 0x1110, 0x1312, 0x1514];
		assert_eq!(address.as_u16_le(), address_16);
	}

	#[test]
	fn test_xor_distance() {
		// TODO Real data

		let a = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]).unwrap();
		let b = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		assert_eq!(Address::xor_distance(&a, &b), (0x01, 0x00));

		let a = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		let b = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		assert_eq!(Address::xor_distance(&a, &b), (0x00, 0x01));

		let a = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		let b = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00]).unwrap();
		assert_eq!(Address::xor_distance(&a, &b), (0x0100, 0x0101));
	}

	#[test]
	fn test_xor_compare() {
		// TODO Real data

		let a = Address::from_bytes(&[0xfc, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15]).unwrap();
		let b = Address::from_bytes(&[0xfc, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15]).unwrap();
		let c = Address::from_bytes(&[0xfc, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15]).unwrap();
		let d = Address::from_bytes(&[0xfc, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15]).unwrap();
		assert_eq!(Address::xor_compare((&a, &b), (&c, &d)), Ordering::Equal);

		let a = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]).unwrap();
		let b = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		let c = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		let d = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		assert_eq!(Address::xor_compare((&a, &b), (&c, &d)), Ordering::Greater);

		let a = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		let b = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		let c = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]).unwrap();
		let d = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		assert_eq!(Address::xor_compare((&a, &b), (&c, &d)), Ordering::Less);

		let a = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		let b = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		let c = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		let d = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		assert_eq!(Address::xor_compare((&a, &b), (&c, &d)), Ordering::Greater);

		let a = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		let b = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		let c = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]).unwrap();
		let d = Address::from_bytes(&[0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
		assert_eq!(Address::xor_compare((&a, &b), (&c, &d)), Ordering::Less);
	}
}
