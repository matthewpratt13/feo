use core::str::FromStr;

use crate::{
    error::TypeErrorKind,
    span::{Span, Spanned},
};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeName {
    BoolType,
    CharType,
    F32Type,
    F64Type,
    I32Type,
    I64Type,
    StringType,
    U8Type,
    U16Type,
    U32Type,
    U64Type,
    VecType,

    CustomType(String),
}

impl TypeName {
    pub fn as_str(&self) -> &str {
        match self {
            TypeName::BoolType => "bool",
            TypeName::CharType => "char",
            TypeName::F32Type => "f32",
            TypeName::F64Type => "f64",
            TypeName::I32Type => "i32",
            TypeName::I64Type => "i64",
            TypeName::StringType => "String",
            TypeName::U8Type => "u8",
            TypeName::U16Type => "u16",
            TypeName::U32Type => "u32",
            TypeName::U64Type => "u64",
            TypeName::VecType => "Vec",
            TypeName::CustomType(s) => s.as_str(),
        }
    }
}

impl FromStr for TypeName {
    type Err = TypeErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let type_name = match s {
            "bool" => Ok(TypeName::BoolType),
            "char" => Ok(TypeName::CharType),
            "f32" => Ok(TypeName::F32Type),
            "f64" => Ok(TypeName::F64Type),
            "i32" => Ok(TypeName::I32Type),
            "i64" => Ok(TypeName::I64Type),
            "String" => Ok(TypeName::StringType),
            "u8" => Ok(TypeName::U8Type),
            "u16" => Ok(TypeName::U16Type),
            "u32" => Ok(TypeName::U32Type),
            "u64" => Ok(TypeName::U64Type),
            "Vec" => Ok(TypeName::VecType),
            _ => Ok(TypeName::CustomType(s.to_string())),
        }?;

        Ok(type_name)
    }
}

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub type_name: TypeName,
    span: Span,
}

impl TypeAnnotation {
    pub fn new(type_name: TypeName, span: Span) -> Self {
        Self { type_name, span }
    }
}

impl Spanned for TypeAnnotation {
    fn span(&self) -> &Span {
        &self.span
    }
}
