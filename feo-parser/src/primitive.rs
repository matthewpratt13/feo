use feo_error::error::CompilerError;
use feo_types::{
    literal::{FloatType, IntType, LiteralKind, UIntType},
    primitive::Primitive,
    U256,
};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for Primitive<char> {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            parser.next_token();

            match l {
                LiteralKind::Char(c) => Ok(Some(Primitive(c.into_inner().unwrap()))),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for Primitive<String> {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            parser.next_token();

            match l {
                LiteralKind::String(s) => Ok(Some(Primitive(s.into_inner().unwrap()))),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for Primitive<bool> {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            parser.next_token();

            match l {
                LiteralKind::Bool(b) => Ok(Some(Primitive(b.into_inner().unwrap()))),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for Primitive<i32> {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            parser.next_token();

            match l {
                LiteralKind::Int(i) => match i.into_inner().unwrap() {
                    IntType::I32(s) => Ok(Some(Primitive(s))),
                    _ => Ok(None),
                },
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for Primitive<i64> {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            parser.next_token();

            match l {
                LiteralKind::Int(i) => match i.into_inner().unwrap() {
                    IntType::I64(s) => Ok(Some(Primitive(s))),
                    _ => Ok(None),
                },
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for Primitive<u8> {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            parser.next_token();

            match l {
                LiteralKind::UInt(ui) => match ui.into_inner().unwrap() {
                    UIntType::U8(s) => Ok(Some(Primitive(s))),
                    _ => Ok(None),
                },
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for Primitive<u16> {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            parser.next_token();

            match l {
                LiteralKind::UInt(ui) => match ui.into_inner().unwrap() {
                    UIntType::U16(s) => Ok(Some(Primitive(s))),
                    _ => Ok(None),
                },
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for Primitive<u32> {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            parser.next_token();

            match l {
                LiteralKind::UInt(ui) => match ui.into_inner().unwrap() {
                    UIntType::U32(s) => Ok(Some(Primitive(s))),
                    _ => Ok(None),
                },
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for Primitive<u64> {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            parser.next_token();

            match l {
                LiteralKind::UInt(ui) => match ui.into_inner().unwrap() {
                    UIntType::U64(s) => Ok(Some(Primitive(s))),
                    _ => Ok(None),
                },
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for Primitive<U256> {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            parser.next_token();

            match l {
                LiteralKind::U256(u) => Ok(Some(Primitive(u.into_inner().unwrap()))),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for Primitive<f32> {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            parser.next_token();

            match l {
                LiteralKind::Float(f) => match f.into_inner().unwrap() {
                    FloatType::F32(s) => Ok(Some(Primitive(s))),
                    _ => Ok(None),
                },
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for Primitive<f64> {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(l) = parser.peek_current::<LiteralKind>() {
            parser.next_token();

            match l {
                LiteralKind::Float(f) => match f.into_inner().unwrap() {
                    FloatType::F64(s) => Ok(Some(Primitive(s))),
                    _ => Ok(None),
                },
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
