use crate::span::Span;
use crate::U256; // native copy of `bnum::types::U256` / `bnum::BUint`

pub trait Primitive {}

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
    P: 'static + Primitive + Clone,
{
    fn new(raw_value: P, span: Span) -> Self;
    fn raw_value(&self) -> &P;
}
