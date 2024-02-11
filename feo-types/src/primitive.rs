use std::fmt::Debug;

use crate::U256;

pub trait PrimitiveType {}

impl PrimitiveType for char {}

impl PrimitiveType for &'static str {}

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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Primitive<P: PrimitiveType>(P);

impl<P> Primitive<P>
where
    P: PrimitiveType,
{
    pub fn raw_value(self) -> P {
        self.0
    }
}
