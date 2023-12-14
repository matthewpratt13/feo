use feo_types::Primitive;

use crate::lexer::{Lexer, Token};

// use for delimiters, char + string + bool literals, punctuation
pub trait Parse {
    fn parse(l: &mut Lexer) -> Option<Token>;
}

// use for digits
pub trait ParseDigit {
    fn parse(
        l: &mut Lexer,
        input: char,
        i: usize,
        is_negative_number: bool,
        is_hexadecimal_int: bool,
    ) -> Option<Token>;
}

// use for comments, doc comments, keywords, identifiers, path expressions, type annotations
pub trait ParseData<T>
where
    T: 'static + Primitive,
{
    fn parse(src: &str, input: T, start: usize, end: usize) -> Option<Token>;
}
