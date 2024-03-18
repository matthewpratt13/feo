use feo_ast::item::{TraitDef, TraitDefItem};
use feo_error::error::CompilerError;

use crate::{parse::ParseItem, parser::Parser};

impl ParseItem for TraitDefItem {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseItem for TraitDef {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
