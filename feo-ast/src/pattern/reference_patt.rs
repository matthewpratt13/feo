use feo_types::{
    span::{Span, Spanned},
    utils::{KwMut, KwRef},
};

use super::PatternWithoutRange;

#[derive(Debug, Clone)]
pub struct ReferencePatt {
    pub kw_ref: KwRef,
    pub kw_mut_opt: Option<KwMut>,
    pub pattern: Box<PatternWithoutRange>,
}

impl Spanned for ReferencePatt {
    fn span(&self) -> Span {
        let s1 = self.kw_ref.span();
        let s2 = self.pattern.span();

        Span::join(s1, s2)
    }
}
