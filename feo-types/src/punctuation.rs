use core::str::FromStr;

use crate::error::TypeErrorKind;
use crate::span::{Span, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub enum PuncKind {
    Colon,
    Semicolon,
    Comma,
    FullStop,
    Underscore,

    DoubleFullStop,
    DoubleColon,

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
}

impl PuncKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            PuncKind::Colon => ":",
            PuncKind::Semicolon => ";",
            PuncKind::Comma => ",",
            PuncKind::FullStop => ".",
            PuncKind::Underscore => "_",
            PuncKind::DoubleFullStop => "..",
            PuncKind::DoubleColon => "::",
            PuncKind::Bang => "!",
            PuncKind::Hash => "#",
            PuncKind::DollarSign => "$",
            PuncKind::Percent => "%",
            PuncKind::Ampersand => "&",
            PuncKind::Asterisk => "*",
            PuncKind::Plus => "+",
            PuncKind::Minus => "-",
            PuncKind::ForwardSlash => "/",
            PuncKind::LessThan => "<",
            PuncKind::Equals => "=",
            PuncKind::GreaterThan => ">",
            PuncKind::QuestionMark => "?",
            PuncKind::AtSign => "@",
            PuncKind::Caret => "^",
            PuncKind::BackTick => "`",
            PuncKind::Pipe => "|",
            PuncKind::Tilde => "~",
            PuncKind::BangEquals => "!=",
            PuncKind::PercentEquals => "%=",
            PuncKind::AsteriskEquals => "*=",
            PuncKind::DoubleAsterisk => "**",
            PuncKind::DoubleAmpersand => "&&",
            PuncKind::PlusEquals => "+=",
            PuncKind::MinusEquals => "-=",
            PuncKind::ForwardSlashEquals => "/=",
            PuncKind::LessThanEquals => "<=",
            PuncKind::DoubleEquals => "==",
            PuncKind::GreaterThanEquals => ">=",
            PuncKind::ThinArrow => "->",
            PuncKind::FatArrow => "=>",
            PuncKind::DoublePipe => "||",
        }
    }
}

impl FromStr for PuncKind {
    type Err = TypeErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let punc_kind = match s {
            ":" => Ok(PuncKind::Colon),
            ";" => Ok(PuncKind::Semicolon),
            "," => Ok(PuncKind::Comma),
            "." => Ok(PuncKind::FullStop),
            "_" => Ok(PuncKind::Underscore),
            ".." => Ok(PuncKind::DoubleFullStop),
            "::" => Ok(PuncKind::DoubleColon),
            "!" => Ok(PuncKind::Bang),
            "#" => Ok(PuncKind::Hash),
            "$" => Ok(PuncKind::DollarSign),
            "%" => Ok(PuncKind::Percent),
            "&" => Ok(PuncKind::Ampersand),
            "*" => Ok(PuncKind::Asterisk),
            "+" => Ok(PuncKind::Plus),
            "-" => Ok(PuncKind::Minus),
            "/" => Ok(PuncKind::ForwardSlash),
            "<" => Ok(PuncKind::LessThan),
            "=" => Ok(PuncKind::Equals),
            ">" => Ok(PuncKind::GreaterThan),
            "?" => Ok(PuncKind::QuestionMark),
            "@" => Ok(PuncKind::AtSign),
            "^" => Ok(PuncKind::Caret),
            "`" => Ok(PuncKind::BackTick),
            "|" => Ok(PuncKind::Pipe),
            "~" => Ok(PuncKind::Tilde),
            "!=" => Ok(PuncKind::BangEquals),
            "%=" => Ok(PuncKind::PercentEquals),
            "*=" => Ok(PuncKind::AsteriskEquals),
            "**" => Ok(PuncKind::DoubleAsterisk),
            "&&" => Ok(PuncKind::DoubleAmpersand),
            "+=" => Ok(PuncKind::PlusEquals),
            "-=" => Ok(PuncKind::MinusEquals),
            "/=" => Ok(PuncKind::ForwardSlashEquals),
            "<=" => Ok(PuncKind::LessThanEquals),
            "==" => Ok(PuncKind::DoubleEquals),
            ">=" => Ok(PuncKind::GreaterThanEquals),
            "->" => Ok(PuncKind::ThinArrow),
            "=>" => Ok(PuncKind::FatArrow),
            "||" => Ok(PuncKind::DoublePipe),
            _ => Err(TypeErrorKind::UnrecognizedPunctuation),
        }?;

        Ok(punc_kind)
    }
}

#[derive(Debug, Clone)]
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
