#![allow(dead_code)]

use feo_ast::token::{Token, TokenStream};
use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    parser_error::{ParserError, ParserErrorKind},
};
use feo_types::span::{Position, Spanned};

#[derive(Default)]
pub enum TokenType {
    Literal(Token),
    Identifier(Token),
    Keyword(Token),
    Delimiter(Token),
    Punctuation(Token),
    EOF(Token),

    #[default]
    InvalidToken,
}

impl TokenType {
    fn into_inner(self) -> Option<Token> {
        match self {
            TokenType::Literal(l) => Some(l),
            TokenType::Identifier(i) => Some(i),
            TokenType::Keyword(k) => Some(k),
            TokenType::Delimiter(d) => Some(d),
            TokenType::Punctuation(p) => Some(p),
            TokenType::InvalidToken => None,
            TokenType::EOF(eof) => Some(eof),
        }
    }
}

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

    pub fn peek_next(&self) -> Token {
        self.stream
            .tokens()
            .get(self.pos + 1)
            .cloned()
            .unwrap_or(Token::EOF)
    }

    fn log_error(&self, error_kind: ParserErrorKind) -> ErrorEmitted {
        let err = ParserError {
            error_kind,
            position: Position::new(&self.stream.span().source(), self.pos),
        };

        self.handler.emit_err(CompilerError::Parser(err))
    }
}
