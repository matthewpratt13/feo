use std::iter::Iterator;

use feo_error::ParserErrorKind;

use crate::lexer::{Lexer, Token};

pub trait Parse {
    fn parse(src: &mut Lexer) -> Result<Option<Token>, ParserErrorKind>;
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
        is_hexadecimal_int: bool,
    ) -> Result<Option<Token>, ParserErrorKind>;
}
