use std::sync::Arc;

use feo_error::LexErrorKind;
use feo_types::{Comment, Delimiter, DocComment, Identifier, Keyword, Punctuation, Span, Spanned};

use crate::literals::{
    BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, U256Literal, UIntLiteral,
};

pub enum Token {
    CharLit(CharLiteral),
    StringLit(StringLiteral),
    IntLit(IntLiteral),
    UIntLit(UIntLiteral),
    U256Lit(U256Literal),
    FloatLit(FloatLiteral),
    BoolLit(BoolLiteral),

    Comment(Comment),
    DocCommet(DocComment),
    Delim(Delimiter),
    Iden(Identifier),
    Keyword(Keyword),
    Punc(Punctuation),

    EOF, // end of file
}

pub struct TokenStream<T> {
    tokens: Vec<Option<T>>,
    span: Span,
}

impl<T> TokenStream<T> {
    pub fn build(
        src: &str,
        tokens: Vec<Option<T>>,
        start: usize,
        end: usize,
    ) -> Result<Self, LexErrorKind> {
        Ok(Self {
            tokens,
            span: Span::build(src, start, end)?,
        })
    }

    pub fn tokens(&self) -> &Vec<Option<T>> {
        &self.tokens
    }
}

impl<T> Spanned for TokenStream<T> {
    fn span(&self) -> &Span {
        &self.span
    }
}

pub struct TokenTree(TokenStream<Token>);

impl TokenTree {
    pub fn build(
        src: &str,
        tokens: Vec<Option<Token>>,
        start: usize,
        end: usize,
    ) -> Result<Self, LexErrorKind> {
        Ok(Self(TokenStream::build(src, tokens, start, end)?))
    }
}
