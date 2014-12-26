
pub fn bits_used_for_label(_label: u64) -> u8 {
	4
}

pub fn bits_used_for_number(_number: u32) -> u8 {
	4
}

pub fn compress(number: u32) -> u64 {
	assert!(number <= 0b1111);
	number as u64
}

pub fn decompress(label: u64) -> u32 {
	assert!(label <= 0b1111);
	label as u32
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_compress_decompress() {
		for i in range(0, 16) {
			assert_eq!(i, decompress(compress(i)))
		}
	}
}
