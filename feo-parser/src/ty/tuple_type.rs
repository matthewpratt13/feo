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

            while let Some(element) = Type::parse(parser)? {
                parser.next_token();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Comma,
                    ..
                }) = parser.peek_current::<Punctuation>()
                {
                    elements.push(element);
                    parser.next_token();
                } else {
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
                                elements_opt: None,
                                trailing_element: Box::new(trailing_element),
                                close_parenthesis: close_parenthesis_opt.unwrap(),
                            }))
                        }

                        false => {
                            return Ok(Some(TupleType {
                                open_parenthesis: open_parenthesis_opt.unwrap(),
                                elements_opt: Some(elements),
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

    use crate::test_utils;

    use super::*;

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_tuple_type() {
        let source_code = r#"(u64, char, bool)"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let tuple_type = TupleType::parse(&mut parser).expect("unable to parse tuple type");

        println!("{:#?}", tuple_type);
    }

    #[test]
    fn parse_unit_type() {
        let source_code = r#"()"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let unit_type = UnitType::parse(&mut parser).expect("unable to parse unit type");

        println!("{:#?}", unit_type);
    }
}
