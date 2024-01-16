#![allow(dead_code)]

pub use self::trait_object_type::{TraitBound, TraitObjectType};

pub trait Type {}

mod array_type {
    use feo_types::span::{Span, Spanned};

    use crate::type_utils::{Bracket, Semicolon};

    use super::Type;

    pub struct ArrayType {
        open_bracket: Bracket,
        element_type: Box<dyn Type>,
        semicolon: Semicolon,
        close_bracket: Bracket,
    }

    impl Type for ArrayType {}

    impl Spanned for ArrayType {
        fn span(&self) -> Span {
            let start_pos = self.open_bracket.span().start();
            let end_pos = self.close_bracket.span().end();
            let source = self.open_bracket.span().source();

            let span = Span::new(source.as_str(), start_pos, end_pos);

            span
        }
    }
}

mod trait_object_type {
    use feo_types::span::{Span, Spanned};

    use crate::{
        keyword::Keyword,
        path::SimplePath,
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
            todo!()
        }
    }

    pub struct TraitBounds {
        first_trait_bound: TraitBound,
        subsequent_trait_bounds: Vec<(Plus, TraitBound)>,
    }

    impl Spanned for TraitBounds {
        fn span(&self) -> Span {
            let start_pos = self.first_trait_bound.span().start();

            let end_pos = match self.subsequent_trait_bounds.last() {
                Some(t) => t.1.span().end(),
                None => self.first_trait_bound.span().end(),
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
            let start_pos = match &self.question_mark_opt {
                Some(q) => q.span().start(),
                None => self.trait_path.span().start(),
            };

            let end_pos = self.trait_path.span().end();
            let source = self.trait_path.span().source();

            let span = Span::new(source.as_str(), start_pos, end_pos);

            span
        }
    }
}

mod tuple_type {
    use crate::type_utils::{Comma, Parenthesis};

    use super::Type;

    pub struct TupleType {
        open_parenthesis: Parenthesis,
        elements: Vec<(Box<dyn Type>, Comma)>,
        trailing_element: Box<dyn Type>,
        close_parenthesis: Parenthesis,
    }

    impl Type for TupleType {}
}
