use feo_types::span::Position;

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
pub struct LexError<'a> {
    pub error_kind: LexErrorKind,
    pub pos: Option<Position<'a>>,
}
