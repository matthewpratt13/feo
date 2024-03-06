use feo_error::error::CompilerError;
use feo_types::{literal::LiteralKind, primitive::Primitive};

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
