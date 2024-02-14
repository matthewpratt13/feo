use feo_ast::{
    expression::{Expression, OuterAttr, StructExpr, StructExprField, StructExprFields},
    path::PathInExpr,
};
use feo_error::{handler::ErrorEmitted, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
    utils::Comma,
    Delimiter, Identifier, Punctuation,
};

use crate::{parse::Parse, parser::Parser};

impl Parse for StructExprField {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        if let Some(attr) = parser.peek() {
            attributes.push(attr);
            parser.advance();

            while let Some(attr) = parser.peek() {
                attributes.push(attr);
                parser.advance();
            }

            if let Some(iden) = parser.peek() {
                parser.advance();

                let colon_opt = parser.take::<Punctuation>();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Colon,
                    ..
                }) = colon_opt
                {
                    parser.advance();

                    if let Some(expr) = parser.peek() {
                        parser.advance();

                        let field =
                            StructExprField(attributes, (iden, colon_opt.unwrap(), Box::new(expr)));

                        Ok(Some(field))
                    } else {
                        Err(parser.log_error(ParserErrorKind::TokenNotFound))
                    }
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                }
            } else {
                return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
            }
        } else {
            Err(parser.log_error(ParserErrorKind::TokenNotFound))
        }
    }
}

impl Parse for StructExprFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let mut subsequent_fields: Vec<(Comma, StructExprField)> = Vec::new();

        if let Some(first_field) = StructExprField::parse(parser)? {
            let mut comma_opt = parser.peek::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = comma_opt
            {
                if let Some(next_field) = StructExprField::parse(parser)? {
                    subsequent_fields.push((comma_opt.unwrap(), next_field));
                    comma_opt = parser.take::<Punctuation>();
                } else {
                    break;
                }
            }

            parser.advance();

            let fields = StructExprFields {
                first_field,
                subsequent_fields,
            };

            Ok(Some(fields))
        } else {
            Err(parser.log_error(ParserErrorKind::TokenNotFound))
        }
    }
}

impl Parse for StructExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        if let Some(item_path) = PathInExpr::parse(parser)? {
            let open_brace_opt = parser.peek::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = open_brace_opt
            {
                parser.advance();

                if let Some(fields_opt) = StructExprFields::parse(parser)? {
                    let close_brace_opt = parser.peek::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        // consume last token and move to next token in prep for next parser
                        parser.advance();

                        let expr = StructExpr {
                            item_path,
                            open_brace: open_brace_opt.unwrap(),
                            struct_expr_fields_opt: Some(fields_opt),
                            close_brace: close_brace_opt.unwrap(),
                        };

                        Ok(Some(expr))
                    } else {
                        return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                    }
                } else {
                    return Err(parser.log_error(ParserErrorKind::TokenNotFound));
                }
            } else {
                return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
            }
        } else {
            Err(parser.log_error(ParserErrorKind::TokenNotFound))
        }
    }
}
