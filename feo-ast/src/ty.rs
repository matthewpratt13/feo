#![allow(dead_code)]

mod array_type;
mod function_type;
mod impl_trait_type;
mod parenthesized_type;
mod reference_type;
mod tuple_type;

use feo_types::{
    span::{Span, Spanned},
    BuiltInType,
};

use crate::path::PathType;

pub use self::{
    array_type::ArrayType,
    function_type::{ClosureType, FunctionType},
    impl_trait_type::{ImplTraitType, TraitBound},
    parenthesized_type::ParenthesizedType,
    reference_type::ReferenceType,
    tuple_type::{TupleType, UnitType},
};

#[derive(Debug, Clone)]
pub enum Type {
    // primitives (built-in)
    Char(BuiltInType),
    Str(BuiltInType),
    Bool(BuiltInType),
    I32(BuiltInType),
    I64(BuiltInType),
    U8(BuiltInType),
    U16(BuiltInType),
    U32(BuiltInType),
    U64(BuiltInType),
    U256(BuiltInType),
    F32(BuiltInType),
    F64(BuiltInType),

    // built-in sequence types
    Array(ArrayType),
    Tuple(TupleType),

    Unit(UnitType),

    // user-defined types
    Struct(PathType),
    Enum(PathType),

    // function types
    Function(FunctionType),
    Closure(ClosureType),

    // trait type
    ImplTrait(ImplTraitType), // TODO: come up with a better name

    ReferenceType(ReferenceType),

    ParenthesizedType(ParenthesizedType),
    SelfType(PathType),

    InferredType(BuiltInType),
}

impl Spanned for Type {
    fn span(&self) -> Span {
        match self {
            Type::Char(c) => c.span(),
            Type::Str(s) => s.span(),
            Type::Bool(b) => b.span(),
            Type::I32(ia) => ia.span(),
            Type::I64(ib) => ib.span(),
            Type::U8(uia) => uia.span(),
            Type::U16(uib) => uib.span(),
            Type::U32(uic) => uic.span(),
            Type::U64(uid) => uid.span(),
            Type::U256(u) => u.span(),
            Type::F32(fa) => fa.span(),
            Type::F64(fb) => fb.span(),
            Type::Array(arr) => arr.span(),
            Type::Tuple(tup) => tup.span(),
            Type::Unit(ut) => ut.span(),
            Type::Struct(stc) => stc.span(),
            Type::Enum(e) => e.span(),
            Type::Function(fun) => fun.span(),
            Type::Closure(clo) => clo.span(),
            Type::ReferenceType(r) => r.span(),
            Type::ImplTrait(imp) => imp.span(),
            Type::ParenthesizedType(par) => par.span(),
            Type::SelfType(st) => st.span(),
            Type::InferredType(it) => it.span(),
        }
    }
}
