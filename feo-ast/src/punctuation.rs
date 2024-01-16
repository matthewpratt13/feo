use std::str::FromStr;

use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    type_error::{TypeError, TypeErrorKind},
};

use feo_types::span::{Position, Span, Spanned};

use crate::{
    expression::{ExprWithoutBlock, Expression, RangeExpr},
    token::{Token, Tokenize},
};

#[derive(Debug, Clone, PartialEq)]
pub enum PuncKind {
    Colon,
    Semicolon,
    Comma,
    FullStop,
    Underscore,

    DblDot,
    DotDotEquals,
    DblColon,

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
    Backslash,
    Caret,
    BackTick,
    Pipe,
    Tilde,

    HashBang,
    BangEquals,
    PercentEquals,
    AsteriskEquals,
    DblAsterisk, // (exponent)
    DblAmpersand,
    PlusEquals,
    MinusEquals,
    ForwardSlashEquals,
    DblLessThan,
    LessThanEquals,
    DblEquals,
    DblGreaterThan,
    GreaterThanEquals,
    ThinArrow, // "->"
    FatArrow,  // "=>"
    DblPipe,
}

impl PuncKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            PuncKind::Colon => ":",
            PuncKind::Semicolon => ";",
            PuncKind::Comma => ",",
            PuncKind::FullStop => ".",
            PuncKind::Underscore => "_",
            PuncKind::DblDot => "..",
            PuncKind::DotDotEquals => "..=",
            PuncKind::DblColon => "::",
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
            PuncKind::Backslash => "\\",
            PuncKind::Caret => "^",
            PuncKind::BackTick => "`",
            PuncKind::Pipe => "|",
            PuncKind::Tilde => "~",
            PuncKind::HashBang => "#!",
            PuncKind::BangEquals => "!=",
            PuncKind::PercentEquals => "%=",
            PuncKind::AsteriskEquals => "*=",
            PuncKind::DblAsterisk => "**",
            PuncKind::DblAmpersand => "&&",
            PuncKind::PlusEquals => "+=",
            PuncKind::MinusEquals => "-=",
            PuncKind::ForwardSlashEquals => "/=",
            PuncKind::DblLessThan => "<<",
            PuncKind::LessThanEquals => "<=",
            PuncKind::DblEquals => "==",
            PuncKind::DblGreaterThan => ">>",
            PuncKind::GreaterThanEquals => ">=",
            PuncKind::ThinArrow => "->",
            PuncKind::FatArrow => "=>",
            PuncKind::DblPipe => "||",
        }
    }
}

impl Expression for PuncKind {}

impl<E> ExprWithoutBlock<E> for PuncKind where E: Expression {}

impl<R> RangeExpr<R> for PuncKind where R: Expression {}

impl FromStr for PuncKind {
    type Err = TypeErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let punc_kind = match s {
            ":" => Ok(PuncKind::Colon),
            ";" => Ok(PuncKind::Semicolon),
            "," => Ok(PuncKind::Comma),
            "." => Ok(PuncKind::FullStop),
            "_" => Ok(PuncKind::Underscore),
            ".." => Ok(PuncKind::DblDot),
            "..=" => Ok(PuncKind::DotDotEquals),
            "::" => Ok(PuncKind::DblColon),
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
            "\\" => Ok(PuncKind::Backslash),
            "^" => Ok(PuncKind::Caret),
            "`" => Ok(PuncKind::BackTick),
            "|" => Ok(PuncKind::Pipe),
            "~" => Ok(PuncKind::Tilde),
            "#!" => Ok(PuncKind::HashBang),
            "!=" => Ok(PuncKind::BangEquals),
            "%=" => Ok(PuncKind::PercentEquals),
            "*=" => Ok(PuncKind::AsteriskEquals),
            "**" => Ok(PuncKind::DblAsterisk),
            "&&" => Ok(PuncKind::DblAmpersand),
            "+=" => Ok(PuncKind::PlusEquals),
            "-=" => Ok(PuncKind::MinusEquals),
            "/=" => Ok(PuncKind::ForwardSlashEquals),
            "<<" => Ok(PuncKind::DblLessThan),
            "<=" => Ok(PuncKind::LessThanEquals),
            "==" => Ok(PuncKind::DblEquals),
            ">>" => Ok(PuncKind::DblGreaterThan),
            ">=" => Ok(PuncKind::GreaterThanEquals),
            "->" => Ok(PuncKind::ThinArrow),
            "=>" => Ok(PuncKind::FatArrow),
            "||" => Ok(PuncKind::DblPipe),
            _ => Err(TypeErrorKind::UnexpectedPunctuation),
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

impl Tokenize for Punctuation {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let error = TypeError {
            error_kind: TypeErrorKind::UnexpectedPunctuation,
            position: Position::new(src, start),
        };

        // convert `TypeErrorKind` to `CompilerError::Type(TypeError)`
        let punc_kind = PuncKind::from_str(content)
            .map_err(|_| handler.emit_err(CompilerError::Type(error)))?;

        let punctuation = Punctuation::new(punc_kind, span);

        let token = Token::Punc(punctuation);

        Ok(Some(token))
    }
}

impl Spanned for Punctuation {
    fn span(&self) -> &Span {
        &self.span
    }
}

pub fn is_quote(c: char) -> bool {
    ['\'', '"'].contains(&c)
}

pub fn is_separator(c: char) -> bool {
    [',', ';'].contains(&c)
}
