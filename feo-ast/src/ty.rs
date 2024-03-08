#![allow(dead_code)]

mod array_type;
mod impl_trait_type;
mod parenthesized_type;
mod primitive_type;
mod reference_type;
mod tuple_type;

use feo_types::span::{Span, Spanned};

use crate::{
    expression::{ClosureExprKind, StructExprKind},
    item::{EnumDef, FunctionDefKind},
};

pub use self::{
    array_type::ArrayType,
    impl_trait_type::{ImplTraitType, TraitBound},
    parenthesized_type::ParenthesizedType,
    primitive_type::PrimitiveType,
    reference_type::ReferenceType,
    tuple_type::{TupleType, UnitType},
};

#[derive(Debug, Clone)]
pub enum Type {
    // primitives (built-in)
    Char(PrimitiveType),
    String(PrimitiveType),
    Bool(PrimitiveType),
    I32(PrimitiveType),
    I64(PrimitiveType),
    U8(PrimitiveType),
    U16(PrimitiveType),
    U32(PrimitiveType),
    U64(PrimitiveType),
    U256(PrimitiveType),
    F32(PrimitiveType),
    F64(PrimitiveType),

    // built-in sequence types
    Array(ArrayType),
    Tuple(TupleType),

    Unit(UnitType),

    // user-defined types
    Struct(StructExprKind),
    Enum(EnumDef),

    // function types
    Function(FunctionDefKind),
    Closure(ClosureExprKind),

    // trait type
    ImplTrait(ImplTraitType), // TODO: come up with a better name

    ReferenceType(ReferenceType),

    ParenthesizedType(ParenthesizedType),
    InferredType,
}

impl Spanned for Type {
    fn span(&self) -> Span {
        match self {
            Type::Char(_)
            | Type::String(_)
            | Type::Bool(_)
            | Type::I32(_)
            | Type::I64(_)
            | Type::U8(_)
            | Type::U16(_)
            | Type::U32(_)
            | Type::U64(_)
            | Type::U256(_)
            | Type::F32(_)
            | Type::F64(_) => Span::default(),
            Type::Array(arr) => arr.span(),
            Type::Tuple(tup) => tup.span(),
            Type::Unit(ut) => ut.span(),
            Type::Struct(stc) => stc.span(),
            Type::Enum(e) => e.span(),
            Type::Function(fun) => fun.span(),
            Type::Closure(clo) => match clo {
                ClosureExprKind::ClosureWithBlock(cwb) => cwb.span(),
                ClosureExprKind::ClosureWithoutBlock(cb) => cb.span(),
            },
            Type::ReferenceType(r) => r.span(),
            Type::ImplTrait(imp) => imp.span(),
            Type::ParenthesizedType(par) => par.span(),
            Type::InferredType => Span::default(),
        }
    }
}
