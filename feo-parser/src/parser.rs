#![allow(dead_code)]

use feo_ast::{
    literal::{Literal, LiteralKind},
    token::{Token, TokenStream},
};
use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    parser_error::{ParserError, ParserErrorKind},
};
use feo_types::{
    span::{Position, Spanned},
    Delimiter, DocComment, Identifier, Keyword, Punctuation, U256,
};

use crate::parse::Peek;

// TODO: remove this code and implementations after adjusting `Expression::parse()`
#[derive(Default)]
pub enum TokenType {
    Literal(Token),
    Identifier(Token),
    Keyword(Token),
    DocComment(Token),
    Delimiter(Token),
    Punctuation(Token),
    EOF(Token),

    #[default]
    InvalidToken,
}

// impl TokenType {
//     fn into_inner(self) -> Option<Token> {
//         match self {
//             TokenType::Literal(l) => Some(l),
//             TokenType::Identifier(i) => Some(i),
//             TokenType::Keyword(k) => Some(k),
//             TokenType::Delimiter(d) => Some(d),
//             TokenType::DocComment(dc) => Some(dc),
//             TokenType::Punctuation(p) => Some(p),
//             TokenType::InvalidToken => None,
//             TokenType::EOF(eof) => Some(eof),
//         }
//     }
// }

impl From<Token> for TokenType {
    fn from(value: Token) -> Self {
        match value {
            Token::CharLit(c) => TokenType::Literal(Token::CharLit(c)),
            Token::StringLit(s) => TokenType::Literal(Token::StringLit(s)),
            Token::BoolLit(b) => TokenType::Literal(Token::BoolLit(b)),
            Token::IntLit(i) => TokenType::Literal(Token::IntLit(i)),
            Token::UIntLit(ui) => TokenType::Literal(Token::UIntLit(ui)),
            Token::U256Lit(u) => TokenType::Literal(Token::U256Lit(u)),
            Token::FloatLit(f) => TokenType::Literal(Token::FloatLit(f)),
            Token::Iden(i) => TokenType::Identifier(Token::Iden(i)),
            Token::Keyword(k) => TokenType::Identifier(Token::Keyword(k)),
            Token::Comment(_) => TokenType::InvalidToken,
            Token::DocComment(_) => TokenType::InvalidToken,
            Token::Delim(d) => TokenType::Delimiter(Token::Delim(d)),
            Token::Punc(p) => TokenType::Punctuation(Token::Punc(p)),
            Token::EOF => TokenType::EOF(Token::EOF),
        }
    }
}

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

    pub fn stream(&self) -> TokenStream {
        self.stream.clone()
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

    fn log_error(&self, error_kind: ParserErrorKind) -> ErrorEmitted {
        let err = ParserError {
            error_kind,
            position: Position::new(&self.stream.span().source(), self.pos),
        };

        self.handler.emit_err(CompilerError::Parser(err))
    }

    pub fn peek<T: Peek>(&mut self) -> Result<Option<T>, ErrorEmitted> {
        Peeker::with(self.stream().tokens().as_mut_slice()).map(|(v, _)| v)
    }

    pub fn take<T: Peek>(&mut self) -> Result<Option<T>, ErrorEmitted> {
        let (value, _) = Peeker::with(self.stream().tokens().as_mut_slice())?;
        self.advance();
        Ok(value)
    }
}

pub struct Peeker<'a>(pub &'a mut [Token]);

impl<'a> Peeker<'a> {
    pub fn with<T: Peek>(
        tokens: &'a mut [Token],
    ) -> Result<(Option<T>, &'a [Token]), ErrorEmitted> {
        let peeker = Peeker(tokens);
        let value = T::peek(peeker);

        Ok((value, tokens))
    }

