use std::sync::Arc;

use crate::span::Span;

pub trait Primitive {}

impl Primitive for char {}

impl Primitive for String {}

impl Primitive for Vec<String> {}

impl Primitive for Arc<String> {}

impl Primitive for i32 {}

impl Primitive for i64 {}

impl Primitive for u8 {}

impl Primitive for u16 {}

impl Primitive for u32 {}

impl Primitive for u64 {}

impl Primitive for f32 {}

impl Primitive for f64 {}

impl Primitive for bool {}

pub trait PrimitiveType<P>
where
    Self: Sized,
    P: 'static + Primitive,
{
    fn new(raw_value: P, span: Span) -> Self;
    fn raw_value(&self) -> &P;
}
