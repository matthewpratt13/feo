use feo_types::{
    span::{Span, Spanned},
    utils::{KwMut, KwRef},
    Identifier,
};

#[derive(Debug, Clone)]
pub struct IdentifierPatt {
    kw_ref_opt: Option<KwRef>,
    kw_mut_opt: Option<KwMut>,
    name: Identifier,
}

impl Spanned for IdentifierPatt {
    fn span(&self) -> Span {
        let start_pos = match &self.kw_ref_opt {
            Some(kwr) => kwr.span().start(),
            None => match &self.kw_mut_opt {
                Some(kwm) => kwm.span().start(),
                None => self.name.span().start(),
            },
        };

        let end_pos = self.name.span().end();
        let source = self.name.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
