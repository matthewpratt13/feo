use feo_error::ParserError;
use feo_types::Primitive;

use crate::lexer::Token;

pub trait Parse<P, I>
where
    P: 'static + Primitive,
    I: std::iter::Iterator,
{
    fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserError>;
}

pub trait ParseDigit<P, I>
where
    P: 'static + Primitive,
    I: std::iter::Iterator,
{
    fn parse(
        src: &mut I,
        input: char,
        i: usize,
        is_negative_number: bool,
    ) -> Result<Option<Token>, ParserError>;
}
