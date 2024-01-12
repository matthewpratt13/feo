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

pub struct TypePath {
    dbl_colon_opt: Option<DblColon>,
    first_segment: PathSegment,
    subsequent_segments: Vec<(DblColon, PathSegment)>,
}
