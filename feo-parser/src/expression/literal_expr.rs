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
        let literal_kind = if let Some(c) = Literal::<char>::peek(peeker) {
            LiteralKind::Char(c)
        } else if let Some(s) = Literal::<String>::peek(peeker) {
            LiteralKind::String(s)
        } else if let Some(b) = Literal::<bool>::peek(peeker) {
            LiteralKind::Bool(b)
        } else if let Some(i) = Literal::<IntType>::peek(peeker) {
            match i.clone().into_inner() {
                Some(t) => match t {
                    IntType::I32(_) => LiteralKind::Int(i),
                    IntType::I64(_) => LiteralKind::Int(i),
                },
                _ => return None,
            }
        } else if let Some(ui) = Literal::<UIntType>::peek(peeker) {
            match ui.clone().into_inner() {
                Some(t) => match t {
                    UIntType::U8(_) => LiteralKind::UInt(ui),
                    UIntType::U16(_) => LiteralKind::UInt(ui),
                    UIntType::U32(_) => LiteralKind::UInt(ui),
                    UIntType::U64(_) => LiteralKind::UInt(ui),
                },
                _ => return None,
            }
        } else if let Some(u) = Literal::<U256>::peek(peeker) {
            LiteralKind::U256(u)
        } else if let Some(f) = Literal::<FloatType>::peek(peeker) {
            match f.clone().into_inner() {
                Some(t) => match t {
                    FloatType::F32(_) => LiteralKind::Float(f),
                    FloatType::F64(_) => LiteralKind::Float(f),
                },
                _ => return None,
            }
        } else {
            return None;
        };

        Some(literal_kind)
    }
}
