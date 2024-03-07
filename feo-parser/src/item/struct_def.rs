use feo_ast::item::{StructDefField, StructDefFields};
use feo_error::error::CompilerError;

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for StructDefField {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseTerm for StructDefFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
