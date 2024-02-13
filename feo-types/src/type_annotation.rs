use std::str::FromStr;

use crate::{
    error::TypeErrorKind,
    span::{Span, Spanned},
};

// TODO: find a way to check that the type annotation matches the actual token's type

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnKind {
    TypeAnnChar,
    TypeAnnStr,
    TypeAnnBool,
    TypeAnnI32,
    TypeAnnI64,
    TypeAnnU8,
    TypeAnnU16,
    TypeAnnU32,
    TypeAnnU64,
    TypeAnnU256,
    TypeAnnF32,
    TypeAnnF64,

    CustomTypeAnn(String),
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
            TypeAnnKind::CustomTypeAnn(t) => t,
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
            _ => Ok(TypeAnnKind::CustomTypeAnn(s.to_string())),
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
        self.span.clone()
    }
}

pub fn is_built_in_type_annotation(iden: &str) -> bool {
    [
        "char", "str", "bool", "i32", "i64", "u8", "u16", "u32", "u64", "u256", "f32", "f64",
    ]
    .contains(&iden)
}
