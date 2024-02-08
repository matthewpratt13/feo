use feo_types::{
    primitive::{Primitive, PrimitiveType},
    span::{Span, Spanned},
    U256,
};

use crate::pattern::RangePattBound;

#[derive(Debug, Clone)]
pub struct Literal<T: Clone + PrimitiveType> {
    pub inner_value: Primitive<T>,
    span: Span,
}

impl<T> Literal<T>
where
    T: Clone + PrimitiveType,
{
    pub fn new(raw_value: Primitive<T>, span: Span) -> Literal<T> {
        Literal {
            inner_value: raw_value,
            span,
        }
    }

    pub fn into_inner(&self) -> T {
        self.inner_value.raw_value()
    }
}

impl<T> Spanned for Literal<T>
where
    T: Clone + PrimitiveType,
{
    fn span(&self) -> Span {
        self.span.clone()
    }
}

// pub trait LiteralPatt
// where
//     Self: Sized + Pattern + 'static,
// {
// }

#[derive(Clone)]
pub enum LiteralKind {
    Char(Literal<char>),
    String(Literal<&'static str>),
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
            LiteralKind::I32(ia) => ia.span(),
            LiteralKind::I64(ib) => ib.span(),
            LiteralKind::U8(ua) => ua.span(),
            LiteralKind::U16(ub) => ub.span(),
            LiteralKind::U32(uc) => uc.span(),
            LiteralKind::U64(ud) => ud.span(),
            LiteralKind::U256(ue) => ue.span(),
            LiteralKind::F32(fa) => fa.span(),
            LiteralKind::F64(fb) => fb.span(),
        }
    }
}

// impl<T> LiteralPatt for Literal<T> where T: Clone + PrimitiveType + 'static {}

// impl<T> Pattern for Literal<T> where T: Clone + PrimitiveType {}

// impl<T> PatternWithoutRange for Literal<T> where T: Clone + PrimitiveType {}

impl RangePattBound for Literal<char> {}

impl RangePattBound for Literal<i32> {}

impl RangePattBound for Literal<i64> {}

impl RangePattBound for Literal<u8> {}

impl RangePattBound for Literal<u16> {}

impl RangePattBound for Literal<u32> {}

impl RangePattBound for Literal<u64> {}

impl RangePattBound for Literal<U256> {}

impl RangePattBound for Literal<f32> {}

impl RangePattBound for Literal<f64> {}
