use feo_ast::{token::Token, ty::TupleType, Type};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
    utils::Comma,
    Delimiter, Punctuation,
};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for TupleType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut elements: Vec<(Type, Comma)> = Vec::new();

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

                let next_comma_opt = parser.peek_current::<Punctuation>();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Comma,
                    ..
                }) = next_comma_opt
                {
                    elements.push((element, next_comma_opt.unwrap()));

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
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`)`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
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
