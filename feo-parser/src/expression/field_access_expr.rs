use feo_ast::expression::FieldAccessExpr;
use feo_error::handler::ErrorEmitted;

use crate::{parse::ParseExpr, parser::Parser};

impl ParseExpr for FieldAccessExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}
