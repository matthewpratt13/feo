use std::fmt::Debug;

use crate::{
    span::{Span, Spanned},
    U256,
};

pub trait PrimitiveType
where
    Self: Sized,
{
}

impl PrimitiveType for char {}

impl PrimitiveType for String {}

impl PrimitiveType for bool {}

impl PrimitiveType for i32 {}

impl PrimitiveType for i64 {}

impl PrimitiveType for u8 {}

impl PrimitiveType for u16 {}

impl PrimitiveType for u32 {}

impl PrimitiveType for u64 {}

impl PrimitiveType for U256 {}

impl PrimitiveType for f32 {}

impl PrimitiveType for f64 {}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Primitive<P: PrimitiveType>(pub P);

impl<P> Primitive<P>
where
    P: PrimitiveType + 'static,
{
    pub fn inner_ref(&self) -> &P {
        &self.0
    }
}

impl<P> Spanned for Primitive<P>
where
    P: PrimitiveType + Spanned,
{
    fn span(&self) -> Span {
        self.0.span()
    }
}

impl Spanned for char {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for String {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for bool {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for i32 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for i64 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for u8 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for u16 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for u32 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for u64 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for U256 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for f32 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for f64 {
    fn span(&self) -> Span {
        Span::default()
    }
}
