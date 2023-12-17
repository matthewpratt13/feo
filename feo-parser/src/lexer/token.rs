use feo_types::span::{Span, Spanned};
use feo_types::{
    Comment, Delimiter, DocComment, Identifier, Keyword, PathExpression, Punctuation,
    TypeAnnotation,
};

use crate::error::ParserError;
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
    Path(PathExpression),

    Delim(Delimiter),
    Punc(Punctuation),

    // type annotation
    Type(TypeAnnotation),
}

#[allow(dead_code)]
pub struct TokenStream<T> {
    tokens: Vec<Option<T>>,
    span: Span,
}

#[allow(dead_code)]
impl<T> TokenStream<T> {
    pub fn build(
        src: &str,
        tokens: Vec<Option<T>>,
        start: usize,
        end: usize,
    ) -> Result<Self, ParserError> {
        Ok(Self {
            tokens,
            span: Span::new(src, start, end),
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
    ) -> Result<Option<Self>, ParserError> {
        Ok(Some(Self(TokenStream::build(src, tokens, start, end)?)))
    }
}
