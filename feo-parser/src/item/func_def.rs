use feo_ast::{
    expression::ExprWithBlock,
    item::{FuncOrMethodParam, FuncParam, FuncSig, FuncWithBlock, SelfParam},
    pattern::Pattern,
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    Delimiter, Identifier, Keyword, Punctuation,
};

use crate::{
    parse::{ParseExpr, ParseItem, ParsePatt, ParseTerm, ParseType},
    parser::Parser,
    test_utils::{self, LogMsgType},
    utils,
};

impl ParseTerm for FuncOrMethodParam {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(sp) = SelfParam::parse(parser)? {
            Ok(Some(FuncOrMethodParam::MethodParam(sp)))
        } else if let Some(fp) = FuncParam::parse(parser)? {
            Ok(Some(FuncOrMethodParam::FuncParam(fp)))
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for SelfParam {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let ampersand_opt = parser.peek_current();

        if let Some(Punctuation {
            punc_kind: PuncKind::Ampersand,
            ..
        }) = ampersand_opt
        {
            parser.next_token();
        }

        let kw_mut_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwMut,
            ..
        }) = kw_mut_opt
        {
            parser.next_token();
        }

        let kw_self_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwSelf,
            ..
        }) = kw_self_opt
        {
            test_utils::log_msg(LogMsgType::Detect, "`self` parameter", parser);

            let type_ann_opt = if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = parser.peek_next()
            {
                parser.next_token();
                parser.next_token();

                if let Some(ty) = Type::parse(parser)? {
                    Some(Box::new(ty))
                } else {
                    None
                }
            } else {
                None
            };

            return Ok(Some(SelfParam {
                ampersand_opt,
                kw_mut_opt,
                kw_self: kw_self_opt.unwrap(),
                type_ann_opt,
            }));
        } else {
            return Ok(None);
        }
    }
}

impl ParseTerm for FuncParam {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(param_pattern) = Pattern::parse(parser)? {
            test_utils::log_msg(LogMsgType::Detect, "function parameter", parser);

            let colon_opt = parser.peek_next();

            if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = colon_opt
            {
                parser.next_token();
                parser.next_token();

                test_utils::log_msg(LogMsgType::Detect, "function parameter type", parser);

                if let Some(param_type) = Type::parse(parser)? {
                    return Ok(Some(FuncParam {
                        param_pattern: Box::new(param_pattern),
                        param_type: Box::new(param_type),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "type".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseItem for FuncSig {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = utils::get_visibility(parser)?;

        let kw_func_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwFunc,
            ..
        }) = kw_func_opt
        {
            test_utils::log_msg(LogMsgType::Detect, "`func` keyword", parser);

            if let Some(func_name) = parser.peek_next::<Identifier>() {
                parser.next_token();

                test_utils::log_msg(LogMsgType::Detect, "function name", parser);

                let open_parenthesis_opt = parser.peek_next();

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) = open_parenthesis_opt
                {
                    parser.next_token();
                    parser.next_token();

                    let func_params_opt = utils::get_term_collection::<FuncOrMethodParam>(parser)?;

                    let close_parenthesis_opt = parser.peek_current();

                    if let Some(Delimiter {
                        delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                        ..
                    }) = close_parenthesis_opt
                    {
                        test_utils::log_msg(LogMsgType::Detect, "`)`", parser);

                        let return_type_opt = if let Some(Punctuation {
                            punc_kind: PuncKind::ThinArrow,
                            ..
                        }) = parser.peek_next()
                        {
                            parser.next_token();
                            parser.next_token();

                            test_utils::log_msg(LogMsgType::Detect, "return type", parser);

                            if let Some(ty) = Type::parse(parser)? {
                                Some(Box::new(ty))
                            } else {
                                test_utils::log_msg(LogMsgType::Expect, "`->`", parser);
                                None
                            }
                        } else {
                            None
                        };

                        if let Some(Punctuation {
                            punc_kind: PuncKind::Semicolon,
                            ..
                        }) = parser.peek_next()
                        {
                            test_utils::log_msg(LogMsgType::Detect, "`;`", parser);

                            parser.next_token();
                        }

                        test_utils::log_msg(LogMsgType::Exit, "function signature", parser);

                        return Ok(Some(FuncSig {
                            attributes_opt,
                            visibility_opt,
                            kw_func: kw_func_opt.unwrap(),
                            func_name,
                            open_parenthesis: open_parenthesis_opt.unwrap(),
                            func_params_opt,
                            close_parenthesis: close_parenthesis_opt.unwrap(),
                            return_type_opt,
                        }));
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`)`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`(`".to_string(),
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

impl ParseItem for FuncWithBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(function_sig) = FuncSig::parse(parser)? {
            parser.next_token();

            test_utils::log_msg(LogMsgType::Expect, "function block", parser);

            if let Some(function_body) = ExprWithBlock::parse(parser)? {
                test_utils::log_msg(LogMsgType::Exit, "function block", parser);

                return Ok(Some(FuncWithBlock {
                    function_sig,
                    function_body,
                }));
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "expression with block".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }

            return Ok(None);
        } else {
            return Ok(None);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_self_param() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"&mut self: u64"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let self_param = SelfParam::parse(&mut parser).expect("unable to parse `self` parameter");

        Ok(println!("{:#?}", self_param))
    }

    #[test]
    fn parse_function_param() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"foo: u64"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let function_param =
            FuncParam::parse(&mut parser).expect("unable to parse function parameter");

        Ok(println!("{:#?}", function_param))
    }

    #[test]
    fn parse_function_sig() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        pub func foo(bar: bool, baz: char) -> u64
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let function_sig = FuncSig::parse(&mut parser).expect("unable to parse function signature");

        Ok(println!("{:#?}", function_sig))
    }

    #[test]
    fn parse_function_with_block() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        pub func foo(bar: bool, baz: char) -> u64 {
            if (x > 2) {
                return 12
            }
        }
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let function_with_block =
            FuncWithBlock::parse(&mut parser).expect("unable to parse function with block");

        Ok(println!("{:#?}", function_with_block))
    }
}
