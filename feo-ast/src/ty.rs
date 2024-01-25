#![allow(dead_code)]

use crate::span::Spanned;

mod array_type;
mod trait_object_type;
mod tuple_type;

pub use self::array_type::ArrayType;
pub use self::trait_object_type::{TraitBound, TraitObjectType};
pub use self::tuple_type::TupleType;

// built-in types:
// - array
// - literals (char, str, int, uint, float, bytes32, bool)
// - function, closure
// - path
// - trait object
// - tuple

pub trait Type
where
    Self: Spanned,
{
}
