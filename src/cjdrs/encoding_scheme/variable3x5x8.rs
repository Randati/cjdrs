
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
	if number < 8 {
		4
	} else if number < 33 {
		7
	} else {
		10
	}
}

pub fn compress(number: u32) -> u64 {
	if number == 1 {
		return 1;
	}

	match bits_used_for_number(number) {
		4 => {
			if number == 0 {
				3
			} else {
				((number << 1) | 0b1) as u64
			}
		},
		7 => {
			if number == 0 {
				2
			} else {
				(((number - 1) << 2) | 0b10) as u64
			}
		},
		10 => {
			if number == 0 {
				0
			} else {
				((number - 1) << 2) as u64
			}
		},
		_ => panic!()
	}
}

pub fn decompress(label: u64) -> u32 {
	match bits_used_for_label(label) {
		4 => {
			match (label >> 1) & 0b0111 {
				0 => 1,
				1 => 0,
				n => n as u32
			}
		},
		7 => {
			let label = (label >> 2) & 0b001_1111;
			if label == 0 { 0 } else { (label + 1) as u32 }
		},
		10 => {
			let label = (label >> 2) & 0b00_1111_1111;
			if label == 0 { 0 } else { (label + 1) as u32 }
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
