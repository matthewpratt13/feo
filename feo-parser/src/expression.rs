#![allow(dead_code)]
#![allow(unused_variables)]

use feo_ast::{
    expression::{
        ArithmeticOrLogicalExpr, ArrayExpr, ClosureWithBlock, ClosureWithoutBlock, DereferenceExpr,
        FieldAccessExpr, FunctionCallExpr, IndexExpr, MethodCallExpr, NegationExpr,
        ParenthesizedExpr, ReferenceExpr, Returnable, StructExpr, StructExprKind, TupleExpr,
        TupleIndexExpr, TupleStructExpr, TypeCastExpr, UnderscoreExpr, UnitStructExpr,
    },
    path::PathInExpr,
};
use feo_error::{handler::ErrorEmitted, parser_error::ParserErrorKind};

use feo_types::{literal::LiteralKind, Delimiter, Identifier, Keyword, Punctuation};

use crate::{parse::Parse, parser::Parser};

mod array_expr;
mod call_expr;
mod closure_expr;
mod field_access_expr;
mod literal_expr;
mod operator_expr;
mod parenthesized_expr;
mod struct_expr;
mod tuple_expr;

impl Parse for Returnable {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let expr = if let Some(id) = parser.peek::<Identifier>() {
            parser.advance();

            if let Some(fc) = FunctionCallExpr::parse(parser)? {
                Returnable::FunctionCallExpr(fc)
            } else if let Some(mc) = MethodCallExpr::parse(parser)? {
                Returnable::MethodCallExpr(mc)
            } else if let Some(fa) = FieldAccessExpr::parse(parser)? {
                Returnable::FieldAccessExpr(fa)
            } else if let Some(se) = StructExpr::parse(parser)? {
                Returnable::StructExpr(StructExprKind::Struct(se))
            } else if let Some(ts) = TupleStructExpr::parse(parser)? {
                Returnable::StructExpr(StructExprKind::TupleStruct(ts))
            } else if let Some(us) = UnitStructExpr::parse(parser)? {
                Returnable::StructExpr(StructExprKind::UnitStruct(us))
            } else if let Some(pat) = PathInExpr::parse(parser)? {
                Returnable::PathExpr(pat)
            } else if let Some(al) = ArithmeticOrLogicalExpr::parse(parser)? {
                Returnable::ArithmeticOrLogicalExpr(al)
            } else {
                Returnable::Identifier(id)
            }
        } else if let Some(_) = parser.peek::<Delimiter>() {
            parser.advance();

            if let Some(ae) = ArrayExpr::parse(parser)? {
                Returnable::ArrayExpr(ae)
            } else if let Some(ie) = IndexExpr::parse(parser)? {
                Returnable::IndexExpr(ie)
            } else if let Some(te) = TupleExpr::parse(parser)? {
                Returnable::TupleExpr(te)
            } else if let Some(ti) = TupleIndexExpr::parse(parser)? {
                Returnable::TupleIndexExpr(ti)
            } else if let Some(par) = ParenthesizedExpr::parse(parser)? {
                Returnable::ParenthesizedExpr(par)
            } else {
                return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
            }
        } else if let Some(l) = parser.peek::<LiteralKind>() {
            parser.advance();

            if let Some(al) = ArithmeticOrLogicalExpr::parse(parser)? {
                Returnable::ArithmeticOrLogicalExpr(al)
            } else if let Some(tc) = TypeCastExpr::parse(parser)? {
                Returnable::TypeCastExpr(tc)
            } else {
                Returnable::LiteralExpr(l)
            }
        } else if let Some(_) = parser.peek::<Keyword>() {
            parser.advance();

            if let Some(se) = StructExpr::parse(parser)? {
                Returnable::StructExpr(StructExprKind::Struct(se))
            } else if let Some(ts) = TupleStructExpr::parse(parser)? {
                Returnable::StructExpr(StructExprKind::TupleStruct(ts))
            } else if let Some(us) = UnitStructExpr::parse(parser)? {
                Returnable::StructExpr(StructExprKind::UnitStruct(us))
            } else if let Some(pe) = PathInExpr::parse(parser)? {
                Returnable::PathExpr(pe)
            } else {
                return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
            }
        } else if let Some(_) = parser.peek::<Punctuation>() {
            parser.advance();

            if let Some(cwb) = ClosureWithBlock::parse(parser)? {
                Returnable::ClosureWithBlock(cwb)
            } else if let Some(c) = ClosureWithoutBlock::parse(parser)? {
                Returnable::ClosureWithoutBlock(c)
            } else if let Some(de) = DereferenceExpr::parse(parser)? {
                Returnable::DereferenceExpr(de)
            } else if let Some(ne) = NegationExpr::parse(parser)? {
                Returnable::NegationExpr(ne)
            } else if let Some(re) = ReferenceExpr::parse(parser)? {
                Returnable::ReferenceExpr(re)
            } else if let Some(ue) = UnderscoreExpr::parse(parser)? {
                Returnable::UnderscoreExpr(ue)
            } else {
                return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
            }
        } else {
            return Ok(None);
        };

        Ok(Some(expr))
    }
}
