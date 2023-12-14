use crate::lexer::Lexer;

pub trait Tokenizer {}

impl Tokenizer for Lexer<'_> {}


pub trait Tokenize<T> {
    fn tokenize(l: &mut Lexer) -> T {};
}