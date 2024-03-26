mod array_type;
mod function_type;
mod impl_trait_type;
mod parenthesized_type;
mod reference_type;
mod self_type;
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
    self_type::SelfType,
    tuple_type::{TupleType, UnitType},
};

/// Defines a value's memory interpretation and the appropriate operations that may be performed/
#[derive(Debug, Clone)]
pub enum Type {
    // `char`, `str`, `bool`, numeric types
    PrimitiveType(BuiltInType),

    // sequence types (built in)
    ArrayType(ArrayType),
    TupleType(TupleType),

    UnitType(UnitType),

    // structs, enums
    UserDefinedType(PathType),

    // function types
    FunctionType(FunctionType),
    ClosureType(ClosureType),

    // trait type
    ImplTraitType(ImplTraitType), // TODO: come up with a better name

    ReferenceType(ReferenceType),

    ParenthesizedType(ParenthesizedType),
    SelfType(SelfType),

    InferredType(BuiltInType),
}

impl Spanned for Type {
    fn span(&self) -> Span {
        match self {
            Type::PrimitiveType(pt) => pt.span(),
            Type::ArrayType(arr) => arr.span(),
            Type::TupleType(tup) => tup.span(),
            Type::UnitType(ut) => ut.span(),
            Type::UserDefinedType(ud) => ud.span(),
            Type::FunctionType(fun) => fun.span(),
            Type::ClosureType(clo) => clo.span(),
            Type::ReferenceType(rt) => rt.span(),
            Type::ImplTraitType(imp) => imp.span(),
            Type::ParenthesizedType(par) => par.span(),
            Type::SelfType(st) => st.span(),
            Type::InferredType(it) => it.span(),
        }
    }
}
