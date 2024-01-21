use bnum::cast::As;

use feo_error::error::CompilerError;
use feo_error::handler::{ErrorEmitted, Handler};
use feo_error::parser_error::{ParserError, ParserErrorKind};

use feo_types::span::{Position, Span, Spanned};
use feo_types::{Literal, PrimitiveType, U256};

use crate::expression::{Constant, ExprWithoutBlock, Expression};
use crate::pattern::{Pattern, RangePattBound};
use crate::statement::Statement;
use crate::token::{Token, Tokenize};
use crate::ty::Type;

pub trait LiteralExpr<E>
where
    Self: Sized + Constant + ExprWithoutBlock<E>,
{
}

pub trait LiteralPatt
where
    Self: Sized + 'static + Pattern,
{
}

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

        let parsed = content
            .parse::<char>()
            .map_err(|_| handler.emit_err(CompilerError::Parser(err)))?;

        let char_lit = Literal::new(parsed, span);

        let token = Token::CharLit(CharLiteral(char_lit));

        Ok(Some(token))
    }
}

impl<E> LiteralExpr<E> for CharLiteral {} // raw value

impl Expression for CharLiteral {} // raw value

impl<E> ExprWithoutBlock<E> for CharLiteral {} // raw value

impl Statement for CharLiteral {} // raw value

impl Constant for CharLiteral {} // raw value

impl LiteralPatt for CharLiteral {} // raw value

impl Pattern for CharLiteral {} // raw value

impl RangePattBound for CharLiteral {} // raw value

impl Type for CharLiteral {}

impl Spanned for CharLiteral {
    fn span(&self) -> Span {
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

        let literal = Literal::new(content.to_string(), span);

        let token = Token::StringLit(StringLiteral(literal));

        Ok(Some(token))
    }
}

impl<E> LiteralExpr<E> for StringLiteral {} // raw value

impl Expression for StringLiteral {} // raw value

impl<E> ExprWithoutBlock<E> for StringLiteral {} // raw value

impl Statement for StringLiteral {} // raw value

impl Constant for StringLiteral {} // raw value

impl LiteralPatt for StringLiteral {} // raw value

impl Pattern for StringLiteral {} // raw value

impl Type for StringLiteral {}

impl Spanned for StringLiteral {
    fn span(&self) -> Span {
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

        let error = ParserError {
            error_kind: ParserErrorKind::ParseIntError,
            position: Position::new(src, start),
        };

        let parsed = i64::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
            .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?;

        let literal = Literal::new(parsed, span);

        let token = Token::IntLit(IntLiteral(literal));

        Ok(Some(token))
    }
}

impl<E> LiteralExpr<E> for IntLiteral {} // raw value

impl Expression for IntLiteral {} // raw value

impl<E> ExprWithoutBlock<E> for IntLiteral {} // raw value

impl Statement for IntLiteral {} // raw value

impl Constant for IntLiteral {} // raw value

impl LiteralPatt for IntLiteral {} // raw value

impl Pattern for IntLiteral {} // raw value

impl RangePattBound for IntLiteral {} // raw value

impl Type for IntLiteral {}

impl Spanned for IntLiteral {
    fn span(&self) -> Span {
        self.0.span()
    }
}

#[derive(Debug, Clone)]
pub struct UIntLiteral(pub Literal<u64>);

// impl UIntValue {
//     fn trim_leading_zeros(self) -> Self {
//         let uint_string = format!("{}", self.0);
//         let stripped = uint_string.as_str().trim_start_matches('0');
//         let new_uint = u64::from_str_radix(stripped, 10).expect("Unable to parse str to u64");

//         Self(new_uint)
//     }
// }

impl Tokenize for UIntLiteral {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let uint_error = ParserError {
            error_kind: ParserErrorKind::ParseUIntError,
            position: Position::new(src, start),
        };

        let u256_error = ParserError {
            error_kind: ParserErrorKind::ParseU256Error,
            position: Position::new(src, start),
        };

        let parsed = if content.starts_with("0x") {
            let without_prefix = content.trim_start_matches("0x");

            let content_as_hex_u256 = U256::from_str_radix(
                &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                16,
            )
            .map_err(|_| handler.emit_err(CompilerError::Parser(u256_error)))?;

            if content_as_hex_u256 > u64::MAX.as_::<U256>() {
                panic!("Integer overflow: Input exceeds maximum `u64` value");
            } else {
                u64::from_str_radix(
                    &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                    16,
                )
                .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?
            }
        } else {
            let content_as_dec_u256 =
                U256::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                    .map_err(|_| handler.emit_err(CompilerError::Parser(u256_error)))?;

            if content_as_dec_u256 > u64::MAX.as_::<U256>() {
                panic!("Integer overflow: Input exceeds maximum `u64` value");
            } else {
                u64::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                    .map_err(|_| handler.emit_err(CompilerError::Parser(uint_error)))?
            }
        };

