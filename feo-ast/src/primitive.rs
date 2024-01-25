use crate::span::{Span, Spanned};
use crate::ty::Type;
use crate::U256; // native copy of `bnum::types::U256` / `bnum::BUint`

pub trait Primitive
where
    Self: Type,
{
}

impl Primitive for char {}

impl Primitive for String {}

impl Primitive for i32 {}

impl Primitive for i64 {}

impl Primitive for u8 {}

impl Primitive for u16 {}

impl Primitive for u32 {}

impl Primitive for u64 {}

impl Primitive for U256 {}

impl Primitive for [u8; 32] {}

impl Primitive for f32 {}

impl Primitive for f64 {}

impl Primitive for bool {}

pub trait PrimitiveType<P>
where
    Self: Sized,
    P: 'static + Clone + Primitive,
{
    fn new(raw_value: P, span: Span) -> Self;
    fn raw_value(&self) -> &P;
}

impl Type for char {}

impl Spanned for char {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Type for String {}

impl Spanned for String {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Type for i32 {}

impl Spanned for i32 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Type for i64 {}

impl Spanned for i64 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Type for u8 {}

impl Spanned for u8 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Type for u16 {}

impl Spanned for u16 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Type for u32 {}

impl Spanned for u32 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Type for u64 {}

impl Spanned for u64 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Type for U256 {}

impl Spanned for U256 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Type for f32 {}

impl Spanned for f32 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Type for f64 {}

impl Spanned for f64 {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Type for [u8; 32] {}

impl Spanned for [u8; 32] {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Type for bool {}

impl Spanned for bool {
    fn span(&self) -> Span {
        Span::default()
    }
}
