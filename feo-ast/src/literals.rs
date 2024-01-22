use bnum::cast::As;

use feo_error::error::CompilerError;
use feo_error::handler::{ErrorEmitted, Handler};
use feo_error::parser_error::{ParserError, ParserErrorKind};

use feo_types::span::{Position, Span, Spanned};
use feo_types::{Literal, Primitive, PrimitiveType, U256};

use crate::expression::{Constant, ExprWithoutBlock, Expression};
use crate::pattern::{Pattern, RangePattBound};
use crate::statement::Statement;
use crate::token::{Token, Tokenize};
use crate::ty::Type;

pub trait LiteralExpr<E>
where
    Self: Constant + ExprWithoutBlock<E>,
{
}

pub trait LiteralPatt
where
    Self: Sized + 'static + Pattern,
{
}

impl<L, E> LiteralExpr<E> for Literal<L> where L: 'static + Clone + Primitive {}

impl<L, E> ExprWithoutBlock<E> for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> Expression for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> Statement for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> Constant for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> LiteralPatt for Literal<L> where L: 'static + Clone + Primitive {}

impl<L> Pattern for Literal<L> where L: 'static + Clone + Primitive {}

impl RangePattBound for Literal<char> {}

impl RangePattBound for Literal<i64> {}

impl RangePattBound for Literal<u64> {}

impl RangePattBound for Literal<U256> {}

impl RangePattBound for Literal<f64> {}

impl<L> Type for Literal<L> where L: 'static + Clone + Primitive {}

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

impl Spanned for IntLiteral {
    fn span(&self) -> Span {
        self.0.span()
    }
}

#[derive(Debug, Clone)]
pub struct UIntLiteral(pub Literal<u64>);

// impl UIntLiteral {
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

impl Spanned for FloatLiteral {
    fn span(&self) -> Span {
        self.0.span()
    }
}

#[derive(Debug, Clone)]
pub struct Bytes32Literal(pub Literal<&'static [u8; 32]>);

// TODO: implement Tokenize ?

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

impl Spanned for BoolLiteral {
    fn span(&self) -> Span {
        self.0.span()
    }
}
