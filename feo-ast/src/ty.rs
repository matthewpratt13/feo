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

pub struct TypePath {}
