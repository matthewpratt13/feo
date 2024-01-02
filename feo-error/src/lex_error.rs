use feo_types::span::Position;

#[derive(Debug, Clone)]
pub enum LexErrorKind {
    UnclosedDelimiters,
    UnclosedStringLiteral,
    EmptyCharLiteral,
    InvalidCharLiteral,
    ExpectedCharLiteral,
    ExpectedClosingSingleQuote,
    InvalidEscapeSequence,
    ExpectedEscapeSequence,

    InvalidChar(char),
}

#[derive(Debug, Clone)]
pub struct LexError {
    pub error_kind: LexErrorKind,
    pub position: Position,
}
