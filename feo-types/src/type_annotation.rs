use std::str::FromStr;

use crate::{
    span::{Span, Spanned},
    utils::TypeErrorKind,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnKind {
    // primitives: pushed to the stack, memory allocated at compile time
    TypeAnnChar, // 8-bit (`u8`) ASCII value
    TypeAnnStr,  // arbitrary length, static, immutable; for fixed length (const), use `[char; n]`
    TypeAnnBool,
    TypeAnnI32,
    TypeAnnI64,
    TypeAnnU8,
    TypeAnnU16,
    TypeAnnU32,
    TypeAnnU64,  // default numeric type
    TypeAnnU256, // equivalent to `[u64; 4]`; fixed length, constant, immutable
    TypeAnnF32,
    TypeAnnF64,
    TypeAnnBytes32, // equivalent to `[u8; 32]`; fixed length, static, immutable

    // built-in dynamic types: stored on the heap, memory allocated at runtime
    TypeAnnString, // arbitrary length, dynamic, mutable string
    TypeAnnVec,    // arbitrary length, dynamic, mutable array

    CustomTypeAnn,
}

impl TypeAnnKind {
    pub fn as_str(&self) -> &str {
        match self {
            TypeAnnKind::TypeAnnChar => "char",
            TypeAnnKind::TypeAnnStr => "str",
            TypeAnnKind::TypeAnnBool => "bool",
            TypeAnnKind::TypeAnnI32 => "i32",
            TypeAnnKind::TypeAnnI64 => "i64",
            TypeAnnKind::TypeAnnU8 => "u8",
            TypeAnnKind::TypeAnnU16 => "u16",
            TypeAnnKind::TypeAnnU32 => "u32",
            TypeAnnKind::TypeAnnU64 => "u64",
            TypeAnnKind::TypeAnnU256 => "u256",
            TypeAnnKind::TypeAnnF32 => "f32",
            TypeAnnKind::TypeAnnF64 => "f64",
            TypeAnnKind::TypeAnnBytes32 => "bytes32",
            TypeAnnKind::TypeAnnString => "String",
            TypeAnnKind::TypeAnnVec => "Vec",
            TypeAnnKind::CustomTypeAnn => "_",
        }
    }
}

impl FromStr for TypeAnnKind {
    type Err = TypeErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let type_ann = match s {
            "char" => Ok(TypeAnnKind::TypeAnnChar),
            "str" => Ok(TypeAnnKind::TypeAnnStr),
            "bool" => Ok(TypeAnnKind::TypeAnnBool),
            "i32" => Ok(TypeAnnKind::TypeAnnI32),
            "i64" => Ok(TypeAnnKind::TypeAnnI64),
            "u8" => Ok(TypeAnnKind::TypeAnnU8),
            "u16" => Ok(TypeAnnKind::TypeAnnU16),
            "u32" => Ok(TypeAnnKind::TypeAnnU32),
            "u64" => Ok(TypeAnnKind::TypeAnnU64),
            "u256" => Ok(TypeAnnKind::TypeAnnU256),
            "f32" => Ok(TypeAnnKind::TypeAnnF32),
            "f64" => Ok(TypeAnnKind::TypeAnnF64),
            "bytes32" => Ok(TypeAnnKind::TypeAnnBytes32),
            "String" => Ok(TypeAnnKind::TypeAnnString),
            "Vec" => Ok(TypeAnnKind::TypeAnnVec),
            _ => Ok(TypeAnnKind::CustomTypeAnn),
        }?;

        Ok(type_ann)
    }
}

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub type_ann_kind: TypeAnnKind,
    span: Span,
}

impl TypeAnnotation {
    pub fn new(type_ann_kind: TypeAnnKind, span: Span) -> Self {
        Self {
            type_ann_kind,
            span,
        }
    }
}

impl Spanned for TypeAnnotation {
    fn span(&self) -> Span {
        self.clone().span
    }
}

pub fn is_built_in_type_annotation(iden: &str) -> bool {
    [
        "char", "str", "bool", "i32", "i64", "u8", "u16", "u32", "u64", "u256", "f32", "f64",
        "bytes32", "String", "Vec",
    ]
    .contains(&iden)
}
