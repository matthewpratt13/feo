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
    SlashBang,     // doc comment
    AsteriskSlash, // multiline / inline comment close

    Bang, // (exclamation point)
    Hash,
    DollarSign,
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
    Caret,
    BackTick,
    Pipe,
    Tilde,

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

    // escape sequences
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
    pub punc_kind: PuncKind,
    span: Span,
}

impl Punctuation {
    pub fn new(punc_kind: PuncKind, span: Span) -> Self {
        Self { punc_kind, span }
    }
}

impl Spanned for Punctuation {
    fn span(&self) -> &Span {
        &self.span
    }
}
