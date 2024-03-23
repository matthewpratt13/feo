use feo_ast::{
    expression::ExprWithBlock,
    item::{FuncOrMethodParam, FunctionParam, FunctionSig, FunctionWithBlock, SelfParam},
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
    utils::{self, LogMsgType},
};

impl ParseTerm for FuncOrMethodParam {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(sp) = SelfParam::parse(parser)? {
            Ok(Some(FuncOrMethodParam::MethodParam(sp)))
        } else if let Some(fp) = FunctionParam::parse(parser)? {
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
            parser.next_token();

            let type_annotation_opt = if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = parser.peek_current()
            {
                parser.next_token();

                if let Some(ty) = Type::parse(parser)? {
                    parser.next_token();
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
                type_annotation_opt,
            }));
        } else {
            return Ok(None);
        }
    }
}

impl ParseTerm for FunctionParam {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(param_pattern) = Pattern::parse(parser)? {
            parser.next_token();

            let colon_opt = parser.peek_current();

            if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = colon_opt
            {
                parser.next_token();

                if let Some(param_type) = Type::parse(parser)? {
                    // parser.next_token();

                    return Ok(Some(FunctionParam {
                        param_pattern: Box::new(param_pattern),
                        param_type: Box::new(param_type),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "type".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
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

impl ParseItem for FunctionSig {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        utils::log_msg(LogMsgType::Enter, "function signature", parser);

        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = utils::get_visibility(parser)?;

        let kw_func_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwFunc,
            ..
        }) = kw_func_opt
        {
            parser.next_token();

            if let Some(function_name) = parser.peek_current::<Identifier>() {
                parser.next_token();

                let open_parenthesis_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) = open_parenthesis_opt
                {
                    parser.next_token();

                    let function_params_opt = utils::get_term_collection(parser)?;

                    let close_parenthesis_opt = parser.peek_current();

                    if let Some(Delimiter {
                        delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                        ..
                    }) = close_parenthesis_opt
                    {
                        parser.next_token();

                        let return_type_opt = if let Some(Punctuation {
                            punc_kind: PuncKind::ThinArrow,
                            ..
                        }) = parser.peek_current()
                        {
                            parser.next_token();

                            if let Some(ty) = Type::parse(parser)? {
                                parser.next_token();
                                Some(Box::new(ty))
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        if let Some(Punctuation {
                            punc_kind: PuncKind::Semicolon,
                            ..
                        }) = parser.peek_current()
                        {
                            parser.next_token();
                        }

                        utils::log_msg(LogMsgType::Exit, "function signature", parser);

                        return Ok(Some(FunctionSig {
                            attributes_opt,
                            visibility_opt,
                            kw_func: kw_func_opt.unwrap(),
                            function_name,
                            open_parenthesis: open_parenthesis_opt.unwrap(),
                            function_params_opt,
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
                    expected: "`func`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseItem for FunctionWithBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        utils::log_msg(LogMsgType::Enter, "function with block", parser);

        if let Some(function_sig) = FunctionSig::parse(parser)? {
            utils::log_msg(LogMsgType::Expect, "`)` or type", parser);

            if let Some(function_body) = ExprWithBlock::parse(parser)? {
                utils::log_msg(LogMsgType::Exit, "function with block", parser);

                return Ok(Some(FunctionWithBlock {
                    function_sig,
                    function_body,
                }));
            }

            return Ok(None);
        } else {
            return Ok(None);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils;

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
            FunctionParam::parse(&mut parser).expect("unable to parse function parameter");

        Ok(println!("{:#?}", function_param))
    }

    #[test]
    fn parse_function_sig() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        pub func foo(bar: bool, baz: char) -> u64
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let function_sig =
            FunctionSig::parse(&mut parser).expect("unable to parse function signature");

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
            FunctionWithBlock::parse(&mut parser).expect("unable to parse function with block");

        // Ok(println!("{:#?}", function_with_block))

        Ok(())
    }
}
