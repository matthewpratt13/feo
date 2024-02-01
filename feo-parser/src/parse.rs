use crate::parser::Parser;

pub trait Parse {
    fn parse(parser: &mut Parser) -> Option<Self>
    where
        Self: Sized;
}
