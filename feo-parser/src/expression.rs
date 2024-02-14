#![allow(dead_code)]

use feo_ast::expression::Expression;
use feo_types::literal::{FloatType, IntType, LiteralKind, UIntType};

use crate::{parse::Peek, parser::Peeker};

mod array_expr;
mod literal_expr;
mod struct_expr;

impl Peek for Expression {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let expr = if let Ok(c) = peeker.peek_char_lit() {
            Expression::LiteralExpr(LiteralKind::Char(c))
        } else if let Ok(s) = peeker.peek_string_lit() {
            Expression::LiteralExpr(LiteralKind::String(s))
        } else if let Ok(b) = peeker.peek_bool_lit() {
            Expression::LiteralExpr(LiteralKind::Bool(b))
        } else if let Ok(i) = peeker.peek_int_lit() {
            // TODO: peek the next token to verify what `Expression` should be returned 
            // TODO: just because this is a literal, doesn't mean that it's a `LitExpr`
            // TODO: e.g., it could be the first token in an `ArithmeticOrLogicalExpr`
            // TODO: ditto for the other numeric literals below
            match i.clone().into_inner() {
                Some(t) => match t {
                    IntType::I32(_) => Expression::LiteralExpr(LiteralKind::I32(i)),
                    IntType::I64(_) => Expression::LiteralExpr(LiteralKind::I64(i)),
                },
                None => return None,
            }
        } else if let Ok(ui) = peeker.peek_uint_lit() {
            match ui.clone().into_inner() {
                Some(t) => match t {
                    UIntType::U8(_) => Expression::LiteralExpr(LiteralKind::U8(ui)),
                    UIntType::U16(_) => Expression::LiteralExpr(LiteralKind::U16(ui)),
                    UIntType::U32(_) => Expression::LiteralExpr(LiteralKind::U32(ui)),
                    UIntType::U64(_) => Expression::LiteralExpr(LiteralKind::U64(ui)),
                },
                None => return None,
            }
        } else if let Ok(u) = peeker.peek_u256_lit() {
            Expression::LiteralExpr(LiteralKind::U256(u))
        } else if let Ok(f) = peeker.peek_float_lit() {
            match f.clone().into_inner() {
                Some(t) => match t {
                    FloatType::F32(_) => Expression::LiteralExpr(LiteralKind::F32(f)),
                    FloatType::F64(_) => Expression::LiteralExpr(LiteralKind::F64(f)),
                },
                None => return None,
            }
        } else {
            todo!()
        };

        Some(expr)
    }
}
