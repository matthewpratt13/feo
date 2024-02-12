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

impl LiteralType for i32 {}

impl LiteralType for i64 {}

impl LiteralType for IntType {}

impl LiteralType for u8 {}

impl LiteralType for u16 {}

impl LiteralType for u32 {}

impl LiteralType for u64 {}

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
            IntType::I64(_) => Err(TypeErrorKind::MismatchedTypeAnnotation),
        }
    }
}

impl TryFrom<IntType> for i64 {
    type Error = TypeErrorKind;

    fn try_from(value: IntType) -> Result<Self, Self::Error> {
        match value {
            IntType::I64(i) => Ok(i),
            IntType::I32(_) => Err(TypeErrorKind::MismatchedTypeAnnotation),
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
    U8(Literal<u8>),
    U16(Literal<u16>),
    U32(Literal<u32>),
    U64(Literal<u64>),
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
