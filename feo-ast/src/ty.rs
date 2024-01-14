#![allow(dead_code)]

use crate::{
    item::{Bracket, Comma, EnumItem, FunctionItem, Parenthesis, Semicolon, StructItem},
    literals::{
        BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, U256Literal, UIntLiteral,
    },
    path::SimplePath,
};

use self::trait_object_type::TraitObjectType;

pub enum Type {
    Char(CharLiteral),
    Str(StringLiteral),
    I32(IntLiteral),
    I64(IntLiteral),
    U8(UIntLiteral),
    U16(UIntLiteral),
    U32(UIntLiteral),
    U64(UIntLiteral),
    U256(U256Literal),
    F32(FloatLiteral),
    F64(FloatLiteral),
    Bytes32([u8; 32]),
    Bool(BoolLiteral),
    Struct(Box<StructItem>),
    Enum(Box<EnumItem>),
    Function(Box<FunctionItem>),
    Array(ArrayType),
    Tuple(TupleType),
    TraitObject(TraitObjectType),
    TypePath(SimplePath),
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
