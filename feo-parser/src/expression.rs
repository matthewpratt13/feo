#![allow(dead_code)]
#![allow(unused_variables)] // temporary – used to remove compiler warnings

mod array_expr;
mod literal_expr;
mod struct_expr;

use feo_ast::expression::{Expression, IterableExpr};
use feo_error::handler::ErrorEmitted;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    literal::{FloatType, IntType, Literal, LiteralKind, UIntType},
    punctuation::PuncKind,
    Delimiter, DocComment, Identifier, Keyword, Punctuation, TypeAnnotation, U256,
};

use crate::{
    parse::{Parse, Peek},
    parser::{Parser, Peeker},
};


// impl Parse for Expression {
//     fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
//     where
//         Self: Sized,
//     {
//         let expr = if let Some(_) = parser.peek::<Literal<char>>() {
//             Expression::LiteralExpr(
//                 LiteralKind::parse(parser)?.unwrap_or(LiteralKind::default_char()),
//             )
//         } else if let Some(_) = parser.peek::<Literal<String>>() {
//             Expression::LiteralExpr(
//                 LiteralKind::parse(parser)?.unwrap_or(LiteralKind::default_string()),
//             )
//         } else if let Some(_) = parser.peek::<Literal<bool>>() {
//             Expression::LiteralExpr(
//                 LiteralKind::parse(parser)?.unwrap_or(LiteralKind::default_bool()),
//             )
//         } else if let Some(_) = parser.peek::<Literal<IntType>>() {
//             Expression::LiteralExpr(
//                 LiteralKind::parse(parser)?.unwrap_or(LiteralKind::default_int()),
//             )
//         } else if let Some(_) = parser.peek::<Literal<UIntType>>() {
//             Expression::LiteralExpr(
//                 LiteralKind::parse(parser)?.unwrap_or(LiteralKind::default_uint()),
//             )
//         } else if let Some(_) = parser.peek::<Literal<U256>>() {
//             Expression::LiteralExpr(
//                 LiteralKind::parse(parser)?.unwrap_or(LiteralKind::default_u256()),
//             )
//         } else if let Some(_) = parser.peek::<Literal<FloatType>>() {
//             Expression::LiteralExpr(
//                 LiteralKind::parse(parser)?.unwrap_or(LiteralKind::default_float()),
//             )
//         } else if let Some(_) = parser.peek::<Identifier>() {
//             // [ArrayElements]
//             // ArithmeticOrLogicalExpr
//             // AssignmentExpr
//             // CompoundAssignmentExpr
//             // ComparisonExpr
//             // LazyBoolExpr
//             // TypeCastExpr
//             // [UnwrapOperandKind] UnwrapExpr
//             // FunctionCallExpr
//             // MethodCallExpr
//             // [CallParams]
//             // FieldAccessExpr
//             // RangeFromToExpr
//             // RangeFromExpr
//             // RangeInclusiveExpr
//             // [PathIdenSegmentKind] (PathInExpr -> Struct | TupleStruct | UnitStruct)
//             todo!()
//         } else if let Some(k) = parser.peek::<Keyword>() {
//             match k.keyword_kind {
//                 KeywordKind::KwBreak => todo!(),    // BreakExpr
//                 KeywordKind::KwContinue => todo!(), // ContinueExpr
//                 // [PathIdenSegmentKind] (PathInExpr -> Struct | TupleStruct | UnitStruct)
//                 KeywordKind::KwCrate
//                 | KeywordKind::KwSelf
//                 | KeywordKind::KwSelfType
//                 | KeywordKind::KwSuper => todo!(),
//                 KeywordKind::KwIf => todo!(), // IfExpr
//                 // [IterationExprKind] InfiniteLoopExpr | PredicateLoopExpr | IterLoopExpr
//                 KeywordKind::KwLoop | KeywordKind::KwWhile | KeywordKind::KwFor => todo!(),
//                 KeywordKind::KwMatch => todo!(),  // MatchExpr
//                 KeywordKind::KwReturn => todo!(), // ReturnExpr
//                 _ => todo!(),
//             }
//         } else if let Some(_) = parser.peek::<DocComment>() {
//             todo!()
//         } else if let Some(d) = parser.peek::<Delimiter>() {
//             match d.delim {
//                 (DelimKind::Parenthesis, DelimOrientation::Open) => {
//                     // ParenthesizedExpr
//                     // TupleExpr
//                     todo!()
//                 }
//                 (DelimKind::Bracket, DelimOrientation::Open) => todo!(), // ArrayExpr

//                 (DelimKind::Brace, DelimOrientation::Open) => todo!(), // BlockExpr

//                 _ => todo!(),
//             }
//         } else if let Some(p) = parser.peek::<Punctuation>() {
//             match p.punc_kind {
//                 PuncKind::DblDot => todo!(),       // RangeFullExpr | RangeToExpr
//                 PuncKind::DotDotEquals => todo!(), // RangeToInclusiveExpr
//                 // [NegationOperatorKind] NegationExpr
//                 PuncKind::Bang | PuncKind::Minus => todo!(),
//                 PuncKind::Hash => todo!(),      // OuterAttr
//                 PuncKind::Ampersand => todo!(), // ReferenceExpr
//                 PuncKind::Asterisk => todo!(),  // DereferenceExpr
//                 // [ClosureParamsOpt] ClosureWithBlock | ClosureWithoutBlock
//                 PuncKind::Pipe | PuncKind::DblPipe => todo!(),
//                 PuncKind::HashBang => todo!(), // InnerAttr
//                 _ => todo!(),
//             }
//         } else if let Some(_) = parser.peek::<TypeAnnotation>() {
//             todo!()
//         } else {
//             todo!()
//         };

//         Ok(Some(expr))
//     }
// }

impl Parse for IterableExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}
