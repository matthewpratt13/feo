#![allow(dead_code)]

use feo_ast::token::{Token, TokenStream};
use feo_error::{
    handler::Handler,
    parser_error::{ParserError, ParserErrorKind},
};
use feo_types::span::{Position, Spanned};

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

    pub fn advance(&mut self) -> Option<Token> {
        self.pos += 1;
        self.stream.next()
    }

    pub fn current_token(&self) -> Result<Token, ParserError> {
        self.stream
            .tokens()
            .get(self.pos)
            .cloned()
            .ok_or(ParserError {
                error_kind: ParserErrorKind::TokenNotFound,
                position: Position::new(&self.stream.span().source(), self.pos),
            })
    }

    pub fn peek_next(&self) -> Token {
        self.stream
            .tokens()
            .get(self.pos + 1)
            .cloned()
            .unwrap_or(Token::EOF)
    }
}
