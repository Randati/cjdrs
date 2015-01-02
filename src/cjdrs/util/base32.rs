//! http://dnscurve.org/in-implement.html

use std::char;

static NUM_TO_ASCII: &'static str = "0123456789bcdfghjklmnpqrstuvwxyz";

static ASCII_TO_NUM: [u8; 128] = [
	255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
	255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
	255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
	  0,   1,   2,   3,   4,   5,   6,   7,   8,   9, 255, 255, 255, 255, 255, 255,
	255, 255,  10,  11,  12, 255,  13,  14,  15, 255,  16,  17,  18,  19,  20, 255,
	 21,  22,  23,  24,  25,  26,  27,  28,  29,  30,  31, 255, 255, 255, 255, 255,
	255, 255,  10,  11,  12, 255,  13,  14,  15, 255,  16,  17,  18,  19,  20, 255,
	 21,  22,  23,  24,  25,  26,  27,  28,  29,  30,  31, 255, 255, 255, 255, 255
];


#[inline]
pub fn bytes_needed_to_encode(data_len: uint) -> uint {
	(data_len * 8 + 4) / 5
}

pub fn encode(bytes: &[u8]) -> String {
	let ascii_bytes = NUM_TO_ASCII.as_bytes();
	let mut ret = String::with_capacity(bytes_needed_to_encode(bytes.len()));

	for chunk in bytes.chunks(5) {
		let buf = {
			let mut buf = [0u8; 5];
			buf.clone_from_slice(chunk);
			buf
		};

		let chars = [
			ascii_bytes[  (buf[0] & 0b0001_1111) as uint],
			ascii_bytes[(((buf[0] & 0b1110_0000) >> 5) | ((buf[1] & 0b0000_0011) << 3)) as uint],
			ascii_bytes[ ((buf[1] & 0b0111_1100) >> 2) as uint],
			ascii_bytes[(((buf[1] & 0b1000_0000) >> 7) | ((buf[2] & 0b0000_1111) << 1)) as uint],
			ascii_bytes[(((buf[2] & 0b1111_0000) >> 4) | ((buf[3] & 0b0000_0001) << 4)) as uint],
			ascii_bytes[ ((buf[3] & 0b0011_1110) >> 1) as uint],
			ascii_bytes[(((buf[3] & 0b1100_0000) >> 6) | ((buf[4] & 0b0000_0111) << 2)) as uint],
			ascii_bytes[ ((buf[4] & 0b1111_1000) >> 3) as uint]
		];

		for &c in chars.slice_to(bytes_needed_to_encode(chunk.len())).iter() {
			ret.push(char::from_u32(c as u32).unwrap());
		}
	}

	ret
}

#[inline]
pub fn bytes_needed_to_decode(encoded_len: uint) -> uint {
	encoded_len * 5 / 8
}

