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
        let start_pos = self.kw_ref.span().start();
        let end_pos = self.pattern.span().end();
        let source = self.kw_ref.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
