use feo_ast::{
    expression::Expression,
    token::{Token, TokenStream},
};
use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    parser_error::{ParserError, ParserErrorKind},
};
use feo_types::span::{Position, Spanned};

use crate::{
    peek::{Peek, Peeker},
    precedence::Precedence,
};

/// Struct that stores a token stream and the current character index, and handles errors.
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

    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        todo!()
    }

    fn parse_prefix(&mut self) -> Option<Expression> {
        todo!()
    }

    fn parse_infix(&mut self, infix: Token, left: Expression) -> Option<Expression> {
        todo!()
    }

    fn peek_precedence(&mut self) -> Precedence {
        todo!()
    }

    /// Return the current token.
    pub fn current_token(&self) -> Option<Token> {
        self.stream.tokens().get(self.pos).cloned()
    }

    /// Advance the parser and return the current token.
    pub fn next_token(&mut self) -> Option<Token> {
        let token = self.current_token();
        if token.is_some() {
            self.pos += 1;
        }

        token
    }

    pub fn peek_num_tokens_ahead(&self, num_tokens: usize) -> Option<Token> {
        self.stream.tokens().get(self.pos + num_tokens).cloned()
    }

    /// Return the previous token.
    pub fn previous_token(&mut self) -> Option<Token> {
        if self.pos > 0 {
            self.stream.tokens().get(self.pos - 1).cloned()
        } else {
            None
        }
    }

    /// Peek at the current `T` and return it if it exists (without advancing) or return `None`.
    pub fn peek_current<T: Peek>(&self) -> Option<T> {
        Peeker::with(&self.stream().tokens(), self.pos)
    }

    /// Peek at the next `T` and return it if it exists (without advancing) or return `None`.
    pub fn peek_next<T: Peek>(&self) -> Option<T> {
        Peeker::with(&self.stream().tokens(), self.pos + 1)
    }

    /// Peek at the `T` at `num_tokens` index and return it if it exists (without advancing)
    /// or return `None`.
    pub fn peek_ahead<T: Peek>(&self, offset: usize) -> Option<T> {
        Peeker::with(&self.stream().tokens(), self.pos + offset)
    }

    /// Push `ParserError` to the `Handler`.
    /// Return `ErrorEmitted` just to confirm that the action happened.
    pub fn log_error(&self, error_kind: ParserErrorKind) -> ErrorEmitted {
        let err = ParserError {
            error_kind,
            position: Position::new(
                &self.stream.span().source(),
                self.stream()
                    .tokens()
                    .get(self.pos)
                    .expect("PositionError: token not found")
                    .span()
                    .start(),
            ),
        };

        self.handler.emit_err(CompilerError::Parser(err))
    }
    pub fn errors(&self) -> Vec<CompilerError> {
        self.handler.clone().get_inner().0
    }
}
