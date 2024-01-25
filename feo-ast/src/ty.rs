#![allow(dead_code)]

use crate::span::Spanned;

mod array_type;
mod impl_trait_type;
mod tuple_type;

pub use self::array_type::ArrayType;
pub use self::impl_trait_type::{ImplTraitType, TraitBound};
pub use self::tuple_type::TupleType;

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

mod parenthesized_type {
    use crate::{
        span::{Span, Spanned},
        type_utils::Parenthesis,
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