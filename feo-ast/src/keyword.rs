use std::str::FromStr;
use std::sync::Arc;

use feo_error::error::{CompileError, ErrorEmitted};
use feo_error::type_error::{TypeError, TypeErrorKind};

use feo_types::span::{Span, Spanned};

use crate::token::{Token, Tokenize};

#[derive(Debug, Clone, PartialEq)]
pub enum KeywordKind {
    BreakKw,
    ConstKw,
    ContinueKw,
    ElseKw,
    EnumKw,
    ForKw,
    FuncKw,
    IfKw,
    ImplKw,
    ImportKw,
    InKw,
    LetKw,
    LoopKw,
    MatchKw,
    ModKw,
    MutKw,
    PubKw,
    RefKw, // same as reference operator ('&')
    ReturnKw,
    SelfKw,
    StaticKw,
    StructKw,
    SuperKw, // only relevant in path expressions
    TraitKw,
    TypeKw,
    WhileKw,
}

impl KeywordKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            KeywordKind::BreakKw => "break",
            KeywordKind::ConstKw => "const",
            KeywordKind::ContinueKw => "continue",
            KeywordKind::ElseKw => "else",
            KeywordKind::EnumKw => "enum",
            KeywordKind::ForKw => "for",
            KeywordKind::FuncKw => "func",
            KeywordKind::IfKw => "if",
            KeywordKind::ImplKw => "impl",
            KeywordKind::ImportKw => "import",
            KeywordKind::InKw => "in",
            KeywordKind::LetKw => "let",
            KeywordKind::LoopKw => "loop",
            KeywordKind::MatchKw => "match",
            KeywordKind::ModKw => "mod",
            KeywordKind::MutKw => "mut",
            KeywordKind::PubKw => "pub",
            KeywordKind::RefKw => "ref",
            KeywordKind::ReturnKw => "return",
            KeywordKind::SelfKw => "self",
            KeywordKind::StaticKw => "static",
            KeywordKind::StructKw => "struct",
            KeywordKind::SuperKw => "super",
            KeywordKind::TraitKw => "trait",
            KeywordKind::TypeKw => "type",
            KeywordKind::WhileKw => "while",
        }
    }
}

impl FromStr for KeywordKind {
    type Err = TypeErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let keyword_kind = match s {
            "break" => Ok(KeywordKind::BreakKw),
            "const" => Ok(KeywordKind::ConstKw),
            "continue" => Ok(KeywordKind::ContinueKw),
            "else" => Ok(KeywordKind::ElseKw),
            "enum" => Ok(KeywordKind::EnumKw),
            "for" => Ok(KeywordKind::ForKw),
            "func" => Ok(KeywordKind::FuncKw),
            "if" => Ok(KeywordKind::IfKw),
            "impl" => Ok(KeywordKind::ImplKw),
            "import" => Ok(KeywordKind::ImportKw),
            "in" => Ok(KeywordKind::InKw),
            "let" => Ok(KeywordKind::LetKw),
            "loop" => Ok(KeywordKind::LoopKw),
            "match" => Ok(KeywordKind::MatchKw),
            "mod" => Ok(KeywordKind::ModKw),
            "mut" => Ok(KeywordKind::MutKw),
            "pub" => Ok(KeywordKind::PubKw),
            "ref" => Ok(KeywordKind::RefKw),
            "return" => Ok(KeywordKind::ReturnKw),
            "self" => Ok(KeywordKind::SelfKw),
            "static" => Ok(KeywordKind::StaticKw),
            "struct" => Ok(KeywordKind::StructKw),
            "super" => Ok(KeywordKind::SuperKw),
            "trait" => Ok(KeywordKind::TraitKw),
            "type" => Ok(KeywordKind::TypeKw),
            "while" => Ok(KeywordKind::WhileKw),
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
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = TypeError {
            error_kind: TypeErrorKind::UnrecognizedKeyword,
            pos: start,
        };

        // convert `TypeErrorKind` to `CompileError::Type(TypeError)`
        let keyword_kind = KeywordKind::from_str(content)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Type(err)))?;

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
