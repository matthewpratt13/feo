use std::fmt::{self, Debug};

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

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]

pub enum IntType {
    I32(i32),
    I64(i64),
}

impl fmt::Display for IntType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntType::I32(i) => write!(f, "{}", i),
            IntType::I64(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum UIntType {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl fmt::Display for UIntType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIntType::U8(ui) => write!(f, "{}", ui),
            UIntType::U16(ui) => write!(f, "{}", ui),
            UIntType::U32(ui) => write!(f, "{}", ui),
            UIntType::U64(ui) => write!(f, "{}", ui),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum FloatType {
    F32(f32),
    F64(f64),
}

impl fmt::Display for FloatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FloatType::F32(fl) => write!(f, "{}", fl),
            FloatType::F64(fl) => write!(f, "{}", fl),
        }
    }
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

impl<T> fmt::Display for Literal<T>
where
    T: LiteralType + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner_value)
    }
}

#[derive(Debug, Clone)]
pub enum LiteralKind {
    Char(Literal<char>),
    String(Literal<String>),
    Bool(Literal<bool>),
    Int(Literal<IntType>),
    UInt(Literal<UIntType>),
    U256(Literal<U256>),
    Float(Literal<FloatType>),
}

impl LiteralKind {
    pub fn default_char() -> LiteralKind {
        LiteralKind::Char(Literal::new(char::default(), Span::default()))
    }

    pub fn default_string() -> LiteralKind {
        LiteralKind::String(Literal::new(String::default(), Span::default()))
    }

    pub fn default_bool() -> LiteralKind {
        LiteralKind::Bool(Literal::new(bool::default(), Span::default()))
    }

    pub fn default_int() -> LiteralKind {
        LiteralKind::Int(Literal::new(IntType::I64(i64::default()), Span::default()))
    }

    pub fn default_uint() -> LiteralKind {
        LiteralKind::UInt(Literal::new(UIntType::U64(u64::default()), Span::default()))
    }

    pub fn default_u256() -> LiteralKind {
        LiteralKind::U256(Literal::new(U256::default(), Span::default()))
    }

    pub fn default_float() -> LiteralKind {
        LiteralKind::Float(Literal::new(
            FloatType::F64(f64::default()),
            Span::default(),
        ))
    }
}

impl Spanned for LiteralKind {
    fn span(&self) -> Span {
        match self {
            LiteralKind::Char(c) => c.span(),
            LiteralKind::String(s) => s.span(),
            LiteralKind::Bool(b) => b.span(),
            LiteralKind::Int(i) => i.span(),
            LiteralKind::UInt(ui) => ui.span(),
            LiteralKind::U256(u) => u.span(),
            LiteralKind::Float(f) => f.span(),
        }
    }
}
