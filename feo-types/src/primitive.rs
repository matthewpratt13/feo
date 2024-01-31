use crate::{
    span::{Span, Spanned},
    Bytes32, U256,
};

pub trait PrimitiveType {}

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

impl PrimitiveType for Bytes32 {}

#[derive(Debug, Clone, PartialEq)]
pub struct Primitive<P: Clone + PrimitiveType>(P);

impl<P> Primitive<P>
where
    P: Clone + PrimitiveType,
{
    pub fn new(raw_value: P) -> Primitive<P> {
        Primitive(raw_value)
    }

    pub fn raw_value(&self) -> P {
        self.0.clone()
    }
}

impl<P> Spanned for Primitive<P>
where
    P: Clone + PrimitiveType,
{
    fn span(&self) -> Span {
        Span::default()
    }
}
