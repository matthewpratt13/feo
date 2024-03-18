use feo_ast::item::TypeAliasDef;
use feo_error::error::CompilerError;

use crate::{parse::ParseItem, parser::Parser};

impl ParseItem for TypeAliasDef {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
