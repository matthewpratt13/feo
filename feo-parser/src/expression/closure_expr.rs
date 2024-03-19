use feo_ast::{
    expression::{
        ClosureParam, ClosureParams, ClosureParamsOpt, ClosureWithBlock, ClosureWithoutBlock,
        Expression,
    },
    pattern::Pattern,
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{punctuation::PuncKind, Punctuation};

use crate::{
    parse::{ParseExpr, ParsePatt, ParseTerm, ParseType},
    parser::Parser,
    utils,
};

impl ParseTerm for ClosureParamsOpt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let pipe_or_dbl_pipe_opt = parser.peek_current();

        if let Some(Punctuation {
            punc_kind: PuncKind::Pipe,
            ..
        }) = pipe_or_dbl_pipe_opt
        {
            parser.next_token();

            if let Some(params) = ClosureParams::parse(parser)? {
                let pipe_opt = parser.peek_current();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Pipe,
                    ..
                }) = pipe_opt
                {
                    parser.next_token();

                    return Ok(Some(ClosureParamsOpt::Some((
                        pipe_or_dbl_pipe_opt.unwrap(),
                        params,
                        pipe_opt.unwrap(),
                    ))));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`|`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else if let Some(Punctuation {
            punc_kind: PuncKind::DblPipe,
            ..
        }) = pipe_or_dbl_pipe_opt
        {
            parser.next_token();

            return Ok(Some(ClosureParamsOpt::None(pipe_or_dbl_pipe_opt.unwrap())));
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseTerm for ClosureParam {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        if let Some(pattern) = Pattern::parse(parser)? {
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

            return Ok(Some(ClosureParam {
                attributes_opt,
                pattern: Box::new(pattern),
                type_annotation_opt,
            }));
        } else {
            return Ok(None);
        }
    }
}

impl ParseTerm for ClosureParams {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_params: Vec<ClosureParam> = Vec::new();

        if let Some(first_param) = ClosureParam::parse(parser)? {
            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = parser.peek_current()
            {
                parser.next_token();

                if let Some(next_param) = ClosureParam::parse(parser)? {
                    subsequent_params.push(next_param);
                } else {
                    break;
                }
            }

            match &subsequent_params.is_empty() {
                true => Ok(Some(ClosureParams {
                    first_param,
                    subsequent_params_opt: None,
                })),

                false => Ok(Some(ClosureParams {
                    first_param,
                    subsequent_params_opt: Some(subsequent_params),
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseExpr for ClosureWithoutBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(params) = ClosureParamsOpt::parse(parser)? {
            parser.next_token();

            if let Some(body_operand) = Expression::parse(parser)? {
                parser.next_token();

                return Ok(Some(ClosureWithoutBlock {
                    params,
                    body_operand: Box::new(body_operand),
                }));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "expression".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for ClosureWithBlock {
    #[allow(unused_variables)]
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

    #[test]
    fn parse_closure_param() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        #[export]
        foo: u64
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let closure_param =
            ClosureParam::parse(&mut parser).expect("unable to parse closure parameter");

        Ok(println!("{:#?}", closure_param))
    }

    #[test]
    fn parse_closure_params() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        #[export]
        foo: u64,
        #[unsafe]
        bar: bool,
        baz: char
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let closure_params =
            ClosureParams::parse(&mut parser).expect("unable to parse `ClosureParams`");

        Ok(println!("{:#?}", closure_params))
    }

    #[test]
    fn parse_closure_without_block() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"|foo: u64| return bar"#;

        let mut parser = test_utils::get_parser(source_code, true)?;

        let closure_without_block =
            ClosureWithoutBlock::parse(&mut parser).expect("unable to parse closure-without-block");

        Ok(println!("{:#?}", closure_without_block))
    }
}
