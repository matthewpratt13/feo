use std::str::FromStr;

use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    type_error::{TypeError, TypeErrorKind},
};

use feo_types::span::{Position, Span, Spanned};

use crate::token::{Token, Tokenize};

#[derive(Debug, Clone, PartialEq)]
pub enum KeywordKind {
    KwAbstract,
    KwAs,
    KwBreak,
    KwConst,
    KwContinue,
    KwContract,
    KwCrate,
    KwDeref, // replaces dereference operator ('*')
    KwElse,
    KwEnum,
    KwExtern,
    KwFor,
    KwFunc,
    KwIf,
    KwImpl,
    KwImport,
    KwIn,
    KwInterface,
    KwLet,
    KwLibrary,
    KwLoop,
    KwMatch,
    KwMod,
    KwMut,
    KwNone,
    KwProgram,
    KwPub,
    KwRef, // replaces reference operator ('&')
    KwReturn,
    KwScript,
    KwSelf,
    KwSome,
    KwStatic,
    KwStorage,
    KwStruct,
    KwSuper, // only relevant in path expressions
    KwTrait,
    KwType,
    KwWhile,
}

impl KeywordKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            KeywordKind::KwAbstract => "abstract",
            KeywordKind::KwAs => "as",
            KeywordKind::KwBreak => "break",
            KeywordKind::KwConst => "const",
            KeywordKind::KwContinue => "continue",
            KeywordKind::KwContract => "contract",
            KeywordKind::KwCrate => "crate",
            KeywordKind::KwDeref => "deref",
            KeywordKind::KwElse => "else",
            KeywordKind::KwEnum => "enum",
            KeywordKind::KwExtern=> "extern",
            KeywordKind::KwFor => "for",
            KeywordKind::KwFunc => "func",
            KeywordKind::KwIf => "if",
            KeywordKind::KwImpl => "impl",
            KeywordKind::KwImport => "import",
            KeywordKind::KwIn => "in",
            KeywordKind::KwInterface => "interface",
            KeywordKind::KwLet => "let",
            KeywordKind::KwLibrary => "library",
            KeywordKind::KwLoop => "loop",
            KeywordKind::KwMatch => "match",
            KeywordKind::KwMod => "mod",
            KeywordKind::KwMut => "mut",
            KeywordKind::KwNone => "None",
            KeywordKind::KwProgram => "program",
            KeywordKind::KwPub => "pub",
            KeywordKind::KwRef => "ref",
            KeywordKind::KwReturn => "return",
            KeywordKind::KwScript => "script",
            KeywordKind::KwSelf => "self",
            KeywordKind::KwSome => "Some",
            KeywordKind::KwStatic => "static",
            KeywordKind::KwStorage => "storage",
            KeywordKind::KwStruct => "struct",
            KeywordKind::KwSuper => "super",
            KeywordKind::KwTrait => "trait",
            KeywordKind::KwType => "type",
            KeywordKind::KwWhile => "while",
        }
    }
}

impl FromStr for KeywordKind {
    type Err = TypeErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let keyword_kind = match s {
            "abstract" => Ok(KeywordKind::KwAbstract),
            "as" => Ok(KeywordKind::KwAs),
            "break" => Ok(KeywordKind::KwBreak),
            "const" => Ok(KeywordKind::KwConst),
            "continue" => Ok(KeywordKind::KwContinue),
            "contract" => Ok(KeywordKind::KwContract),
            "crate" => Ok(KeywordKind::KwCrate),
            "deref" => Ok(KeywordKind::KwDeref),
            "else" => Ok(KeywordKind::KwElse),
            "enum" => Ok(KeywordKind::KwEnum),
            "extern" => Ok(KeywordKind::KwExtern),
            "for" => Ok(KeywordKind::KwFor),
            "func" => Ok(KeywordKind::KwFunc),
            "if" => Ok(KeywordKind::KwIf),
            "impl" => Ok(KeywordKind::KwImpl),
            "import" => Ok(KeywordKind::KwImport),
            "in" => Ok(KeywordKind::KwIn),
            "interface" => Ok(KeywordKind::KwInterface),
            "let" => Ok(KeywordKind::KwLet),
            "library" => Ok(KeywordKind::KwLibrary),
            "loop" => Ok(KeywordKind::KwLoop),
            "match" => Ok(KeywordKind::KwMatch),
            "mod" => Ok(KeywordKind::KwMod),
            "mut" => Ok(KeywordKind::KwMut),
            "None" => Ok(KeywordKind::KwNone),
            "program" => Ok(KeywordKind::KwProgram),
            "pub" => Ok(KeywordKind::KwPub),
            "ref" => Ok(KeywordKind::KwRef),
            "return" => Ok(KeywordKind::KwReturn),
            "script" => Ok(KeywordKind::KwScript),
            "self" => Ok(KeywordKind::KwSelf),
            "Some" => Ok(KeywordKind::KwSome),
            "static" => Ok(KeywordKind::KwStatic),
            "storage" => Ok(KeywordKind::KwStorage),
            "struct" => Ok(KeywordKind::KwStruct),
            "super" => Ok(KeywordKind::KwSuper),
            "trait" => Ok(KeywordKind::KwTrait),
            "type" => Ok(KeywordKind::KwType),
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

impl Tokenize for Keyword {
    fn tokenize(
        src: &str,
        content: &str,
        start: usize,
        end: usize,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let error = TypeError {
            error_kind: TypeErrorKind::UnrecognizedKeyword,
            position: Position::new(src, start),
        };

        // convert `TypeErrorKind` to `CompilerError::Type(TypeError)`
        let keyword_kind = KeywordKind::from_str(content)
            .map_err(|_| handler.emit_err(CompilerError::Type(error)))?;

        let keyword = Keyword::new(keyword_kind, span);

        let token = Token::Keyword(keyword);

        Ok(Some(token))
    }
}

impl Spanned for Keyword {
    fn span(&self) -> &Span {
        &self.span
    }
}
