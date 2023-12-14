use feo_types::Literal;

use crate::lexer::{LexError, Token};

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
}

#[derive(Debug)]
pub enum FloatKind {
    F32,
    F64,
}

#[derive(Debug)]
pub struct CharLiteral(Literal<char>);

// convert `Token` to inner `CharLiteral`
impl<T> TryFrom<Token<T>> for CharLiteral {
    type Error = LexError;

    fn try_from(value: Token<T>) -> Result<Self, Self::Error> {
        match value {
            Token::CharLit(c) => Ok(c),
            _ => Err(LexError::TokenNotFound),
        }
    }
}

#[derive(Debug)]
pub struct StringLiteral(Literal<String>);

// convert `Token` to inner `StringLiteral`
impl<T> TryFrom<Token<T>> for StringLiteral {
    type Error = LexError;

    fn try_from(value: Token<T>) -> Result<Self, Self::Error> {
        match value {
            Token::StringLit(s) => Ok(s),
            _ => Err(LexError::TokenNotFound),
        }
    }
}

#[derive(Debug)]
pub struct IntLiteral {
    int_kind: Option<IntKind>,
    lit: Literal<i64>,
}

// convert `Token` to inner `IntLiteral`
impl<T> TryFrom<Token<T>> for IntLiteral {
    type Error = LexError;

    fn try_from(value: Token<T>) -> Result<Self, Self::Error> {
        match value {
            Token::IntLit(i) => Ok(i),
            _ => Err(LexError::TokenNotFound),
        }
    }
}

#[derive(Debug)]
pub struct UIntLiteral {
    uint_kind: Option<UintKind>,
    lit: Literal<u64>,
}

// convert `Token` to inner `U256Literal`
impl<T> TryFrom<Token<T>> for UIntLiteral {
    type Error = LexError;

    fn try_from(value: Token<T>) -> Result<Self, Self::Error> {
        match value {
            Token::UIntLit(u) => Ok(u),
            _ => Err(LexError::TokenNotFound),
        }
    }
}

#[derive(Debug)]
pub struct FloatLiteral {
    float_kind: Option<FloatKind>,
    lit: Literal<f64>,
}

// convert `Token` to inner `FloatLiteral`
impl<T> TryFrom<Token<T>> for FloatLiteral {
    type Error = LexError;

    fn try_from(value: Token<T>) -> Result<Self, Self::Error> {
        match value {
            Token::FloatLit(f) => Ok(f),
            _ => Err(LexError::TokenNotFound),
        }
    }
}

#[derive(Debug)]
pub struct BoolLiteral(Literal<bool>);

// convert `Token` to inner `BoolLiteral`
impl<T> TryFrom<Token<T>> for BoolLiteral {
    type Error = LexError;

    fn try_from(value: Token<T>) -> Result<Self, Self::Error> {
        match value {
            Token::BoolLit(b) => Ok(b),
            _ => Err(LexError::TokenNotFound),
        }
    }
}
