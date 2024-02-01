#![allow(dead_code)]

use feo_ast::token::{Token, TokenStream};
use feo_error::handler::Handler;

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

    fn next_token(&mut self) -> Token {
        if let Some(t) = self.stream.next() {
            self.pos += 1;
            t
        } else {
            Token::EOF
        }
    }

    fn peek_next(&self) -> Option<Token> {
        self.stream.tokens().get(self.pos + 1).cloned()?
    }
}
