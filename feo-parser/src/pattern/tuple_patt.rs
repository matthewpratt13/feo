use feo_ast::pattern::{TuplePatt, TuplePattElements};
use feo_error::error::CompilerError;

use crate::{
    parse::{ParsePatt, ParseTerm},
    parser::Parser,
};

impl ParseTerm for TuplePattElements {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParsePatt for TuplePatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
