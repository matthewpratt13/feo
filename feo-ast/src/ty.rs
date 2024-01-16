#![allow(dead_code)]

use crate::{
    item::{EnumItem, FunctionItem, StructItem},
    literals::{
        BoolLiteral, Bytes32Literal, CharLiteral, FloatLiteral, IntLiteral, StringLiteral,
        U256Literal, UIntLiteral,
    },
    path::SimplePath,
    type_utils::{Bracket, Comma, Parenthesis, Semicolon},
};

pub use self::trait_object_type::{TraitBound, TraitObjectType};

pub trait Type {}

impl Type for CharLiteral {}

impl Type for StringLiteral {}

impl Type for IntLiteral {}

impl Type for UIntLiteral {}

impl Type for U256Literal {}

impl Type for FloatLiteral {}

impl Type for Bytes32Literal {}

impl Type for BoolLiteral {}

impl<S> Type for dyn StructItem<S> {}

impl Type for EnumItem {}

impl<T> Type for dyn FunctionItem<T> {}

impl Type for ArrayType {}

impl Type for TraitObjectType {}

impl Type for TupleType {}

impl Type for SimplePath {}

pub struct ArrayType {
    open_bracket: Bracket,
    element_type: Box<dyn Type>,
    semicolon: Semicolon,
    close_bracket: Bracket,
}

pub struct TupleType {
    open_parenthesis: Parenthesis,
    elements: Vec<(Box<dyn Type>, Comma)>,
    trailing_element: Box<dyn Type>,
    close_parenthesis: Parenthesis,
}

mod trait_object_type {
    use crate::{
        keyword::KeywordKind,
        path::SimplePath,
        type_utils::{Plus, QuestionMark},
    };

    pub struct TraitObjectType {
        kw_dyn: KeywordKind,
        trait_bounds: TraitBounds,
    }

    pub struct TraitBounds {
        first_trait_bound: TraitBound,
        subsequent_trait_bounds: Vec<(Plus, TraitBound)>,
    }

    pub struct TraitBound {
        question_mark_opt: Option<QuestionMark>,
        trait_path: SimplePath,
    }
}

