#![allow(dead_code)]

use feo_ast::token::{Token, TokenStream};
use feo_error::{
    handler::Handler,
    parser_error::{ParserError, ParserErrorKind},
};
use feo_types::span::{Position, Spanned};

pub struct Parser {
    stream: TokenStream,
    pub pos: usize,
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

    pub fn current_token(&self) -> Option<Token> {
        self.stream.tokens().get(self.pos).cloned()?
    }

    pub fn next_token(&mut self) -> Result<Token, ParserError> {
        self.pos += 1;
        self.stream.next().ok_or(ParserError {
            error_kind: ParserErrorKind::TokenNotFound,
            position: Position::new(&self.stream.span().source(), self.stream.span().start()),
        })
    }

    pub fn peek_next(&self) -> Option<Token> {
        self.stream.tokens().get(self.pos + 1).cloned()?
    }
}
