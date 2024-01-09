use crate::expression::PathExpr;
use crate::literals::{
    BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, U256Literal, UIntLiteral,
};

pub enum Pattern {
    Literal(LiteralPatt),
    Grouped(GroupedPatt),
    Identifier(IdentifierPatt),
    Path(PathPatt),
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

pub type PathPatt = PathExpr;

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
