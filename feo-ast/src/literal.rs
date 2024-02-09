use std::fmt::Debug;

use feo_types::{
    span::{Span, Spanned},
    U256,
};

pub trait LiteralType
where
    Self: Sized + Debug + Clone + PartialEq,
{
}

impl LiteralType for char {}

impl LiteralType for String {}

impl LiteralType for bool {}

impl LiteralType for i32 {}

impl LiteralType for i64 {}

impl LiteralType for u8 {}

impl LiteralType for u16 {}

impl LiteralType for u32 {}

impl LiteralType for u64 {}

impl LiteralType for U256 {}

impl LiteralType for f32 {}

impl LiteralType for f64 {}

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

#[derive(Clone)]
pub enum LiteralKind {
    Char(Literal<char>),
    String(Literal<String>),
    Bool(Literal<bool>),
    I32(Literal<i32>),
    I64(Literal<i64>),
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

// impl<T> LiteralPatt for Literal<T> where T: Clone + PrimitiveType + 'static {}

// impl<T> Pattern for Literal<T> where T: Clone + PrimitiveType {}

// impl<T> PatternWithoutRange for Literal<T> where T: Clone + PrimitiveType {}
