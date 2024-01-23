#![allow(dead_code)]

use feo_types::span::Spanned;

pub use self::trait_object_type::{TraitBound, TraitObjectType};

pub trait Type
where
    Self: Spanned,
{
}

// also:
// - primitive literals (char, str, int, uint, float, bytes32, bool)
// - path

mod array_type {
    use feo_types::span::{Span, Spanned};

    use crate::{
        expression::Expression,
        type_utils::{Bracket, Semicolon},
    };

    use super::Type;

    pub struct ArrayType {
        open_bracket: Bracket,
        element_type: Box<dyn Type>,
        semicolon: Semicolon,
        num_elements: Box<dyn Expression>,
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
}

mod tuple_type {
    use feo_types::span::{Span, Spanned};

    use crate::type_utils::{Comma, Parenthesis};

    use super::Type;

    pub struct TupleType {
        open_parenthesis: Parenthesis,
        elements: Vec<(Box<dyn Type>, Comma)>,
        trailing_element: Box<dyn Type>,
        close_parenthesis: Parenthesis,
    }

    impl Type for TupleType {}

    impl Spanned for TupleType {
        fn span(&self) -> Span {
            let start_pos = self.open_parenthesis.span().start();
            let end_pos = self.close_parenthesis.span().end();
            let source = self.open_parenthesis.span().source();

            let span = Span::new(source.as_str(), start_pos, end_pos);

            span
        }
    }
}
