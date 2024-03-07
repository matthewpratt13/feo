use feo_types::{
    span::{Span, Spanned},
    utils::{KwCrate, KwPub, Parenthesis},
};

#[derive(Debug, Clone)]
pub enum VisibilityKind {
    Pub(KwPub),
    PubCrate(PubCrateVisibility),
}

impl Spanned for VisibilityKind {
    fn span(&self) -> Span {
        match self {
            VisibilityKind::Pub(p) => p.span(),
            VisibilityKind::PubCrate(pc) => pc.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PubCrateVisibility {
    kw_pub: KwPub,
    open_parenthesis: Parenthesis,
    kw_crate: KwCrate,
    close_parenthesis: Parenthesis,
}

impl Spanned for PubCrateVisibility {
    fn span(&self) -> Span {
        let s1 = self.kw_pub.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}
