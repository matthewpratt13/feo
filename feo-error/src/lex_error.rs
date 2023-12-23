#[derive(Debug, Clone)]
pub enum LexErrorKind {
    UninitializedBlockComment,
    UnclosedDelimiters,
    EmptyCharLiteral,
    ExpectedCharLiteral,
    ExpectedClosingSingleQuote,
    UnexpectedIdentifier(String),
    InvalidEscapeSequence,
    InvalidPunctuation,
    ExpectedEscapeSequence,
    MismatchedTokenType,
    ParseCharError,
    ParseBoolError,
    ParseIntError,
    ParseUIntError,
    ParseFloatError,
    ParseDelimError,
    ParseDocCommentError,
    ParseTypeAnnError,
    InvalidChar(char),
    UnexpectedChar(char),
}

#[derive(Debug, Clone)]
pub struct LexError {
    pub error_kind: LexErrorKind,
    pub pos: usize,
}
