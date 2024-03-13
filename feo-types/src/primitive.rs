use crate::{
    error::TypeErrorKind,
    literal::UIntType,
    span::{Span, Spanned},
    type_annotation::TypeAnnKind,
    Literal, U256,
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
    pub type_ann: TypeAnnKind,
    span: Span,
}

impl CharPrimitive {
    pub fn new(value: char, span: Span) -> Self {
        Self {
            value,
            type_ann: TypeAnnKind::TypeAnnChar,
            span,
        }
    }
}

impl Spanned for CharPrimitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug, Clone)]
pub struct StrPrimitive {
    pub value: String,
    pub type_ann: TypeAnnKind,
    span: Span,
}

impl StrPrimitive {
    pub fn new(value: String, span: Span) -> Self {
        Self {
            value,
            type_ann: TypeAnnKind::TypeAnnStr,
            span,
        }
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
    pub type_ann: TypeAnnKind,
    span: Span,
}

impl BoolPrimitive {
    pub fn new(value: bool, span: Span) -> Self {
        Self {
            value,
            type_ann: TypeAnnKind::TypeAnnBool,
            span,
        }
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
    pub type_ann: TypeAnnKind,
    span: Span,
}

impl I32Primitive {
    pub fn new(value: i32, span: Span) -> Self {
        Self {
            value,
            type_ann: TypeAnnKind::TypeAnnI32,
            span,
        }
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
    pub type_ann: TypeAnnKind,
    span: Span,
}

impl I64Primitive {
    pub fn new(value: i64, span: Span) -> Self {
        Self {
            value,
            type_ann: TypeAnnKind::TypeAnnI64,
            span,
        }
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
    pub type_ann: TypeAnnKind,
    span: Span,
}

impl U8Primitive {
    pub fn new(value: u8, span: Span) -> Self {
        Self {
            value,
            type_ann: TypeAnnKind::TypeAnnU8,
            span,
        }
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
    pub type_ann: TypeAnnKind,
    span: Span,
}

impl U16Primitive {
    pub fn new(value: u16, span: Span) -> Self {
        Self {
            value,
            type_ann: TypeAnnKind::TypeAnnU16,
            span,
        }
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
    pub type_ann: TypeAnnKind,
    span: Span,
}

impl U32Primitive {
    pub fn new(value: u32, span: Span) -> Self {
        Self {
            value,
            type_ann: TypeAnnKind::TypeAnnU32,
            span,
        }
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
    pub type_ann: TypeAnnKind,
    span: Span,
}

impl U64Primitive {
    pub fn new(value: u64, span: Span) -> Self {
        Self {
            value,
            type_ann: TypeAnnKind::TypeAnnU64,
            span,
        }
    }
}

impl Spanned for U64Primitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl TryFrom<Literal<UIntType>> for U64Primitive {
    type Error = TypeErrorKind;

    fn try_from(value: Literal<UIntType>) -> Result<Self, Self::Error> {
        let uint = match value.clone().into_inner() {
            Some(v) => match v {
                UIntType::U64(ui) => ui,
                _ => return Err(TypeErrorKind::MismatchedTypes),
            },
            None => return Err(TypeErrorKind::ValueNotFound),
        };

        Ok(Self {
            value: uint,
            type_ann: TypeAnnKind::TypeAnnU64,
            span: value.span(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct U256Primitive {
    pub value: U256,
    pub type_ann: TypeAnnKind,
    span: Span,
}

impl U256Primitive {
    pub fn new(value: U256, span: Span) -> Self {
        Self {
            value,
            type_ann: TypeAnnKind::TypeAnnU256,
            span,
        }
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
    pub type_ann: TypeAnnKind,
    span: Span,
}

impl F32Primitive {
    pub fn new(value: f32, span: Span) -> Self {
        Self {
            value,
            type_ann: TypeAnnKind::TypeAnnF32,
            span,
        }
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
    pub type_ann: TypeAnnKind,
    span: Span,
}

impl F64Primitive {
    pub fn new(value: f64, span: Span) -> Self {
        Self {
            value,
            type_ann: TypeAnnKind::TypeAnnF64,
            span,
        }
    }
}

impl Spanned for F64Primitive {
    fn span(&self) -> Span {
        self.span.clone()
    }
}
