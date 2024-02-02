#![allow(dead_code)]

use feo_ast::expression::{Expr, Expression, OuterAttr, Struct, StructExprField, StructExprFields};
use feo_error::parser_error::ParserError;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    Delimiter, Identifier, Punctuation,
};

use crate::{parse::Parse, parser::Parser};

impl Parse for Struct {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        if let Ok(Delimiter {
            delim: (DelimKind::Brace, DelimOrientation::Open),
            ..
        }) = Delimiter::try_from(parser.next_token()?)
        {
            if let Ok(fields) = StructExprFields::parse(parser) {
                todo!()
            }
        } else {
            todo!()
        }
    }
}

impl Parse for StructExprFields {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for StructExprField {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();
        let _ = parser.next_token();

        if let Ok(attr) = OuterAttr::parse(parser) {
            while let Ok(attr) = OuterAttr::parse(parser) {
                attributes.push(attr);
                parser.next_token();
            }

            if let Ok(iden) = Identifier::try_from(parser.next_token()?) {
                if let Ok(colon) = Punctuation::try_from(parser.next_token()?) {
                    if let Ok(expr) = Expression::parse(parser) {
                        let field = StructExprField(attributes, (iden, colon, expr));

                        Ok(field)
                    }
                }
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}

impl Parse for Expression {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        todo!()
    }
}
