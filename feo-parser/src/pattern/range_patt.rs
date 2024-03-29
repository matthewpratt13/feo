use feo_ast::{
    pattern::{RangeFromPatt, RangeInclusivePatt, RangePattBound, RangeToInclusivePatt},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{literal::LiteralKind, punctuation::PuncKind, Punctuation};

use crate::{
    parse::ParsePatt,
    parser::Parser,
    peek::{Peek, Peeker},
};

impl Peek for RangePattBound {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(l) = LiteralKind::peek(peeker) {
            match l {
                LiteralKind::Char(c) => Some(RangePattBound::CharLit(c)),
                LiteralKind::Int(i) => Some(RangePattBound::IntLit(i)),
                LiteralKind::UInt(ui) => Some(RangePattBound::UIntLit(ui)),
                LiteralKind::U256(u) => Some(RangePattBound::U256Lit(u)),
                LiteralKind::Float(f) => Some(RangePattBound::FloatLit(f)),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl ParsePatt for RangeFromPatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(from) = parser.peek_current::<RangePattBound>() {
            let dbl_dot_opt = parser.peek_next();

            if let Some(Punctuation {
                punc_kind: PuncKind::DblDot,
                ..
            }) = dbl_dot_opt
            {
                parser.next_token();

                return Ok(Some(RangeFromPatt {
                    from,
                    dbl_dot: dbl_dot_opt.unwrap(),
                }));
            } else {
                return Ok(None);
            }
        }

        Err(parser.errors())
    }
}

impl ParsePatt for RangeInclusivePatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(from) = parser.peek_current::<RangePattBound>() {
            let dot_dot_equals_opt = parser.peek_next();

            if let Some(Punctuation {
                punc_kind: PuncKind::DotDotEquals,
                ..
            }) = dot_dot_equals_opt
            {
                parser.next_token();
                parser.next_token();

                if let Some(to_incl) = parser.peek_current::<RangePattBound>() {
                    parser.next_token();

                    return Ok(Some(RangeInclusivePatt {
                        from,
                        dot_dot_equals: dot_dot_equals_opt.unwrap(),
                        to_incl,
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "range pattern bound".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParsePatt for RangeToInclusivePatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let dot_dot_equals_opt = parser.peek_current();

        if let Some(Punctuation {
            punc_kind: PuncKind::DotDotEquals,
            ..
        }) = dot_dot_equals_opt
        {
            parser.next_token();

            if let Some(to_incl) = parser.peek_current::<RangePattBound>() {
                parser.next_token();

                return Ok(Some(RangeToInclusivePatt {
                    dot_dot_equals: dot_dot_equals_opt.unwrap(),
                    to_incl,
                }));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "range pattern bound".to_string(),
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

    #[test]
    fn parse_range_from_patt() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"1.."#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let range_from_patt =
            RangeFromPatt::parse(&mut parser).expect("unable to parse from range pattern");

        Ok(println!("{:#?}", range_from_patt))
    }

    #[test]
    fn parse_range_inclusive_patt() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"1..=10"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let range_inclusive_patt = RangeInclusivePatt::parse(&mut parser)
            .expect("unable to parse inclusive range pattern");

        Ok(println!("{:#?}", range_inclusive_patt))
    }

    #[test]
    fn parse_range_to_inclusive_patt() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"..=10"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let range_to_inclusive_patt = RangeToInclusivePatt::parse(&mut parser)
            .expect(" unable to parse to-inclusive range pattern");

        Ok(println!("{:#?}", range_to_inclusive_patt))
    }
}
