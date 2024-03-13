#![allow(dead_code)]

mod array_type;
mod function_type;
mod impl_trait_type;
mod parenthesized_type;
mod reference_type;
mod tuple_type;

use feo_types::{
    span::{Span, Spanned},
    BoolPrimitive, CharPrimitive, F32Primitive, F64Primitive, I32Primitive, I64Primitive,
    StrPrimitive, U16Primitive, U256Primitive, U32Primitive, U64Primitive, U8Primitive,
};

use crate::path::PathType;

use self::function_type::{ClosureType, FunctionType};
pub use self::{
    array_type::ArrayType,
    impl_trait_type::{ImplTraitType, TraitBound},
    parenthesized_type::ParenthesizedType,
    reference_type::ReferenceType,
    tuple_type::{TupleType, UnitType},
};

#[derive(Debug, Clone)]
pub enum Type {
    // primitives (built-in)
    Char(CharPrimitive),
    Str(StrPrimitive),
    Bool(BoolPrimitive),
    I32(I32Primitive),
    I64(I64Primitive),
    U8(U8Primitive),
    U16(U16Primitive),
    U32(U32Primitive),
    U64(U64Primitive),
    U256(U256Primitive),
    F32(F32Primitive),
    F64(F64Primitive),

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

    InferredType,
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
            Type::InferredType => Span::default(),
        }
    }
}
