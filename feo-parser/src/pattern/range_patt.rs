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
                punc_kind: PuncKind::DotDotEquals,
                ..
            }) = dbl_dot_opt
            {
                parser.next_token();

                return Ok(Some(RangeFromPatt {
                    from,
                    dbl_dot: dbl_dot_opt.unwrap(),
                }));
            }
            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`..`".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
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
        todo!()
    }
}

impl ParseTerm for RangeToInclusivePatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
