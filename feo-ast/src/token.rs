use std::fmt;

use feo_types::{
    literal::{FloatType, IntType, Literal, UIntType},
    span::{Span, Spanned},
    Comment, Delimiter, DocComment, Identifier, Keyword, Punctuation, U256,
};

#[derive(Debug, Clone)]
pub enum Token {
    CharLit(Literal<char>),
    StringLit(Literal<String>),
    BoolLit(Literal<bool>),
    IntLit(Literal<IntType>),
    UIntLit(Literal<UIntType>),
    U256Lit(Literal<U256>),
    FloatLit(Literal<FloatType>),

    Iden(Identifier),
    Keyword(Keyword),

    Comment(Comment),
    DocComment(DocComment),

    Delim(Delimiter),
    Punc(Punctuation),

    EOF,
}

impl Spanned for Token {
    fn span(&self) -> Span {
        match self {
            Token::CharLit(c) => c.span(),
            Token::StringLit(s) => s.span(),
            Token::BoolLit(b) => b.span(),
            Token::IntLit(i) => i.span(),
            Token::UIntLit(ui) => ui.span(),
            Token::U256Lit(u) => u.span(),
            Token::FloatLit(f) => f.span(),
            Token::Iden(id) => id.span(),
            Token::Keyword(k) => k.span(),
            Token::Comment(c) => c.span(),
            Token::DocComment(dc) => dc.span(),
            Token::Delim(d) => d.span(),
            Token::Punc(p) => p.span(),
            Token::EOF => Span::default(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::CharLit(c) => write!(f, "{}", c),
            Token::StringLit(s) => {
                write!(f, "{}", s)
            }
            Token::BoolLit(b) => write!(f, "`{}`", b),
            Token::IntLit(i) => write!(f, "`{}`", i),
            Token::UIntLit(ui) => write!(f, "`{}`", ui),
            Token::U256Lit(u) => write!(f, "`{}`", u),
            Token::FloatLit(fl) => write!(f, "`{}`", fl),
            Token::Iden(id) => write!(f, "`{}`", id.name),
            Token::Keyword(k) => write!(f, "`{}`", k.keyword_kind.as_str()),
            Token::Comment(c) => write!(f, "`{}`", c.data),
            Token::DocComment(dc) => write!(f, "`{}`", dc.content),
            Token::Delim(d) => write!(f, "`{}`", d.clone().as_char()),
            Token::Punc(p) => write!(f, "`{}`", p.punc_kind.as_str()),
            Token::EOF => write!(f, "end of file"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenStream {
    tokens: Vec<Option<Token>>,
    span: Span,
}

impl TokenStream {
    pub fn new(src: &str, tokens: Vec<Option<Token>>, start: usize, end: usize) -> Self {
        Self {
            tokens,
            span: Span::new(src, start, end),
        }
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens
            .clone()
            .into_iter()
            .map(|t| t.expect("invalid token"))
            .collect::<Vec<Token>>()
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}

impl Spanned for TokenStream {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.tokens().into_iter().next() {
            Some(t)
        } else {
            None
        }
    }
}
