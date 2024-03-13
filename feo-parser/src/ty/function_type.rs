use feo_ast::{
    expression::ClosureParamsOpt,
    item::FunctionParams,
    token::Token,
    ty::{ClosureType, FunctionType},
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    Delimiter, Keyword, Punctuation,
};

use crate::{
    parse::{ParseTerm, ParseType},
    parser::Parser,
};

impl ParseType for FunctionType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_func_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwFunc,
            ..
        }) = kw_func_opt
        {
            parser.next_token();

            let open_parenthesis_opt = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                ..
            }) = open_parenthesis_opt
            {
                parser.next_token();

                let function_params_opt = if let Some(p) = FunctionParams::parse(parser)? {
                    parser.next_token();
                    Some(p)
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

                    return Ok(Some(FunctionType {
                        kw_func: kw_func_opt.unwrap(),
                        open_parenthesis: open_parenthesis_opt.unwrap(),
                        function_params_opt,
                        close_parenthesis: close_parenthesis_opt.unwrap(),
                        return_type_opt,
                    }));
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
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseType for ClosureType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(params) = ClosureParamsOpt::parse(parser)? {
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

            return Ok(Some(ClosureType {
                params,
                return_type_opt,
            }));
        } else {
            return Ok(None);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils;

    use super::*;

    #[ignore]
    #[test]
    fn parse_function_type() {
        let source_code = r#"
        func (foo: u64, bar: bool) -> char
        func ()
        "#;

        let mut parser = test_utils::get_parser(source_code, false);

        let function_type =
            FunctionType::parse(&mut parser).expect("unable to parse function type");

        println!("{:#?}", function_type);
    }

    #[ignore]
    #[test]
    fn parse_closure_type() {
        let source_code = r#"
        |foo: u64, bar: bool| -> char
        ||
        "#;

        let mut parser = test_utils::get_parser(source_code, false);

        let closure_type = ClosureType::parse(&mut parser).expect("unable to parse closure type");

        println!("{:#?}", closure_type);
    }
}