        let literal = Literal::new(parsed, span);

        let token = Token::UIntLit(UIntLiteral(literal));

        Ok(Some(token))
    }
}

impl<E> LiteralExpr<E> for UIntLiteral {} // raw value

impl Expression for UIntLiteral {} // raw value

impl<E> ExprWithoutBlock<E> for UIntLiteral {} // raw value

impl Statement for UIntLiteral {} // raw value

impl Constant for UIntLiteral {} // raw value

impl LiteralPatt for UIntLiteral {} // raw value

impl Pattern for UIntLiteral {} // raw value

impl RangePattBound for UIntLiteral {} // raw value

impl Type for UIntLiteral {}

impl Spanned for UIntLiteral {
    fn span(&self) -> Span {
        self.0.span()
    }
}

#[derive(Debug, Clone)]
pub struct U256Literal(pub Literal<U256>);

impl Tokenize for U256Literal {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let error = ParserError {
            error_kind: ParserErrorKind::ParseU256Error,
            position: Position::new(src, start),
        };

        let parsed = if content.starts_with("0x") {
            let without_prefix = content.trim_start_matches("0x");

            U256::from_str_radix(
                &without_prefix.split('_').collect::<Vec<&str>>().concat(),
                16,
            )
            .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?
        } else {
            U256::from_str_radix(&content.split('_').collect::<Vec<&str>>().concat(), 10)
                .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?
        };

        let literal = Literal::new(parsed, span);

        let token = Token::U256Lit(U256Literal(literal));

        Ok(Some(token))
    }
}

impl<E> LiteralExpr<E> for U256Literal {} // raw value

impl Expression for U256Literal {} // raw value

impl<E> ExprWithoutBlock<E> for U256Literal {} // raw value

impl Statement for U256Literal {} // raw value

impl Constant for U256Literal {} // raw value

impl LiteralPatt for U256Literal {} // raw value

impl Pattern for U256Literal {} // raw value

impl RangePattBound for U256Literal {} // raw value

impl Type for U256Literal {}

impl Spanned for U256Literal {
    fn span(&self) -> Span {
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

        let error = ParserError {
            error_kind: ParserErrorKind::ParseFloatError,
            position: Position::new(src, start),
        };

        let parsed = content
            .parse::<f64>()
            .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?;

        let literal = Literal::new(parsed, span);

        let token = Token::FloatLit(FloatLiteral(literal));

        Ok(Some(token))
    }
}

impl<E> LiteralExpr<E> for FloatLiteral {} // raw value

impl Expression for FloatLiteral {} // raw value

impl<E> ExprWithoutBlock<E> for FloatLiteral {} // raw value

impl Statement for FloatLiteral {} // raw value

impl Constant for FloatLiteral {} // raw value

impl LiteralPatt for FloatLiteral {} // raw value

impl Pattern for FloatLiteral {} // raw value

impl RangePattBound for FloatLiteral {} // raw value

impl Type for FloatLiteral {}

impl Spanned for FloatLiteral {
    fn span(&self) -> Span {
        self.0.span()
    }
}

#[derive(Debug, Clone)]
pub struct Bytes32Literal(pub Literal<&'static [u8; 32]>);

// TODO: implement Tokenize ?

impl<E> LiteralExpr<E> for Bytes32Literal {} // raw value

impl Expression for Bytes32Literal {} // raw value

impl<E> ExprWithoutBlock<E> for Bytes32Literal {} // raw value

impl Statement for Bytes32Literal {} // raw value

impl Constant for Bytes32Literal {} // raw value

impl LiteralPatt for Bytes32Literal {} // raw value

impl Pattern for Bytes32Literal {} // raw value

impl Type for Bytes32Literal {}

impl Spanned for Bytes32Literal {
    fn span(&self) -> Span {
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

        let error = ParserError {
            error_kind: ParserErrorKind::ParseBoolError,
            position: Position::new(src, start),
        };

        let parsed = content
            .parse::<bool>()
            .map_err(|_| handler.emit_err(CompilerError::Parser(error)))?;

        let literal = Literal::new(parsed, span);

        let token = Token::BoolLit(BoolLiteral(literal));

        Ok(Some(token))
    }
}

impl<E> LiteralExpr<E> for BoolLiteral {} // raw value

impl Expression for BoolLiteral {} // raw value

impl<E> ExprWithoutBlock<E> for BoolLiteral {} // raw value

impl Statement for BoolLiteral {} // raw value

impl Constant for BoolLiteral {} // raw value

impl LiteralPatt for BoolLiteral {} // raw value

impl Pattern for BoolLiteral {} // raw value

impl Type for BoolLiteral {}

impl Spanned for BoolLiteral {
    fn span(&self) -> Span {
        self.0.span()
    }
}
