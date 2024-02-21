use feo_ast::expression::{FunctionCallExpr, MethodCallExpr};

use feo_error::error::CompilerError;

use crate::{parse::ParseExpr, parser::Parser};

impl ParseExpr for FunctionCallExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for MethodCallExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
