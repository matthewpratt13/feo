#[derive(Debug)]
pub enum LexError {
    FinalIndex,
    MismatchedDelimiter,
    MismatchedTokenType,
    NoTokenFound,
    NoTokenTreeFound,
}

#[derive(Debug)]
pub enum ParserError {
    InvalidDelimiter,
    InvalidEscapeSequence,
    InvalidKeyword,
    InvalidPunctuation,
    ParseBoolError,
    ParseCharError,
    ParseFloatError,
    ParseIntError,
    UnexpectedChar,
}
