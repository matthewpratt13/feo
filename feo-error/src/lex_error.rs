pub struct LexError {
    pub error_kind: LexErrorKind,
    pub pos: usize,
}

pub enum LexErrorKind {
    UnclosedBlockComment,
    MismatchedDelimiters,
    UnclosedDocComment,
    UnclosedDelimiter,
    UnclosedStringLiteral,
    UnclosedCharLiteral,
    MismatchedTokenType,
    NoTokenFound,
    UnopenedBlockComment,
    FinalIndex,
    InvalidEscapeSequence,
    InvalidKeyword,
    InvalidDelimiter,
    InvalidPunctuation,
    ParseBoolError,
    ParseIntError,
    ParseFloatError,
    ParseCharError,
}
