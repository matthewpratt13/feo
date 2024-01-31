use std::error::Error;
use std::fmt;

use crate::{delimiter::Delimiter, punctuation::Punctuation, Keyword};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum TypeErrorKind {
    UnrecognizedCommentOpener,
    UnrecognizedDelimiter,
    UnrecognizedKeyword,
    UnexpectedPunctuation,
    InvalidTypeAnnotation,

    #[default]
    UnknownError,
}

impl fmt::Display for TypeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeErrorKind::UnrecognizedCommentOpener => {
                write!(f, "unrecognized comment open sequence")
            }
            TypeErrorKind::UnrecognizedDelimiter => write!(f, "unrecognized delimiter"),
            TypeErrorKind::UnrecognizedKeyword => write!(f, "unrecognized keyword"),
            TypeErrorKind::UnexpectedPunctuation => write!(f, "unexpected punctuation"),
            TypeErrorKind::InvalidTypeAnnotation => write!(f, "invalid type annotation"),
            TypeErrorKind::UnknownError => write!(f, "unknown error"),
        }
    }
}

impl Error for TypeErrorKind {}

pub type Brace = Delimiter;
pub type Bracket = Delimiter;
pub type Parenthesis = Delimiter;

pub type KwAbstract = Keyword;
pub type KwAs = Keyword;
pub type KwConst = Keyword;
pub type KwCrate = Keyword;
pub type KwDeref = Keyword;
pub type KwElse = Keyword;
pub type KwEnum = Keyword;
pub type KwExport = Keyword;
pub type KwExtern = Keyword;
pub type KwFor = Keyword;
pub type KwFunc = Keyword;
pub type KwIf = Keyword;
pub type KwImpl = Keyword;
pub type KwImport = Keyword;
pub type KwIn = Keyword;
pub type KwLet = Keyword;
pub type KwLoop = Keyword;
pub type KwMatch = Keyword;
pub type KwMod = Keyword;
pub type KwMut = Keyword;
pub type KwPub = Keyword;
pub type KwRef = Keyword;
pub type KwReturn = Keyword;
pub type KwSelf = Keyword;
pub type KwSelfType = Keyword;
pub type KwStatic = Keyword;
pub type KwStruct = Keyword;
pub type KwSuper = Keyword;
pub type KwTrait = Keyword;
pub type KwType = Keyword;
pub type KwUnsafe = Keyword;
pub type KwWhile = Keyword;

pub type Ampersand = Keyword;
pub type Asterisk = Punctuation;
pub type AsteriskEquals = Punctuation;
pub type Bang = Punctuation;
pub type BangEquals = Punctuation;
pub type Colon = Punctuation;
pub type Comma = Punctuation;
pub type DblAmpersand = Punctuation;
pub type DblColon = Punctuation;
pub type DblDot = Punctuation;
pub type DblEquals = Punctuation;
pub type DblPipe = Punctuation;
pub type Dot = Punctuation;
pub type DotDotEquals = Punctuation;
pub type Equals = Punctuation;
pub type FatArrow = Punctuation;
pub type ForwardSlash = Punctuation;
pub type GreaterThan = Punctuation;
pub type GreaterThanEquals = Punctuation;
pub type HashBang = Punctuation;
pub type HashSign = Punctuation;
pub type LessThan = Punctuation;
pub type LessThanEquals = Punctuation;
pub type Minus = Punctuation;
pub type MinusEquals = Punctuation;
pub type Percent = Punctuation;
pub type PercentEquals = Punctuation;
pub type Pipe = Punctuation;
pub type Plus = Punctuation;
pub type PlusEquals = Punctuation;
pub type QuestionMark = Punctuation;
pub type Semicolon = Punctuation;
pub type ForwardSlashEquals = Punctuation;
pub type ThinArrow = Punctuation;
pub type Underscore = Punctuation;
