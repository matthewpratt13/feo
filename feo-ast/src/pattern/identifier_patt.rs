use feo_types::{
    span::{Span, Spanned},
    utils::{KwMut, KwRef},
    Identifier,
};

#[derive(Debug, Clone)]
pub struct IdentifierPatt {
    pub kw_ref_opt: Option<KwRef>,
    pub kw_mut_opt: Option<KwMut>,
    pub name: Identifier,
}

impl Spanned for IdentifierPatt {
    fn span(&self) -> Span {
        let s1 = match &self.kw_ref_opt {
            Some(kwr) => kwr.span(),
            None => match &self.kw_mut_opt {
                Some(kwm) => kwm.span(),
                None => self.name.span(),
            },
        };

        let s2 = self.name.span();

        Span::join(s1, s2)
    }
}
