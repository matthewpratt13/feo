use feo_types::span::{Span, Spanned};
use feo_types::{
    Comment, Delimiter, DocComment, Identifier, Keyword, PathExpression, Punctuation,
    TypeAnnotation,
};

use crate::error::{LexError, ParserError};
use crate::literals::{
    BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral,
};

// token type
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
            span: Span::new(src, start, end),
        })
    }

    pub fn tokens(&self) -> &[Option<T>] {
        self.tokens.as_slice()
    }
}

impl<T> Spanned for TokenStream<T> {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug, Clone)]
pub struct TokenTree(TokenStream<Token>);

impl TokenTree {
    pub fn build(
        src: &str,
        tokens: Vec<Option<Token>>,
        start: usize,
        end: usize,
    ) -> Result<Option<Self>, LexError> {
        Ok(Some(Self(TokenStream::build(src, tokens, start, end)?)))
    }

    pub fn tokens(&self) -> &[Option<Token>] {
        self.0.tokens.as_slice()
    }
}
