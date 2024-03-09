use feo_types::{
    span::{Span, Spanned},
    utils::KwMut,
};

use crate::expression::DerefOperator;

use super::PatternWithoutRange;

#[derive(Debug, Clone)]
pub struct ReferencePatt {
    kw_ref: DerefOperator,
    kw_mut_opt: Option<KwMut>,
    pattern: Box<PatternWithoutRange>,
}

impl Spanned for ReferencePatt {
    fn span(&self) -> Span {
        let s1 = self.kw_ref.span();
        let s2 = self.pattern.span();

        Span::join(s1, s2)
    }
}
