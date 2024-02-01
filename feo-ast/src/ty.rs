#![allow(dead_code)]

mod array_type;
mod impl_trait_type;
mod tuple_type;

use feo_types::{primitive::Primitive, span::Spanned, U256};

pub use self::{impl_trait_type::TraitBound, tuple_type::TupleType};

// built-in types:
// - primitives (char, str, int, uint, float, bytes32, bool)
// - sequence types (array, tuple)
// - unit type
//
// user-defined types:
// - struct
// - enum
//
// function types:
// - function
// - closure
//
// trait types:
// - trait object (not used)
// - impl trait (one bound only)

pub trait Type
where
    Self: Spanned,
{
}

impl Type for Primitive<char> {}

impl Type for Primitive<String> {}

impl Type for Primitive<i32> {}

impl Type for Primitive<i64> {}

impl Type for Primitive<u8> {}

impl Type for Primitive<u16> {}

impl Type for Primitive<u32> {}

impl Type for Primitive<u64> {}

impl Type for Primitive<U256> {}

impl Type for Primitive<f32> {}

impl Type for Primitive<f64> {}

impl Type for Primitive<bool> {}

mod parenthesized_type {
    use feo_types::{
        span::{Span, Spanned},
        utils::Parenthesis,
    };

    use super::Type;

    pub struct ParenthesizedType {
        open_parenthesis: Parenthesis,
        ty: Box<dyn Type>,
        close_parenthesis: Parenthesis,
    }

    impl Type for ParenthesizedType {}

    impl Spanned for ParenthesizedType {
        fn span(&self) -> Span {
            let start_pos = self.open_parenthesis.span().start();
            let end_pos = self.close_parenthesis.span().end();
            let source = self.open_parenthesis.span().source();

            let span = Span::new(source.as_str(), start_pos, end_pos);

            span
        }
    }
}

// TODO: `ReferenceType` ?
