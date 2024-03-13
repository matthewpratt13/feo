use feo_ast::ty::{ClosureType, FunctionType};
use feo_error::error::CompilerError;

use crate::{parse::ParseType, parser::Parser};

impl ParseType for FunctionType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseType for ClosureType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
