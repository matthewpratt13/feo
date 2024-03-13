use feo_ast::{
    item::{FunctionParam, FunctionParams, FunctionSig, SelfParam},
    pattern::Pattern,
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Keyword, Punctuation};

use crate::{
    parse::{ParsePatt, ParseTerm, ParseType},
    parser::Parser,
};

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
        todo!()
    }
}

impl ParseTerm for FunctionSig {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_self_param() {
        let source_code = r#"&mut self: u64"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let self_param = SelfParam::parse(&mut parser).expect("unable to parse self param");

        println!("{:#?}", self_param);
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_function_param() {
        let source_code = r#"foo: u64"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let function_param =
            FunctionParam::parse(&mut parser).expect("unable to parse function param");

        println!("{:#?}", function_param);
    }
}
