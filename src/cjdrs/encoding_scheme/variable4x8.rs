
pub fn bits_used_for_label(label: u64) -> u8 {
	if label & 1 != 0 {
		5
	} else {
		9
	}
}

pub fn bits_used_for_number(number: u32) -> u8 {
	if number < 15 {
		5
	} else {
		9
	}
}

pub fn compress(number: u32) -> u64 {
	if number == 1 {
		return 1;
	}

	match bits_used_for_number(number) {
		5 => {
			let number = if number == 0 { 1 } else { number };
			((number << 1) | 0b0_0001) as u64
		},
		9 => {
			let number = if number == 0 { 0 } else { number - 1 };
			(number << 1) as u64
		},
		_ => panic!()
	}
}

pub fn decompress(label: u64) -> u32 {
	if label & 0b0_0001_1111 == 1 {
		return 1;
	}

	match bits_used_for_label(label) {
		5 => {
			let label = (label >> 1) & 0b1111;
			if label == 1 { 0 } else { label as u32 }
		},
		9 => {
			let label = (label >> 1) & 0b1111_1111;
			if label == 0 { 0 } else { (label + 1) as u32 }
		},
		_ => panic!(false)
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_compress_decompress() {
		for i in range(0, 257) {
			assert_eq!(i, decompress(compress(i)))
		}
	}
}
