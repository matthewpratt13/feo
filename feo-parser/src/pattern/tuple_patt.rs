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
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(next_element) = Pattern::parse(parser)? {
                    subsequent_elements.push(next_element);
                    parser.next_token();
                } else {
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
        let open_parenthesis_opt = parser.peek_current::<Delimiter>();

        if let Some(Delimiter {
            delim: (DelimKind::Parenthesis, DelimOrientation::Open),
            ..
        }) = open_parenthesis_opt
        {
            parser.next_token();

            if let Some(elements) = TuplePattElements::parse(parser)? {
                let close_parenthesis_opt = parser.peek_next::<Delimiter>();

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

                parser.log_error(ParserErrorKind::MissingDelimiter {
                    delim: ")".to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`TuplePattElements`".to_string(),
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

    #[ignore]
    #[test]
    fn parse_tuple_patt_elements() {
        let source_code = r#"1, "a", x"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let tuple_patt_elements =
            TuplePattElements::parse(&mut parser).expect("unable to parse tuple pattern elements");

        println!("{:#?}", tuple_patt_elements);
    }
}
