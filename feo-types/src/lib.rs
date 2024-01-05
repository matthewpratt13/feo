// copy all functionality of bnum::BUint to native type
pub type U256 = bnum::types::U256;

mod literal;
pub use crate::literal::*;

mod primitive;
pub use crate::primitive::*;

pub mod span;
