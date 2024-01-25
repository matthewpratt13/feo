use crate::{
    keyword::Keyword,
    path::SimplePath,
    span::{Span, Spanned},
    type_utils::{Plus, QuestionMark},
};

use super::Type;

pub struct TraitObjectType {
    kw_dyn: Keyword,
    trait_bounds: TraitBounds,
}

impl Type for TraitObjectType {}

impl Spanned for TraitObjectType {
    fn span(&self) -> Span {
        let start_pos = self.kw_dyn.span().start();
        let end_pos = self.trait_bounds.span().end();
        let source = self.kw_dyn.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TraitBounds {
    first_trait_bound: TraitBound,
    subsequent_trait_bounds: Vec<(Plus, TraitBound)>,
}

impl Spanned for TraitBounds {
    fn span(&self) -> Span {
        let start_pos = self.first_trait_bound.span().start();

        let end_pos = if let Some(s) = self.subsequent_trait_bounds.last() {
            s.1.span().end()
        } else {
            self.first_trait_bound.span().end()
        };

        let source = self.first_trait_bound.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TraitBound {
    question_mark_opt: Option<QuestionMark>,
    trait_path: SimplePath,
}

impl Spanned for TraitBound {
    fn span(&self) -> Span {
        let start_pos = if let Some(q) = &self.question_mark_opt {
            q.span().start()
        } else {
            self.trait_path.span().start()
        };

        let end_pos = self.trait_path.span().end();
        let source = self.trait_path.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
