use crate::span::{Span, Spanned};

#[derive(Debug)]
pub enum PuncKind {
    Colon,
    Semicolon,
    Comma,
    FullStop,
    Underscore,

    DoubleColon,
    DoubleFullStop,
    DoubleSlash,   // trailing / line comment
    TripleSlash,   // doc comment
    SlashAsterisk, // multiline / inline comment open
    AsteriskSlash, // multiline / inline comment close

    Bang, // (exclamation point)
    Hash,
    Percent, // (modulus)
    Ampersand,
    Asterisk, // (multiply)
    Plus,
    Minus,
    ForwardSlash, // (divide)
    LessThan,
    Equals,
    GreaterThan,
    QuestionMark,
    AtSign,
    Pipe,

    BangEquals,
    PercentEquals,
    AsteriskEquals,
    DoubleAsterisk, // (exponent)
    DoubleAmpersand,
    PlusEquals,
    MinusEquals,
    ForwardSlashEquals,
    LessThanEquals,
    DoubleEquals,
    GreaterThanEquals,
    ThinArrow, // "->"
    FatArrow,  // "=>"
    DoublePipe,

    // escape codes
    Newline,
    Return,
    Tab,
    Backslash,
    Null,
    SingleQuote,
    DoubleQuote,
}

#[derive(Debug)]
pub struct Punctuation {
    punc_kind: PuncKind,
    span: Span,
}

impl Spanned for Punctuation {
    fn span(&self) -> &Span {
        &self.span
    }
}
