#[derive(Debug, Clone)]
pub enum LexErrorKind {
    UnclosedDelimiters,
    UnclosedStringLiteral,
    EmptyCharLiteral,
    ExpectedCharLiteral,
    InvalidEscapeSequence,
    InvalidPunctuation,
    ExpectedEscapeSequence,
    MismatchedTokenType,

    InvalidChar(char),
}

#[derive(Debug, Clone)]
pub struct LexError {
    pub error_kind: LexErrorKind,
    pub pos: usize,
}
