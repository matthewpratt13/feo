use std::iter::Iterator;

use feo_error::ParserErrorKind;

use crate::lexer::Token;

pub trait Parse<I>
where
    I: Iterator,
{
    fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind>;
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
        is_hexadecimal_int: bool
    ) -> Result<Option<Token>, ParserErrorKind>;
}
