#![allow(dead_code)]

use feo_types::U256;

use crate::item::{Bracket, Comma, EnumItem, FunctionItem, Parenthesis, Semicolon, StructItem};

use self::trait_object_type::TraitObjectType;

pub enum Type {
    Char(char),
    Str(&'static str),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U256(U256),
    F32(f32),
    F64(f64),
    Bool(bool),
    Struct(Box<StructItem>),
    Enum(Box<EnumItem>),
    Function(Box<FunctionItem>),
    Array(ArrayType),
    Tuple(TupleType),
    TraitObject(TraitObjectType),
}

pub struct ArrayType {
    open_bracket: Bracket,
    element_type: Box<Type>,
    semicolon: Semicolon,
    close_bracket: Bracket,
}

pub struct TupleType {
    open_parenthesis: Parenthesis,
    elements: Vec<(Type, Comma)>,
    trailing_element: Box<Type>,
    close_parenthesis: Parenthesis,
}

mod trait_object_type {
    use crate::{
        item::{Plus, QuestionMark},
        keyword::KeywordKind,
        path::SimplePath,
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
        path: SimplePath,
    }
}
