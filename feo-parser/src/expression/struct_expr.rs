use feo_ast::{
    attribute::OuterAttr,
    expression::{
        Returnable, StructExpr, StructExprField, StructExprFields, TupleStructExpr, UnitStructExpr,
    },
    token::Token,
};

use feo_error::{error::CompilerError, parser_error::ParserErrorKind};

use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
    utils::Comma,
    Delimiter, Identifier, Punctuation,
};

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
};

impl ParseTerm for StructExprField {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        if let Some(Punctuation {
            punc_kind: PuncKind::HashSign,
            ..
        }) = parser.peek_current()
        {
            while let Some(next_attr) = OuterAttr::parse(parser)? {
                attributes.push(next_attr);

                parser.next_token();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Comma,
                    ..
                }) = parser.peek_current()
                {
                    parser.next_token();
                } else {
                    break;
                }
            }
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

                    if !attributes.is_empty() {
                        return Ok(Some(StructExprField(Some(attributes), field_content)));
                    }
                    return Ok(Some(StructExprField(None, field_content)));
                }
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`Returnable`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "colon punctuation (`:`)".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "identifier".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        }

        Err(parser.errors())
    }
}

impl ParseTerm for StructExprFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_fields: Vec<(Comma, StructExprField)> = Vec::new();

        if let Some(first_field) = StructExprField::parse(parser)? {
            parser.next_token();

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
                        parser.next_token();
                        next_comma_opt = Some(p);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            return Ok(Some(StructExprFields {
                first_field,
                subsequent_fields,
            }));
        }
        parser.log_error(ParserErrorKind::UnexpectedToken {
            expected: "`StructExprField`".to_string(),
            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
        });

        Err(parser.errors())
    }
}

impl ParseExpr for StructExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            parser.next_token();

            let open_brace_opt = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = open_brace_opt
            {
                parser.next_token();

                if let Some(struct_expr_fields) = StructExprFields::parse(parser)? {
                    parser.next_token();

                    let close_brace_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        parser.next_token();

                        return Ok(Some(StructExpr {
                            id,
                            open_brace: open_brace_opt.unwrap(),
                            struct_expr_fields_opt: Some(struct_expr_fields),
                            close_brace: close_brace_opt.unwrap(),
                        }));
                    }
                    parser.log_error(ParserErrorKind::MissingDelimiter {
                        delim: "}".to_string(),
                    });
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`StructExprFields`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::MissingDelimiter {
                    delim: "{".to_string(),
                });
            }
        } else {
            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`PathExpr`".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        };

        Err(parser.errors())
    }
}

impl ParseExpr for TupleStructExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for UnitStructExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            parser.next_token();
            return Ok(Some(UnitStructExpr(id)));
        }
        parser.log_error(ParserErrorKind::UnexpectedToken {
            expected: "`PathExpr`".to_string(),
            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
        });
        Err(parser.errors())
    }
}

#[cfg(test)]
mod tests {
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_struct() {
        let source_code = r#"
        SomeStruct {
            #[abstract]
            foo: "a",
            bar: 1,
            baz: x
        }"#;

        let handler = Handler::default();
        let mut lexer = Lexer::new(&source_code, handler.clone());
        let token_stream = lexer.lex().expect("unable to lex source code");

        println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let struct_expr =
            StructExpr::parse(&mut parser).expect("unable to parse struct expression");

        println!("{:#?}", struct_expr);
    }
}
