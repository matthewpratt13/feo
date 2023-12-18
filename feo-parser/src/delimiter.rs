use feo_error::lex_error::LexErrorKind;
use feo_types::Delimiter;

use crate::lexer::Token;

// convert `Token` to inner `IntLiteral`
impl TryFrom<Token> for Delimiter {
    type Error = LexErrorKind;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Delim(d) => Ok(d),
            _ => Err(LexErrorKind::MismatchedDelimiters),
        }
    }
}
