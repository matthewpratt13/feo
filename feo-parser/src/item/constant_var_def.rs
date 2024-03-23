use feo_ast::{
    expression::Expression,
    item::{ConstantVarDef, StaticVarDef},
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Identifier, Keyword, Punctuation};

use crate::{
    parse::{ParseExpr, ParseItem, ParseType},
    parser::Parser,
    utils::{self, LogMsgType},
};

impl ParseItem for ConstantVarDef {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = utils::get_visibility(parser)?;

        let kw_const_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwConst,
            ..
        }) = kw_const_opt
        {
            utils::log_msg(
                LogMsgType::Enter,
                "constant variable definition (item)",
                parser,
            );

            parser.next_token();

            if let Some(item_name) = parser.peek_current::<Identifier>() {
                parser.next_token();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Colon,
                    ..
                }) = parser.peek_current()
                {
                    parser.next_token();

                    if let Some(item_type) = Type::parse(parser)? {
                        parser.next_token();

                        let equals_opt = parser.peek_current();

                        if let Some(Punctuation {
                            punc_kind: PuncKind::Equals,
                            ..
                        }) = equals_opt
                        {
                            parser.next_token();

                            let assignment_opt = if let Some(e) = Expression::parse(parser)? {
                                parser.next_token();
                                Some(Box::new(e))
                            } else {
                                None
                            };

                            let semicolon_opt = parser.peek_current();

                            if let Some(Punctuation {
                                punc_kind: PuncKind::Semicolon,
                                ..
                            }) = semicolon_opt
                            {
                                utils::log_msg(
                                    LogMsgType::Exit,
                                    "constant variable definition",
                                    parser,
                                );

                                return Ok(Some(ConstantVarDef {
                                    attributes_opt,
                                    visibility_opt,
                                    kw_const: kw_const_opt.unwrap(),
                                    item_name,
                                    item_type: Box::new(item_type),
                                    assignment_opt,
                                    semicolon: semicolon_opt.unwrap(),
                                }));
                            }

                            parser.log_error(ParserErrorKind::UnexpectedToken {
                                expected: "`;`".to_string(),
                                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                            });
                        } else {
                            parser.log_error(ParserErrorKind::UnexpectedToken {
                                expected: "`=`".to_string(),
                                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                            });
                        }
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "type".to_string(),
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
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseItem for StaticVarDef {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = utils::get_visibility(parser)?;

        let kw_static_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwStatic,
            ..
        }) = kw_static_opt
        {
            utils::log_msg(LogMsgType::Enter, "static variable definition", parser);

            parser.next_token();

            let kw_mut_opt = parser.peek_current();

            if let Some(Keyword {
                keyword_kind: KeywordKind::KwMut,
                ..
            }) = kw_mut_opt
            {
                parser.next_token();

                if let Some(item_name) = parser.peek_current::<Identifier>() {
                    parser.next_token();

                    if let Some(Punctuation {
                        punc_kind: PuncKind::Colon,
                        ..
                    }) = parser.peek_current()
                    {
                        parser.next_token();

                        if let Some(item_type) = Type::parse(parser)? {
                            parser.next_token();

                            let equals_opt = parser.peek_current();

                            if let Some(Punctuation {
                                punc_kind: PuncKind::Equals,
                                ..
                            }) = equals_opt
                            {
                                parser.next_token();

                                let assignment_opt = if let Some(e) = Expression::parse(parser)? {
                                    parser.next_token();
                                    Some(Box::new(e))
                                } else {
                                    None
                                };

                                let semicolon_opt = parser.peek_current();

                                if let Some(Punctuation {
                                    punc_kind: PuncKind::Semicolon,
                                    ..
                                }) = semicolon_opt
                                {
                                    utils::log_msg(
                                        LogMsgType::Exit,
                                        "static variable definition",
                                        parser,
                                    );

                                    return Ok(Some(StaticVarDef {
                                        attributes_opt,
                                        visibility_opt,
                                        kw_static: kw_static_opt.unwrap(),
                                        kw_mut_opt,
                                        item_name,
                                        item_type,
                                        assignment_opt,
                                        semicolon: semicolon_opt.unwrap(),
                                    }));
                                } else {
                                    parser.log_error(ParserErrorKind::UnexpectedToken {
                                        expected: "`;`".to_string(),
                                        found: parser
                                            .current_token()
                                            .unwrap_or(Token::EOF)
                                            .to_string(),
                                    });
                                }
                            } else {
                                parser.log_error(ParserErrorKind::UnexpectedToken {
                                    expected: "`=`".to_string(),
                                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                                });
                            }
                        } else {
                            parser.log_error(ParserErrorKind::UnexpectedToken {
                                expected: "type".to_string(),
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
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "identifier".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
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
    fn parse_constant_var_def() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"const foo: u64 = 2;"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let constant_var_def = ConstantVarDef::parse(&mut parser)
            .expect("unable to parse constant variable definition");

        Ok(println!("{:#?}", constant_var_def))
    }

    #[test]
    fn parse_static_var_def() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"pub static mut foo: u64 = 2;"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let static_var_def =
            StaticVarDef::parse(&mut parser).expect("unable to parse static variable definition");

        Ok(println!("{:#?}", static_var_def))
    }
}
