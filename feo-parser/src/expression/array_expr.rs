use feo_ast::{
    expression::{
        ArrayElementsCommaSeparated, ArrayElementsKind, ArrayElementsRepeatedValue, ArrayExpr,
        IndexExpr, Iterable,
    },
    token::Token,
};

use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    literal::UIntType,
    punctuation::PuncKind,
    utils::Comma,
    Delimiter, Literal, Punctuation,
};

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
};

impl ParseTerm for ArrayElementsCommaSeparated {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_elements: Vec<(Comma, Iterable)> = Vec::new();

        if let Some(first_element) = Iterable::parse(parser)? {
            parser.next_token();

            let mut next_comma_opt = parser.peek_current::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = next_comma_opt
            {
                parser.next_token();

                if let Some(next_element) = Iterable::parse(parser)? {
                    subsequent_elements.push((next_comma_opt.unwrap(), next_element));

                    parser.next_token();

                    if let Some(p) = parser.peek_current::<Punctuation>() {
                        next_comma_opt = Some(p);
                    } else {
                        break;
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`Iterable`".to_string(),
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

            match subsequent_elements.is_empty() {
                true => Ok(Some(ArrayElementsCommaSeparated {
                    first_element: Box::new(first_element),
                    subsequent_elements: None,
                    trailing_comma_opt,
                })),
                false => Ok(Some(ArrayElementsCommaSeparated {
                    first_element: Box::new(first_element),
                    subsequent_elements: Some(subsequent_elements),
                    trailing_comma_opt,
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for ArrayElementsRepeatedValue {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(repeat_operand) = Iterable::parse(parser)? {
            parser.next_token();

            let semicolon_opt = parser.peek_current::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::Semicolon,
                ..
            }) = semicolon_opt
            {
                parser.next_token();

                if let Some(num_repeats) = parser.peek_current::<Literal<UIntType>>() {
                    parser.next_token();

                    return Ok(Some(ArrayElementsRepeatedValue {
                        repeat_operand: Box::new(repeat_operand),
                        semicolon: semicolon_opt.unwrap(),
                        num_repeats,
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`UIntType`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`;`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for ArrayExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let open_bracket_opt = parser.peek_current::<Delimiter>();

        if let Some(Delimiter {
            delim: (DelimKind::Bracket, DelimOrientation::Open),
            ..
        }) = open_bracket_opt
        {
            parser.next_token();

            if let Some(elements) = ArrayElementsCommaSeparated::parse(parser)? {
                let close_bracket_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Bracket, DelimOrientation::Close),
                    ..
                }) = close_bracket_opt
                {
                    parser.next_token();

                    return Ok(Some(ArrayExpr {
                        open_bracket: open_bracket_opt.unwrap(),
                        elements_opt: Some(ArrayElementsKind::CommaSeparated(elements)),
                        close_bracket: close_bracket_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::MissingDelimiter {
                    delim: "]".to_string(),
                });
            } else if let Some(elements) = ArrayElementsRepeatedValue::parse(parser)? {
                parser.next_token();

                let close_bracket_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Bracket, DelimOrientation::Close),
                    ..
                }) = close_bracket_opt
                {
                    parser.next_token();

                    return Ok(Some(ArrayExpr {
                        open_bracket: open_bracket_opt.unwrap(),
                        elements_opt: Some(ArrayElementsKind::RepeatedValue(elements)),
                        close_bracket: close_bracket_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::MissingDelimiter {
                    delim: "]".to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for IndexExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(indexed_operand) = ArrayExpr::parse(parser)? {
            let open_bracket_opt = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_opt
            {
                parser.next_token();

                if let Some(index) = parser.peek_current::<Literal<UIntType>>() {
                    parser.next_token();

                    let close_bracket_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_opt
                    {
                        parser.next_token();

                        return Ok(Some(IndexExpr {
                            indexed_operand,
                            open_bracket: open_bracket_opt.unwrap(),
                            index,
                            close_bracket: close_bracket_opt.unwrap(),
                        }));
                    } else {
                        parser.log_error(ParserErrorKind::MissingDelimiter {
                            delim: "]".to_string(),
                        });
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`UIntType`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`[`".to_string(),
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
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_array_expr() {
        let source_code = r#"[1, 2, 3, 4] [a; 4] []"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let array_expr = ArrayExpr::parse(&mut parser).expect("unable to parse array expression");

        println!("{:#?}", array_expr);
    }

    #[test]
    fn parse_index_expr() {
        let source_code = r#"foo[1]"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let index_expr = IndexExpr::parse(&mut parser).expect("unable to parse index expression");

        println!("{:#?}", index_expr);
    }
}
