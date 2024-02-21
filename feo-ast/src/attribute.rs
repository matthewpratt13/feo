use feo_types::{
    span::{Span, Spanned},
    utils::{
        Bracket, HashBang, HashSign, KwAbstract, KwContract, KwExport, KwExtern, KwPayable,
        KwStorage, KwTopic, KwUnsafe,
    },
};

use crate::path::SimplePathSegmentKind;

#[derive(Debug, Clone)]
pub enum AttributeKind {
    KwAbstract(KwAbstract),
    KwContract(KwContract),
    KwExport(KwExport),
    KwExtern(KwExtern),
    KwPayable(KwPayable),
    KwStorage(KwStorage),
    KwTopic(KwTopic),
    KwUnsafe(KwUnsafe),

    Path(SimplePathSegmentKind),
}

#[derive(Debug, Clone)]
pub struct InnerAttr {
    pub hash_bang: HashBang,
    pub open_bracket: Bracket,
    pub attribute: AttributeKind,
    pub close_bracket: Bracket,
}

impl Spanned for InnerAttr {
    fn span(&self) -> Span {
        let s1 = self.hash_bang.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct OuterAttr {
    pub hash_sign: HashSign,
    pub open_bracket: Bracket,
    pub attribute: AttributeKind,
    pub close_bracket: Bracket,
}

impl Spanned for OuterAttr {
    fn span(&self) -> Span {
        let s1 = self.hash_sign.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)
    }
}
