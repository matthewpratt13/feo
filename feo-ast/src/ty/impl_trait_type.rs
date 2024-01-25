use crate::{
    keyword::Keyword,
    path::TypePath,
    span::{Span, Spanned},
    type_utils::QuestionMark,
};

use super::Type;

// (one bound)
pub struct ImplTraitType {
    kw_impl: Keyword,
    trait_bound: TraitBound,
}

impl Type for ImplTraitType {}

impl Spanned for ImplTraitType {
    fn span(&self) -> Span {
        let start_pos = self.kw_impl.span().start();
        let end_pos = self.trait_bound.span().end();
        let source = self.kw_impl.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct TraitBound {
    question_mark_opt: Option<QuestionMark>,
    trait_path: TypePath,
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
