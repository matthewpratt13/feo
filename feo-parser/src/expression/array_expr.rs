use feo_ast::expression::{ArrayExpr, IndexExpr};

use feo_error::handler::ErrorEmitted;

use crate::{parse::Parse, parser::Parser};

impl Parse for ArrayExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for IndexExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}
