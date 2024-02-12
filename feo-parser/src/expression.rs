#![allow(dead_code)]

mod array_expr;
mod literal_expr;
mod struct_expr;

use feo_ast::expression::Expression;
use feo_error::handler::ErrorEmitted;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    literal::{Literal, LiteralKind},
    punctuation::PuncKind,
    Delimiter, DocComment, Identifier, Keyword, Punctuation, U256,
};

use crate::{parse::Parse, parser::Parser};

impl Parse for Expression {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let expr = if let Some(_) = parser.peek::<Literal<char>>() {
            Expression::LiteralExpr(
                LiteralKind::parse(parser)?
                    .expect("expected some `char` literal token, found none"),
            )
        } else if let Some(_) = parser.peek::<Literal<String>>() {
            Expression::LiteralExpr(
                LiteralKind::parse(parser)?
                    .expect("expected some `string` literal token, found none"),
            )
        } else if let Some(_) = parser.peek::<Literal<i64>>() {
            Expression::LiteralExpr(
                LiteralKind::parse(parser)?.expect("expected some `i64` literal token, found none"),
            )
        } else if let Some(_) = parser.peek::<Literal<u64>>() {
            Expression::LiteralExpr(
                LiteralKind::parse(parser)?.expect("expected some `u64` literal token, found none"),
            )
        } else if let Some(_) = parser.peek::<Literal<U256>>() {
            Expression::LiteralExpr(
                LiteralKind::parse(parser)?
                    .expect("expected some `U256` literal token, found none"),
            )
        } else if let Some(_) = parser.peek::<Literal<f64>>() {
            Expression::LiteralExpr(
                LiteralKind::parse(parser)?.expect("expected some `f64` token, found none"),
            )
        } else if let Some(_) = parser.peek::<Identifier>() {
            // [ArrayElements]
            // ArithmeticOrLogicalExpr
            // AssignmentExpr
            // CompoundAssignmentExpr
            // ComparisonExpr
            // LazyBoolExpr
            // TypeCastExpr
            // [UnwrapOperandKind] UnwrapExpr
            // FunctionCallExpr
            // MethodCallExpr
            // [CallParams]
            // FieldAccessExpr
            // RangeFromToExpr
            // RangeFromExpr
            // RangeInclusiveExpr
            // [PathIdenSegmentKind] (PathInExpr -> Struct | TupleStruct | UnitStruct)
            todo!()
        } else if let Some(k) = parser.peek::<Keyword>() {
            match k.keyword_kind {
                KeywordKind::KwBreak => todo!(),    // BreakExpr
                KeywordKind::KwContinue => todo!(), // ContinueExpr
                // [PathIdenSegmentKind] (PathInExpr -> Struct | TupleStruct | UnitStruct)
                KeywordKind::KwCrate
                | KeywordKind::KwSelf
                | KeywordKind::KwSelfType
                | KeywordKind::KwSuper => todo!(),
                KeywordKind::KwIf => todo!(), // IfExpr
                // [IterationExprKind] InfiniteLoopExpr | PredicateLoopExpr | IterLoopExpr
                KeywordKind::KwLoop | KeywordKind::KwWhile | KeywordKind::KwFor => todo!(),
                KeywordKind::KwMatch => todo!(),  // MatchExpr
                KeywordKind::KwReturn => todo!(), // ReturnExpr
                _ => todo!(),
            }
        } else if let Some(_) = parser.peek::<DocComment>() {
            todo!()
        } else if let Some(d) = parser.peek::<Delimiter>() {
            match d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    // ParenthesizedExpr
                    // TupleExpr
                    todo!()
                }
                (DelimKind::Bracket, DelimOrientation::Open) => todo!(), // ArrayExpr

                (DelimKind::Brace, DelimOrientation::Open) => todo!(), // BlockExpr

                _ => todo!(),
            }
        } else if let Some(p) = parser.peek::<Punctuation>() {
            match p.punc_kind {
                PuncKind::DblDot => todo!(),       // RangeFullExpr | RangeToExpr
                PuncKind::DotDotEquals => todo!(), // RangeToInclusiveExpr
                // [NegationOperatorKind] NegationExpr
                PuncKind::Bang | PuncKind::Minus => todo!(),
                PuncKind::Hash => todo!(),      // OuterAttr
                PuncKind::Ampersand => todo!(), // ReferenceExpr
                PuncKind::Asterisk => todo!(),  // DereferenceExpr
                // [ClosureParamsOpt] ClosureWithBlock | ClosureWithoutBlock
                PuncKind::Pipe | PuncKind::DblPipe => todo!(),
                PuncKind::HashBang => todo!(), // InnerAttr
                _ => todo!(),
            }
        } else {
            todo!()
        };

        Ok(Some(expr))
    }
}
