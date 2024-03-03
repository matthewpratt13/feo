use feo_ast::expression::BlockExpr;
use feo_error::error::CompilerError;

use crate::{parse::ParseExpr, parser::Parser};

impl ParseExpr for BlockExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}