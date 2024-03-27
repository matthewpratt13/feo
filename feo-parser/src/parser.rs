use feo_ast::{
    expression::{Expression, MethodCallExpr, Value},
    path::{PathIdenSegmentKind, PathInExpr},
    token::{Token, TokenStream},
};
use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    parser_error::{ParserError, ParserErrorKind},
};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    literal::LiteralKind,
    punctuation::PuncKind,
    span::{Position, Spanned},
    Delimiter, Keyword,
};

use crate::{
    parse::ParseExpr,
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

    pub fn parse_expression(
        &mut self,
        precedence: Precedence,
    ) -> Result<Option<Expression>, ParserErrorKind> {
        let mut left_expr = self.parse_prefix()?.expect("token not found");

        if let Some(input) = Precedence::token_precedence(self)? {
            while precedence < input {
                let infix = self.next_token().expect("token not found");
                left_expr = self
                    .parse_infix(infix, left_expr)?
                    .expect("token not found");
            }
        }

        Ok(Some(left_expr))
    }

    fn parse_prefix(&mut self) -> Result<Option<Expression>, ParserErrorKind> {
        match self.current_token() {
            Some(Token::BoolLit(b)) => Ok(Some(Expression::Literal(LiteralKind::Bool(b)))),
            Some(Token::IntLit(i)) => Ok(Some(Expression::Literal(LiteralKind::Int(i)))),
            Some(Token::UIntLit(ui)) => Ok(Some(Expression::Literal(LiteralKind::UInt(ui)))),
            Some(Token::U256Lit(u)) => Ok(Some(Expression::Literal(LiteralKind::U256(u)))),
            Some(Token::FloatLit(f)) => Ok(Some(Expression::Literal(LiteralKind::Float(f)))),
            Some(Token::Identifier(id)) => Ok(Some(Expression::PathExpr(PathInExpr {
                first_segment: PathIdenSegmentKind::Identifier(id),
                subsequent_segments: None,
            }))),

            _ => Ok(None),
        }
    }

    fn parse_infix(
        &mut self,
        infix: Token,
        left: Expression,
    ) -> Result<Option<Expression>, ParserErrorKind> {
        match infix {
            Token::CharLit(_) => todo!(),
            Token::StringLit(_) => todo!(),
            Token::BoolLit(_) => todo!(),
            Token::IntLit(_) => todo!(),
            Token::UIntLit(_) => todo!(),
            Token::U256Lit(_) => todo!(),
            Token::FloatLit(_) => todo!(),
            Token::Identifier(_) => todo!(),
            Token::Keyword(_) => todo!(),
            Token::Comment(_) => todo!(),
            Token::DocComment(_) => todo!(),
            Token::Delim(_) => todo!(),
            Token::Punc(_) => todo!(),
            Token::EOF => todo!(),
        }
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
