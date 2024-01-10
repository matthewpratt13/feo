use crate::{
    literals::{
        BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, U256Literal, UIntLiteral,
    },
    path::SimplePath,
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
    Wildcard(WildcardPatt),
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

pub struct GroupedPatt {}

pub struct IdentifierPatt {}

pub struct ReferencePatt {}

pub struct SlicePatt {}

pub struct SlicePattItems {}

pub struct StructPatt {}

pub struct StructPattFields {}

pub struct StructPattField {}

pub struct TuplePatt {}

pub struct TuplePattItems {}

pub struct TupleStructPatt {}

pub struct TupleStructItems {}

pub struct WildcardPatt {}
