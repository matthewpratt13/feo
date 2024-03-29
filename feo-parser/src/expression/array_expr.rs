use feo_ast::{
    expression::{ArrayExpr, IndexExpr, Value},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    Delimiter,
};

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
    utils,
};

impl ParseExpr for ArrayExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let open_bracket_opt = parser.peek_current();

        if let Some(Delimiter {
            delim: (DelimKind::Bracket, DelimOrientation::Open),
            ..
        }) = open_bracket_opt
        {
            parser.next_token();

            let elements_opt = utils::get_value_collection(parser)?;

            let close_bracket_opt = parser.peek_current();

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Close),
                ..
            }) = close_bracket_opt
            {
                return Ok(Some(ArrayExpr {
                    open_bracket: open_bracket_opt.unwrap(),
                    elements_opt,
                    close_bracket: close_bracket_opt.unwrap(),
                }));
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`]`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
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
        // TODO: find a way to prevent stack overflow when accessing indexes (as an index expression)
        if let Some(indexed_operand) = Value::parse(parser)? {
            let open_bracket_opt = parser.peek_next();

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_opt
            {
                parser.next_token();
                parser.next_token();

                if let Some(index) = Value::parse(parser)? {
                    let close_bracket_opt = parser.peek_next();

                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_opt
                    {
                        parser.next_token();
                        
                        return Ok(Some(IndexExpr {
                            indexed_operand: Box::new(indexed_operand),
                            open_bracket: open_bracket_opt.unwrap(),
                            index: Box::new(index),
                            close_bracket: close_bracket_opt.unwrap(),
                        }));
                    }

                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`]`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`Literal<UIntType>`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                return Ok(None);
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
    fn parse_array_expr_with_elements() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"[1, 2, 3, 4,]"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let array_expr = ArrayExpr::parse(&mut parser).expect("unable to parse array");

        Ok(println!("{:#?}", array_expr))
    }

    #[test]
    fn parse_array_empty() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"[]"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let array_expr = ArrayExpr::parse(&mut parser).expect("unable to parse empty array");

        Ok(println!("{:#?}", array_expr))
    }

    #[test]
    fn parse_index_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"foo[1]"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let index_expr = IndexExpr::parse(&mut parser).expect("unable to parse index expression");

        Ok(println!("{:#?}", index_expr))
    }
}
