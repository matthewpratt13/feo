use std::str::FromStr;

use crate::span::{Span, Spanned};

#[derive(Debug, Clone)]
pub enum KeywordKind {
    AsKw,
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
    ImportKw, // same as "use"
    InKw,
    LetKw,
    LibraryKw, // same as "crate"
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let keyword_kind = match s {
            "as" => Ok(KeywordKind::AsKw),
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
            "library" => Ok(KeywordKind::LibraryKw),
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
            _ => Err(()),
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

#[derive(Debug, Clone)]
pub enum TypeName {
    BoolType,
    CharType,
    F32Type,
    F64Type,
    I32Type,
    I64Type,
    StringType,
    U8Type,
    U16Type,
    U32Type,
    U64Type,

    CustomType(String),
}

impl FromStr for TypeName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let type_name = match s {
            "bool" => Ok(TypeName::BoolType),
            "char" => Ok(TypeName::CharType),
            "f32" => Ok(TypeName::F32Type),
            "f64" => Ok(TypeName::F64Type),
            "i32" => Ok(TypeName::I32Type),
            "i64" => Ok(TypeName::I64Type),
            "String" => Ok(TypeName::StringType),
            "u8" => Ok(TypeName::U8Type),
            "u16" => Ok(TypeName::U16Type),
            "u32" => Ok(TypeName::U32Type),
            "u64" => Ok(TypeName::U32Type),
            _ => Ok(TypeName::CustomType(s.to_string())),
        }?;

        Ok(type_name)
    }
}

#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    pub type_name: TypeName,
    span: Span,
}

impl TypeAnnotation {
    pub fn new(type_name: TypeName, span: Span) -> Self {
        Self { type_name, span }
    }
}

impl Spanned for TypeAnnotation {
    fn span(&self) -> &Span {
        &self.span
    }
}
