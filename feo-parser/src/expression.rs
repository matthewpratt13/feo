#![allow(dead_code)]

use feo_ast::expression::{Struct, StructExprField};
use feo_error::parser_error::ParserError;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    Delimiter,
};

use crate::{parse::Parse, parser::Parser};

impl Parse for Struct {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        if let Ok(d) = Delimiter::try_from(parser.next_token()?) {
            match d.delim {
                (DelimKind::Brace, DelimOrientation::Open) => match parser.peek_next() {
                    Some(s) => todo!(),
                    None => todo!(),
                },
                _ => todo!(),
            }
        } else {
            todo!()
        }
    }
}

impl Parse for StructExprField {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        todo!()
    }
}
