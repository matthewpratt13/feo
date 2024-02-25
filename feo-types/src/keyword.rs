use std::str::FromStr;

use crate::{
    error::TypeErrorKind,
    span::{Span, Spanned},
};

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordKind {
    KwAbi,
    KwAbstract,
    KwAs,
    KwBreak,
    KwConst,
    KwContinue,
    KwContract,
    KwCrate,
    KwElse,
    KwEnum,
    KwExport,
    KwExtern,
    KwFor,
    KwFunc,
    KwIf,
    KwImpl,
    KwImport,
    KwIn,
    KwLet,
    KwLibrary,
    KwLoop,
    KwMatch,
    KwMod,
    KwMut,
    KwPayable,
    KwPub,
    KwRef,
    KwReturn,
    KwScript,
    KwSelf,
    KwSelfType,
    KwStatic,
    KwStorage,
    KwStruct,
    KwSuper,
    KwTest,
    KwTopic,
    KwTrait,
    KwType,
    KwUnsafe,
    KwWhere,
    KwWhile,
}

impl KeywordKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            KeywordKind::KwAbi => "abi",
            KeywordKind::KwAbstract => "abstract",
            KeywordKind::KwAs => "as",
            KeywordKind::KwBreak => "break",
            KeywordKind::KwConst => "const",
            KeywordKind::KwContinue => "continue",
            KeywordKind::KwContract => "contract",
            KeywordKind::KwCrate => "crate",
            KeywordKind::KwElse => "else",
            KeywordKind::KwEnum => "enum",
            KeywordKind::KwExport => "export",
            KeywordKind::KwExtern => "extern",
            KeywordKind::KwFor => "for",
            KeywordKind::KwFunc => "func",
            KeywordKind::KwIf => "if",
            KeywordKind::KwImpl => "impl",
            KeywordKind::KwImport => "import",
            KeywordKind::KwIn => "in",
            KeywordKind::KwLet => "let",
            KeywordKind::KwLibrary => "library", // TODO: remove
            KeywordKind::KwLoop => "loop",
            KeywordKind::KwMatch => "match",
            KeywordKind::KwMod => "mod",
            KeywordKind::KwMut => "mut",
            KeywordKind::KwPayable => "payable",
            KeywordKind::KwPub => "pub",
            KeywordKind::KwRef => "ref",
            KeywordKind::KwReturn => "return",
            KeywordKind::KwScript => "script", // TODO: remove
            KeywordKind::KwSelf => "self",
            KeywordKind::KwSelfType => "Self",
            KeywordKind::KwStatic => "static",
            KeywordKind::KwStorage => "storage",
            KeywordKind::KwStruct => "struct",
            KeywordKind::KwSuper => "super",
            KeywordKind::KwTest => "test",
            KeywordKind::KwTopic => "topic",
            KeywordKind::KwTrait => "trait",
            KeywordKind::KwType => "type",
            KeywordKind::KwUnsafe => "unsafe",
            KeywordKind::KwWhere => "where",
            KeywordKind::KwWhile => "while",
        }
    }
}

impl FromStr for KeywordKind {
    type Err = TypeErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let keyword_kind = match s {
            "abi" => Ok(KeywordKind::KwAbi),
            "abstract" => Ok(KeywordKind::KwAbstract),
            "as" => Ok(KeywordKind::KwAs),
            "break" => Ok(KeywordKind::KwBreak),
            "const" => Ok(KeywordKind::KwConst),
            "continue" => Ok(KeywordKind::KwContinue),
            "contract" => Ok(KeywordKind::KwContract),
            "crate" => Ok(KeywordKind::KwCrate),
            "else" => Ok(KeywordKind::KwElse),
            "enum" => Ok(KeywordKind::KwEnum),
            "export" => Ok(KeywordKind::KwExport),
            "extern" => Ok(KeywordKind::KwExtern),
            "for" => Ok(KeywordKind::KwFor),
            "func" => Ok(KeywordKind::KwFunc),
            "if" => Ok(KeywordKind::KwIf),
            "impl" => Ok(KeywordKind::KwImpl),
            "import" => Ok(KeywordKind::KwImport),
            "in" => Ok(KeywordKind::KwIn),
            "let" => Ok(KeywordKind::KwLet),
            "library" => Ok(KeywordKind::KwLibrary), // TODO: remove
            "loop" => Ok(KeywordKind::KwLoop),
            "match" => Ok(KeywordKind::KwMatch),
            "mod" => Ok(KeywordKind::KwMod),
            "mut" => Ok(KeywordKind::KwMut),
            "payable" => Ok(KeywordKind::KwPayable),
            "pub" => Ok(KeywordKind::KwPub),
            "ref" => Ok(KeywordKind::KwRef),
            "return" => Ok(KeywordKind::KwReturn),
            "script" => Ok(KeywordKind::KwScript), // TODO: remove
            "self" => Ok(KeywordKind::KwSelf),
            "Self" => Ok(KeywordKind::KwSelfType),
            "static" => Ok(KeywordKind::KwStatic),
            "storage" => Ok(KeywordKind::KwStorage),
            "struct" => Ok(KeywordKind::KwStruct),
            "super" => Ok(KeywordKind::KwSuper),
            "test" => Ok(KeywordKind::KwTest),
            "topic" => Ok(KeywordKind::KwTopic),
            "trait" => Ok(KeywordKind::KwTrait),
            "type" => Ok(KeywordKind::KwType),
            "unsafe" => Ok(KeywordKind::KwUnsafe),
            "where" => Ok(KeywordKind::KwWhere),
            "while" => Ok(KeywordKind::KwWhile),
            _ => Err(TypeErrorKind::UnrecognizedKeyword),
        }?;

        Ok(keyword_kind)
    }
}

#[derive(Debug, Clone)]
pub struct Keyword {
    pub keyword_kind: KeywordKind,
    span: Span,
}

impl Keyword {
    pub fn new(keyword_kind: KeywordKind, span: Span) -> Self {
        Self { keyword_kind, span }
    }
}

impl Spanned for Keyword {
    fn span(&self) -> Span {
        self.clone().span
    }
}
