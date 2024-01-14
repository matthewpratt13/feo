#![allow(dead_code)]

use feo_types::U256;

use crate::{
    identifier::Identifier,
    item::{Parenthesis, Underscore},
    keyword::KeywordKind,
    path::SimplePath,
};

mod range_patt;
mod struct_patt;
mod tuple_patt;

use self::{
    range_patt::RangePattKind,
    struct_patt::{StructPatt, TupleStructPatt},
    tuple_patt::TuplePatt,
};

pub enum Pattern {
    Literal(LiteralPatt),
    Grouped(GroupedPatt),
    Identifier(IdentifierPatt),
    Path(SimplePath),
    Range(RangePattKind),
    Struct(StructPatt),
    Tuple(TuplePatt),
    TupleStruct(TupleStructPatt),
    Wildcard(Underscore),
}

pub enum LiteralPatt {
    Char(char),
    Str(&'static str),
    Int(i64),
    UInt(u64),
    U256(U256),
    Float(f64),
    Bytes32([u8; 32]),
    Bool(bool),
}

pub struct GroupedPatt {
    open_parenthesis: Parenthesis,
    pattern: Box<Pattern>,
    close_parenthesis: Parenthesis,
}

pub struct IdentifierPatt {
    kw_ref_opt: Option<KeywordKind>,
    kw_mut_opt: Option<KeywordKind>,
    name: Identifier,
}
