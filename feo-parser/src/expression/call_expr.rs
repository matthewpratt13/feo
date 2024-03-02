use feo_ast::{
    expression::{CallParams, Callable, FunctionCallExpr, MethodCallExpr, Returnable},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
    utils::Comma,
    Delimiter, Punctuation,
};

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
        if let Some(function_operand) = Callable::parse(parser)? {
            parser.next_token();

            let open_parenthesis_opt = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                ..
            }) = open_parenthesis_opt
            {
                parser.next_token();

                if let Some(call_params) = CallParams::parse(parser)? {
                    // parser.next_token();

                    let close_parenthesis_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                        ..
                    }) = close_parenthesis_opt
                    {
                        parser.next_token();

                        return Ok(Some(FunctionCallExpr {
                            function_operand: Box::new(function_operand),
                            open_parenthesis: open_parenthesis_opt.unwrap(),
                            call_params_opt: Some(call_params),
                            close_parenthesis: close_parenthesis_opt.unwrap(),
                        }));
                    }

                    parser.log_error(ParserErrorKind::MissingDelimiter {
                        delim: "`)`".to_string(),
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

impl ParseExpr for MethodCallExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_function_call_expr() {
        let source_code = r#"foo(bar, "a", 1)"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let function_call_expr =
            FunctionCallExpr::parse(&mut parser).expect("unable to parse function call expression");

        println!("{:#?}", function_call_expr);
    }
}
