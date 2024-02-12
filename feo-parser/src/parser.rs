#![allow(dead_code)]

use feo_ast::token::{Token, TokenStream};
use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    parser_error::{ParserError, ParserErrorKind},
};
use feo_types::{
    literal::{FloatType, IntType, Literal, UIntType},
    span::{Position, Spanned},
    Delimiter, DocComment, Identifier, Keyword, Punctuation, U256,
};

use crate::parse::Peek;

pub struct Parser {
    stream: TokenStream,
    pos: usize,
    handler: Handler,
}

impl Parser {
    pub fn new(stream: TokenStream, handler: Handler) -> Self {
        Parser {
            stream,
            pos: 0,
            handler,
        }
    }

    pub fn stream(&self) -> &TokenStream {
        &self.stream
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.pos += 1;
        self.stream.next()
    }

    pub fn current_token(&self) -> Token {
        self.stream
            .tokens()
            .get(self.pos)
            .cloned()
            .expect("token not found")
    }

    pub fn log_error(&self, error_kind: ParserErrorKind) -> ErrorEmitted {
        let err = ParserError {
            error_kind,
            position: Position::new(&self.stream.span().source(), self.pos),
        };

        self.handler.emit_err(CompilerError::Parser(err))
    }

    // peek at the next `Token` and return it if it exists (without advancing)
    pub fn peek<T: Peek>(&mut self) -> Option<T> {
        Peeker::with(&self.stream().tokens()).map(|(v, _)| v)
    }

    // peek at the next `Token`, advance the `Parser` and return the peeked `Token` if it exists
    pub fn take<T: Peek>(&mut self) -> Option<T> {
        let (value, _) = Peeker::with(&self.stream().tokens())?;
        self.advance();
        Some(value)
    }
}

// type that allows for peeking at the next `Token` in a `&[Token]` without advancing
pub struct Peeker<'a>(pub &'a [Token]);

impl<'a> Peeker<'a> {
    // peek for a `T` in `&[Token]'; return `T` if it exists and the remaining `&[Token]`
    pub fn with<T: Peek>(tokens: &'a [Token]) -> Option<(T, &'a [Token])> {
        let peeker = Peeker(tokens);
        let value = T::peek(peeker)?;

        Some((value, tokens))
    }

    // peek for a `Literal` or return `Self` so that the `Peeker` can try again without advancing
    pub fn peek_char_lit(self) -> Result<&'a Literal<char>, Self> {
        match self.0 {
            [Token::CharLit(c), ..] => Ok(c),
            _ => Err(self),
        }
    }

    pub fn peek_string_lit(self) -> Result<&'a Literal<String>, Self> {
        match self.0 {
            [Token::StringLit(s), ..] => Ok(s),
            _ => Err(self),
        }
    }

    pub fn peek_bool_lit(self) -> Result<&'a Literal<bool>, Self> {
        match self.0 {
            [Token::BoolLit(b), ..] => Ok(b),
            _ => Err(self),
        }
    }

    pub fn peek_int_lit(self) -> Result<&'a Literal<IntType>, Self> {
        match self.0 {
            [Token::IntLit(i), ..] => Ok(i),
            _ => Err(self),
        }
    }

    pub fn peek_uint_lit(self) -> Result<&'a Literal<UIntType>, Self> {
        match self.0 {
            [Token::UIntLit(ui), ..] => Ok(ui),
            _ => Err(self),
        }
    }

    pub fn peek_u256_lit(self) -> Result<&'a Literal<U256>, Self> {
        match self.0 {
            [Token::U256Lit(u), ..] => Ok(u),
            _ => Err(self),
        }
    }

    pub fn peek_float_lit(self) -> Result<&'a Literal<FloatType>, Self> {
        match self.0 {
            [Token::FloatLit(f), ..] => Ok(f),
            _ => Err(self),
        }
    }

    pub fn peek_identifier(self) -> Result<&'a Identifier, Self> {
        match self.0 {
            [Token::Iden(id)] => Ok(id),
            _ => Err(self),
        }
    }

    pub fn peek_keyword(self) -> Result<&'a Keyword, Self> {
        match self.0 {
            [Token::Keyword(k), ..] => Ok(k),
            _ => Err(self),
        }
    }

    pub fn peek_doc_comment(self) -> Result<&'a DocComment, Self> {
        match self.0 {
            [Token::DocComment(dc), ..] => Ok(dc),
            _ => Err(self),
        }
    }

    pub fn peek_delimiter(self) -> Result<&'a Delimiter, Self> {
        match self.0 {
            [Token::Delim(d), ..] => Ok(d),
            _ => Err(self),
        }
    }

    pub fn peek_punctuation(self) -> Result<&'a Punctuation, Self> {
        match self.0 {
            [Token::Punc(p), ..] => Ok(p),
            _ => Err(self),
        }
    }
}

impl Peek for Literal<char> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_char_lit() {
            Ok(c) => Some(c.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<String> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_string_lit() {
            Ok(s) => Some(s.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<bool> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_bool_lit() {
            Ok(b) => Some(b.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<IntType> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_int_lit() {
            Ok(i) => Some(i.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<UIntType> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_uint_lit() {
            Ok(ui) => Some(ui.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<U256> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_u256_lit() {
            Ok(u) => Some(u.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Literal<FloatType> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_float_lit() {
            Ok(f) => Some(f.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Identifier {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_identifier() {
            Ok(id) => Some(id.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Keyword {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_keyword() {
            Ok(k) => Some(k.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for DocComment {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_doc_comment() {
            Ok(d) => Some(d.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Delimiter {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_delimiter() {
            Ok(d) => Some(d.clone()),
            Err(_) => None,
        }
    }
}

impl Peek for Punctuation {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_punctuation() {
            Ok(p) => Some(p.clone()),
            Err(_) => None,
        }
    }
}
