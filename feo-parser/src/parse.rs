use feo_error::handler::ErrorEmitted;

use crate::parser::Parser;

pub trait Parse {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized;
}
