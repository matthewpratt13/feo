#![allow(dead_code)]

use feo_ast::expression::{OuterAttr, Struct, StructExprField, StructExprFields};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    Delimiter,
};

use crate::{parse::Parse, parser::Parser};

impl Parse for Struct {
    fn parse(parser: &mut Parser) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(d) = Delimiter::try_from(parser.next_token()?).ok() {
            match d.delim {
                (DelimKind::Brace, DelimOrientation::Open) => match parser.next_token() {
                    Some(_) => todo!(),
                    None => todo!(),
                },
                _ => todo!(),
            }
        } else {
            todo!()
        }
    }
}

impl Parse for StructExprFields {
    fn parse(parser: &mut Parser) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for StructExprField {
    fn parse(parser: &mut Parser) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for OuterAttr {
    fn parse(parser: &mut Parser) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
