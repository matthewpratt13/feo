use std::fmt::Debug;

use crate::{
    error::TypeErrorKind,
    span::{Span, Spanned},
    TypeAnnotation, U256,
};

pub trait LiteralType
where
    Self: 'static,
{
}

impl LiteralType for char {}

impl LiteralType for String {}

impl LiteralType for bool {}

impl LiteralType for IntType {}

impl LiteralType for UintType {}

impl LiteralType for U256 {}

impl LiteralType for f32 {}

impl LiteralType for f64 {}

#[derive(Debug, Clone)]
pub enum IntType {
    I32(i32),
    I64(i64),
}

impl TryFrom<IntType> for i32 {
    type Error = TypeErrorKind;

    fn try_from(value: IntType) -> Result<Self, Self::Error> {
        match value {
            IntType::I32(i) => Ok(i),
            IntType::I64(_) => Err(TypeErrorKind::MismatchedIntTypeAnnotation),
        }
    }
}

impl TryFrom<IntType> for i64 {
    type Error = TypeErrorKind;

    fn try_from(value: IntType) -> Result<Self, Self::Error> {
        match value {
            IntType::I32(_) => Err(TypeErrorKind::MismatchedIntTypeAnnotation),
            IntType::I64(i) => Ok(i),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum UintType {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl TryFrom<UintType> for u8 {
    type Error = TypeErrorKind;

    fn try_from(value: UintType) -> Result<Self, Self::Error> {
        match value {
            UintType::U8(ui) => Ok(ui),
            UintType::U16(_) => Err(TypeErrorKind::MismatchedUintTypeAnnotation),
            UintType::U32(_) => Err(TypeErrorKind::MismatchedUintTypeAnnotation),
            UintType::U64(_) => Err(TypeErrorKind::MismatchedUintTypeAnnotation),
        }
    }
}

impl TryFrom<UintType> for u16 {
    type Error = TypeErrorKind;

    fn try_from(value: UintType) -> Result<Self, Self::Error> {
        match value {
            UintType::U8(_) => Err(TypeErrorKind::MismatchedUintTypeAnnotation),
            UintType::U16(ui) => Ok(ui),
            UintType::U32(_) => Err(TypeErrorKind::MismatchedUintTypeAnnotation),
            UintType::U64(_) => Err(TypeErrorKind::MismatchedUintTypeAnnotation),
        }
    }
}

impl TryFrom<UintType> for u32 {
    type Error = TypeErrorKind;

    fn try_from(value: UintType) -> Result<Self, Self::Error> {
        match value {
            UintType::U8(_) => Err(TypeErrorKind::MismatchedUintTypeAnnotation),
            UintType::U16(_) => Err(TypeErrorKind::MismatchedUintTypeAnnotation),
            UintType::U32(ui) => Ok(ui),
            UintType::U64(_) => Err(TypeErrorKind::MismatchedUintTypeAnnotation),
        }
    }
}

impl TryFrom<UintType> for u64 {
    type Error = TypeErrorKind;

    fn try_from(value: UintType) -> Result<Self, Self::Error> {
        match value {
            UintType::U8(_) => Err(TypeErrorKind::MismatchedUintTypeAnnotation),
            UintType::U16(_) => Err(TypeErrorKind::MismatchedUintTypeAnnotation),
            UintType::U32(_) => Err(TypeErrorKind::MismatchedUintTypeAnnotation),
            UintType::U64(ui) => Ok(ui),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Literal<T: LiteralType> {
    inner_value: T,
    span: Span,
    type_ann_opt: Option<TypeAnnotation>,
}

impl<T> Literal<T>
where
    T: LiteralType,
{
    pub fn new(raw_value: T, span: Span, type_ann_opt: Option<TypeAnnotation>) -> Literal<T> {
        Literal::<T> {
            inner_value: raw_value,
            span,
            type_ann_opt,
        }
    }

    pub fn type_annotation(&self) -> Option<TypeAnnotation> {
        self.type_ann_opt.clone()
    }

    pub fn into_inner(self) -> Option<T> {
        Some(self.inner_value)
    }
}

impl<T> Spanned for Literal<T>
where
    T: LiteralType,
{
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub enum LiteralKind {
    Char(Literal<char>),
    String(Literal<String>),
    Bool(Literal<bool>),
    I32(Literal<IntType>),
    I64(Literal<IntType>),
    U8(Literal<UintType>),
    U16(Literal<UintType>),
    U32(Literal<UintType>),
    U64(Literal<UintType>),
    U256(Literal<U256>),
    F32(Literal<f32>),
    F64(Literal<f64>),
}

impl Spanned for LiteralKind {
    fn span(&self) -> Span {
        match self {
            LiteralKind::Char(c) => c.span(),
            LiteralKind::String(s) => s.span(),
            LiteralKind::Bool(b) => b.span(),
            LiteralKind::I32(i) => i.span(),
            LiteralKind::I64(i) => i.span(),
            LiteralKind::U8(ui) => ui.span(),
            LiteralKind::U16(ui) => ui.span(),
            LiteralKind::U32(ui) => ui.span(),
            LiteralKind::U64(ui) => ui.span(),
            LiteralKind::U256(u) => u.span(),
            LiteralKind::F32(f) => f.span(),
            LiteralKind::F64(f) => f.span(),
        }
    }
}
