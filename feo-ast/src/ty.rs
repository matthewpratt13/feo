#![allow(dead_code)]

mod array_type;
mod impl_trait_type;
mod tuple_type;

use feo_types::{
    primitive::Primitive,
    span::{Span, Spanned},
    U256,
};

use crate::{
    expression::{ClosureType, StructExprKind},
    item::{EnumItem, FunctionItem},
};

use self::{
    array_type::ArrayType, impl_trait_type::ImplTraitType, parenthesized_type::ParenthesizedType,
};
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

#[derive(Clone)]
pub enum Type {
    Char(Primitive<char>),
    Str(Primitive<&'static str>),
    Bool(Primitive<bool>),
    I32(Primitive<i32>),
    I64(Primitive<i64>),
    U8(Primitive<u8>),
    U16(Primitive<u16>),
    U32(Primitive<u32>),
    U64(Primitive<u64>),
    U256(Primitive<U256>),
    F32(Primitive<f32>),
    F64(Primitive<f64>),
    Unit(()),
    Array(ArrayType),
    Tuple(TupleType),
    Struct(StructExprKind),
    Enum(EnumItem),
    Function(FunctionItem),
    Closure(ClosureType),
    ImplTrait(ImplTraitType),
    InferredType,
    ParenthesizedType(ParenthesizedType),
}

impl Spanned for Type {
    fn span(&self) -> Span {
        todo!()
    }
}

mod parenthesized_type {
    use feo_types::{
        span::{Span, Spanned},
        utils::Parenthesis,
    };

    use super::Type;

    #[derive(Clone)]
    pub struct ParenthesizedType {
        open_parenthesis: Parenthesis,
        ty: Box<Type>,
        close_parenthesis: Parenthesis,
    }

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
