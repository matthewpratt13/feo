use std::fmt::Display;

use feo_types::Primitive;

use crate::lexer::Token;

pub trait Parse<T>
where
    T: 'static + Primitive + Display,
{
    fn parse(src: &str, content: &T, start: usize, end: usize) -> Result<Option<Token>, ()>;
}
