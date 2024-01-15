#![allow(dead_code)]

use crate::{
    path::SimplePath,
    type_utils::{Parenthesis, Underscore},
};

mod range_patt;
mod struct_patt;
mod tuple_patt;

use self::{
    identifier_patt::IdentifierPatt,
    literal_patt::LiteralPattKind,
    range_patt::RangePattKind,
    struct_patt::{StructPatt, TupleStructPatt},
    tuple_patt::TuplePatt,
};

pub enum PatternKind {
    Grouped(GroupedPatt),
    Identifier(IdentifierPatt),
    Literal(LiteralPattKind),
    Path(SimplePath),
    Range(RangePattKind),
    Struct(StructPatt),
    Tuple(TuplePatt),
    TupleStruct(TupleStructPatt),
    Wildcard(Underscore),
}

pub struct GroupedPatt {
    open_parenthesis: Parenthesis,
    pattern: Box<PatternKind>,
    close_parenthesis: Parenthesis,
}

mod identifier_patt {
    use crate::{identifier::Identifier, keyword::KeywordKind};

    pub struct IdentifierPatt {
        kw_ref_opt: Option<KeywordKind>,
        kw_mut_opt: Option<KeywordKind>,
        name: Identifier,
    }
}

mod literal_patt {
    use feo_types::U256;

    pub enum LiteralPattKind {
        Char(char),
        Str(&'static str),
        Int(i64),
        UInt(u64),
        U256(U256),
        Float(f64),
        Bytes32([u8; 32]),
        Bool(bool),
    }
}
