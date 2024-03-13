use std::str::FromStr;

use crate::span::{Span, Spanned};

// TODO: find a way to check that the type annotation matches the actual token's type

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
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
    TypeAnnInferred,
}

impl TypeAnnotation {
    pub fn as_str(&self) -> &str {
        match self {
            TypeAnnotation::TypeAnnChar => "char",
            TypeAnnotation::TypeAnnStr => "str",
            TypeAnnotation::TypeAnnBool => "bool",
            TypeAnnotation::TypeAnnI32 => "i32",
            TypeAnnotation::TypeAnnI64 => "i64",
            TypeAnnotation::TypeAnnU8 => "u8",
            TypeAnnotation::TypeAnnU16 => "u16",
            TypeAnnotation::TypeAnnU32 => "u32",
            TypeAnnotation::TypeAnnU64 => "u64",
            TypeAnnotation::TypeAnnU256 => "u256",
            TypeAnnotation::TypeAnnF32 => "f32",
            TypeAnnotation::TypeAnnF64 => "f64",
            TypeAnnotation::TypeAnnInferred => "_",
        }
    }
}

impl FromStr for TypeAnnotation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let type_ann = match s {
            "char" => Ok(TypeAnnotation::TypeAnnChar),
            "str" => Ok(TypeAnnotation::TypeAnnStr),
            "bool" => Ok(TypeAnnotation::TypeAnnBool),
            "i32" => Ok(TypeAnnotation::TypeAnnI32),
            "i64" => Ok(TypeAnnotation::TypeAnnI64),
            "u8" => Ok(TypeAnnotation::TypeAnnU8),
            "u16" => Ok(TypeAnnotation::TypeAnnU16),
            "u32" => Ok(TypeAnnotation::TypeAnnU32),
            "u64" => Ok(TypeAnnotation::TypeAnnU64),
            "u256" => Ok(TypeAnnotation::TypeAnnU256),
            "f32" => Ok(TypeAnnotation::TypeAnnF32),
            "f64" => Ok(TypeAnnotation::TypeAnnF64),
            "_" => Ok(TypeAnnotation::TypeAnnInferred),
            _ => Err(()),
        }?;

        Ok(type_ann)
    }
}

#[derive(Debug, Clone)]
pub struct BuiltInType {
    pub type_annotation: TypeAnnotation,
    span: Span,
}

impl BuiltInType {
    pub fn new(type_annotation: TypeAnnotation, span: Span) -> Self {
        Self {
            type_annotation,
            span,
        }
    }
}

impl Spanned for BuiltInType {
    fn span(&self) -> Span {
        self.span.clone()
    }
}
