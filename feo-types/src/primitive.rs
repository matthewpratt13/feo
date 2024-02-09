use std::fmt::Debug;

use crate::U256;

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
