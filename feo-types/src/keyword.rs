use crate::span::{Span, Spanned};

#[derive(Debug)]
pub enum KeywordKind {
    AbstractKw,
    AsKw,
    BoolKw,
    BreakKw,
    CharKw,
    ConstKw,
    ContractKw, // new type (fixed object with basic trait implementations)
    ContinueKw,
    DerefKw, // same as deref operator ('*')
    ElseKw,
    EnumKw,
    F32Kw,
    F64Kw,
    FalseKw,
    ForKw,
    FuncKw,
    I32Kw,
    I64Kw,
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
    StringKw,
    StructKw,
    SuperKw, // only relevant in path expressions
    TraitKw,
    TrueKw,
    TypeKw,
    U8Kw,
    U16Kw,
    U32Kw,
    U64Kw,
    U256Kw,
    WhileKw,
}

#[derive(Debug)]
pub struct Keyword {
    pub keyword_kind: KeywordKind,
    span: Span,
}

impl Spanned for Keyword {
    fn span(&self) -> &Span {
        &self.span
    }
}
