#![allow(dead_code)]

mod struct_expr;

use feo_ast::expression::{Expression, Struct, StructKind};
use feo_error::handler::ErrorEmitted;

use crate::{
    parse::{Parse, ParseExpr},
    parser::Parser,
};

impl Parse for Expression {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for Expression {
    fn parse_expr(&mut self, parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match self {
            Expression::ArrayExpr(_) => todo!(),
            Expression::IndexExpr(_) => todo!(),
            Expression::BlockExpr(_) => todo!(),
            Expression::FunctionCallExpr(_) => todo!(),
            Expression::MethodCallExpr(_) => todo!(),
            Expression::ClosureWithBlock(_) => todo!(),
            Expression::ClosureWithoutBlock(_) => todo!(),
            Expression::FieldAccessExpr(_) => todo!(),
            Expression::IfExpr(_) => todo!(),
            Expression::IterationExpr(_) => todo!(),
            Expression::BreakExpr(_) => todo!(),
            Expression::ContinueExpr(_) => todo!(),
            Expression::LiteralExpr(_) => todo!(),
            Expression::MatchExpr(_) => todo!(),
            Expression::OperatorExpr(_) => todo!(),
            Expression::ParenthesizedExpr(_) => todo!(),
            Expression::PathExpr(_) => todo!(),
            Expression::RangeExpr(_) => todo!(),
            Expression::ReturnExpr(_) => todo!(),
            Expression::StructExpr(se) => match se {
                StructKind::Struct(_) => Ok(Some(Expression::StructExpr(StructKind::Struct(
                    Struct::parse(parser)?.expect("error parsing struct expression"),
                )))),
                StructKind::TupleStruct(_) => todo!(),
                StructKind::UnitStruct(_) => todo!(),
            },
            Expression::TupleExpr(_) => todo!(),
            Expression::UnderscoreExpr(_) => todo!(),
        }
    }
}
