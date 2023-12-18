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
