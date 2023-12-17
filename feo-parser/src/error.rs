#[derive(Debug)]
pub enum LexError {
    MismatchedTokenType,
}

#[derive(Debug)]
pub enum ParserError {
    InvalidEscapeCode,
    InvalidKeyword,
    InvalidDelimiter,
    InvalidPunctuation,
    UnexpectedChar,
    ParseBoolError,
    ParseIntError,
    ParseFloatError,
    ParseCharError,
}
