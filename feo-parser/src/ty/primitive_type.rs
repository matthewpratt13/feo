use feo_ast::ty::PrimitiveType;
use feo_error::error::CompilerError;
use feo_types::literal::{FloatType, IntType, LiteralKind, UIntType};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for PrimitiveType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let primitive = if let Some(l) = parser.peek_current::<LiteralKind>() {
            match l {
                LiteralKind::Char(c) => Some(PrimitiveType::Char(c.into_inner().unwrap())),
                LiteralKind::String(s) => Some(PrimitiveType::String(s.into_inner().unwrap())),
                LiteralKind::Bool(b) => Some(PrimitiveType::Bool(b.into_inner().unwrap())),
                LiteralKind::Int(it) => match it.into_inner().unwrap() {
                    IntType::I32(i) => Some(PrimitiveType::I32(i)),
                    IntType::I64(i) => Some(PrimitiveType::I64(i)),
                },
                LiteralKind::UInt(uit) => match uit.into_inner().unwrap() {
                    UIntType::U8(ui) => Some(PrimitiveType::U8(ui)),
                    UIntType::U16(ui) => Some(PrimitiveType::U16(ui)),
                    UIntType::U32(ui) => Some(PrimitiveType::U32(ui)),
                    UIntType::U64(ui) => Some(PrimitiveType::U64(ui)),
                },
                LiteralKind::U256(u) => Some(PrimitiveType::U256(u.into_inner().unwrap())),
                LiteralKind::Float(ft) => match ft.into_inner().unwrap() {
                    FloatType::F32(f) => Some(PrimitiveType::F32(f)),
                    FloatType::F64(f) => Some(PrimitiveType::F64(f)),
                },
            }
        } else {
            None
        };

        Ok(primitive)
    }
}
