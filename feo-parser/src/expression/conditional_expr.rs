use feo_ast::expression::{IfExpr, MatchExpr};
use feo_error::error::CompilerError;

use crate::{parse::ParseExpr, parser::Parser};

impl ParseExpr for IfExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for MatchExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
