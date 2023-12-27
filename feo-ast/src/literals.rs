use std::sync::Arc;

use feo_error::error::{CompileError, ErrorEmitted};
use feo_error::lex_error::{LexError, LexErrorKind};

use feo_types::span::{Span, Spanned};
use feo_types::{Literal, PrimitiveType};

use crate::token::{Token, Tokenize};

#[derive(Debug, Clone)]
pub struct CharLiteral(pub Literal<char>);

impl Tokenize for CharLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = LexError {
            error_kind: LexErrorKind::ParseCharError,
            pos: start,
        };

        // convert `core::char::ParseCharError` to `CompileError::Lex(LexError)`
        let parsed = content
            .parse::<char>()
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Lex(err)))?;

        let char_lit = Literal::new(parsed, span);

        let token = Token::CharLit(CharLiteral(char_lit));

        Ok(Some(token))
    }
}

impl Spanned for CharLiteral {
    fn span(&self) -> &Span {
        self.0.span()
    }
}

// convert `Token` to inner `CharLiteral`
impl TryFrom<Token> for CharLiteral {
    type Error = LexErrorKind;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::CharLit(c) => Ok(c),
            _ => Err(LexErrorKind::MismatchedTokenType),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteral(pub Literal<String>);

impl Tokenize for StringLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let string_lit = Literal::new(content.to_string(), span);

        let token = Token::StringLit(StringLiteral(string_lit));

        Ok(Some(token))
    }
}

impl Spanned for StringLiteral {
    fn span(&self) -> &Span {
        self.0.span()
    }
}

// convert `Token` to inner `StringLiteral`
impl TryFrom<Token> for StringLiteral {
    type Error = LexErrorKind;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::StringLit(s) => Ok(s),
            _ => Err(LexErrorKind::MismatchedTokenType),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntLiteral(pub Literal<i64>);

impl Tokenize for IntLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = LexError {
            error_kind: LexErrorKind::ParseIntError,
            pos: start,
        };

        // convert `core::num::ParseIntError` to `CompileError::Lex(LexError)`
        let parsed = i64::from_str_radix(content, 10)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Lex(err)))?;

        let int_lit = Literal::new(parsed, span);

        let token = Token::IntLit(IntLiteral(int_lit));

        Ok(Some(token))
    }
}

impl Spanned for IntLiteral {
    fn span(&self) -> &Span {
        self.0.span()
    }
}

// convert `Token` to inner `IntLiteral`
impl TryFrom<Token> for IntLiteral {
    type Error = LexErrorKind;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::IntLit(i) => Ok(i),
            _ => Err(LexErrorKind::MismatchedTokenType),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UIntLiteral(pub Literal<u64>);

impl Tokenize for UIntLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = LexError {
            error_kind: LexErrorKind::ParseUIntError,
            pos: start,
        };

        // convert `core::num::ParseIntError` to `CompileError::Lex(LexError)`
        let parsed = u64::from_str_radix(content, 10)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Lex(err)))?;

        let uint_lit = Literal::new(parsed, span);

        let token = Token::UIntLit(UIntLiteral(uint_lit));

        Ok(Some(token))
    }
}

impl Spanned for UIntLiteral {
    fn span(&self) -> &Span {
        self.0.span()
    }
}

// convert `Token` to inner `UIntLiteral`
impl TryFrom<Token> for UIntLiteral {
    type Error = LexErrorKind;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::UIntLit(u) => Ok(u),
            _ => Err(LexErrorKind::MismatchedTokenType),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FloatLiteral(pub Literal<f64>);

impl Tokenize for FloatLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = LexError {
            error_kind: LexErrorKind::ParseFloatError,
            pos: start,
        };

        // convert `core::num::ParseFloatError` to `CompileError::Lex(LexError)`
        let parsed = content
            .parse::<f64>()
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Lex(err)))?;

        let float_lit = Literal::new(parsed, span);

        let token = Token::FloatLit(FloatLiteral(float_lit));

        Ok(Some(token))
    }
}

impl Spanned for FloatLiteral {
    fn span(&self) -> &Span {
        self.0.span()
    }
}

// convert `Token` to inner `FloatLiteral`
impl TryFrom<Token> for FloatLiteral {
    type Error = LexErrorKind;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::FloatLit(f) => Ok(f),
            _ => Err(LexErrorKind::MismatchedTokenType),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoolLiteral(pub Literal<bool>);

impl Tokenize for BoolLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = LexError {
            error_kind: LexErrorKind::ParseBoolError,
            pos: start,
        };

        // convert `core::str::ParseBoolError` to `CompileError::Lex(LexError)`
        let parsed = content
            .parse::<bool>()
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Lex(err)))?;

        let bool_lit = Literal::new(parsed, span);

        let token = Token::BoolLit(BoolLiteral(bool_lit));

        Ok(Some(token))
    }
}

impl Spanned for BoolLiteral {
    fn span(&self) -> &Span {
        self.0.span()
    }
}

// convert `Token` to inner `BoolLiteral`
impl TryFrom<Token> for BoolLiteral {
    type Error = LexErrorKind;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::BoolLit(b) => Ok(b),
            _ => Err(LexErrorKind::MismatchedTokenType),
        }
    }
}
