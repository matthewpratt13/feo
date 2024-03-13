use feo_ast::expression::{
    ClosureParam, ClosureParams, ClosureParamsOpt, ClosureWithBlock, ClosureWithoutBlock,
};
use feo_error::error::CompilerError;

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
};

impl ParseTerm for ClosureParamsOpt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseTerm for ClosureParam {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseTerm for ClosureParams {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for ClosureWithoutBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for ClosureWithBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
