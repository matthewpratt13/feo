use std::fmt::Debug;

use crate::U256;

// pub trait PrimitiveType
// where
//     Self: Sized + Debug + Clone + PartialEq + 'static,
// {
// }

// impl PrimitiveType for char {}

// impl PrimitiveType for &'static str {}

// impl PrimitiveType for String {}

// impl PrimitiveType for bool {}

// impl PrimitiveType for i32 {}

// impl PrimitiveType for i64 {}

// impl PrimitiveType for u8 {}

// impl PrimitiveType for u16 {}

// impl PrimitiveType for u32 {}

// impl PrimitiveType for u64 {}

// impl PrimitiveType for U256 {}

// impl PrimitiveType for f32 {}

// impl PrimitiveType for f64 {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Primitive {
    Char(char),
    Str(&'static str),
    Bool(bool),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U256(U256),
    F32(f32),
    F64(f64),
}

// #[derive(Debug, Clone)]
// pub struct Primitive<P: PrimitiveType>(P);

// impl<P> Primitive<P>
// where
//     P: PrimitiveType,
// {
//     pub fn new(raw_value: P) -> Primitive<P> {
//         Primitive(raw_value)
//     }

//     pub fn raw_value(self) -> P {
//         self.0
//     }
// }

// impl<P> Spanned for Primitive<P>
// where
//     P: PrimitiveType,
// {
//     fn span(&self) -> Span {
//         Span::default()
//     }
// }
