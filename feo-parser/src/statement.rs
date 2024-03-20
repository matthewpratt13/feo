use feo_ast::statement::{ExprStatement, LetStatement};
use feo_error::error::CompilerError;

use crate::{parse::ParseStatement, parser::Parser};

impl ParseStatement for ExprStatement {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseStatement for LetStatement {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
