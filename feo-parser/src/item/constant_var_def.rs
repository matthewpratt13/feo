use feo_ast::item::ConstantVarDef;
use feo_error::error::CompilerError;

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for ConstantVarDef {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
