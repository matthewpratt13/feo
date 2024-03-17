use feo_ast::{
    expression::{
        RangeFromExpr, RangeFromToExpr, RangeFullExpr, RangeInclusiveExpr, RangeToExpr,
        RangeToInclusiveExpr, Value,
    },
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{punctuation::PuncKind, Punctuation};

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
};

impl ParseExpr for RangeFullExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let dbl_dot_opt = parser.peek_current::<Punctuation>();

        if let Some(Punctuation {
            punc_kind: PuncKind::DblDot,
            ..
        }) = dbl_dot_opt
        {
            parser.next_token();
            return Ok(Some(RangeFullExpr(dbl_dot_opt.unwrap())));
        } else {
            return Ok(None);
        }
    }
}

impl ParseExpr for RangeFromToExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(from_operand) = Value::parse(parser)? {
            parser.next_token();

            let dbl_dot_opt = parser.peek_current::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::DblDot,
                ..
            }) = dbl_dot_opt
            {
                parser.next_token();

                if let Some(to_operand_excl) = Value::parse(parser)? {
                    parser.next_token();

                    return Ok(Some(RangeFromToExpr {
                        from_operand: Box::new(from_operand),
                        dbl_dot: dbl_dot_opt.unwrap(),
                        to_operand_excl: Box::new(to_operand_excl),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`Value`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`..`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for RangeFromExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for RangeToExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for RangeInclusiveExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(from_operand) = Value::parse(parser)? {
            parser.next_token();

            let dot_dot_equals_opt = parser.peek_current::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::DotDotEquals,
                ..
            }) = dot_dot_equals_opt
            {
                parser.next_token();

                if let Some(to_operand_incl) = Value::parse(parser)? {
                    parser.next_token();

                    return Ok(Some(RangeInclusiveExpr {
                        from_operand: Box::new(from_operand),
                        dot_dot_equals: dot_dot_equals_opt.unwrap(),
                        to_operand_incl: Box::new(to_operand_incl),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`Value`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`..=`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for RangeToInclusiveExpr {
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
    fn parse_range_full_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#".."#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let range_full_expr =
            RangeFullExpr::parse(&mut parser).expect("unable to parse full range expression");

        Ok(println!("{:#?}", range_full_expr))
    }

    #[test]
    fn parse_range_from_to_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"1..5"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let range_from_to_expr =
            RangeFromToExpr::parse(&mut parser).expect("unable to parse from-to range expression");

        Ok(println!("{:#?}", range_from_to_expr))
    }

     #[test]
    fn parse_range_inclusive_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"1..=5"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let range_inclusive_expr =
            RangeInclusiveExpr::parse(&mut parser).expect("unable to parse from-to inclusive range expression");

        Ok(println!("{:#?}", range_inclusive_expr))
    }
}
