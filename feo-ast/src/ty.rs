#![allow(dead_code)]

mod array_type;
mod impl_trait_type;
mod parenthesized_type;
mod reference_type;
mod tuple_type;

use feo_types::{
    primitive::Primitive,
    span::{Span, Spanned},
    U256,
};

use crate::{
    expression::{ClosureType, StructExprKind},
    item::{EnumDef, FunctionDef},
};

use self::{
    array_type::ArrayType, impl_trait_type::ImplTraitType, parenthesized_type::ParenthesizedType,
};
pub use self::{impl_trait_type::TraitBound, tuple_type::TupleType};

#[derive(Clone)]
pub enum Type {
    // primitives (built-in)
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

    // built-in sequence types
    Array(ArrayType),
    Tuple(TupleType),

    Unit(()),

    // user-defined types
    Struct(StructExprKind),
    Enum(EnumDef),

    // function types
    Function(FunctionDef),
    Closure(ClosureType),

    // trait type
    ImplTrait(ImplTraitType),

    ParenthesizedType(ParenthesizedType),
    InferredType,
}

impl Spanned for Type {
    fn span(&self) -> Span {
        todo!()
    }
}
