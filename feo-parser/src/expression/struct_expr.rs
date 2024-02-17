use feo_ast::{
    attribute::OuterAttr,
    expression::{
        Returnable, StructExpr, StructExprField, StructExprFields, TupleStructExpr, UnitStructExpr,
    },
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
        // prepare an empty vector to store attributes
        let mut attributes: Vec<OuterAttr> = Vec::new();

        let struct_expr_field = if let Some(first_attr) = OuterAttr::parse(parser)? {
            // push the first attribute to the vector
            attributes.push(first_attr);

            parser.advance(); // advance the parser

            // push `OuterAttr` to `attributes` as long as there are some,
            // else break
            while let Ok(next_attr) = OuterAttr::parse(parser) {
                if let Some(a) = next_attr {
                    attributes.push(a);
                    parser.advance();
                } else {
                    break;
                }
            }

            // advance the `Parser` once there are no more `OuterAttr`
            parser.advance();

            // peek the next `Token`, expecting an `Identifier`
            if let Some(id) = parser.peek::<Identifier>()? {
                parser.advance();

                // peek the next `Token`, expecting a `Punctuation`
                let colon_opt = parser.peek::<Punctuation>()?;

                // check to see if `colon_opt` has `PuncKind::Colon`
                if let Some(Punctuation {
                    punc_kind: PuncKind::Colon,
                    ..
                }) = colon_opt
                {
                    // if so, advance the `Parser`
                    parser.advance();

                    // parse the next `Token`, continue if it is `Some`
                    if let Some(r) = Returnable::parse(parser)? {
                        parser.advance();

                        // collect the elements for the `StructExprField` into a tuple
                        let field_content = (id, colon_opt.unwrap(), Box::new(r));

                        // assign the `StructExprField`
                        StructExprField(attributes, field_content)
                    } else {
                        // in this case, the next `Expression` is either `Some(_)` or `None`
                        // i.e., not some `Returnable`
                        // however, we checked that it is not `None` inside the `parse()` function
                        // therefore it has to be some other `Expression` (or `Token`)
                        return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                    }
                } else {
                    // in this case, the next `Token` is either `Some(_)` or `None`
                    // i.e., not some `Punctuation`
                    // however, we checked that it is not `None` inside `Peeker::peek_punctuation()`
                    // therefore it has to be some other `Token`
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                }
            } else {
                // in this case, the next `Token` is either `Some(_)` or `None`
                // i.e., not some `Identifier`
                // however, we checked that it is not `None` inside `Peeker::peek_identifier()`
                // therefore it has to be some other `Token`
                return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
            }
        } else {
            // in this case, the next `Token` is either `Some(_)` or `None`
            // i.e., not some `OuterAttr`
            // however, we checked that it is not `None` inside the `parse()` function
            // therefore it has to be some other `Token`
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
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
                    // push the current `Punctuation` and the next `StructExprField`
                    subsequent_fields.push((next_comma_opt.unwrap(), next_field));

                    parser.advance();
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
        let struct_expr = if let Some(item_path) = PathInExpr::parse(parser)? {
            parser.advance();

            let open_brace_opt = parser.peek::<Delimiter>()?;

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = open_brace_opt
            {
                parser.advance();

                if let Some(s) = StructExprFields::parse(parser)? {
                    parser.advance();

                    let close_brace_opt = parser.peek::<Delimiter>()?;

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        parser.advance();

                        StructExpr {
                            item_path,
                            open_brace: open_brace_opt
                                .ok_or_else(|| parser.log_error(ParserErrorKind::Infallible))?,
                            struct_expr_fields_opt: Some(s),
                            close_brace: close_brace_opt
                                .ok_or_else(|| parser.log_error(ParserErrorKind::Infallible))?,
                        }
                    } else {
                        return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                    }
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
                }
            } else {
                return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
            }
        } else {
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
        };

        Ok(Some(struct_expr))
    }
}

impl Parse for TupleStructExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for UnitStructExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}
