use std::fmt::Debug;

use crate::{
    span::{Span, Spanned},
    U256,
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

impl LiteralType for UIntType {}

impl LiteralType for U256 {}

impl LiteralType for FloatType {}

#[derive(Debug, Clone, PartialEq, PartialOrd)]

pub enum IntType {
    I32(i32),
    I64(i64),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum UIntType {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FloatType {
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone)]
pub struct Literal<T: LiteralType> {
    inner_value: T,
    span: Span,
}

impl<T> Literal<T>
where
    T: LiteralType,
{
    pub fn new(raw_value: T, span: Span) -> Literal<T> {
        Literal::<T> {
            inner_value: raw_value,
            span,
        }
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
    U8(Literal<UIntType>),
    U16(Literal<UIntType>),
    U32(Literal<UIntType>),
    U64(Literal<UIntType>),
    U256(Literal<U256>),
    F32(Literal<FloatType>),
    F64(Literal<FloatType>),
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
