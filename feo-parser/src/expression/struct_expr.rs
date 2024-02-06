use feo_ast::{
    expression::{Expression, OuterAttr, Struct, StructExprField, StructExprFields},
    path::PathInExpr,
};
use feo_error::handler::ErrorEmitted;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
    Delimiter, Identifier, Punctuation,
};

use crate::{parse::Parse, parser::Parser};

impl Parse for StructExprFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

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

            if let Ok(iden) = Identifier::try_from(parser.current_token()) {
                parser.advance();

                let colon_res = Punctuation::try_from(parser.current_token());

                if let Ok(Punctuation {
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

impl Parse for Struct {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        if let Some(item_path) = PathInExpr::parse(parser)? {
            let open_brace_res = Delimiter::try_from(parser.current_token());

            if let Ok(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = open_brace_res
            {
                parser.advance();

                if let Some(fields_opt) = StructExprFields::parse(parser)? {
                    parser.advance();

                    let close_brace_res = Delimiter::try_from(parser.current_token());

                    if let Ok(Delimiter {
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
