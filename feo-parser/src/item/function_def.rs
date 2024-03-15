use feo_ast::{
    attribute::OuterAttr,
    item::{
        FuncOrMethodParam, FunctionParam, FunctionParams, FunctionSig, SelfParam, VisibilityKind,
    },
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
    parse::{ParseItem, ParsePatt, ParseTerm, ParseType},
    parser::Parser,
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
        let ampersand_opt = parser.peek_current::<Punctuation>();

        if let Some(Punctuation {
            punc_kind: PuncKind::Ampersand,
            ..
        }) = ampersand_opt
        {
            parser.next_token();
        }

        let kw_mut_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwMut,
            ..
        }) = kw_mut_opt
        {
            parser.next_token();
        }

        let kw_self_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwSelf,
            ..
        }) = kw_self_opt
        {
            parser.next_token();

            let type_annotation_opt = if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = parser.peek_current::<Punctuation>()
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

            let colon_opt = parser.peek_current::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = colon_opt
            {
                parser.next_token();

                if let Some(param_type) = Type::parse(parser)? {
                    parser.next_token();

                    return Ok(Some(FunctionParam {
                        param_pattern: Box::new(param_pattern),
                        param_type: Box::new(param_type),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`Type`".to_string(),
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

impl ParseTerm for FunctionParams {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_params: Vec<FunctionParam> = Vec::new();

        if let Some(first_param) = FuncOrMethodParam::parse(parser)? {
            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(next_param) = FunctionParam::parse(parser)? {
                    subsequent_params.push(next_param);
                } else {
                    break;
                }
            }

            match &subsequent_params.is_empty() {
                true => {
                    return Ok(Some(FunctionParams {
                        first_param,
                        subsequent_params_opt: None,
                    }))
                }
                false => {
                    return Ok(Some(FunctionParams {
                        first_param,
                        subsequent_params_opt: Some(subsequent_params),
                    }))
                }
            }
        } else {
            return Ok(None);
        }
    }
}

impl ParseItem for FunctionSig {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(oa) = OuterAttr::parse(parser)? {
            attributes.push(oa);
            parser.next_token();
        }

        let visibility_opt = if let Some(v) = VisibilityKind::parse(parser)? {
            parser.next_token();
            Some(v)
        } else {
            None
        };

        let kw_func_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwFunc,
            ..
        }) = kw_func_opt
        {
            parser.next_token();

            if let Some(function_name) = parser.peek_current::<Identifier>() {
                parser.next_token();

                let open_parenthesis_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) = open_parenthesis_opt
                {
                    parser.next_token();

                    let function_params_opt = if let Some(fp) = FunctionParams::parse(parser)? {
                        Some(fp)
                    } else {
                        None
                    };

                    let close_parenthesis_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                        ..
                    }) = close_parenthesis_opt
                    {
                        parser.next_token();

                        let return_type_opt = if let Some(Punctuation {
                            punc_kind: PuncKind::ThinArrow,
                            ..
                        }) = parser.peek_current::<Punctuation>()
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

                        match &attributes.is_empty() {
                            true => {
                                return Ok(Some(FunctionSig {
                                    attributes_opt: None,
                                    visibility_opt,
                                    kw_func: kw_func_opt.unwrap(),
                                    function_name,
                                    open_parenthesis: open_parenthesis_opt.unwrap(),
                                    function_params_opt,
                                    close_parenthesis: close_parenthesis_opt.unwrap(),
                                    return_type_opt,
                                }))
                            }
                            false => {
                                return Ok(Some(FunctionSig {
                                    attributes_opt: Some(attributes),
                                    visibility_opt,
                                    kw_func: kw_func_opt.unwrap(),
                                    function_name,
                                    open_parenthesis: open_parenthesis_opt.unwrap(),
                                    function_params_opt,
                                    close_parenthesis: close_parenthesis_opt.unwrap(),
                                    return_type_opt,
                                }))
                            }
                        }
                    } else {
                        parser.log_error(ParserErrorKind::MissingDelimiter {
                            delim: ")".to_string(),
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

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_self_param() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"&mut self: u64"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let self_param = SelfParam::parse(&mut parser).expect("unable to parse self param");

        Ok(println!("{:#?}", self_param))
    }

    #[test]
    fn parse_function_param() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"foo: u64"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let function_param =
            FunctionParam::parse(&mut parser).expect("unable to parse function param");

        Ok(println!("{:#?}", function_param))
    }

    #[test]
    fn parse_function_params() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"foo: u64, bar: bool, baz: char"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let function_params =
            FunctionParams::parse(&mut parser).expect("unable to parse function params");

        Ok(println!("{:#?}", function_params))
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
}
