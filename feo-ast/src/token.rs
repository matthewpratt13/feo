use feo_types::{
    comment::Comment,
    delimiter::Delimiter,
    doc_comment::DocComment,
    keyword::Keyword,
    literal::Literal,
    punctuation::Punctuation,
    span::{Span, Spanned},
    Identifier, U256,
};

#[derive(Debug, Clone)]
pub enum Token {
    CharLit(Literal<char>),
    StringLit(Literal<String>),
    BoolLit(Literal<bool>),
    IntLit(Literal<i64>),
    UIntLit(Literal<u64>),
    U256Lit(Literal<U256>),
    FloatLit(Literal<f64>),

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
