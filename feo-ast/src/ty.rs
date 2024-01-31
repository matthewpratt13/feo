#![allow(dead_code)]

mod array_type;
mod impl_trait_type;
mod tuple_type;

use feo_types::{span::Spanned, U256};

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

impl Type for char {}

impl Type for String {}

impl Type for i32 {}

impl Type for i64 {}

impl Type for u8 {}

impl Type for u16 {}

impl Type for u32 {}

impl Type for u64 {}

impl Type for U256 {}

impl Type for f32 {}

impl Type for f64 {}

impl Type for [u8; 32] {}

impl Type for bool {}

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
