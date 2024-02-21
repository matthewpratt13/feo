use feo_ast::expression::ParenthesizedExpr;
use feo_error::handler::ErrorEmitted;

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for ParenthesizedExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}
