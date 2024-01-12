#![allow(dead_code)]

use crate::{
    identifier::Identifier,
    item::{Parenthesis, Underscore},
    keyword::KeywordKind,
    literals::{
        BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, U256Literal, UIntLiteral,
    },
    path::SimplePath,
};

mod range_patt;
mod slice_patt;
mod struct_patt;
mod tuple_patt;

use self::{
    slice_patt::SlicePatt,
    struct_patt::{StructPatt, TupleStructPatt},
    tuple_patt::TuplePatt,
};

pub enum Pattern {
    Literal(LiteralPatt),
    Grouped(GroupedPatt),
    Identifier(IdentifierPatt),
    Path(SimplePath),
    Reference(ReferencePatt),
    Slice(SlicePatt),
    Struct(StructPatt),
    Tuple(TuplePatt),
    TupleStruct(TupleStructPatt),
    Wildcard(Underscore),
}

pub enum LiteralPatt {
    Char(CharLiteral),
    String(StringLiteral),
    Int(IntLiteral),
    UInt(UIntLiteral),
    U256(U256Literal),
    Float(FloatLiteral),
    Bool(BoolLiteral),
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

pub struct ReferencePatt {
    kw_ref_opt: Option<KeywordKind>,
    kw_mut_opt: Option<KeywordKind>,
    name: Box<Pattern>,
}
