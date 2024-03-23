use feo_ast::{
    expression::{StructExpr, StructExprField, TupleStructExpr, Value},
    path::PathInExpr,
    token::Token,
};

use feo_error::{error::CompilerError, parser_error::ParserErrorKind};

use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    Delimiter, Identifier, Keyword, Punctuation,
};

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
    utils::{self, LogMsgType},
};

impl ParseTerm for StructExprField {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        if let Some(field_name) = parser.peek_current::<Identifier>() {
            parser.next_token();

            if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = parser.peek_current()
            {
                parser.next_token();

                if let Some(value) = Value::parse(parser)? {
                    let field_content = (field_name, Box::new(value));

                    return Ok(Some(StructExprField {
                        attributes_opt,
                        field_content,
                    }));
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "value".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`:`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for StructExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        utils::log_msg(LogMsgType::Expect, "path expression", parser);

        if let Some(Token::Keyword(Keyword {
            keyword_kind: KeywordKind::KwMatch,
            ..
        })) = parser.previous_token()
        {
            return Ok(None);
        }

        if let Some(path) = PathInExpr::parse(parser)? {
            utils::log_msg(LogMsgType::Enter, "struct expression", parser);

            parser.next_token();

            let open_brace_opt = parser.peek_current();

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = open_brace_opt
            {
                parser.next_token();

                let fields_opt = utils::get_term_collection::<StructExprField>(parser)?;

                let close_brace_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Close),
                    ..
                }) = close_brace_opt
                {
                    utils::log_msg(LogMsgType::Exit, "struct expression", parser);

                    return Ok(Some(StructExpr {
                        path,
                        open_brace: open_brace_opt.unwrap(),
                        fields_opt,
                        close_brace: close_brace_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`}`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`{`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for TupleStructExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(path) = PathInExpr::parse(parser)? {
            parser.next_token();

            let open_parenthesis_opt = parser.peek_current();

            if let Some(Delimiter {
                delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                ..
            }) = open_parenthesis_opt
            {
                parser.next_token();

                let fields_opt = utils::get_value_collection(parser)?;

                let close_parenthesis_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                    ..
                }) = close_parenthesis_opt
                {
                    parser.next_token();

                    return Ok(Some(TupleStructExpr {
                        path,
                        open_parenthesis: open_parenthesis_opt.unwrap(),
                        fields_opt,
                        close_parenthesis: close_parenthesis_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`)`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`(`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_struct_expr_field() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
            #[abstract]
            #[unsafe]
            foo: "a"
            "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let struct_expr_field =
            StructExprField::parse(&mut parser).expect("unable to parse struct expression field");

        Ok(println!("{:#?}", struct_expr_field))
    }

    #[test]
    fn parse_struct_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        SomeStruct {
            foo: "a",
            bar: 1,
            baz: x
        }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let struct_expr =
            StructExpr::parse(&mut parser).expect("unable to parse struct expression");

        Ok(println!("{:#?}", struct_expr))
    }

    #[test]
    fn parse_tuple_struct_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"SomeStruct(foo, "a", x)"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let tuple_struct_expr =
            TupleStructExpr::parse(&mut parser).expect("unable to parse tuple struct expression");

        Ok(println!("{:#?}", tuple_struct_expr))
    }
}