pub fn decode(encoded: &str) -> Option<Vec<u8>> {
	let encoded = encoded.as_bytes();
	let mut ret = Vec::with_capacity(bytes_needed_to_decode(encoded.len()));

	for chunk in encoded.chunks(8) {
		let buf = {
			let mut buf = [0u8; 8];
			for (i, &c) in chunk.iter().enumerate() {
				if c >= 128 { return None; }
				let b = ASCII_TO_NUM[c as uint];
				if b >= 32 { return None; }
				buf[i] = b;
			}
			buf
		};

		let bytes = [
			 buf[0]       | (buf[1] << 5),
			(buf[1] >> 3) | (buf[2] << 2) | (buf[3] << 7),
			(buf[3] >> 1) | (buf[4] << 4),
			(buf[4] >> 4) | (buf[5] << 1) | (buf[6] << 6),
			(buf[6] >> 2) | (buf[7] << 3)
		];

		ret.push_all(bytes.slice_to(bytes_needed_to_decode(chunk.len())));
	}

	Some(ret)
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_encode() {
		assert_eq!(encode(&[]), "");

		assert_eq!(encode(&[0x64, 0x88]), "4321");

		let pub_key = [
			70, 197, 17, 133, 42, 61, 168, 77, 30, 202, 145, 14, 129, 130, 124, 194,
			6, 77, 78, 144, 95, 114, 235, 169, 42, 35, 167, 1, 124, 66, 200, 40];
		assert_eq!(encode(&pub_key), "6bk3k2b5x1bv4h8tkn3281lh2q1u471lzlwqynb53t930y9886b0");
	}

	#[test]
	fn test_decode() {
		assert_eq!(decode("").unwrap(), []);

		assert_eq!(decode("4321").unwrap(), [0x64, 0x88]);

		let pub_key = [
			70, 197, 17, 133, 42, 61, 168, 77, 30, 202, 145, 14, 129, 130, 124, 194,
			6, 77, 78, 144, 95, 114, 235, 169, 42, 35, 167, 1, 124, 66, 200, 40];
		assert_eq!(decode("6bk3k2b5x1bv4h8tkn3281lh2q1u471lzlwqynb53t930y9886b0").unwrap(), pub_key);
	}

	#[test]
	fn test_encode_decode() {
		let data = [];
		let processed = decode(encode(&data).as_slice()).unwrap();
		assert_eq!(processed.as_slice(), data.as_slice());

		let data = [0x01];
		let processed = decode(encode(&data).as_slice()).unwrap();
		assert_eq!(processed.as_slice(), data.as_slice());

		let data = [0x01, 0x02];
		let processed = decode(encode(&data).as_slice()).unwrap();
		assert_eq!(processed.as_slice(), data.as_slice());

		let data = [0x01, 0x02, 0x03];
		let processed = decode(encode(&data).as_slice()).unwrap();
		assert_eq!(processed.as_slice(), data.as_slice());

		let data = [0x01, 0x02, 0x03, 0x04];
		let processed = decode(encode(&data).as_slice()).unwrap();
		assert_eq!(processed.as_slice(), data.as_slice());

		let data = [0x01, 0x02, 0x03, 0x04, 0x05];
		let processed = decode(encode(&data).as_slice()).unwrap();
		assert_eq!(processed.as_slice(), data.as_slice());

		let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
		let processed = decode(encode(&data).as_slice()).unwrap();
		assert_eq!(processed.as_slice(), data.as_slice());

		let data = [
			70, 197, 17, 133, 42, 61, 168, 77, 30, 202, 145, 14, 129, 130, 124, 194,
			6, 77, 78, 144, 95, 114, 235, 169, 42, 35, 167, 1, 124, 66, 200, 40];
		let processed = decode(encode(&data).as_slice()).unwrap();
		assert_eq!(processed.as_slice(), data.as_slice());
	}

	#[test]
	fn test_bytes_needed_to_encode() {
		assert_eq!(bytes_needed_to_encode(0), 0);
		assert_eq!(bytes_needed_to_encode(1), 2);
		assert_eq!(bytes_needed_to_encode(2), 4);
		assert_eq!(bytes_needed_to_encode(3), 5);
		assert_eq!(bytes_needed_to_encode(4), 7);
		assert_eq!(bytes_needed_to_encode(5), 8);
		assert_eq!(bytes_needed_to_encode(6), 10);
		assert_eq!(bytes_needed_to_encode(32), 52);
	}

	#[test]
	fn test_bytes_needed_to_decode() {
		assert_eq!(bytes_needed_to_decode(0), 0);
		assert_eq!(bytes_needed_to_decode(2), 1);
		assert_eq!(bytes_needed_to_decode(4), 2);
		assert_eq!(bytes_needed_to_decode(5), 3);
		assert_eq!(bytes_needed_to_decode(7), 4);
		assert_eq!(bytes_needed_to_decode(8), 5);
		assert_eq!(bytes_needed_to_decode(10), 6);
		assert_eq!(bytes_needed_to_decode(52), 32);
	}
}
