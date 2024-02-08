#![allow(dead_code)]

mod array_expr;
mod literal_expr;
mod struct_expr;

use feo_ast::{
    expression::Expression,
    literal::{Literal, LiteralKind},
    token::Token,
};
use feo_error::handler::ErrorEmitted;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    primitive::PrimitiveType,
    punctuation::PuncKind,
    Identifier,
};

use crate::{
    parse::Parse,
    parser::{Parser, TokenType},
};

impl<T: Clone + PrimitiveType> Parse for Expression {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let expr = if let Some(id) = parser.peek::<Identifier>()? {
            todo!()
        } else if let Some(l) = parser.peek::<Literal<T>>()? {
            todo!()
        } else if let Some(l) = parser.peek::<Literal<String>>()? {
            todo!()
        } else {
        };

        // TODO: change this pattern to not use `TokenType`
        match TokenType::from(parser.current_token()) {
            TokenType::Literal(_) => Ok(Some(Expression::LiteralExpr(
                LiteralKind::parse(parser)?.expect("error parsing literal"),
            ))),
            TokenType::Identifier(_) => {
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
            }
            TokenType::Keyword(t) => match t {
                Token::Keyword(k) => match k.keyword_kind {
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
                },
                _ => todo!(),
            },
            TokenType::Delimiter(t) => match t {
                Token::Delim(d) => match d.delim {
                    (DelimKind::Parenthesis, DelimOrientation::Open) => {
                        // ParenthesizedExpr
                        // TupleExpr
                        todo!()
                    }
                    (DelimKind::Bracket, DelimOrientation::Open) => todo!(), // ArrayExpr

                    (DelimKind::Brace, DelimOrientation::Open) => todo!(), // BlockExpr

                    _ => todo!(),
                },
                _ => todo!(),
            },
            TokenType::Punctuation(t) => match t {
                Token::Punc(p) => match p.punc_kind {
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
                },
                _ => todo!(),
            },
            TokenType::EOF(_) => todo!(),
            TokenType::InvalidToken => todo!(),
            TokenType::DocComment(_) => todo!(),
        }
    }
}
