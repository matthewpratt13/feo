use bnum::types::U256;

use feo_error::LexErrorKind;
use feo_types::Literal;

use crate::lexer::Token;

#[derive(Debug)]
pub enum IntKind {
    I32,
    I64,
}

#[derive(Debug)]
pub enum UintKind {
    U8,
    U16,
    U32,
    U64,
    U256,
}

#[derive(Debug)]
pub enum FloatKind {
    F32,
    F64,
}

#[derive(Debug)]
pub struct CharLiteral(Literal<char>);

// convert `Token` to inner `CharLiteral`
impl TryFrom<Token> for CharLiteral {
    type Error = LexErrorKind;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::CharLit(c) => Ok(c),
            _ => Err(LexErrorKind::TokenNotFound),
        }
    }
}

#[derive(Debug)]
pub struct StringLiteral(Literal<String>);

// convert `Token` to inner `StringLiteral`
impl TryFrom<Token> for StringLiteral {
    type Error = LexErrorKind;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::StringLit(s) => Ok(s),
            _ => Err(LexErrorKind::TokenNotFound),
        }
    }
}

#[derive(Debug)]
pub struct IntLiteral {
    int_kind: Option<IntKind>,
    lit: Literal<i64>,
}

// convert `Token` to inner `IntLiteral`
impl TryFrom<Token> for IntLiteral {
    type Error = LexErrorKind;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::IntLit(i) => Ok(i),
            _ => Err(LexErrorKind::TokenNotFound),
        }
    }
}

#[derive(Debug)]
pub struct UIntLiteral {
    uint_kind: Option<UintKind>,
    lit: Literal<U256>,
}

// convert `Token` to inner `U256Literal`
impl TryFrom<Token> for UIntLiteral {
    type Error = LexErrorKind;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::UIntLit(u) => Ok(u),
            _ => Err(LexErrorKind::TokenNotFound),
        }
    }
}

#[derive(Debug)]
pub struct FloatLiteral {
    float_kind: Option<FloatKind>,
    lit: Literal<f64>,
}

// convert `Token` to inner `FloatLiteral`
impl TryFrom<Token> for FloatLiteral {
    type Error = LexErrorKind;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::FloatLit(f) => Ok(f),
            _ => Err(LexErrorKind::TokenNotFound),
        }
    }
}

#[derive(Debug)]
pub struct BoolLiteral(Literal<bool>);

// convert `Token` to inner `BoolLiteral`
impl TryFrom<Token> for BoolLiteral {
    type Error = LexErrorKind;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::BoolLit(b) => Ok(b),
            _ => Err(LexErrorKind::TokenNotFound),
        }
    }
}