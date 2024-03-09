use feo_ast::{
    pattern::{
        RangeFromPatt, RangeInclusivePatt, RangePattBound, RangePattKind, RangeToInclusivePatt,
    },
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{literal::LiteralKind, punctuation::PuncKind, Punctuation};

use crate::{
    parse::ParseTerm,
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

impl ParseTerm for RangePattKind {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseTerm for RangeFromPatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(from) = parser.peek_current::<RangePattBound>() {
            parser.next_token();

            let dbl_dot_opt = parser.peek_current::<Punctuation>();

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
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseTerm for RangeInclusivePatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(from) = parser.peek_current::<RangePattBound>() {
            parser.next_token();

            let dot_dot_equals_opt = parser.peek_current::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::DotDotEquals,
                ..
            }) = dot_dot_equals_opt
            {
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
                    expected: "`RangePattBound`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseTerm for RangeToInclusivePatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let dot_dot_equals_opt = parser.peek_current::<Punctuation>();

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
                expected: "`RangePattBound`".to_string(),
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
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_range_from_patt() {
        let source_code = r#"1.."#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let range_from_patt =
            RangeFromPatt::parse(&mut parser).expect("unable to parse `RangeFromPatt`");

        println!("{:#?}", range_from_patt);
    }

    #[test]
    fn parse_range_inclusive_patt() {
        let source_code = r#"1..=10"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let range_inclusive_patt =
            RangeFromPatt::parse(&mut parser).expect("unable to parse `RangeInclusivePatt`");

        println!("{:#?}", range_inclusive_patt);
    }

    #[test]
    fn parse_range_to_inclusive_patt() {
        let source_code = r#"..=10"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let range_to_inclusive_patt =
            RangeFromPatt::parse(&mut parser).expect("unable to parse `RangeToInclusivePatt`");

        println!("{:#?}", range_to_inclusive_patt);
    }
}
