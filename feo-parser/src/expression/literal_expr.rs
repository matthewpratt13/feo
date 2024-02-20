use feo_error::{handler::ErrorEmitted, parser_error::ParserErrorKind};
use feo_types::{
    literal::{FloatType, IntType, LiteralKind, UIntType},
    Literal, U256,
};

use crate::{parse::Peek, parser::Peeker};

impl Peek for LiteralKind {
    fn peek(peeker: &Peeker<'_, '_>) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let literal_kind = if let Some(c) = Literal::<char>::peek(peeker)? {
            LiteralKind::Char(c)
        } else if let Some(s) = Literal::<String>::peek(peeker)? {
            LiteralKind::String(s)
        } else if let Some(b) = Literal::<bool>::peek(peeker)? {
            LiteralKind::Bool(b)
        } else if let Some(i) = Literal::<IntType>::peek(peeker)? {
            match i.clone().into_inner() {
                Some(t) => match t {
                    IntType::I32(_) => LiteralKind::I32(i),
                    IntType::I64(_) => LiteralKind::I64(i),
                },
                _ => {
                    return Err(peeker.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "integer type",
                        found: "unknown",
                    })); // TODO
                }
            }
        } else if let Some(ui) = Literal::<UIntType>::peek(peeker)? {
            match ui.clone().into_inner() {
                Some(t) => match t {
                    UIntType::U8(_) => LiteralKind::U8(ui),
                    UIntType::U16(_) => LiteralKind::U16(ui),
                    UIntType::U32(_) => LiteralKind::U32(ui),
                    UIntType::U64(_) => LiteralKind::U64(ui),
                },
                _ => {
                    return Err(peeker.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "unsigned integer type",
                        found: "unknown",
                    })); // TODO
                }
            }
        } else if let Some(u) = Literal::<U256>::peek(peeker)? {
            LiteralKind::U256(u)
        } else if let Some(f) = Literal::<FloatType>::peek(peeker)? {
            match f.clone().into_inner() {
                Some(t) => match t {
                    FloatType::F32(_) => LiteralKind::F32(f),
                    FloatType::F64(_) => LiteralKind::F64(f),
                },
                _ => {
                    return Err(peeker.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "floating-point number type",
                        found: "unknown",
                    })); // TODO
                }
            }
        } else {
            return Err(peeker.log_error(ParserErrorKind::UnexpectedToken {
                expected: "literal type",
                found: "unknown",
            })); // TODO
        };

        Ok(Some(literal_kind))
    }
}
