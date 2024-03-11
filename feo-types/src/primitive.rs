use crate::{
    span::{Span, Spanned},
    U256,
};

#[derive(Debug, Clone)]
pub enum Primitive {
    Char(CharPrimitive),
    Str(StrPrimitive),
    Bool(BoolPrimitive),
    Int(I64Primitive),
    UInt(U64Primitive),
    U256(U256Primitive),
    Float(F64Primitive),
}

#[derive(Debug, Clone)]
pub struct CharPrimitive {
    pub value: char,
    span: Span,
}

impl CharPrimitive {
    pub fn new(value: char, span: Span) -> Self {
        Self { value, span }
    }
}

impl Spanned for CharPrimitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct StrPrimitive {
    pub value: &'static str,
    span: Span,
}

impl StrPrimitive {
    pub fn new(value: &'static str, span: Span) -> Self {
        Self { value, span }
    }
}

impl Spanned for StrPrimitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct BoolPrimitive {
    pub value: bool,
    span: Span,
}

impl BoolPrimitive {
    pub fn new(value: bool, span: Span) -> Self {
        Self { value, span }
    }
}

impl Spanned for BoolPrimitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct I32Primitive {
    pub value: i32,
    span: Span,
}

impl I32Primitive {
    pub fn new(value: i32, span: Span) -> Self {
        Self { value, span }
    }
}

impl Spanned for I32Primitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct I64Primitive {
    pub value: i64,
    span: Span,
}

impl I64Primitive {
    pub fn new(value: i64, span: Span) -> Self {
        Self { value, span }
    }
}

impl Spanned for I64Primitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct U8Primitive {
    pub value: u8,
    span: Span,
}

impl U8Primitive {
    pub fn new(value: u8, span: Span) -> Self {
        Self { value, span }
    }
}

impl Spanned for U8Primitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct U16Primitive {
    pub value: u16,
    span: Span,
}

impl U16Primitive {
    pub fn new(value: u16, span: Span) -> Self {
        Self { value, span }
    }
}

impl Spanned for U16Primitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct U32Primitive {
    pub value: u32,
    span: Span,
}

impl U32Primitive {
    pub fn new(value: u32, span: Span) -> Self {
        Self { value, span }
    }
}

impl Spanned for U32Primitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct U64Primitive {
    pub value: u64,
    span: Span,
}

impl U64Primitive {
    pub fn new(value: u64, span: Span) -> Self {
        Self { value, span }
    }
}

impl Spanned for U64Primitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct U256Primitive {
    pub value: U256,
    span: Span,
}

impl U256Primitive {
    pub fn new(value: U256, span: Span) -> Self {
        Self { value, span }
    }
}

impl Spanned for U256Primitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct F32Primitive {
    pub value: f32,
    span: Span,
}

impl F32Primitive {
    pub fn new(value: f32, span: Span) -> Self {
        Self { value, span }
    }
}

impl Spanned for F32Primitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct F64Primitive {
    pub value: f64,
    span: Span,
}

impl F64Primitive {
    pub fn new(value: f64, span: Span) -> Self {
        Self { value, span }
    }
}

impl Spanned for F64Primitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}
