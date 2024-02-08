use std::marker::PhantomData;

use feo_types::{
    primitive::Primitive,
    span::{Span, Spanned},
    U256,
};

#[derive(Debug, Clone)]
pub struct Literal<T> {
    pub inner_value: Primitive,
    span: Span,
    _phantom: PhantomData<T>,
}

impl<T> Literal<T> {
    pub fn new(raw_value: Primitive, span: Span) -> Literal<T> {
        Literal::<T> {
            inner_value: raw_value,
            span,
            _phantom: PhantomData,
        }
    }

    pub fn into_inner(self) -> Primitive {
        self.inner_value
    }
}

impl<T> Spanned for Literal<T> {
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
