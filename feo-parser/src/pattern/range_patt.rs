use feo_ast::pattern::{
    RangeFromPatt, RangeInclusivePatt, RangePatt, RangePattBound, RangeToInclusivePatt,
};
use feo_error::error::CompilerError;
use feo_types::literal::LiteralKind;

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

impl ParseTerm for RangePatt {
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
        todo!()
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
