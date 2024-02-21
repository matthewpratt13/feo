#![allow(dead_code)]

use feo_ast::token::{Token, TokenStream};
use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    parser_error::{ParserError, ParserErrorKind},
};
use feo_types::{
    literal::{FloatType, IntType, Literal, UIntType},
    span::{Position, Spanned},
    Delimiter, Identifier, Keyword, Punctuation, U256,
};

use crate::parse::Peek;

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

    pub fn advance(&mut self) -> Option<Token> {
        let token = self.current_token();
        if token.is_some() {
            self.pos += 1;
        }

        token
    }

    pub fn current_token(&self) -> Option<Token> {
        self.stream.tokens().get(self.pos).cloned()
    }

    // peek at the current `T` and return it if it exists (without advancing)
    pub fn peek_current<T: Peek>(&self) -> Result<T, ErrorEmitted> {
        Peeker::with(&self.stream().tokens(), self.pos, &self.handler)?
            .ok_or_else(|| self.log_error(ParserErrorKind::TokenNotFound))
    }

    // peek at the next `T` and return it if it exists (without advancing)
    pub fn peek_next<T: Peek>(&self) -> Result<T, ErrorEmitted> {
        Peeker::with(&self.stream().tokens(), self.pos + 1, &self.handler)?
            .ok_or_else(|| self.log_error(ParserErrorKind::TokenNotFound))
    }

    // peek at the current `Token`, advance the `Parser`; return the peeked `Token` or return `None`
    pub fn take<T: Peek>(&mut self) -> Result<Option<T>, ErrorEmitted> {
        let value = Peeker::with(&self.stream().tokens(), self.pos, &self.handler);
        self.advance();
        value
    }

    pub fn log_error(&self, error_kind: ParserErrorKind) -> ErrorEmitted {
        let err = ParserError {
            error_kind,
            position: Position::new(
                &self.stream.span().source(),
                self.stream()
                    .tokens()
                    .get(self.pos)
                    .expect("token not found")
                    .span()
                    .start(),
            ),
        };

        self.handler.emit_err(CompilerError::Parser(err))
    }
}

// type that allows for peeking at the next `Token` in a `&[Token]` without advancing the parser
#[derive(Clone)]
pub struct Peeker<'a, 'b> {
    tokens: &'a [Token],
    pos: usize,
    handler: &'b Handler,
}

impl<'a, 'b> Peeker<'a, 'b> {
    // peek for a `T` in `&[Token]'; return `T` if it exists or return `None`
    fn with<T: Peek>(
        tokens: &'a [Token],
        pos: usize,
        handler: &'b Handler,
    ) -> Result<Option<T>, ErrorEmitted> {
        let peeker = Peeker {
            tokens,
            pos,
            handler,
        };
        let value = T::peek(&peeker);

        value
    }

    // peek for the current `Token`; return it if it exists or return `None`
    pub fn peek_token(&self) -> Option<Token> {
        self.tokens.get(self.pos).cloned()
    }

