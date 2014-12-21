use std::num::Int;
use std::fmt;
use std::u64;

#[deriving(Copy, Clone, Eq, PartialEq)]
pub struct Route {
	bits: u64
}

impl Route {
	#[inline]
	pub fn new(bits: u64) -> Route {
		Route { bits: bits }
	}

	/// AB + BC = AC
	/// Splice
	#[inline]
	pub fn combine(&self, other: &Route) -> Option<Route> {
		if (self.bit_len() - 1) + (self.bit_len() - 1) > 59 { // TODO Why 59?
			// Route too long
			return None
		}

		let combined_route = ((other.bits ^ 1) << (self.bit_len() - 1)) ^ self.bits;
		Some(Route { bits: combined_route })
	}

	/// AC - AB = BC
	/// Unsplice
	#[inline]
	pub fn get_end(&self, front: &Route) -> Route {
		Route { bits: self.bits >> (front.bit_len() - 1) }
	}

	/// ABCD.goes_through(A/AB/ABC)
	#[inline]
	pub fn goes_through(&self, other: &Route) -> bool {
		if other.bits > self.bits {
			false
		}
		else if other.bits == 0 || other.bits == 1 {
			true
		}
		else {
			let mask = u64::MAX >> (other.bits.leading_zeros() + 1);
			(self.bits & mask) == (other.bits & mask)
		}
	}

	#[inline]
	pub fn bit_len(&self) -> uint {
		64 - self.bits.leading_zeros()
	}
}

impl fmt::Show for Route {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:064b}", self.bits)
	}
}



#[cfg(test)]
mod tests {
	use route::Route;

	#[test]
	fn test_combine() {
		let ab = Route::new(0b0000000000000000000000000000000000000000000001011101110101011001);
		let bc = Route::new(0b0000000000000000000000000000000000000000000000000000110101010100);
		let ac = Route::new(0b0000000000000000000000000000000000110101010100011101110101011001);
		assert_eq!(ab.combine(&bc).unwrap(), ac);
	}


	#[test]
	fn test_bit_len() {
		assert_eq!(Route::new(0b0000000000000000000000000000000000000000000000000000000000000000).bit_len(), 0);
		assert_eq!(Route::new(0b0000000000000000000000000000000000000000000000000000000000000001).bit_len(), 1);
		assert_eq!(Route::new(0b0000000000000000000000000000000000000000000000000000000000000010).bit_len(), 2);
		assert_eq!(Route::new(0b0000000000000000000000000000000000000000000000000000000000000011).bit_len(), 2);
		assert_eq!(Route::new(0b0000000000000000000000000000000000000000000000000000000000000011).bit_len(), 2);
		assert_eq!(Route::new(0b1000000000000000000000000000000000000000000000000000000000000000).bit_len(), 64);
		assert_eq!(Route::new(0b0100000000000000000000000000000000000000000000000000000000000000).bit_len(), 63);
		assert_eq!(Route::new(0b1111111111111111111111111111111111111111111111111111111111111111).bit_len(), 64);
		assert_eq!(Route::new(0b0000000000000000000000000000000000000000000001011101110101011001).bit_len(), 19);
	}
}
