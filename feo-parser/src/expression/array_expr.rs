use feo_ast::expression::ArrayExpr;
use feo_error::handler::ErrorEmitted;

use crate::parse::Parse;

impl Parse for ArrayExpr {
    fn parse(parser: &mut crate::parser::Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}
