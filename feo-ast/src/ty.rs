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
    expression::{ClosureExprKind, StructExprKind},
    item::{EnumDef, FunctionDefKind},
};

use self::{parenthesized_type::ParenthesizedType, reference_type::ReferenceType};

pub use self::{
    array_type::ArrayType,
    impl_trait_type::{ImplTraitType, TraitBound},
    tuple_type::{TupleType, UnitType},
};

#[derive(Debug, Clone)]
pub enum Type {
    // primitives (built-in)
    Char(Primitive<char>),
    Str(Primitive<String>),
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
            Type::Char(c) => c.span(),
            Type::Str(s) => s.span(),
            Type::Bool(b) => b.span(),
            Type::I32(i) => i.span(),
            Type::I64(i) => i.span(),
            Type::U8(ui) => ui.span(),
            Type::U16(ui) => ui.span(),
            Type::U32(ui) => ui.span(),
            Type::U64(ui) => ui.span(),
            Type::U256(u) => u.span(),
            Type::F32(f) => f.span(),
            Type::F64(f) => f.span(),
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
