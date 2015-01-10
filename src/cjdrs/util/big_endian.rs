use std::fmt;
use std::num::Int;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BigEndian<T: Int + fmt::Show>(T);

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

impl<T: Int + fmt::Show> fmt::Show for BigEndian<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.val())
	}
}
