use feo_types::Literal;

use crate::{error::LexError, lexer::Token};

#[derive(Debug, Clone)]
pub struct CharLiteral(pub Literal<char>);

// convert `Token` to inner `CharLiteral`
impl TryFrom<Token> for CharLiteral {
    type Error = LexError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::CharLit(c) => Ok(c),
            _ => Err(LexError::MismatchedTokenType),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteral(pub Literal<String>);

// convert `Token` to inner `StringLiteral`
impl TryFrom<Token> for StringLiteral {
    type Error = LexError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::StringLit(s) => Ok(s),
            _ => Err(LexError::MismatchedTokenType),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntLiteral(pub Literal<i64>);

// convert `Token` to inner `IntLiteral`
impl TryFrom<Token> for IntLiteral {
    type Error = LexError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::IntLit(i) => Ok(i),
            _ => Err(LexError::MismatchedTokenType),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UIntLiteral(pub Literal<u64>);

// convert `Token` to inner `U256Literal`
impl TryFrom<Token> for UIntLiteral {
    type Error = LexError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::UIntLit(u) => Ok(u),
            _ => Err(LexError::MismatchedTokenType),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FloatLiteral(pub Literal<f64>);

// convert `Token` to inner `FloatLiteral`
impl TryFrom<Token> for FloatLiteral {
    type Error = LexError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::FloatLit(f) => Ok(f),
            _ => Err(LexError::MismatchedTokenType),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoolLiteral(pub Literal<bool>);

// convert `Token` to inner `BoolLiteral`
impl TryFrom<Token> for BoolLiteral {
    type Error = LexError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::BoolLit(b) => Ok(b),
            _ => Err(LexError::MismatchedTokenType),
        }
    }
}
