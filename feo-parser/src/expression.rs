#![allow(dead_code)]

use feo_ast::expression::Expression;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    literal::LiteralKind,
    punctuation::PuncKind,
};

use crate::{parse::Peek, parser::Peeker};

mod array_expr;
mod literal_expr;
mod struct_expr;

// impl Peek for Expression {
//     fn peek(peeker: Peeker<'_>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         let expr = if let Some(l) = LiteralKind::peek(peeker) {
//             // TODO: peek the next token to verify which `Expression` should be returned
//             // TODO: just because this is a `LiteralKind`, doesn't mean that it's a `LitExpr`
//             // TODO: e.g., it could be the LHS / RHS in an `ArithmeticOrLogicalExpr`
//             Expression::LiteralExpr(l)
//         } else if let Ok(_) = peeker.peek_identifier() {
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
//         } else if let Ok(k) = peeker.peek_keyword() {
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
//                 _ => return None,
//             }
//         } else if let Ok(d) = peeker.peek_delimiter() {
//             match d.delim {
//                 (DelimKind::Parenthesis, DelimOrientation::Open) => {
//                     // ParenthesizedExpr
//                     // TupleExpr
//                     todo!()
//                 }
//                 (DelimKind::Bracket, DelimOrientation::Open) => todo!(), // ArrayExpr

//                 (DelimKind::Brace, DelimOrientation::Open) => todo!(), // BlockExpr

//                 _ => return None,
//             }
//         } else if let Ok(p) = peeker.peek_punctuation() {
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
//                 _ => return None,
//             }
//         } else if let Ok(_) = peeker.peek_type_ann() {
//             todo!()
//         } else {
//             return None;
//         };

//         Some(expr)
//     }
// }
