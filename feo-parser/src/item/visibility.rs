use feo_ast::item::{PubCrateVisibility, VisibilityKind};
use feo_error::error::CompilerError;

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for VisibilityKind {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseTerm for PubCrateVisibility {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
