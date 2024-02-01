use feo_ast::{
    literal::Literal,
    token::{Token, TokenStream, TokenType},
};
use feo_error::handler::Handler;
use feo_types::{punctuation::PuncKind, Delimiter, Identifier, Punctuation, U256};

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

    fn peek_int(&self) -> Option<Literal<i64>> {
        match self.stream.tokens().get(self.pos + 1)?.clone() {
            Some(t) if t.token_type() == TokenType::Int => Literal::<i64>::try_from(t).ok(),
            Some(_) | None => None,
        }
    }

    fn peek_uint(&self) -> Option<Literal<u64>> {
        match self.stream.tokens().get(self.pos + 1)?.clone() {
            Some(t) if t.token_type() == TokenType::UInt => Literal::<u64>::try_from(t).ok(),
            Some(_) | None => None,
        }
    }

    fn peek_u256(&self) -> Option<Literal<U256>> {
        match self.stream.tokens().get(self.pos + 1)?.clone() {
            Some(t) if t.token_type() == TokenType::U256 => Literal::<U256>::try_from(t).ok(),
            Some(_) | None => None,
        }
    }

    fn peek_float(&self) -> Option<Literal<f64>> {
        match self.stream.tokens().get(self.pos + 1)?.clone() {
            Some(t) if t.token_type() == TokenType::U256 => Literal::<f64>::try_from(t).ok(),
            Some(_) | None => None,
        }
    }

    fn peek_bool(&self) -> Option<Literal<bool>> {
        match self.stream.tokens().get(self.pos + 1)?.clone() {
            Some(t) if t.token_type() == TokenType::Bool => Literal::<bool>::try_from(t).ok(),
            Some(_) | None => None,
        }
    }

    fn peek_iden(&self) -> Option<Identifier> {
        match self.stream.tokens().get(self.pos + 1)?.clone() {
            Some(t) if t.token_type() == TokenType::Identifier => Identifier::try_from(t).ok(),
            Some(_) | None => None,
        }
    }

    fn peek_punc_kind(&self) -> Option<PuncKind> {
        match self.stream.tokens().get(self.pos + 1)?.clone() {
            Some(t) if t.token_type() == TokenType::Punctuation => PuncKind::try_from(t).ok(),
            Some(_) | None => None,
        }
    }

    fn peek_delim(&self) -> Option<Delimiter> {
        match self.stream.tokens().get(self.pos + 1)?.clone() {
            Some(t) if t.token_type() == TokenType::Identifier => Delimiter::try_from(t).ok(),
            Some(_) | None => None,
        }
    }
}

// pub struct Peeker<'a> {
//     pub tokens: &'a [Option<Token>],
// }
