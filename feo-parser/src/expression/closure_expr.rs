use feo_ast::expression::{ClosureWithBlock, ClosureWithoutBlock};
use feo_error::handler::ErrorEmitted;

use crate::{parse::Parse, parser::Parser};

impl Parse for ClosureWithBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for ClosureWithoutBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}
