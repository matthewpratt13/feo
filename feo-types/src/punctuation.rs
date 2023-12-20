use std::str::FromStr;

use crate::error::TypeError;
use crate::span::{Span, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub enum PuncKind {
    Colon,
    Semicolon,
    Comma,
    FullStop,
    Underscore,

    DoubleColon,
    DoubleFullStop,

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

impl FromStr for PuncKind {
    type Err = TypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let punc_kind = match s {
            ":" => Ok(PuncKind::Colon),
            ";" => Ok(PuncKind::Semicolon),
            "," => Ok(PuncKind::Comma),
            "." => Ok(PuncKind::FullStop),
            "_" => Ok(PuncKind::Underscore),
            ".." => Ok(PuncKind::DoubleFullStop),
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
            "\n" => Ok(PuncKind::Newline),
            "\r" => Ok(PuncKind::Return),
            "\t" => Ok(PuncKind::Tab),
            "\\" => Ok(PuncKind::Backslash),
            "\0" => Ok(PuncKind::Null),
            "\'" => Ok(PuncKind::SingleQuote),
            "\"" => Ok(PuncKind::DoubleQuote),
            _ => Err(TypeError::UnrecognizedPunctuation),
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
