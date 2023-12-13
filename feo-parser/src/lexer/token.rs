use std::sync::Arc;

use feo_error::LexErrorKind;
use feo_types::{Comment, Delimiter, Identifier, Keyword, Punctuation, Span, Spanned};

use crate::literals::{
    BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral,
};

// token type
#[derive(Debug)]
pub enum Token {
    // literals
    CharLit(CharLiteral),
    StringLit(StringLiteral),
    BoolLit(BoolLiteral),
    IntLit(IntLiteral),
    UIntLit(UIntLiteral),
    FloatLit(FloatLiteral),

    // identifiers and keywords
    Iden(Identifier),
    Keyword(Keyword),

    Comment(Comment),
    DocComment(DocComment),

    // path expression, e.g. crate::module::Struct
    // `Token::Path(vec!["crate".to_string(), "module".to_string(), "Struct".to_string()])`
    Path(Vec<String>), // TODO: make a separate type

    Delim(Delimiter),
    Punc(Punctuation),

    // type annotation
    Type(String), // TODO: make a separate type

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
    ) -> Result<Self, LexErrorKind> {
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
