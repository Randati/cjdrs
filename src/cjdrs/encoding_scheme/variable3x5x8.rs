
pub fn bits_used_for_label(label: u64) -> u8 {
	if label & 0b01 != 0 {
		4
	} else if label & 0b10 != 0 {
		7
	} else {
		10
	}
}

pub fn bits_used_for_number(number: u32) -> u8 {
	match number {
		n if n <  8 => 4,
		n if n < 33 => 7,
		_           => 10
	}
}

pub fn compress(number: u32) -> u64 {
	if number == 1 {
		return 1;
	}

	match bits_used_for_number(number) {
		4 => {
			match number {
				0 => 0b0011,
				n => ((n << 1) | 0b1) as u64
			}
		},
		7 => {
			match number {
				0 => 0b000_0010,
				n => (((n - 1) << 2) | 0b10) as u64
			}
		},
		10 => {
			match number {
				0 => 0b00_0000_0000,
				n => ((n - 1) << 2) as u64
			}
		},
		_ => panic!()
	}
}

pub fn decompress(label: u64) -> u32 {
	match bits_used_for_label(label) {
		4 => {
			match (label >> 1) & 0b111 {
				0b000 => 1,
				0b001 => 0,
				n     => n as u32
			}
		},
		7 => {
			match (label >> 2) & 0b1_1111 {
				0b0_0000 => 0,
				n        => (n + 1) as u32
			}
		},
		10 => {
			match (label >> 2) & 0b1111_1111 {
				0b0000_0000 => 0,
				n           => (n + 1) as u32
			}
		},
		_ => panic!()
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
