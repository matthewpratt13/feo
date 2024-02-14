#![allow(dead_code)]

use feo_ast::expression::Expression;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    literal::{FloatType, IntType, LiteralKind, UIntType},
    punctuation::PuncKind,
};

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
            if let Ok(_) = peeker.peek_identifier() {}
            if let Ok(k) = peeker.peek_keyword() {
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
                    _ => return None,
                }
            } else if let Ok(d) = peeker.peek_delimiter() {
                match d.delim {
                    (DelimKind::Parenthesis, DelimOrientation::Open) => {
                        // ParenthesizedExpr
                        // TupleExpr
                        todo!()
                    }
                    (DelimKind::Bracket, DelimOrientation::Open) => todo!(), // ArrayExpr

                    (DelimKind::Brace, DelimOrientation::Open) => todo!(), // BlockExpr

                    _ => return None,
                }
            } else if let Ok(p) = peeker.peek_punctuation() {
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
                    _ => return None,
                }
            } else if let Ok(_) = peeker.peek_type_ann() {
                todo!()
            } else {
                return None;
            }
        };

        Some(expr)
    }
}
