#![allow(dead_code)]
#![allow(unused_variables)]

use feo_error::handler::{ErrorEmitted, Handler};
use feo_types::span::{Span, Spanned};

use crate::{
    comment::Comment,
    delimiter::Delimiter,
    doc_comment::DocComment,
    identifier::Identifier,
    keyword::Keyword,
    literals::{
        BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, U256Literal, UIntLiteral,
    },
    punctuation::Punctuation,
    type_annotation::TypeAnnotation,
};

pub trait Tokenize {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted>;
}

// token type
#[derive(Debug, Clone)]
pub enum Token {
    CharLit(CharLiteral),
    StringLit(StringLiteral),
    BoolLit(BoolLiteral),
    IntLit(IntLiteral),
    UIntLit(UIntLiteral),
    U256Lit(U256Literal),
    FloatLit(FloatLiteral),

    Iden(Identifier),
    Keyword(Keyword),
    TypeAnn(TypeAnnotation),

    Comment(Comment),
    DocComment(DocComment),

    Delim(Delimiter),
    Punc(Punctuation),
}

impl Spanned for Token {
    fn span(&self) -> &Span {
        match self {
            Token::CharLit(c) => c.span(),
            Token::StringLit(s) => s.span(),
            Token::BoolLit(b) => b.span(),
            Token::IntLit(i) => i.span(),
            Token::UIntLit(ui) => ui.span(),
            Token::U256Lit(u) => u.span(),
            Token::FloatLit(f) => f.span(),
            Token::Iden(i) => i.span(),
            Token::Keyword(k) => k.span(),
            Token::TypeAnn(ta) => ta.span(),
            Token::Comment(c) => c.span(),
            Token::DocComment(dc) => dc.span(),
            Token::Delim(d) => d.span(),
            Token::Punc(p) => p.span(),
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
