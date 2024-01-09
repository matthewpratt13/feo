use crate::literals::{
    BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral,
};

pub enum LiteralExpr {
    Char(CharLiteral),
    String(StringLiteral),
    Int(IntLiteral),
    UInt(UIntLiteral),
    Float(FloatLiteral),
    Bool(BoolLiteral),
}
