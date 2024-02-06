use feo_error::parser_error::ParserError;

use crate::parser::Parser;

pub trait Parse {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ParserError>
    where
        Self: Sized;
}
