
pub fn bits_used_for_label(label: u64) -> u8 {
	if label & 0b1111 == 0b0001 {
		4
	} else {
		8
	}
}

pub fn bits_used_for_number(number: u32) -> u8 {
	if number == 1 {
		4
	} else {
		8
	}
}

pub fn compress(number: u32) -> u64 {
	assert!(number <= 0b1111_0000);

	let low  =  number & 0b0000_1111;
	let high = (number & 0b1111_0000) >> 4;

	match (high, low) {
		(0b0000, 0b0001) => 0b0000_0001,
		(0b1111, 0b0000) => 0b0001_0000,
		(0b0000,      _) => (low << 4) as u64,
		(     _,      _) => ((low << 4) | (high + 1)) as u64
	}
}

pub fn decompress(label: u64) -> u32 {
	assert!(label <= 0b1111_1111);

	let low  =  label & 0b0000_1111;
	let high = (label & 0b1111_0000) >> 4;

	match (high, low) {
		(     _, 0b0001) => 0b0000_0001,
		(0b0001, 0b0000) => 0b1111_0000,
		(     _, 0b0000) => high as u32,
		(     _,      _) => (((low - 1) << 4) | high) as u32
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_compress_decompress() {
		for i in range(0, 241) {
			assert_eq!(i, decompress(compress(i)))
		}
	}
}
