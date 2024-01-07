use feo_types::span::Position;

#[derive(Default, Debug, Copy, Clone)]
pub enum LexErrorKind {
    UnclosedDelimiters,
    UnclosedStringLiteral,
    EmptyCharLiteral,
    InvalidCharLiteral,
    ExpectedCharLiteral,
    ExpectedClosingSingleQuote,
    InvalidEscapeSequence,
    ExpectedEscapeSequence,
    TypeAnnotationError,

    InvalidChar(char),

    #[default]
    UnknownError,
}

#[derive(Default, Debug, Clone)]
pub struct LexError {
    pub error_kind: LexErrorKind,
    pub position: Position,
}
