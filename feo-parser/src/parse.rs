use feo_error::handler::ErrorEmitted;

use crate::parser::{Parser, Peeker};

pub trait Parse {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized;
}

pub trait Peek {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized;
}
