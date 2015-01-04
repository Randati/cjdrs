use std::num::Int;

#[deriving(Copy, Clone, Eq, PartialEq)]
pub struct BigEndian<T: Int>(T);

impl<T: Int> BigEndian<T> {
	#[inline]
	pub fn val(&self) -> T {
		Int::from_be(self.val_be())
	}

	#[inline]
	pub fn val_be(&self) -> T {
		self.0
	}
}
