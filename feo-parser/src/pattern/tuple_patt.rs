use feo_ast::{
    pattern::{Pattern, TuplePatt, TuplePattElements},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
    Delimiter, Punctuation,
};

use crate::{
    parse::{ParsePatt, ParseTerm},
    parser::Parser,
};

impl ParseTerm for TuplePattElements {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_elements: Vec<Pattern> = Vec::new();

        if let Some(first_element) = Pattern::parse(parser)? {
            parser.next_token();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = parser.peek_current()
            {
                parser.next_token();

                if let Some(next_element) = Pattern::parse(parser)? {
                    subsequent_elements.push(next_element);
                    parser.next_token();
                } else {
                    break;
                }
            }

            parser.next_token();

            let trailing_comma_opt = parser.peek_current();

            if let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = trailing_comma_opt
            {
                parser.next_token();
            }

            match &subsequent_elements.is_empty() {
                true => Ok(Some(TuplePattElements {
                    first_element: Box::new(first_element),
                    subsequent_elements_opt: None,
                    trailing_comma_opt,
                })),

                false => Ok(Some(TuplePattElements {
                    first_element: Box::new(first_element),
                    subsequent_elements_opt: Some(subsequent_elements),
                    trailing_comma_opt,
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParsePatt for TuplePatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let open_parenthesis_opt = parser.peek_current();

        if let Some(Delimiter {
            delim: (DelimKind::Parenthesis, DelimOrientation::Open),
            ..
        }) = open_parenthesis_opt
        {
            parser.next_token();

            if let Some(elements) = TuplePattElements::parse(parser)? {
                let close_parenthesis_opt = parser.peek_next();

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                    ..
                }) = close_parenthesis_opt
                {
                    parser.next_token();

                    return Ok(Some(TuplePatt {
                        open_parenthesis: open_parenthesis_opt.unwrap(),
                        elements,
                        close_parenthesis: close_parenthesis_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`)`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "tuple pattern elements".to_string(),
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
    fn parse_tuple_patt_elements() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"1, "a", x"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let tuple_patt_elements =
            TuplePattElements::parse(&mut parser).expect("unable to parse tuple pattern elements");

        Ok(println!("{:#?}", tuple_patt_elements))
    }

    #[test]
    fn parse_tuple_patt() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"(1, "a", x)"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let tuple_patt = TuplePatt::parse(&mut parser).expect("unable to parse tuple pattern");

        Ok(println!("{:#?}", tuple_patt))
    }
}
