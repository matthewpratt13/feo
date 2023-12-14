use std::sync::Arc;

use feo_types::span::{Span, Spanned};
use feo_types::{
    Comment, Delimiter, DocComment, Identifier, Keyword, PathExpression, Punctuation,
    TypeAnnotation,
};

use crate::literals::{
    BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral,
};

use super::LexError;

// token type
#[derive(Debug)]
pub enum Token<T> {
    // literals
    CharLit(CharLiteral),
    StringLit(StringLiteral),
    BoolLit(BoolLiteral),
    IntLit(IntLiteral),
    UIntLit(UIntLiteral),
    FloatLit(FloatLiteral),

    // identifiers and keywords
    Iden(Identifier<T>),
    Keyword(Keyword),

    Comment(Comment<T>),
    DocComment(DocComment<T>),

    // path expression, e.g. crate::module::Struct
    // `Token::Path(vec!["crate".to_string(), "module".to_string(), "Struct".to_string()])`
    Path(PathExpression<T>),

    Delim(Delimiter),
    Punc(Punctuation),

    // type annotation
    Type(TypeAnnotation),

    // other
    NewLine, // for debugging purposes
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
    ) -> Result<Self, LexError> {
        Ok(Self {
            tokens,
            span: Span::build(Arc::new(src.to_string()), start, end)?,
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

pub struct TokenTree<T>(TokenStream<Token<T>>);

impl<T> TokenTree<T> {
    pub fn build(
        src: &str,
        tokens: Vec<Option<Token<T>>>,
        start: usize,
        end: usize,
    ) -> Result<Self, LexError> {
        Ok(Self(TokenStream::build(src, tokens, start, end)?))
    }
}
