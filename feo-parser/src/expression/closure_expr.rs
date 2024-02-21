use feo_ast::expression::{ClosureWithBlock, ClosureWithoutBlock};
use feo_error::handler::ErrorEmitted;

use crate::{parse::ParseExpr, parser::Parser};

impl ParseExpr for ClosureWithBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for ClosureWithoutBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}
