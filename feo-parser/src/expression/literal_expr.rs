use feo_ast::token::Token;
use feo_error::{handler::ErrorEmitted, parser_error::ParserErrorKind};
use feo_types::literal::{FloatType, IntType, LiteralKind, UIntType};

use crate::{parse::Parse, parser::Parser};

impl Parse for LiteralKind {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match parser.current_token() {
            Token::CharLit(c) => Ok(Some(LiteralKind::Char(c))),
            Token::StringLit(s) => Ok(Some(LiteralKind::String(s))),
            Token::BoolLit(b) => Ok(Some(LiteralKind::Bool(b))),
            Token::IntLit(i) => match i.clone().into_inner() {
                Some(t) => match t {
                    IntType::I32(_) => Ok(Some(LiteralKind::I32(i))),
                    IntType::I64(_) => Ok(Some(LiteralKind::I64(i))),
                },
                None => Err(parser.log_error(ParserErrorKind::InvalidToken)),
            },
            Token::UIntLit(ui) => match ui.clone().into_inner() {
                Some(t) => match t {
                    UIntType::U8(_) => Ok(Some(LiteralKind::U8(ui))),
                    UIntType::U16(_) => Ok(Some(LiteralKind::U16(ui))),
                    UIntType::U32(_) => Ok(Some(LiteralKind::U32(ui))),
                    UIntType::U64(_) => Ok(Some(LiteralKind::U64(ui))),
                },
                None => Err(parser.log_error(ParserErrorKind::InvalidToken)),
            },
            // Token::UIntLit(ui) => Ok(Some(LiteralKind::U64(ui))),
            Token::U256Lit(u) => Ok(Some(LiteralKind::U256(u))),
            Token::FloatLit(f) => match f.clone().into_inner() {
                Some(t) => match t {
                    FloatType::F32(_) => Ok(Some(LiteralKind::F32(f))),
                    FloatType::F64(_) => Ok(Some(LiteralKind::F64(f))),
                },
                None => Err(parser.log_error(ParserErrorKind::InvalidToken)),
            },
            _ => Err(parser.log_error(ParserErrorKind::UnexpectedToken)),
        }
    }
}
