use feo_ast::{
    attribute::OuterAttr,
    expression::{StructExpr, StructExprField, StructExprFields},
};

use feo_error::{handler::ErrorEmitted, parser_error::ParserErrorKind};

use feo_types::{punctuation::PuncKind, utils::Comma, Punctuation};

use crate::{parse::Parse, parser::Parser};

impl Parse for StructExprField {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        let mut curr_attr_opt = OuterAttr::parse(parser)?;

        parser.advance();

        let struct_expr_field = if let Some(first_attr) = curr_attr_opt {
            parser.advance();
            attributes.push(first_attr);
            curr_attr_opt = OuterAttr::parse(parser)?;

            while let Some(next_attr) = curr_attr_opt {
                attributes.push(next_attr);
                parser.advance();
                curr_attr_opt = OuterAttr::parse(parser)?;
            }
        };

        Ok(Some(struct_expr_field))
    }
}

impl Parse for StructExprFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        // prepare an empty vector to store fields
        let mut subsequent_fields: Vec<(Comma, StructExprField)> = Vec::new();

        let struct_expr_fields = if let Some(first_field) = StructExprField::parse(parser)? {
            // if the first `Token` is some `StructExprField`, advance the `Parser`
            parser.advance();

            // create a var to store the current `Punctuation`
            let mut next_comma_opt = parser.peek::<Punctuation>()?;

            parser.advance();

            // iterate while the current `Punctuation` has `PuncKind::Comma`
            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = next_comma_opt
            {
                // expect a `StructExprField` (which should be the next `Token`)
                if let Some(next_field) = StructExprField::parse(parser)? {
                    // if it is a `StructExprField`, advance the `Parser`
                    parser.advance();

                    // push the current `Punctuation` and the next `StructExprField`
                    subsequent_fields.push((next_comma_opt.unwrap(), next_field));
                } else {
                    // in this case, the next `Token` is either `Some(_)` or `None`
                    // i.e., not some `StructExprField`
                    // however, we checked that it is not `None` inside `Peeker::peek_punctuation()`
                    // therefore it has to be some other `Token`
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                }

                // peek for a `Punctuation`
                // if one exists, set it to `next_comma_opt` and advance the `Parser`,
                // else break
                if let Some(p) = parser.take::<Punctuation>()? {
                    next_comma_opt = Some(p);
                } else {
                    break;
                }
            }

            // consume the final token
            parser.advance();

            // assign `StructExprFields`
            StructExprFields {
                first_field,
                subsequent_fields,
            }
        } else {
            // in this case, the next `Token` is either `Some(_)` or `None`
            // i.e., not some `StructExprField`
            // however, we checked that it is not `None` inside `Peeker::peek_punctuation()`
            // therefore it has to be some other `Token`
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
        };

        Ok(Some(struct_expr_fields))
    }
}

impl Parse for StructExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}
