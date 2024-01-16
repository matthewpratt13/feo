#![allow(dead_code)]

pub use self::trait_object_type::{TraitBound, TraitObjectType};

pub trait Type {}

mod array_type {
    use crate::type_utils::{Bracket, Semicolon};

    use super::Type;

    pub struct ArrayType {
        open_bracket: Bracket,
        element_type: Box<dyn Type>,
        semicolon: Semicolon,
        close_bracket: Bracket,
    }

    impl Type for ArrayType {}
}

mod trait_object_type {
    use crate::{
        keyword::KeywordKind,
        path::SimplePath,
        type_utils::{Plus, QuestionMark},
    };

    use super::Type;

    pub struct TraitObjectType {
        kw_dyn: KeywordKind,
        trait_bounds: TraitBounds,
    }

    impl Type for TraitObjectType {}

    pub struct TraitBounds {
        first_trait_bound: TraitBound,
        subsequent_trait_bounds: Vec<(Plus, TraitBound)>,
    }

    pub struct TraitBound {
        question_mark_opt: Option<QuestionMark>,
        trait_path: SimplePath,
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
