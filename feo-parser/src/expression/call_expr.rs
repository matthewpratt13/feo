use feo_ast::{
    expression::{CallParams, FunctionCallExpr, MethodCallExpr, Returnable},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{punctuation::PuncKind, utils::Comma, Punctuation};

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
};

impl ParseTerm for CallParams {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_params: Vec<(Comma, Returnable)> = Vec::new();

        if let Some(first_param) = Returnable::parse(parser)? {
            parser.next_token();

            let mut next_comma_opt = parser.peek_current::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = next_comma_opt
            {
                parser.next_token();

                if let Some(next_param) = Returnable::parse(parser)? {
                    subsequent_params.push((next_comma_opt.unwrap(), next_param));

                    parser.next_token();

                    if let Some(p) = parser.peek_current::<Punctuation>() {
                        next_comma_opt = Some(p)
                    } else {
                        break;
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`Returnable`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                    break;
                }
            }

            let trailing_comma_opt = parser.peek_current::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = trailing_comma_opt
            {
                parser.next_token();
            }

            match subsequent_params.is_empty() {
                true => Ok(Some(CallParams {
                    first_param: Box::new(first_param),
                    subsequent_params: None,
                    trailing_comma_opt,
                })),
                false => Ok(Some(CallParams {
                    first_param: Box::new(first_param),
                    subsequent_params: Some(subsequent_params),
                    trailing_comma_opt,
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseExpr for FunctionCallExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for MethodCallExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
