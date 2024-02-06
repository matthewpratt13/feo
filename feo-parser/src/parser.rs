#![allow(dead_code)]

use feo_ast::{
    literal::Literal,
    token::{Token, TokenStream},
};
use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    parser_error::{ParserError, ParserErrorKind},
};
use feo_types::{
    primitive::PrimitiveType,
    span::{Position, Spanned},
    Delimiter, Identifier, Keyword, Punctuation,
};

pub enum TokType<T>
where
    T: PrimitiveType + Clone,
{
    Literal(Literal<T>),
    Identifier(Identifier),
    Keyword(Keyword),
    Punctuation(Punctuation),
    Delimiter(Delimiter),
}

pub struct Tok<T: PrimitiveType + Clone> {
    token_type: TokType<T>,
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
