use feo_ast::{
    pattern::{ParenthesizedPatt, Pattern},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    Delimiter,
};

use crate::{parse::ParsePatt, parser::Parser};

impl ParsePatt for ParenthesizedPatt {
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

            if let Some(pattern) = Pattern::parse(parser)? {
                parser.next_token();

                let close_parenthesis_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                    ..
                }) = close_parenthesis_opt
                {
                    parser.next_token();

                    return Ok(Some(ParenthesizedPatt {
                        open_parenthesis: open_parenthesis_opt.unwrap(),
                        pattern: Box::new(pattern),
                        close_parenthesis: close_parenthesis_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::MissingDelimiter {
                    delim: ")".to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`Pattern`".to_string(),
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
    fn parse_parenthesized_patt() {
        let source_code = r#"(x)"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let parenthesized_patt =
            ParenthesizedPatt::parse(&mut parser).expect("unable to parse parenthesized pattern");

        println!("{:#?}", parenthesized_patt);
    }
}
