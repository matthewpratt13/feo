use feo_ast::{
    attribute::OuterAttr,
    expression::{
        ClosureParam, ClosureParams, ClosureParamsOpt, ClosureWithBlock, ClosureWithoutBlock,
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
};

impl ParseTerm for ClosureParamsOpt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let pipe_or_dbl_pipe_opt = parser.peek_current::<Punctuation>();

        if let Some(Punctuation {
            punc_kind: PuncKind::Pipe,
            ..
        }) = pipe_or_dbl_pipe_opt
        {
            parser.next_token();

            if let Some(params) = ClosureParams::parse(parser)? {
                let pipe_opt = parser.peek_current::<Punctuation>();

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
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(oa) = OuterAttr::parse(parser)? {
            attributes.push(oa);
            parser.next_token();
        }

        if let Some(pattern) = Pattern::parse(parser)? {
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

            match &attributes.is_empty() {
                true => {
                    return Ok(Some(ClosureParam {
                        attributes_opt: None,
                        pattern: Box::new(pattern),
                        type_annotation_opt,
                    }))
                }
                false => {
                    return Ok(Some(ClosureParam {
                        attributes_opt: Some(attributes),
                        pattern: Box::new(pattern),
                        type_annotation_opt,
                    }))
                }
            }
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
            }) = parser.peek_current::<Punctuation>()
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
        todo!()
    }
}

impl ParseExpr for ClosureWithBlock {
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
    fn parse_closure_param() {
        let source_code = r#"
        #[abstract]
        #[export]
        foo: u64
        "#;

        let mut parser = test_utils::get_parser(source_code, false);

        let closure_param =
            ClosureParam::parse(&mut parser).expect("unable to parse closure param");

        println!("{:#?}", closure_param);
    }

    #[test]
    fn parse_closure_params() {
        let source_code = r#"
        #[abstract]
        #[export]
        foo: u64,
        #[unsafe]
        bar: bool,
        baz: char
        "#;

        let mut parser = test_utils::get_parser(source_code, false);

        let closure_params =
            ClosureParams::parse(&mut parser).expect("unable to parse closure params");

        println!("{:#?}", closure_params);
    }
}
