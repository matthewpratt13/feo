use feo_ast::{
    token::Token,
    ty::{TupleType, UnitType},
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
    Delimiter, Punctuation,
};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for TupleType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut elements: Vec<Type> = Vec::new();

        let open_parenthesis_opt = parser.peek_current::<Delimiter>();

        if let Some(Delimiter {
            delim: (DelimKind::Parenthesis, DelimOrientation::Open),
            ..
        }) = open_parenthesis_opt
        {
            parser.next_token();

            let mut next_element_opt = Type::parse(parser)?;

            while let Some(element) = next_element_opt {
                parser.next_token();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Comma,
                    ..
                }) = parser.peek_current::<Punctuation>()
                {
                    elements.push(element);

                    parser.next_token();

                    if let Some(e) = Type::parse(parser)? {
                        next_element_opt = Some(e);
                        parser.next_token();
                    } else {
                        break;
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`,`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                    break;
                }
            }

            if let Some(trailing_element) = Type::parse(parser)? {
                parser.next_token();

                let close_parenthesis_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                    ..
                }) = close_parenthesis_opt
                {
                    parser.next_token();

                    match &elements.is_empty() {
                        true => {
                            return Ok(Some(TupleType {
                                open_parenthesis: open_parenthesis_opt.unwrap(),
                                elements: None,
                                trailing_element: Box::new(trailing_element),
                                close_parenthesis: close_parenthesis_opt.unwrap(),
                            }))
                        }

                        false => {
                            return Ok(Some(TupleType {
                                open_parenthesis: open_parenthesis_opt.unwrap(),
                                elements: Some(elements),
                                trailing_element: Box::new(trailing_element),
                                close_parenthesis: close_parenthesis_opt.unwrap(),
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
                    expected: "`Type`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseTerm for UnitType {
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

            let close_parenthesis_opt = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                ..
            }) = close_parenthesis_opt
            {
                parser.next_token();

                return Ok(Some(UnitType(
                    open_parenthesis_opt.unwrap(),
                    close_parenthesis_opt.unwrap(),
                )));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`)`".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

#[cfg(test)]
mod tests {
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_tuple_type() {
        let source_code = r#"(u64, char, bool)"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let tuple_type = TupleType::parse(&mut parser).expect("unable to parse tuple type");

        println!("{:#?}", tuple_type);
    }

    #[test]
    fn parse_unit_type() {
        let source_code = r#"()"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let unit_type = UnitType::parse(&mut parser).expect("unable to parse unit type");

        println!("{:#?}", unit_type);
    }
}
