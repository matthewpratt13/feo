use feo_types::Delimiter;

use crate::{error::LexError, lexer::Token};

// convert `Token` to inner `IntLiteral`
impl TryFrom<Token> for Delimiter {
    type Error = LexError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Delim(d) => Ok(d),
            _ => Err(LexError::MismatchedDelimiter),
        }
    }
}
