use feo_ast::item::{AsClause, ExternCrateDecl};
use feo_error::error::CompilerError;

use crate::{
    parse::{ParseItem, ParseTerm},
    parser::Parser,
};

impl ParseTerm for AsClause {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseItem for ExternCrateDecl {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
