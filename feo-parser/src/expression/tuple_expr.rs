use feo_ast::{
    expression::{Returnable, TupleElements, TupleExpr, TupleIndexExpr},
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

impl ParseTerm for TupleElements {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_elements: Vec<(Comma, Returnable)> = Vec::new();

        if let Some(first_element) = Returnable::parse(parser)? {
            parser.next_token();

            let mut comma_opt = parser.peek_current::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = comma_opt
            {
                parser.next_token();

                if let Some(next_element) = Returnable::parse(parser)? {
                    subsequent_elements.push((comma_opt.unwrap(), next_element));

                    if let Some(p) = parser.peek_next::<Punctuation>() {
                        comma_opt = Some(p);
                        parser.next_token();
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

            if !subsequent_elements.is_empty() {
                return Ok(Some(TupleElements {
                    first_element: Box::new(first_element),
                    subsequent_elements_opt: Some(subsequent_elements),
                    trailing_comma_opt,
                }));
            }

            return Ok(Some(TupleElements {
                first_element: Box::new(first_element),
                subsequent_elements_opt: None,
                trailing_comma_opt,
            }));
        }

        parser.log_error(ParserErrorKind::UnexpectedToken {
            expected: "`Returnable`".to_string(),
            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
        });

        Err(parser.errors())
    }
}

impl ParseExpr for TupleExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let open_parenthesis_opt = parser.peek_current::<Delimiter>();

        if let Some(Delimiter {
            delim: (DelimKind::Parenthesis, DelimOrientation::Open),
            ..
        }) = open_parenthesis_opt
        {
            parser.next_token();

            if let Some(elements) = TupleElements::parse(parser)? {
                parser.next_token();

                let close_parenthesis_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                    ..
                }) = close_parenthesis_opt
                {
                    parser.next_token();

                    return Ok(Some(TupleExpr {
                        open_parenthesis: open_parenthesis_opt.unwrap(),
                        elements,
                        close_parenthesis: close_parenthesis_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::MissingDelimiter {
                    delim: ")".to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`TupleElement`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`(`".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        }

        Err(parser.errors())
    }
}

impl ParseExpr for TupleIndexExpr {
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
    fn parse_tuple_expr() {
        let source_code = r#"(1, "a", x)"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        println!("{:#?} ", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let tuple_expr = TupleExpr::parse(&mut parser).expect("unable to parse tuple expression");

        println!("{:#?}", tuple_expr);
    }
}
