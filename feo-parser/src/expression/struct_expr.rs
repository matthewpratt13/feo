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
            parser.advance();

            while let Ok(next_attr) = OuterAttr::parse(parser) {
                if let Some(a) = next_attr {
                    attributes.push(a);
                    parser.advance();
                } else {
                    break;
                }
            }

            parser.advance();

            if let Some(id) = parser.peek::<Identifier>() {
                parser.advance();

                let colon_opt = parser.peek::<Punctuation>();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Colon,
                    ..
                }) = colon_opt
                {
                    parser.advance();

                    if let Some(r) = Returnable::parse(parser)? {
                        parser.advance();

                        let field_content = (
                            id,
                            colon_opt.ok_or_else(|| {
                                parser.log_error(ParserErrorKind::UnexpectedToken)
                            })?,
                            Box::new(r),
                        );

                        StructExprField(attributes, field_content)
                    } else {
                        return Ok(None);
                    }
                } else {
                    return Ok(None);
                }
            } else {
                return Ok(None);
            }
        } else {
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
            let mut next_comma_opt = parser.peek::<Punctuation>();

            parser.advance();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = next_comma_opt
            {
                if let Some(next_field) = StructExprField::parse(parser)? {
                    subsequent_fields.push((
                        next_comma_opt
                            .ok_or_else(|| parser.log_error(ParserErrorKind::UnexpectedToken))?,
                        next_field,
                    ));

                    parser.advance();
                } else {
                    break;
                }

                if let Some(p) = parser.take::<Punctuation>() {
                    next_comma_opt = Some(p);
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
            parser.advance();

            let open_brace_opt = parser.peek::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = open_brace_opt
            {
                parser.advance();

                if let Some(struct_expr_fields_opt) = StructExprFields::parse(parser)? {
                    let close_brace_opt = parser.peek::<Delimiter>();
                    
                    parser.advance();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        parser.advance();

                        StructExpr {
                            item_path,

                            open_brace: open_brace_opt.ok_or_else(|| {
                                parser.log_error(ParserErrorKind::UnexpectedToken)
                            })?,

                            struct_expr_fields_opt: Some(struct_expr_fields_opt),

                            close_brace: close_brace_opt.ok_or_else(|| {
                                parser.log_error(ParserErrorKind::UnexpectedToken)
                            })?,
                        }
                    } else {
                        return Ok(None);
                    }
                } else {
                    return Ok(None);
                }
            } else {
                return Ok(None);
            }
        } else {
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
            return Ok(None);
        };

        Ok(Some(unit_struct_expr))
    }
}
