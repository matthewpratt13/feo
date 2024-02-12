use feo_types::{
    span::{Span, Spanned},
    utils::{Bracket, HashBang, HashSign, KwAbstract, KwExport, KwExtern, KwUnsafe},
};

use crate::path::SimplePath;

#[derive(Clone)]
pub enum AttributeKind {
    KwAbstract(KwAbstract),
    KwExport(KwExport),
    KwExtern(KwExtern),
    KwUnsafe(KwUnsafe),

    Path(SimplePath),
}

#[derive(Clone)]
pub struct InnerAttr {
    hash_bang: HashBang,
    open_bracket: Bracket,
    attribute: AttributeKind,
    close_bracket: Bracket,
}

impl Spanned for InnerAttr {
    fn span(&self) -> Span {
        let s1 = self.hash_bang.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct OuterAttr {
    pub hash: HashSign,
    pub open_bracket: Bracket,
    pub attribute: AttributeKind,
    pub close_bracket: Bracket,
}

impl Spanned for OuterAttr {
    fn span(&self) -> Span {
        let s1 = self.hash.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)
    }
}
