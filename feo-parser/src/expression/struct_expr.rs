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
        let mut attributes: Vec<OuterAttr> = Vec::new();

        let struct_expr_field = if let Some(first_attr) = OuterAttr::parse(parser)? {
            attributes.push(first_attr);

            while let Ok(next_attr) = OuterAttr::parse(parser) {
                if let Some(a) = next_attr {
                    attributes.push(a);
                    parser.advance();
                } else {
                    break;
                }
            }

            if let Ok(id) = parser.peek_current::<Identifier>() {
                parser.advance();

                let colon_res = parser.peek_current::<Punctuation>();

                if let Ok(Punctuation {
                    punc_kind: PuncKind::Colon,
                    ..
                }) = colon_res
                {
                    parser.advance();

                    if let Some(r) = Returnable::parse(parser)? {
                        let field_content = (id, colon_res?, Box::new(r));
                        StructExprField(attributes, field_content)
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`Returnable`".to_string(),
                            found: "`unknown`".to_string(),
                        });
                        return Ok(None);
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "colon punctuation (`:`)".to_string(),
                        found: "`unknown`".to_string(),
                    });
                    return Ok(None);
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: "`unknown`".to_string(),
                });
                return Ok(None);
            }
        } else {
            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`OuterAttr`".to_string(),
                found: "`unknown`".to_string(),
            });
            return Ok(None);
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
            let mut next_comma_res = parser.peek_current::<Punctuation>();

            while let Ok(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = next_comma_res
            {
                parser.advance();

                if let Some(next_field) = StructExprField::parse(parser)? {
                    subsequent_fields.push((next_comma_res?, next_field));

                    if let Ok(p) = parser.peek_next::<Punctuation>() {
                        next_comma_res = Ok(p);
                        parser.advance();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            parser.advance();

            StructExprFields {
                first_field,
                subsequent_fields,
            }
        } else {
            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`StructExprField`".to_string(),
                found: "unknown".to_string(), // TODO
            });
            return Ok(None);
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
            let open_brace_res = parser.peek_current::<Delimiter>();

            if let Ok(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = open_brace_res
            {
                parser.advance();

                if let Some(struct_expr_fields) = StructExprFields::parse(parser)? {
                    let close_brace_res = parser.peek_current::<Delimiter>();

                    if let Ok(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_res
                    {
                        parser.advance();

                        StructExpr {
                            item_path,
                            open_brace: open_brace_res?,
                            struct_expr_fields_opt: Some(struct_expr_fields),
                            close_brace: close_brace_res?,
                        }
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "close brace delimiter (`}`)".to_string(),
                            found: "unknown".to_string(), // TODO
                        });
                        return Ok(None);
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`StructExprFields`".to_string(),
                        found: "unknown".to_string(), // TODO
                    });
                    return Ok(None);
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "open brace delimiter (`{`)".to_string(),
                    found: "unknown".to_string(), // TODO
                });
                return Ok(None);
            }
        } else {
            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`PathExpr`".to_string(),
                found: "unknown".to_string(), // TODO
            });
            return Ok(None);
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
            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`PathExpr`".to_string(),
                found: "unknown".to_string(), // TODO
            });
            return Ok(None);
        };

        Ok(Some(unit_struct_expr))
    }
}