    pub fn peek_literal(self) -> Result<LiteralKind, Self> {
        match self.0 {
            [Token::CharLit(c), ..] => Ok(LiteralKind::Char(c.clone())),
            [Token::StringLit(s), ..] => Ok(LiteralKind::String(s.clone())),
            [Token::BoolLit(b), ..] => Ok(LiteralKind::Bool(b.clone())),
            [Token::IntLit(i), ..] => Ok(LiteralKind::I64(i.clone())),
            [Token::UIntLit(ui), ..] => Ok(LiteralKind::U64(ui.clone())),
            [Token::U256Lit(u), ..] => Ok(LiteralKind::U256(u.clone())),
            _ => Err(self),
        }
    }

    pub fn peek_identifier(self) -> Result<Identifier, Self> {
        match self.0 {
            [Token::Iden(id)] => Ok(id.clone()),
            _ => Err(self),
        }
    }

    pub fn peek_keyword(self) -> Result<Keyword, Self> {
        match self.0 {
            [Token::Keyword(k), ..] => Ok(k.clone()),
            _ => Err(self),
        }
    }

    pub fn peek_doc_comment(self) -> Result<DocComment, Self> {
        match self.0 {
            [Token::DocComment(dc), ..] => Ok(dc.clone()),
            _ => Err(self),
        }
    }

    pub fn peek_delimiter(self) -> Result<Delimiter, Self> {
        match self.0 {
            [Token::Delim(d), ..] => Ok(d.clone()),
            _ => Err(self),
        }
    }

    pub fn peek_punctuation(self) -> Result<Punctuation, Self> {
        match self.0 {
            [Token::Punc(p), ..] => Ok(p.clone()),
            _ => Err(self),
        }
    }
}

impl Peek for Literal<char> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::Char(c) => Some(c),
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}

// impl<T: PrimitiveType> Peek for Literal<T> {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_literal() {
//             Ok(l) => match l {
//                 LiteralKind::Char(c) => Some(c),
//                 LiteralKind::String(s) => Some(s),
//                 LiteralKind::Bool(b) => Some(b),
//                 LiteralKind::I32(i) => Some(i),
//                 LiteralKind::I64(i) => Some(i),
//                 LiteralKind::U8(ui) => Some(ui),
//                 LiteralKind::U16(ui) => Some(ui),
//                 LiteralKind::U32(ui) => Some(ui),
//                 LiteralKind::U64(ui) => Some(ui),
//                 LiteralKind::U256(u) => Some(u),
//                 LiteralKind::F32(f) => Some(f),
//                 LiteralKind::F64(f) => Some(f),
//                 _ => todo!(),
//             },
//             Err(_) => todo!(),
//         }
//     }
// }

impl Peek for Literal<String> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::String(s) => Some(s),
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}

impl Peek for Literal<bool> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::Bool(b) => Some(b),
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}

impl Peek for Literal<i32> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::I32(i) => Some(i),
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}

impl Peek for Literal<i64> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::I64(i) => Some(i),
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}

impl Peek for Literal<u8> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::U8(ui) => Some(ui),
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}

impl Peek for Literal<u16> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::U16(ui) => Some(ui),
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}

impl Peek for Literal<u32> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::U32(ui) => Some(ui),
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}

impl Peek for Literal<u64> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::U64(ui) => Some(ui),
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}

impl Peek for Literal<U256> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::U256(u) => Some(u),
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}

impl Peek for Literal<f32> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::F32(f) => Some(f),
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}

impl Peek for Literal<f64> {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_literal() {
            Ok(l) => match l {
                LiteralKind::F64(f) => Some(f),
                _ => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}

impl Peek for Identifier {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_identifier() {
            Ok(id) => Some(id),
            Err(_) => todo!(),
        }
    }
}

impl Peek for Keyword {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_keyword() {
            Ok(k) => Some(k),
            Err(_) => todo!(),
        }
    }
}

impl Peek for DocComment {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_doc_comment() {
            Ok(d) => Some(d),
            Err(_) => todo!(),
        }
    }
}

impl Peek for Delimiter {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_delimiter() {
            Ok(d) => Some(d),
            Err(_) => todo!(),
        }
    }
}

impl Peek for Punctuation {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        match peeker.peek_punctuation() {
            Ok(p) => Some(p),
            Err(_) => todo!(),
        }
    }
}
