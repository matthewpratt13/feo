#![allow(dead_code)]

use feo_ast::{
    expression::{Expression, OuterAttr, Struct, StructExprField, StructExprFields},
    path::PathInExpr,
};
use feo_error::parser_error::ParserError;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    span::Span,
    Delimiter, Identifier, Punctuation,
};

use crate::{parse::Parse, parser::Parser};

impl Parse for Struct {
    fn parse(parser: &mut Parser) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        if let Ok(item_path) = PathInExpr::parse(parser) {
            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = Some(Delimiter::try_from(parser.next_token()?)?)
            {
                if let Some(fields_opt) = Some(StructExprFields::parse(parser)?) {
                    if let Ok(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = Delimiter::try_from(parser.next_token()?)
                    {
                        let expr = Struct {
                            item_path,
                            open_brace: Delimiter {
                                delim: (DelimKind::Brace, DelimOrientation::Open),
                                span: Span::default(),
                            },
                            struct_expr_fields_opt: Some(fields_opt),
                            close_brace: Delimiter {
                                delim: (DelimKind::Brace, DelimOrientation::Close),
                                span: Span::default(),
                            },
                        };

                        Ok(expr)
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            } else {
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
            attributes.push(attr);

            while let Ok(attr) = OuterAttr::parse(parser) {
                attributes.push(attr);
                let _ = parser.next_token();
            }

            if let Ok(iden) = Identifier::try_from(parser.next_token()?) {
                if let Ok(colon) = Punctuation::try_from(parser.next_token()?) {
                    if let Ok(expr) = Expression::parse(parser) {
                        let field = StructExprField(attributes, (iden, colon, Box::new(expr)));

                        Ok(field)
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
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
