use feo_types::{
    span::{Span, Spanned},
    type_utils::{KwPackage, KwPub, Parenthesis},
};

#[derive(Debug, Clone)]
pub enum VisibilityKind {
    Pub(KwPub),
    PubPackage(PubPackageVisibility),
}

impl Spanned for VisibilityKind {
    fn span(&self) -> Span {
        match self {
            VisibilityKind::Pub(p) => p.span(),
            VisibilityKind::PubPackage(pc) => pc.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PubPackageVisibility {
    pub kw_pub: KwPub,
    pub open_parenthesis: Parenthesis,
    pub kw_package: KwPackage,
    pub close_parenthesis: Parenthesis,
}

impl Spanned for PubPackageVisibility {
    fn span(&self) -> Span {
        let s1 = self.kw_pub.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}
