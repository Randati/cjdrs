
pub fn as_hex(slice: &[u8]) -> String {
	let mut ret = "".to_string();
	ret.push_str(format!("({})[", slice.len()).as_slice());
	for &b in slice.iter() {
		let s = format!("{:02x}", b);
		ret.push_str(s.as_slice());
	}
	ret + "]"
}
