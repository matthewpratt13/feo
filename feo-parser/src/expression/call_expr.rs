use feo_ast::expression::{FunctionCallExpr, MethodCallExpr};

use feo_error::handler::ErrorEmitted;

use crate::{parse::Parse, parser::Parser};

impl Parse for FunctionCallExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for MethodCallExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}