
pub fn bits_used_for_label(label: u64) -> u8 {
	if label & 0b1 != 0 {
		4 + 1
	} else {
		8 + 1
	}
}

pub fn bits_used_for_number(number: u32) -> u8 {
	if number < 15 {
		4 + 1
	} else {
		8 + 1
	}
}

pub fn compress(number: u32) -> u64 {
	if number == 1 {
		return 1;
	}

	match bits_used_for_number(number) {
		5 => {
			match number {
				0 => 0b0_0011,
				n => ((n << 1) | 0b1) as u64
			}
		},
		9 => {
			match number {
				0 => 0b0_0000_0000,
				n => ((n - 1) << 1) as u64
			}
		},
		_ => unreachable!()
	}
}

pub fn decompress(label: u64) -> u32 {
	if label & 0b0_0001_1111 == 1 {
		return 1;
	}

	match bits_used_for_label(label) {
		5 => {
			match (label >> 1) & 0b1111 {
				0b0001 => 0,
				n      => n as u32
			}
		},
		9 => {
			match (label >> 1) & 0b1111_1111 {
				0b0000_0000 => 0,
				n           => (n + 1) as u32
			}
		},
		_ => unreachable!()
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
