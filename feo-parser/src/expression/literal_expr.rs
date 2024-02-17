use feo_types::literal::{FloatType, IntType, LiteralKind, UIntType};

use crate::{parse::Peek, parser::Peeker};

impl Peek for LiteralKind {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let literal_kind = if let Ok(c) = peeker.peek_char_lit() {
            LiteralKind::Char(c)
        } else if let Ok(s) = peeker.peek_string_lit() {
            LiteralKind::String(s)
        } else if let Ok(b) = peeker.peek_bool_lit() {
            LiteralKind::Bool(b)
        } else if let Ok(i) = peeker.peek_int_lit() {
            match i.clone().into_inner() {
                Some(t) => match t {
                    IntType::I32(_) => LiteralKind::I32(i),
                    IntType::I64(_) => LiteralKind::I64(i),
                },
                _ => return None,
            }
        } else if let Ok(ui) = peeker.peek_uint_lit() {
            match ui.clone().into_inner() {
                Some(t) => match t {
                    UIntType::U8(_) => LiteralKind::U8(ui),
                    UIntType::U16(_) => LiteralKind::U16(ui),
                    UIntType::U32(_) => LiteralKind::U32(ui),
                    UIntType::U64(_) => LiteralKind::U64(ui),
                },
                _ => return None,
            }
        } else if let Ok(u) = peeker.peek_u256_lit() {
            LiteralKind::U256(u)
        } else if let Ok(f) = peeker.peek_float_lit() {
            match f.clone().into_inner() {
                Some(t) => match t {
                    FloatType::F32(_) => LiteralKind::F32(f),
                    FloatType::F64(_) => LiteralKind::F64(f),
                },
                _ => return None,
            }
        } else {
            return None;
        };

        Some(literal_kind)
    }
}
