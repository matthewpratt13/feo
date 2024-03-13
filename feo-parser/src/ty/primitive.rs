use feo_error::error::CompilerError;
use feo_types::{
    literal::{FloatType, IntType, LiteralKind, UIntType},
    span::Spanned,
    BoolPrimitive, CharPrimitive, F32Primitive, F64Primitive, I32Primitive, I64Primitive,
    StrPrimitive, U16Primitive, U32Primitive, U64Primitive, U8Primitive,
};

use crate::{parse::ParseType, parser::Parser};

impl ParseType for CharPrimitive {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            match &l {
                LiteralKind::Char(c) => Ok(Some(CharPrimitive::new(
                    c.clone().into_inner().unwrap(),
                    l.span(),
                ))),

                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseType for StrPrimitive {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            match &l {
                LiteralKind::String(s) => Ok(Some(StrPrimitive::new(
                    s.clone().into_inner().unwrap(),
                    l.span(),
                ))),

                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseType for BoolPrimitive {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            match &l {
                LiteralKind::Bool(b) => Ok(Some(BoolPrimitive::new(
                    b.clone().into_inner().unwrap(),
                    l.span(),
                ))),

                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseType for I32Primitive {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            match &l {
                LiteralKind::Int(it) => match it.clone().into_inner().unwrap() {
                    IntType::I32(i) => Ok(Some(I32Primitive::new(i, l.span()))),
                    _ => return Ok(None),
                },
                _ => return Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseType for I64Primitive {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            match &l {
                LiteralKind::Int(it) => match it.clone().into_inner().unwrap() {
                    IntType::I64(i) => Ok(Some(I64Primitive::new(i, l.span()))),
                    _ => return Ok(None),
                },
                _ => return Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseType for U8Primitive {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            match &l {
                LiteralKind::UInt(ut) => match ut.clone().into_inner().unwrap() {
                    UIntType::U8(ui) => Ok(Some(U8Primitive::new(ui, l.span()))),
                    _ => return Ok(None),
                },
                _ => return Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseType for U16Primitive {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            match &l {
                LiteralKind::UInt(ut) => match ut.clone().into_inner().unwrap() {
                    UIntType::U16(ui) => Ok(Some(U16Primitive::new(ui, l.span()))),
                    _ => return Ok(None),
                },
                _ => return Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseType for U32Primitive {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            match &l {
                LiteralKind::UInt(ut) => match ut.clone().into_inner().unwrap() {
                    UIntType::U32(ui) => Ok(Some(U32Primitive::new(ui, l.span()))),
                    _ => return Ok(None),
                },
                _ => return Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseType for U64Primitive {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            match &l {
                LiteralKind::UInt(ut) => match ut.clone().into_inner().unwrap() {
                    UIntType::U64(ui) => Ok(Some(U64Primitive::new(ui, l.span()))),
                    _ => return Ok(None),
                },
                _ => return Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseType for F32Primitive {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            match &l {
                LiteralKind::Float(ut) => match ut.clone().into_inner().unwrap() {
                    FloatType::F32(ui) => Ok(Some(F32Primitive::new(ui, l.span()))),
                    _ => return Ok(None),
                },
                _ => return Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseType for F64Primitive {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            match &l {
                LiteralKind::Float(ut) => match ut.clone().into_inner().unwrap() {
                    FloatType::F64(ui) => Ok(Some(F64Primitive::new(ui, l.span()))),
                    _ => return Ok(None),
                },
                _ => return Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
