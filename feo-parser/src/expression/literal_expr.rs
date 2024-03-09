use crate::peek::{Peek, Peeker};
use feo_types::{
    literal::{FloatType, IntType, LiteralKind, UIntType},
    Literal, U256,
};

impl Peek for LiteralKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(c) = Literal::<char>::peek(peeker) {
            Some(LiteralKind::Char(c))
        } else if let Some(s) = Literal::<String>::peek(peeker) {
            Some(LiteralKind::String(s))
        } else if let Some(b) = Literal::<bool>::peek(peeker) {
            Some(LiteralKind::Bool(b))
        } else if let Some(i) = Literal::<IntType>::peek(peeker) {
            match i.clone().into_inner() {
                Some(t) => match t {
                    IntType::I32(_) => Some(LiteralKind::Int(i)),
                    IntType::I64(_) => Some(LiteralKind::Int(i)),
                },
                _ => None,
            }
        } else if let Some(ui) = Literal::<UIntType>::peek(peeker) {
            match ui.clone().into_inner() {
                Some(t) => match t {
                    UIntType::U8(_) => Some(LiteralKind::UInt(ui)),
                    UIntType::U16(_) => Some(LiteralKind::UInt(ui)),
                    UIntType::U32(_) => Some(LiteralKind::UInt(ui)),
                    UIntType::U64(_) => Some(LiteralKind::UInt(ui)),
                },
                _ => None,
            }
        } else if let Some(u) = Literal::<U256>::peek(peeker) {
            Some(LiteralKind::U256(u))
        } else if let Some(f) = Literal::<FloatType>::peek(peeker) {
            match f.clone().into_inner() {
                Some(t) => match t {
                    FloatType::F32(_) => Some(LiteralKind::Float(f)),
                    FloatType::F64(_) => Some(LiteralKind::Float(f)),
                },
                _ => None,
            }
        } else {
            None
        }
    }
}
