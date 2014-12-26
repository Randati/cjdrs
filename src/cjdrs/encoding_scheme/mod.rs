//! Self interface (1) must be encoded as 1 with 3 or more leading zeros.
//! Route label's 3 highest bits must not be all zero.

pub mod fixed4;
pub mod fixed8;
pub mod variable3x5x8;
pub mod variable4x8;

// trait EncodingScheme {
// 	fn bits_used_for_label(label: u64) -> u8;
// 	fn bits_used_for_number(number: u32) -> u8;
// 	fn compress(number: u32) -> u64;
// 	fn decompress(label: u64) -> u32;
// }

// TODO Use a trait
// TODO Tests
