use feo_ast::{
    expression::{Returnable, TupleExpr, TupleExprElements, TupleIndexExpr},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
    Delimiter, Punctuation,
};

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
};

impl ParseTerm for TupleExprElements {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_elements: Vec<Returnable> = Vec::new();

        if let Some(first_element) = Returnable::parse(parser)? {
            parser.next_token();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(next_element) = Returnable::parse(parser)? {
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
                true => Ok(Some(TupleExprElements {
                    first_element: Box::new(first_element),
                    subsequent_elements_opt: None,
                    trailing_comma_opt,
                })),
                false => Ok(Some(TupleExprElements {
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

            if let Some(elements) = TupleExprElements::parse(parser)? {
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
                    expected: "`TupleExprElements`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
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

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_tuple_expr_elements() {
        let source_code = r#"1, "a", x"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let tuple_expr_elements = TupleExprElements::parse(&mut parser).expect("unable to parse tuple expression elements");

        println!("{:#?}", tuple_expr_elements);
    }

    #[test]
    fn parse_tuple_expr() {
        let source_code = r#"(1, "a", x)"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let tuple_expr = TupleExpr::parse(&mut parser).expect("unable to parse tuple expression");

        println!("{:#?}", tuple_expr);
    }
}
