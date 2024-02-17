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

impl Parse for Returnable {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let expr = if let Some(id) = parser.peek::<Identifier>()? {
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
        } else if let Some(d) = parser.peek::<Delimiter>()? {
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
        } else if let Some(l) = parser.peek::<LiteralKind>()? {
            parser.advance();

            if let Some(al) = ArithmeticOrLogicalExpr::parse(parser)? {
                Returnable::ArithmeticOrLogicalExpr(al)
            } else if let Some(tc) = TypeCastExpr::parse(parser)? {
                Returnable::TypeCastExpr(tc)
            } else {
                Returnable::LiteralExpr(l)
            }
        } else if let Some(k) = parser.peek::<Keyword>()? {
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
        } else if let Some(p) = parser.peek::<Punctuation>()? {
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
            return Err(parser.log_error(ParserErrorKind::UnexpectedToken));
        };

        Ok(Some(expr))
    }
}