    // peek for a `Literal`; return it if it exists, or return an error
    pub fn peek_char_lit(&self) -> Result<Literal<char>, ErrorEmitted> {
        match self.peek_token() {
            Some(Token::CharLit(c)) => Ok(c),
            Some(_) => Err(self.log_error(ParserErrorKind::UnexpectedToken {
                expected: "character literal".to_string(),
                found: self
                    .peek_token()
                    .ok_or_else(|| self.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            })),
            None => Err(self.log_error(ParserErrorKind::TokenNotFound)),
        }
    }

    pub fn peek_string_lit(&self) -> Result<Literal<String>, ErrorEmitted> {
        match self.peek_token() {
            Some(Token::StringLit(s)) => Ok(s),
            Some(_) => Err(self.log_error(ParserErrorKind::UnexpectedToken {
                expected: "string literal".to_string(),
                found: self
                    .peek_token()
                    .ok_or_else(|| self.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            })),
            None => Err(self.log_error(ParserErrorKind::TokenNotFound)),
        }
    }

    pub fn peek_bool_lit(&self) -> Result<Literal<bool>, ErrorEmitted> {
        match self.peek_token() {
            Some(Token::BoolLit(b)) => Ok(b),
            Some(_) => Err(self.log_error(ParserErrorKind::UnexpectedToken {
                expected: "bool literal".to_string(),
                found: self
                    .peek_token()
                    .ok_or_else(|| self.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            })),
            None => Err(self.log_error(ParserErrorKind::TokenNotFound)),
        }
    }

    pub fn peek_int_lit(&self) -> Result<Literal<IntType>, ErrorEmitted> {
        match self.peek_token() {
            Some(Token::IntLit(i)) => Ok(i),
            Some(_) => Err(self.log_error(ParserErrorKind::UnexpectedToken {
                expected: "integer literal".to_string(),
                found: self
                    .peek_token()
                    .ok_or_else(|| self.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            })),
            None => Err(self.log_error(ParserErrorKind::TokenNotFound)),
        }
    }

    pub fn peek_uint_lit(&self) -> Result<Literal<UIntType>, ErrorEmitted> {
        match self.peek_token() {
            Some(Token::UIntLit(ui)) => Ok(ui),
            Some(_) => Err(self.log_error(ParserErrorKind::UnexpectedToken {
                expected: "unsigned integer literal".to_string(),
                found: self
                    .peek_token()
                    .ok_or_else(|| self.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            })),
            None => Err(self.log_error(ParserErrorKind::TokenNotFound)),
        }
    }

    pub fn peek_u256_lit(&self) -> Result<Literal<U256>, ErrorEmitted> {
        match self.peek_token() {
            Some(Token::U256Lit(u)) => Ok(u),
            Some(_) => Err(self.log_error(ParserErrorKind::UnexpectedToken {
                expected: "U256 literal".to_string(),
                found: self
                    .peek_token()
                    .ok_or_else(|| self.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            })),
            None => Err(self.log_error(ParserErrorKind::TokenNotFound)),
        }
    }

    pub fn peek_float_lit(&self) -> Result<Literal<FloatType>, ErrorEmitted> {
        match self.peek_token() {
            Some(Token::FloatLit(f)) => Ok(f),
            Some(_) => Err(self.log_error(ParserErrorKind::UnexpectedToken {
                expected: "floating-point number literal".to_string(),
                found: self
                    .peek_token()
                    .ok_or_else(|| self.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            })),
            None => Err(self.log_error(ParserErrorKind::TokenNotFound)),
        }
    }

    pub fn peek_identifier(&self) -> Result<Identifier, ErrorEmitted> {
        match self.peek_token() {
            Some(Token::Iden(id)) => Ok(id),
            Some(_) => Err(self.log_error(ParserErrorKind::UnexpectedToken {
                expected: "identifier".to_string(),
                found: self
                    .peek_token()
                    .ok_or_else(|| self.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            })),
            None => Err(self.log_error(ParserErrorKind::TokenNotFound)),
        }
    }

    pub fn peek_keyword(&self) -> Result<Keyword, ErrorEmitted> {
        match self.peek_token() {
            Some(Token::Keyword(k)) => Ok(k),
            Some(_) => Err(self.log_error(ParserErrorKind::UnexpectedToken {
                expected: "keyword".to_string(),
                found: self
                    .peek_token()
                    .ok_or_else(|| self.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            })),
            None => Err(self.log_error(ParserErrorKind::TokenNotFound)),
        }
    }

    // pub fn peek_doc_comment(&self) -> Result<DocComment, Self> {
    //     match self.current_token() {
    //         Some(Token::DocComment(dc)) => Ok(dc),
    //         _ => Err(*self),
    //     }
    // }

    pub fn peek_delimiter(&self) -> Result<Delimiter, ErrorEmitted> {
        match self.peek_token() {
            Some(Token::Delim(d)) => Ok(d),
            Some(_) => Err(self.log_error(ParserErrorKind::UnexpectedToken {
                expected: "delimiter".to_string(),
                found: self
                    .peek_token()
                    .ok_or_else(|| self.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            })),
            None => Err(self.log_error(ParserErrorKind::TokenNotFound)),
        }
    }

    pub fn peek_punctuation(&self) -> Result<Punctuation, ErrorEmitted> {
        match self.peek_token() {
            Some(Token::Punc(p)) => Ok(p),
            Some(_) => Err(self.log_error(ParserErrorKind::UnexpectedToken {
                expected: "punctuation".to_string(),
                found: self
                    .peek_token()
                    .ok_or_else(|| self.log_error(ParserErrorKind::TokenNotFound))?
                    .to_string(),
            })),
            None => Err(self.log_error(ParserErrorKind::TokenNotFound)),
        }
    }

    // pub fn peek_type_ann(&self) -> Result<TypeAnnotation, Self> {
    //     match self.current_token() {
    //         Some(Token::TypeAnn(ta)) => Ok(ta),
    //         _ => Err(*self),
    //     }
    // }

    pub fn log_error(&self, error_kind: ParserErrorKind) -> ErrorEmitted {
        let err = ParserError {
            error_kind,
            position: Position::new(
                &self.tokens[self.pos].span().source(),
                self.tokens[self.pos].span().start(),
            ),
        };

        self.handler.emit_err(CompilerError::Parser(err))
    }
}

impl Peek for Literal<char> {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match peeker.peek_char_lit() {
            Ok(c) => Ok(Some(c)),
            Err(e) => Err(e),
        }
    }
}

impl Peek for Literal<String> {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match peeker.peek_string_lit() {
            Ok(s) => Ok(Some(s)),
            Err(e) => Err(e),
        }
    }
}

impl Peek for Literal<bool> {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match peeker.peek_bool_lit() {
            Ok(b) => Ok(Some(b)),
            Err(e) => Err(e),
        }
    }
}

impl Peek for Literal<IntType> {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match peeker.peek_int_lit() {
            Ok(i) => Ok(Some(i)),
            Err(e) => Err(e),
        }
    }
}

impl Peek for Literal<UIntType> {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match peeker.peek_uint_lit() {
            Ok(ui) => Ok(Some(ui)),
            Err(e) => Err(e),
        }
    }
}

impl Peek for Literal<U256> {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match peeker.peek_u256_lit() {
            Ok(u) => Ok(Some(u)),
            Err(e) => Err(e),
        }
    }
}

impl Peek for Literal<FloatType> {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match peeker.peek_float_lit() {
            Ok(f) => Ok(Some(f)),
            Err(e) => Err(e),
        }
    }
}

impl Peek for Identifier {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match peeker.peek_identifier() {
            Ok(id) => Ok(Some(id)),
            Err(e) => Err(e),
        }
    }
}

impl Peek for Keyword {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match peeker.peek_keyword() {
            Ok(k) => Ok(Some(k)),
            Err(e) => Err(e),
        }
    }
}

// impl Peek for DocComment {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_doc_comment() {
//             Ok(dc) => Some(dc),
//             Err(_) => None,
//         }
//     }
// }

impl Peek for Delimiter {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match peeker.peek_delimiter() {
            Ok(d) => Ok(Some(d)),
            Err(e) => Err(e),
        }
    }
}

impl Peek for Punctuation {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match peeker.peek_punctuation() {
            Ok(p) => Ok(Some(p)),
            Err(e) => Err(e),
        }
    }
}

// impl Peek for TypeAnnotation {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         match peeker.peek_type_ann() {
//             Ok(ta) => Some(ta),
//             Err(_) => None,
//         }
//     }
// }
