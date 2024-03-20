use feo_ast::{
    pattern::{Pattern, TuplePatt, TuplePattElement},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    Delimiter,
};

use crate::{
    parse::{ParsePatt, ParseTerm},
    parser::Parser,
    utils,
};

impl ParseTerm for TuplePattElement {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(p) = Pattern::parse(parser)? {
            return Ok(Some(TuplePattElement(Box::new(p))));
        } else {
            return Ok(None);
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

            if let Some(elements) = utils::get_term_collection::<TuplePattElement>(parser)? {
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
    fn parse_tuple_patt() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"(1, "a", x)"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let tuple_patt = TuplePatt::parse(&mut parser).expect("unable to parse tuple pattern");

        Ok(println!("{:#?}", tuple_patt))
    }
}
