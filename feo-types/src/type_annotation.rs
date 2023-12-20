use std::str::FromStr;

use crate::span::{Span, Spanned};

#[derive(Debug, Clone)]
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

impl FromStr for TypeName {
    type Err = ();

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
