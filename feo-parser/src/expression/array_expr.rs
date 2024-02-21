use feo_ast::expression::{ArrayExpr, IndexExpr};

use feo_error::error::CompilerError;

use crate::{parse::ParseExpr, parser::Parser};

impl ParseExpr for ArrayExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for IndexExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
