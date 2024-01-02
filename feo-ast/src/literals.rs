use feo_error::error::CompilerError;
use feo_error::handler::{ErrorEmitted, Handler};
use feo_error::parser_error::{ParserError, ParserErrorKind};

use feo_types::span::{Position, Span, Spanned};
use feo_types::{Literal, PrimitiveType};

use crate::token::{Token, Tokenize};

#[derive(Debug, Clone)]
pub struct CharLiteral(pub Literal<char>);

impl Tokenize for CharLiteral {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = ParserError {
            error_kind: ParserErrorKind::ParseCharError,
            position: Position::new(src, start),
        };

        // convert `core::char::ParseCharError` to `CompilerError::Parser(ParserError)`
        let parsed = content
            .parse::<char>()
            .map_err(|_| handler.emit_err(CompilerError::Parser(err)))?;

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

#[derive(Debug, Clone)]
pub struct StringLiteral(pub Literal<String>);

impl Tokenize for StringLiteral {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        _handler: &mut Handler,
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

#[derive(Debug, Clone)]
pub struct IntLiteral(pub Literal<i64>);

impl Tokenize for IntLiteral {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = ParserError {
            error_kind: ParserErrorKind::ParseIntError,
            position: Position::new(src, start),
        };

        // convert `core::num::ParseIntError` to `CompilerError::Parser(ParserError)`
        let parsed = i64::from_str_radix(content, 10)
            .map_err(|_| handler.emit_err(CompilerError::Parser(err)))?;

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

#[derive(Debug, Clone)]
pub struct UIntLiteral(pub Literal<u64>);

impl Tokenize for UIntLiteral {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = ParserError {
            error_kind: ParserErrorKind::ParseUIntError,
            position: Position::new(src, start),
        };

        // convert `core::num::ParseIntError` to `CompilerError::Parser(ParserError)`
        let parsed = if content.starts_with("0x") {
            let without_prefix = content.trim_start_matches("0x");

            u64::from_str_radix(without_prefix, 16)
                .map_err(|_| handler.emit_err(CompilerError::Parser(err)))?
        } else {
            u64::from_str_radix(content, 10)
                .map_err(|_| handler.emit_err(CompilerError::Parser(err)))?
        };

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

#[derive(Debug, Clone)]
pub struct FloatLiteral(pub Literal<f64>);

impl Tokenize for FloatLiteral {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = ParserError {
            error_kind: ParserErrorKind::ParseFloatError,
            position: Position::new(src, start),
        };

        // convert `core::num::ParseFloatError` to `CompilerError::Parser(ParserError)`
        let parsed = content
            .parse::<f64>()
            .map_err(|_| handler.emit_err(CompilerError::Parser(err)))?;

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

#[derive(Debug, Clone)]
pub struct BoolLiteral(pub Literal<bool>);

impl Tokenize for BoolLiteral {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = ParserError {
            error_kind: ParserErrorKind::ParseBoolError,
            position: Position::new(src, start),
        };

        // convert `core::str::ParseBoolError` to `CompilerError::Parser(ParserError)`
        let parsed = content
            .parse::<bool>()
            .map_err(|_| handler.emit_err(CompilerError::Parser(err)))?;

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
