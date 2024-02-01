use feo_ast::token::{Token, TokenStream, TokenType};
use feo_error::handler::Handler;
use feo_types::punctuation::PuncKind;

pub struct Parser {
    stream: TokenStream,
    pos: usize,
    handler: Handler,
}

// pub trait Peek {
//     fn peek<P: Peek>(&self) -> Option<P>
//     where
//         Self: Sized;
// }

impl Parser {
    pub fn new(stream: TokenStream, pos: usize, handler: Handler) -> Self {
        Parser {
            stream,
            pos,
            handler,
        }
    }

    fn advance(&mut self) -> Option<Token> {
        if self.pos < self.stream.len() - 1 {
            self.pos += 1;
        }

        self.stream.next()
    }

    fn current(&self) -> Option<Token> {
        self.stream.tokens().get(self.pos)?.clone()
    }

    fn peek_punctuation(&self) -> Option<PuncKind> {
        match self.stream.tokens().get(self.pos + 1)?.clone() {
            Some(t) if t.token_type() == TokenType::Punctuation => Some(t.punc_kind),
            Some(_) | None => None,
        }
    }
}

// pub struct Peeker<'a> {
//     pub tokens: &'a [Option<Token>],
// }
