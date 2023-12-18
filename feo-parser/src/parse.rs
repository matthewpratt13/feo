use std::fmt::Display;

use feo_error::lex_error::LexErrorKind;
use feo_types::Primitive;

use crate::lexer::Token;

pub trait Parse<T>
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, LexErrorKind>;
}

pub trait ParseVec<T>
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &Vec<T>,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, LexErrorKind>;
}
