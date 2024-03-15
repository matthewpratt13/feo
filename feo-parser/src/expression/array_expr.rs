use feo_ast::{
    expression::{
        ArrayElementsCommaSeparated, ArrayElementsKind, ArrayElementsRepeatedValue, ArrayExpr,
        Assignable, IndexExpr, Iterable,
    },
    token::Token,
};

use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    literal::UIntType,
    punctuation::PuncKind,
    Delimiter, Literal, Punctuation, U64Primitive,
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
        let mut subsequent_elements: Vec<Iterable> = Vec::new();

        if let Some(first_element) = Iterable::parse(parser)? {
            parser.next_token();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(next_element) = Iterable::parse(parser)? {
                    subsequent_elements.push(next_element);

                    parser.next_token();
                } else {
                    break;
                }
            }

            match &subsequent_elements.is_empty() {
                true => Ok(Some(ArrayElementsCommaSeparated {
                    first_element: Box::new(first_element),
                    subsequent_elements_opt: None,
                })),
                false => Ok(Some(ArrayElementsCommaSeparated {
                    first_element: Box::new(first_element),
                    subsequent_elements_opt: Some(subsequent_elements),
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
                        num_repeats: U64Primitive::try_from(num_repeats)
                            .expect("error converting `Literal<UIntType>` to `U64Primitive`"),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`Literal<UIntType>`".to_string(),
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
        if let Some(indexed_operand) = Assignable::parse(parser)? {
            parser.next_token();

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
                            index: U64Primitive::try_from(index)
                                .expect("error converting `Literal<UIntType>` to `U64Primitive`"),
                            close_bracket: close_bracket_opt.unwrap(),
                        }));
                    }

                    parser.log_error(ParserErrorKind::MissingDelimiter {
                        delim: "]".to_string(),
                    });
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`Literal<UIntType>`".to_string(),
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

// #[cfg(test)]
// mod tests {

//     use crate::test_utils;

//     use super::*;

//     #[test]
//     fn parse_array_expr() {
//         let source_code = r#"[1, 2, 3, 4] [a; 4] []"#;

//         let mut parser = test_utils::get_parser(source_code, false);

//         let array_expr = ArrayExpr::parse(&mut parser).expect("unable to parse array expression");

//         println!("{:#?}", array_expr);
//     }

//     #[test]
//     fn parse_index_expr() {
//         let source_code = r#"foo[1]"#;

//         let mut parser = test_utils::get_parser(source_code, false);

//         let index_expr = IndexExpr::parse(&mut parser).expect("unable to parse index expression");

//         println!("{:#?}", index_expr);
//     }
// }
