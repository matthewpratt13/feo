use feo_ast::{
    expression::{FunctionCallExpr, MethodCallExpr, Value},
    path::PathInExpr,
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
    Delimiter, Identifier, Punctuation,
};

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
    utils,
};

impl ParseExpr for FunctionCallExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(function_operand) = PathInExpr::parse(parser)? {
            println!(
                "entering function call expression... \ncurrent_token: {:#?}",
                &parser.current_token()
            );

            parser.next_token();

            let open_parenthesis_opt = parser.peek_current();

            if let Some(Delimiter {
                delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                ..
            }) = open_parenthesis_opt
            {
                println!(
                    "entering function call params... \ncurrent token: {:#?}",
                    parser.current_token()
                );

                parser.next_token();

                let call_params_opt = utils::get_value_collection(parser)?;

                println!("function call params (optional): {:#?}", &call_params_opt);

                let close_parenthesis_opt = parser.peek_current();

                println!(
                    "expects close parenthesis... \nfinds: {:#?}",
                    parser.current_token()
                );

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                    ..
                }) = close_parenthesis_opt
                {
                    parser.next_token();

                    println!(
                        "exit function call expression. \ncurrent token: {:#?}",
                        parser.current_token()
                    );

                    return Ok(Some(FunctionCallExpr {
                        function_operand,
                        open_parenthesis: open_parenthesis_opt.unwrap(),
                        call_params_opt,
                        close_parenthesis: close_parenthesis_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`)`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
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
        if let Some(receiver) = Value::parse(parser)? {
            parser.next_token();

            let full_stop_opt = parser.peek_current::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::FullStop,
                ..
            }) = full_stop_opt
            {
                parser.next_token();

                if let Some(method_name) = parser.peek_current::<Identifier>() {
                    parser.next_token();

                    let open_parenthesis_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                        ..
                    }) = open_parenthesis_opt
                    {
                        parser.next_token();

                        let call_params_opt = utils::get_value_collection(parser)?;

                        let close_parenthesis_opt = parser.peek_current::<Delimiter>();

                        if let Some(Delimiter {
                            delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                            ..
                        }) = close_parenthesis_opt
                        {
                            parser.next_token();

                            return Ok(Some(MethodCallExpr {
                                receiver: Box::new(receiver),
                                full_stop: full_stop_opt.unwrap(),
                                method_name,
                                open_parenthesis: open_parenthesis_opt.unwrap(),
                                call_params_opt,
                                close_parenthesis: close_parenthesis_opt.unwrap(),
                            }));
                        }

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`)`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`(`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                } else {
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`.`".to_string(),
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
    fn parse_call_expr_function() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"foo(bar, "a", 1)"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let function_call_expr =
            FunctionCallExpr::parse(&mut parser).expect("unable to parse function call expression");

        Ok(println!("{:#?}", function_call_expr))
    }

    #[test]
    fn parse_call_expr_method() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"foo.bar(baz, "a", 1)"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let method_call_expr =
            MethodCallExpr::parse(&mut parser).expect("unable to parse method call expression");

        Ok(println!("{:#?}", method_call_expr))
    }
}
