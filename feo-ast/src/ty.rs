use crate::{item::DblColon, path::PathSegment};

pub enum Type {
    ParenthesizedType,
    ImplTraitType,
    TraitObjectType,
    TypePath,
    TupleType,
    ReferenceType,
    ArrayType,
    SliceType,
    InferredType,
    QualifiedPathInType,
}
