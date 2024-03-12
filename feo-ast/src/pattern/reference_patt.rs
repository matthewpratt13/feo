use feo_types::{
    span::{Span, Spanned},
    utils::{Ampersand, KwMut},
};

use super::PatternWithoutRange;

#[derive(Debug, Clone)]
pub struct ReferencePatt {
    pub ampersand: Ampersand,
    pub kw_mut_opt: Option<KwMut>,
    pub pattern: Box<PatternWithoutRange>,
}

impl Spanned for ReferencePatt {
    fn span(&self) -> Span {
        let s1 = self.ampersand.span();
        let s2 = self.pattern.span();

        Span::join(s1, s2)
    }
}
