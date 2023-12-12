use std::iter::Iterator;

use feo_error::ParserError;

use crate::lexer::Token;

pub trait Parse<I>
where
    I: Iterator,
{
    fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserError>;
}

pub trait ParseDigit<I>
where
    I: Iterator,
{
    fn parse(
        src: &mut I,
        input: char,
        i: usize,
        is_negative_number: bool,
    ) -> Result<Option<Token>, ParserError>;
}
