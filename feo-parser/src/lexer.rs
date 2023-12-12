use std::iter::Peekable;
use std::sync::Arc;

use feo_error::LexError;

mod token;
pub use self::token::{Token, TokenStream, TokenTree};

pub struct Lexer<'a> {
    src: Arc<&'a str>,
    pos: usize,
}

type CharReader<'a> = Peekable<Lexer<'a>>;

impl Iterator for Lexer<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src: Arc::new(src),
            pos: 0,
        }
    }

    fn tokenize(&mut self) -> Result<TokenStream<TokenTree>, LexError> {
        todo!()
    }
}
