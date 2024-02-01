use crate::parser::Parser;

pub trait Parse {
    fn parse(parser: &mut Parser) -> Self
    where
        Self: Sized;
}
