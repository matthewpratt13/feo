use core::str::FromStr;

use crate::{
    error::TypeErrorKind,
    span::{Span, Spanned},
};

#[derive(Debug, Clone, PartialEq)]
pub enum KeywordKind {
    BreakKw,
    ConstKw,
    ContinueKw,
    DerefKw, // same as deref operator ('*')
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

impl FromStr for KeywordKind {
    type Err = TypeErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let keyword_kind = match s {
            "break" => Ok(KeywordKind::BreakKw),
            "const" => Ok(KeywordKind::ConstKw),
            "continue" => Ok(KeywordKind::ContinueKw),
            "deref" => Ok(KeywordKind::DerefKw),
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

impl Spanned for Keyword {
    fn span(&self) -> &Span {
        &self.span
    }
}
