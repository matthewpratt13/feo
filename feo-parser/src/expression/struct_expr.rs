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

// TODO: Collect errors in a list rather than stopping at the first error.
// TODO: This allows you to report all encountered errors in a single run,
// TODO: giving the user a comprehensive view of what needs to be fixed
// TODO: You might use a global or passed-through error list for this purpose.

impl Parse for StructExprField {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        let struct_expr_field = if let Some(first_attr) = OuterAttr::parse(parser)? {
            attributes.push(first_attr);

            while let Some(next_attr) = OuterAttr::parse(parser)? {
                attributes.push(next_attr);
                parser.next_token();
            }

            if let Some(id) = parser.peek_current::<Identifier>() {
                parser.next_token();

                let colon_opt = parser.peek_current::<Punctuation>();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Colon,
                    ..
                }) = colon_opt
                {
                    parser.next_token();

                    if let Some(r) = Returnable::parse(parser)? {
                        let field_content = (id, colon_opt.unwrap(), Box::new(r));

                        StructExprField(attributes, field_content)
                    } else {
                        return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`Returnable`".to_string(),
                            found: parser
                                .current_token()
                                .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                                .to_string(),
                        }));
                    }
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "colon punctuation (`:`)".to_string(),
                        found: parser
                            .current_token()
                            .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                            .to_string(),
                    }));
                }
            } else {
                return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: parser
                        .current_token()
                        .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                        .to_string(),
                }));
            }
        } else {
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`OuterAttr`".to_string(),
                found: parser
                    .current_token()
                    .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            }));
        };

        Ok(Some(struct_expr_field))
    }
}

impl Parse for StructExprFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let mut subsequent_fields: Vec<(Comma, StructExprField)> = Vec::new();

        let struct_expr_fields = if let Some(first_field) = StructExprField::parse(parser)? {
            let mut next_comma_opt = parser.peek_current::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = next_comma_opt
            {
                parser.next_token();

                if let Some(next_field) = StructExprField::parse(parser)? {
                    subsequent_fields.push((next_comma_opt.unwrap(), next_field));

                    if let Some(p) = parser.peek_next::<Punctuation>() {
                        next_comma_opt = Some(p);
                        parser.next_token();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            parser.next_token();

            StructExprFields {
                first_field,
                subsequent_fields,
            }
        } else {
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`StructExprField`".to_string(),
                found: parser
                    .current_token()
                    .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            }));
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
            let open_brace_opt = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = open_brace_opt
            {
                parser.next_token();

                if let Some(struct_expr_fields) = StructExprFields::parse(parser)? {
                    let close_brace_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        parser.next_token();

                        StructExpr {
                            item_path,
                            open_brace: open_brace_opt.unwrap(),
                            struct_expr_fields_opt: Some(struct_expr_fields),
                            close_brace: close_brace_opt.unwrap(),
                        }
                    } else {
                        return Err(parser.log_error(ParserErrorKind::MissingDelimiter {
                            delim: "}".to_string(),
                        }));
                    }
                } else {
                    return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`StructExprFields`".to_string(),
                        found: parser
                            .current_token()
                            .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                            .to_string(),
                    }));
                }
            } else {
                return Err(parser.log_error(ParserErrorKind::MissingDelimiter {
                    delim: "{".to_string(),
                }));
            }
        } else {
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`PathExpr`".to_string(),
                found: parser
                    .current_token()
                    .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            }));
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
        let unit_struct_expr = if let Some(path) = PathInExpr::parse(parser)? {
            UnitStructExpr(path)
        } else {
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`PathExpr`".to_string(),
                found: parser
                    .current_token()
                    .ok_or_else(|| parser.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            }));
        };

        Ok(Some(unit_struct_expr))
    }
}
