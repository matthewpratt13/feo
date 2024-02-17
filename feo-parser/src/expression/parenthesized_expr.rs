use feo_ast::expression::ParenthesizedExpr;
use feo_error::handler::ErrorEmitted;

use crate::{parse::Parse, parser::Parser};

impl Parse for ParenthesizedExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}
