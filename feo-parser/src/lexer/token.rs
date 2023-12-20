use feo_types::span::{Span, Spanned};
use feo_types::{Comment, Delimiter, DocComment, Identifier, Keyword, Punctuation, TypeAnnotation};

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

    Delim(Delimiter),
    Punc(Punctuation),

    // type annotation
    Type(TypeAnnotation),
}

impl Spanned for Token {
    fn span(&self) -> &Span {
        match self {
            Token::CharLit(c) => c.span(),
            Token::StringLit(s) => s.span(),
            Token::BoolLit(b) => b.span(),
            Token::IntLit(i) => i.span(),
            Token::UIntLit(u) => u.span(),
            Token::FloatLit(f) => f.span(),
            Token::Iden(i) => i.span(),
            Token::Keyword(k) => k.span(),
            Token::Comment(c) => c.span(),
            Token::DocComment(dc) => dc.span(),
            Token::Delim(d) => d.span(),
            Token::Punc(p) => p.span(),
            Token::Type(t) => t.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenStream<T> {
    tokens: Vec<Option<T>>,
    span: Span,
}

impl<T> TokenStream<T> {
    pub fn new(src: &str, tokens: Vec<Option<T>>, start: usize, end: usize) -> Self {
        Self {
            tokens,
            span: Span::new(src, start, end),
        }
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
    pub fn new(src: &str, tokens: Vec<Option<Token>>, start: usize, end: usize) -> Self {
        Self(TokenStream::new(src, tokens, start, end))
    }

    pub fn tokens(&self) -> &[Option<Token>] {
        self.0.tokens.as_slice()
    }
}
