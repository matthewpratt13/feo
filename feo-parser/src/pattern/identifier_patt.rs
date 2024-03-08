use feo_ast::pattern::IdentifierPatt;
use feo_error::error::CompilerError;

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for IdentifierPatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
