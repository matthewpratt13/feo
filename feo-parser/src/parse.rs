use crate::lexer::{Lexer, Token};

// use for delimiters, keywords, literals, punctuation, type annotations
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

// use for comments, doc comments, identifiers, path expressions
pub trait ParseData<T>
where
    T: 'static + Primitive,
{
    fn parse(src: &str, input: T, start: usize, end: usize) -> Option<Token>;
}
