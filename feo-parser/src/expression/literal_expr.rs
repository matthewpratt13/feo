use feo_types::literal::{FloatType, IntType, LiteralKind, UIntType};

use crate::{parse::Peek, parser::Peeker};

impl Peek for LiteralKind {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let lit = if let Ok(c) = peeker.peek_char_lit() {
            LiteralKind::Char(c.clone())
        } else if let Ok(s) = peeker.peek_string_lit() {
            LiteralKind::String(s.clone())
        } else if let Ok(b) = peeker.peek_bool_lit() {
            LiteralKind::Bool(b.clone())
        } else if let Ok(i) = peeker.peek_int_lit() {
            match i.into_inner() {
                Some(t) => match t {
                    IntType::I32(_) => LiteralKind::I32(i.clone()),
                    IntType::I64(_) => LiteralKind::I64(i.clone()),
                },
                None => return None,
            }
        } else if let Ok(ui) = peeker.peek_uint_lit() {
            match ui.into_inner() {
                Some(t) => match t {
                    UIntType::U8(_) => LiteralKind::U8(ui.clone()),
                    UIntType::U16(_) => LiteralKind::U16(ui.clone()),
                    UIntType::U32(_) => LiteralKind::U32(ui.clone()),
                    UIntType::U64(_) => LiteralKind::U64(ui.clone()),
                },
                None => return None,
            }
        } else if let Ok(f) = peeker.peek_float_lit() {
            match f.into_inner() {
                Some(t) => match t {
                    FloatType::F32(_) => LiteralKind::F32(f.clone()),
                    FloatType::F64(_) => LiteralKind::F64(f.clone()),
                },
                None => return None,
            }
        } else {
            return None;
        };

        Some(lit)
    }
}
