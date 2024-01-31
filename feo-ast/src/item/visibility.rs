use feo_types::{
    span::{Span, Spanned},
    utils::{KwCrate, KwPub, Parenthesis},
};

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

pub struct PubCrateVisibility {
    kw_pub: KwPub,
    open_parenthesis: Parenthesis,
    kw_crate: KwCrate,
    close_parenthesis: Parenthesis,
}

impl Spanned for PubCrateVisibility {
    fn span(&self) -> Span {
        let start_pos = self.kw_pub.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.kw_pub.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
