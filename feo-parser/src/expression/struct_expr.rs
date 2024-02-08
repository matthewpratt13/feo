use feo_ast::{
    expression::{Expression, OuterAttr, Struct, StructExprField, StructExprFields},
    path::PathInExpr,
};
use feo_error::handler::ErrorEmitted;
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

        if let Some(attr) = OuterAttr::parse(parser)? {
            attributes.push(attr);

            while let Some(attr) = OuterAttr::parse(parser)? {
                attributes.push(attr);
                parser.advance();
            }

            if let Some(iden) = parser.peek::<Identifier>()? {
                parser.advance();

                let colon_res = parser.take::<Punctuation>()?;

                if let Some(Punctuation {
                    punc_kind: PuncKind::Colon,
                    ..
                }) = colon_res
                {
                    parser.advance();

                    if let Some(expr) = Expression::parse(parser)? {
                        let field =
                            StructExprField(attributes, (iden, colon_res.unwrap(), Box::new(expr)));

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

impl Parse for StructExprFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let mut subsequent_fields: Vec<(Comma, StructExprField)> = Vec::new();

        if let Some(first_field) = StructExprField::parse(parser)? {
            let mut comma_res = parser.take::<Punctuation>()?;

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = comma_res
            {
                if let Some(next_field) = StructExprField::parse(parser)? {
                    subsequent_fields.push((comma_res.unwrap(), next_field));
                    comma_res = parser.peek::<Punctuation>()?;
                    parser.advance();
                } else {
                    parser.advance();
                    todo!() // log error (ignore output, i.e., do NOT return)

                    // break
                }
            }

            parser.advance();

            let fields = StructExprFields {
                first_field,
                subsequent_fields,
            };

            Ok(Some(fields))
        } else {
            todo!()
        }
    }
}

impl Parse for Struct {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        if let Some(item_path) = PathInExpr::parse(parser)? {
            let open_brace_res = parser.peek::<Delimiter>()?;

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = open_brace_res
            {
                parser.advance();

                if let Some(fields_opt) = StructExprFields::parse(parser)? {
                    let close_brace_res = parser.peek::<Delimiter>()?;

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_res
                    {
                        // consume last token and move to next token in prep for next parser
                        parser.advance();

                        let expr = Struct {
                            item_path,
                            open_brace: open_brace_res.unwrap(),
                            struct_expr_fields_opt: Some(fields_opt),
                            close_brace: close_brace_res.unwrap(),
                        };

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
