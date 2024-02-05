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
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ParserError>
    where
        Self: Sized,
    {
        if let Some(item_path) = PathInExpr::parse(parser)? {
            if let Ok(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = Delimiter::try_from(parser.current_token())
            {
                parser.advance();

                if let Some(fields_opt) = StructExprFields::parse(parser)? {
                    parser.advance();

                    if let Ok(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = Delimiter::try_from(parser.current_token())
                    {
                        let expr = Struct {
                            item_path,
                            open_brace: Delimiter {
                                delim: (DelimKind::Brace, DelimOrientation::Open),
                                span: Span::default(), // TODO
                            },
                            struct_expr_fields_opt: Some(fields_opt),
                            close_brace: Delimiter {
                                delim: (DelimKind::Brace, DelimOrientation::Close),
                                span: Span::default(), // TODO
                            },
                        };

                        parser.advance();

                        Ok(Some(expr))
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
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ParserError>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for StructExprField {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ParserError>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        if let Some(attr) = OuterAttr::parse(parser)? {
            attributes.push(attr);

            while let Some(attr) = OuterAttr::parse(parser)? {
                attributes.push(attr);
                parser.advance();
            }

            if let Ok(iden) = Identifier::try_from(parser.current_token()) {
                parser.advance();

                if let Ok(colon) = Punctuation::try_from(parser.current_token()) {
                    parser.advance();

                    if let Some(expr) = Expression::parse(parser)? {
                        parser.advance();

                        let field = StructExprField(attributes, (iden, colon, Box::new(expr)));

                        Ok(Some(field))
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
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ParserError>
    where
        Self: Sized,
    {
        todo!()
    }
}