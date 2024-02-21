use feo_ast::expression::{FunctionCallExpr, MethodCallExpr};

use feo_error::handler::ErrorEmitted;

use crate::{parse::ParseExpr, parser::Parser};

impl ParseExpr for FunctionCallExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for MethodCallExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}
