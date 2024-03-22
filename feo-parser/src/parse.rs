use feo_ast::{
    attribute::OuterAttr,
    expression::{
        ArithmeticOrLogicalExpr, ArrayExpr, AssignmentExpr, BlockExpr, BreakExpr, ClosureWithBlock,
        ClosureWithoutBlock, ComparisonExpr, CompoundAssignmentExpr, ContinueExpr, DereferenceExpr,
        ExprWithoutBlock, Expression, FieldAccessExpr, FunctionCallExpr, IfExpr, IndexExpr,
        InfiniteLoopExpr, IterLoopExpr, IterationExprKind, LazyBoolExpr, MatchExpr, MethodCallExpr,
        NegationExpr, OperatorExprKind, ParenthesizedExpr, PredicateLoopExpr, RangeExprKind,
        RangeFromExpr, RangeFromToExpr, RangeInclusiveExpr, RangeToExpr, RangeToInclusiveExpr,
        ReferenceExpr, ReturnExpr, StructExpr, TupleExpr, TupleIndexExpr, TupleStructExpr,
        TypeCastExpr, UnderscoreExpr, UnwrapExpr, Value,
    },
    item::{
        ConstantVarDef, EnumDef, FunctionSig, ImportDecl, InherentImplBlock, Item, ModWithoutBody,
        StaticVarDef, StructDef, TraitImplBlock, TupleStructDef, TypeAliasDef,
    },
    path::{PathExpr, PathIdenSegmentKind, PathInExpr, PathType, PathTypeSegment},
    pattern::{
        IdentifierPatt, ParenthesizedPatt, Pattern, RangeFromPatt, RangeInclusivePatt,
        RangePattKind, RangeToInclusivePatt, StructPatt, TuplePatt, TupleStructPatt, WildcardPatt,
    },
    statement::{ExprStatement, LetStatement, Statement},
    ty::{
        ArrayType, ClosureType, FunctionType, ParenthesizedType, ReferenceType, SelfType,
        TupleType, UnitType,
    },
    Type,
};
use feo_error::error::CompilerError;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    literal::LiteralKind,
    punctuation::PuncKind,
    BuiltInType, Delimiter, Identifier, Keyword, Punctuation,
};

use crate::parser::Parser;

// literals, attributes, paths, parenthesized expressions, helper types (e.g., `StructExprField`)
pub trait ParseTerm {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized;
}

pub trait ParseTermCollection {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized;
}

pub trait ParseExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized;
}

pub trait ParseItem {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized;
}

pub trait ParsePatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized;
}

pub trait ParseStatement {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized;
}

pub trait ParseType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized;
}

///////////////////////////////////////////////////////////////////////////////

impl ParseExpr for Expression {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            if &id.name == "_" {
                return Ok(Some(Expression::UnderscoreExpr(UnderscoreExpr(id))));
            }

            println!("Expression: first token: {:#?}", &id);

            if let Some(d) = parser.peek_next::<Delimiter>() {
                println!("Expression: second token: {:#?}", &d);

                match d.delim {
                    (DelimKind::Parenthesis, DelimOrientation::Open) => {
                        if let Some(fc) = FunctionCallExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::FunctionCallExpr(fc)));
                        }

                        if let Some(ts) = TupleStructExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::TupleStructExpr(ts)));
                        }
                    }
                    (DelimKind::Bracket, DelimOrientation::Open) => {
                        if let Some(ie) = IndexExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::IndexExpr(ie)));
                        }
                    }
                    (DelimKind::Brace, DelimOrientation::Open) => {
                        if let Some(se) = StructExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::StructExpr(se)));
                        }
                    }

                    _ => (),
                }
            } else if let Some(p) = parser.peek_next::<Punctuation>() {
                println!("Expression: second token: {:#?}", &p);

                match p.punc_kind {
                    PuncKind::FullStop => {
                        if let Some(fa) = FieldAccessExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::FieldAccessExpr(fa)));
                        }

                        if let Some(mc) = MethodCallExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::MethodCallExpr(mc)));
                        }
                    }

                    PuncKind::DblColon => {
                        if let Some(path_expr) = PathInExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::PathExpr(path_expr)));
                        }
                    }

                    PuncKind::Equals => {
                        if let Some(ae) = AssignmentExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::OperatorExpr(
                                OperatorExprKind::Assignment(ae),
                            )));
                        }
                    }

                    PuncKind::Plus
                    | PuncKind::Minus
                    | PuncKind::Asterisk
                    | PuncKind::ForwardSlash
                    | PuncKind::Percent
                    | PuncKind::Ampersand
                    | PuncKind::Pipe
                    | PuncKind::Caret
                    | PuncKind::DblLessThan
                    | PuncKind::DblGreaterThan => {
                        if let Some(al) = ArithmeticOrLogicalExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::OperatorExpr(
                                OperatorExprKind::ArithmeticOrLogical(al),
                            )));
                        }
                    }

                    PuncKind::LessThan
                    | PuncKind::GreaterThan
                    | PuncKind::LessThanEquals
                    | PuncKind::GreaterThanEquals
                    | PuncKind::DblEquals
                    | PuncKind::BangEquals => {
                        if let Some(ce) = ComparisonExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::OperatorExpr(
                                OperatorExprKind::Comparison(ce),
                            )));
                        }
                    }

                    PuncKind::QuestionMark => {
                        if let Some(ue) = UnwrapExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::OperatorExpr(
                                OperatorExprKind::UnwrapExpr(ue),
                            )));
                        }
                    }

                    PuncKind::PlusEquals
                    | PuncKind::MinusEquals
                    | PuncKind::AsteriskEquals
                    | PuncKind::ForwardSlashEquals
                    | PuncKind::PercentEquals => {
                        if let Some(cae) = CompoundAssignmentExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::OperatorExpr(
                                OperatorExprKind::CompoundAssign(cae),
                            )));
                        }
                    }

                    PuncKind::DblAmpersand | PuncKind::DblPipe => {
                        if let Some(lb) = LazyBoolExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::OperatorExpr(OperatorExprKind::LazyBool(
                                lb,
                            ))));
                        }
                    }

                    PuncKind::DblDot => {
                        if let Some(rft) = RangeFromToExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::RangeExpr(
                                RangeExprKind::RangeFromToExpr(rft),
                            )));
                        }

                        if let Some(rfe) = RangeFromExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::RangeExpr(RangeExprKind::RangeFromExpr(
                                rfe,
                            ))));
                        }
                    }

                    PuncKind::DotDotEquals => {
                        if let Some(rie) = RangeInclusiveExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::RangeExpr(
                                RangeExprKind::RangeInclusiveExpr(rie),
                            )));
                        }
                    }

                    _ => (),
                }
            } else if let Some(k) = parser.peek_next::<Keyword>() {
                println!("Expression: second token: {:#?}", &k);

                match &k.keyword_kind {
                    KeywordKind::KwAs => {
                        if let Some(tc) = TypeCastExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::OperatorExpr(OperatorExprKind::TypeCast(
                                tc,
                            ))));
                        }
                    }

                    _ => (),
                }
            } else {
                let path_expr = PathInExpr {
                    first_segment: PathIdenSegmentKind::Iden(id),
                    subsequent_segments: None,
                };

                println!("Expression: exit as `PathInExpr` ({:#?})", &path_expr);

                return Ok(Some(Expression::PathExpr(path_expr)));
            }
        } else if let Some(d) = parser.peek_current::<Delimiter>() {
            println!("Expression: first token: {:#?}", &d);

            match &d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(par) = ParenthesizedExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::ParenthesizedExpr(par)));
                    }

                    if let Some(fc) = FunctionCallExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::FunctionCallExpr(fc)));
                    }

                    if let Some(mc) = MethodCallExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::MethodCallExpr(mc)));
                    }

                    if let Some(ti) = TupleIndexExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::TupleIndexExpr(ti)));
                    }

                    if let Some(te) = TupleExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::TupleExpr(te)));
                    }

                    if let Some(ce) = ComparisonExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(
                            OperatorExprKind::Comparison(ce),
                        )));
                    }
                }

                (DelimKind::Bracket, DelimOrientation::Open) => {
                    if let Some(ie) = IndexExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::IndexExpr(ie)));
                    }

                    if let Some(ae) = ArrayExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::ArrayExpr(ae)));
                    }

                    if let Some(al) = ArithmeticOrLogicalExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(
                            OperatorExprKind::ArithmeticOrLogical(al),
                        )));
                    }

                    if let Some(ce) = ComparisonExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(
                            OperatorExprKind::Comparison(ce),
                        )));
                    }

                    if let Some(cae) = CompoundAssignmentExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(
                            OperatorExprKind::CompoundAssign(cae),
                        )));
                    }

                    if let Some(ne) = NegationExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(OperatorExprKind::Negation(
                            ne,
                        ))));
                    }

                    if let Some(de) = DereferenceExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(
                            OperatorExprKind::Dereference(de),
                        )));
                    }

                    if let Some(re) = ReferenceExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(OperatorExprKind::Reference(
                            re,
                        ))));
                    }

                    if let Some(ue) = UnwrapExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(
                            OperatorExprKind::UnwrapExpr(ue),
                        )));
                    }

                    if let Some(lb) = LazyBoolExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(OperatorExprKind::LazyBool(
                            lb,
                        ))));
                    }

                    if let Some(rft) = RangeFromToExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::RangeExpr(RangeExprKind::RangeFromToExpr(
                            rft,
                        ))));
                    }

                    if let Some(rfe) = RangeFromExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::RangeExpr(RangeExprKind::RangeFromExpr(
                            rfe,
                        ))));
                    }

                    if let Some(rie) = RangeInclusiveExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::RangeExpr(
                            RangeExprKind::RangeInclusiveExpr(rie),
                        )));
                    }
                }

                (DelimKind::Brace, DelimOrientation::Open) => {
                    if let Some(be) = BlockExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::BlockExpr(be)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(l) = parser.peek_current::<LiteralKind>() {
            println!("Expression: first token: {:#?}", &l);

            if let Some(k) = parser.peek_next::<Keyword>() {
                println!("Expression: second token: {:#?}", &k);

                match &k.keyword_kind {
                    KeywordKind::KwAs => {
                        if let Some(tc) = TypeCastExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::OperatorExpr(OperatorExprKind::TypeCast(
                                tc,
                            ))));
                        }
                    }

                    _ => (),
                }
            }

            match parser.peek_next::<Punctuation>() {
                Some(Punctuation {
                    punc_kind: PuncKind::Plus,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::Minus,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::Asterisk,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::ForwardSlash,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::Percent,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::Ampersand,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::Pipe,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::Caret,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::DblLessThan,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::DblGreaterThan,
                    ..
                }) => {
                    if let Some(al) = ArithmeticOrLogicalExpr::parse(parser).unwrap_or(None) {
                        println!("return expression: {:#?}", &al);

                        return Ok(Some(Expression::OperatorExpr(
                            OperatorExprKind::ArithmeticOrLogical(al),
                        )));
                    }
                }

                Some(Punctuation {
                    punc_kind: PuncKind::LessThan,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::GreaterThan,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::LessThanEquals,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::GreaterThanEquals,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::DblEquals,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::BangEquals,
                    ..
                }) => {
                    if let Some(ce) = ComparisonExpr::parse(parser).unwrap_or(None) {
                        println!("return expression: {:#?}", &ce);

                        return Ok(Some(Expression::OperatorExpr(
                            OperatorExprKind::Comparison(ce),
                        )));
                    }
                }

                Some(Punctuation {
                    punc_kind: PuncKind::DblDot,
                    ..
                }) => {
                    if let Some(rft) = RangeFromToExpr::parse(parser).unwrap_or(None) {
                        println!("return expression: {:#?}", &rft);

                        return Ok(Some(Expression::RangeExpr(RangeExprKind::RangeFromToExpr(
                            rft,
                        ))));
                    }

                    if let Some(rfe) = RangeFromExpr::parse(parser).unwrap_or(None) {
                        println!("return expression: {:#?}", &rfe);

                        return Ok(Some(Expression::RangeExpr(RangeExprKind::RangeFromExpr(
                            rfe,
                        ))));
                    }
                }

                Some(Punctuation {
                    punc_kind: PuncKind::DotDotEquals,
                    ..
                }) => {
                    if let Some(rie) = RangeInclusiveExpr::parse(parser).unwrap_or(None) {
                        println!("return expression: {:#?}", &rie);

                        return Ok(Some(Expression::RangeExpr(
                            RangeExprKind::RangeInclusiveExpr(rie),
                        )));
                    }
                }

                Some(Punctuation {
                    punc_kind: PuncKind::PlusEquals,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::MinusEquals,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::AsteriskEquals,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::ForwardSlashEquals,
                    ..
                })
                | Some(Punctuation {
                    punc_kind: PuncKind::PercentEquals,
                    ..
                }) => {
                    if let Some(cae) = CompoundAssignmentExpr::parse(parser).unwrap_or(None) {
                        println!("return expression: {:#?}", &cae);

                        return Ok(Some(Expression::OperatorExpr(
                            OperatorExprKind::CompoundAssign(cae),
                        )));
                    }
                }

                _ => (),
            }

            if let Some(ne) = NegationExpr::parse(parser).unwrap_or(None) {
                println!("return expression: {:#?}", &ne);

                return Ok(Some(Expression::OperatorExpr(OperatorExprKind::Negation(
                    ne,
                ))));
            }

            println!("Expression: exit as `Literal` ({:#?})", &l);

            return Ok(Some(Expression::Literal(l)));
        } else if let Some(k) = parser.peek_current::<Keyword>() {
            println!("Expression: first token: {:#?}", &k);

            match &k.keyword_kind {
                KeywordKind::KwBreak => {
                    if let Some(be) = BreakExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::BreakExpr(be)));
                    }
                }
                KeywordKind::KwContinue => {
                    if let Some(ce) = ContinueExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::ContinueExpr(ce)));
                    }
                }

                KeywordKind::KwCrate
                | KeywordKind::KwSelf
                | KeywordKind::KwSelfType
                | KeywordKind::KwSuper => match parser.peek_next::<Punctuation>() {
                    Some(Punctuation {
                        punc_kind: PuncKind::DblColon,
                        ..
                    }) => {
                        if let Some(pth) = PathInExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Expression::PathExpr(pth)));
                        }
                    }

                    _ => return Ok(None),
                },

                KeywordKind::KwFor => {
                    if let Some(ile) = IterLoopExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::IterationExpr(
                            IterationExprKind::IterLoop(ile),
                        )));
                    }
                }

                KeywordKind::KwIf => {
                    if let Some(ife) = IfExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::IfExpr(ife)));
                    }
                }

                KeywordKind::KwLoop => {
                    if let Some(inf) = InfiniteLoopExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::IterationExpr(
                            IterationExprKind::InfiniteLoop(inf),
                        )));
                    }
                }

                KeywordKind::KwMatch => {
                    if let Some(me) = MatchExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::MatchExpr(me)));
                    }
                }

                KeywordKind::KwReturn => {
                    if let Some(rtn) = ReturnExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::ReturnExpr(rtn)));
                    }
                }

                KeywordKind::KwWhile => {
                    if let Some(pre) = PredicateLoopExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::IterationExpr(
                            IterationExprKind::PredicateLoop(pre),
                        )));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(p) = parser.peek_current::<Punctuation>() {
            println!("Expression: first token: {:#?}", &p);

            match &p.punc_kind {
                PuncKind::Bang | PuncKind::Minus => {
                    if let Some(ne) = NegationExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(OperatorExprKind::Negation(
                            ne,
                        ))));
                    }

                    if let Some(al) = ArithmeticOrLogicalExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(
                            OperatorExprKind::ArithmeticOrLogical(al),
                        )));
                    }

                    if let Some(ce) = ComparisonExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(
                            OperatorExprKind::Comparison(ce),
                        )));
                    }

                    if let Some(cae) = CompoundAssignmentExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(
                            OperatorExprKind::CompoundAssign(cae),
                        )));
                    }

                    if let Some(rft) = RangeFromToExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::RangeExpr(RangeExprKind::RangeFromToExpr(
                            rft,
                        ))));
                    }

                    if let Some(rfe) = RangeFromExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::RangeExpr(RangeExprKind::RangeFromExpr(
                            rfe,
                        ))));
                    }

                    if let Some(rie) = RangeInclusiveExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::RangeExpr(
                            RangeExprKind::RangeInclusiveExpr(rie),
                        )));
                    }

                    if let Some(lb) = LazyBoolExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(OperatorExprKind::LazyBool(
                            lb,
                        ))));
                    }
                }

                PuncKind::Asterisk => {
                    if let Some(de) = DereferenceExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(
                            OperatorExprKind::Dereference(de),
                        )));
                    }
                }

                PuncKind::Ampersand => {
                    if let Some(re) = ReferenceExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::OperatorExpr(OperatorExprKind::Reference(
                            re,
                        ))));
                    }
                }

                PuncKind::Pipe => {
                    if let Some(cwb) = ClosureWithBlock::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::ClosureWithBlock(cwb)));
                    }
                }

                PuncKind::DblDot => {
                    if let Some(rte) = RangeToExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::RangeExpr(RangeExprKind::RangeToExpr(rte))));
                    }
                }

                PuncKind::DotDotEquals => {
                    if let Some(rti) = RangeToInclusiveExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::RangeExpr(
                            RangeExprKind::RangeToInclusiveExpr(rti),
                        )));
                    }
                }

                PuncKind::DblPipe => {
                    if let Some(c) = ClosureWithoutBlock::parse(parser).unwrap_or(None) {
                        return Ok(Some(Expression::ClosureWithoutBlock(c)));
                    }
                }

                _ => return Ok(None),
            }
        } else {
            println!(
                "Expression: return none. current token: {:#?}",
                parser.current_token()
            );
            return Ok(None);
        }

        Err(parser.errors())
    }
}

///////////////////////////////////////////////////////////////////////////////

impl ParseExpr for ExprWithoutBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            if &id.name == "_" {
                return Ok(Some(ExprWithoutBlock::UnderscoreExpr(UnderscoreExpr(id))));
            }

            if let Some(d) = parser.peek_next::<Delimiter>() {
                println!("Expression: second token: {:#?}", &d);

                match d.delim {
                    (DelimKind::Parenthesis, DelimOrientation::Open) => {
                        if let Some(fc) = FunctionCallExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::FunctionCallExpr(fc)));
                        }

                        if let Some(ts) = TupleStructExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::TupleStructExpr(ts)));
                        }
                    }
                    (DelimKind::Bracket, DelimOrientation::Open) => {
                        if let Some(ie) = IndexExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::IndexExpr(ie)));
                        }
                    }
                    (DelimKind::Brace, DelimOrientation::Open) => {
                        if let Some(se) = StructExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::StructExpr(se)));
                        }
                    }

                    _ => (),
                }
            } else if let Some(p) = parser.peek_next::<Punctuation>() {
                println!("Expression: second token: {:#?}", &p);

                match p.punc_kind {
                    PuncKind::FullStop => {
                        if let Some(fa) = FieldAccessExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::FieldAccessExpr(fa)));
                        }

                        if let Some(mc) = MethodCallExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::MethodCallExpr(mc)));
                        }
                    }

                    PuncKind::DblColon => {
                        if let Some(path_expr) = PathInExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::PathExpr(path_expr)));
                        }
                    }

                    PuncKind::Equals => {
                        if let Some(ae) = AssignmentExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::Assignment(ae),
                            )));
                        }
                    }

                    PuncKind::Plus
                    | PuncKind::Minus
                    | PuncKind::Asterisk
                    | PuncKind::ForwardSlash
                    | PuncKind::Percent
                    | PuncKind::Ampersand
                    | PuncKind::Pipe
                    | PuncKind::Caret
                    | PuncKind::DblLessThan
                    | PuncKind::DblGreaterThan => {
                        if let Some(al) = ArithmeticOrLogicalExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::ArithmeticOrLogical(al),
                            )));
                        }
                    }

                    PuncKind::LessThan
                    | PuncKind::GreaterThan
                    | PuncKind::LessThanEquals
                    | PuncKind::GreaterThanEquals
                    | PuncKind::DblEquals
                    | PuncKind::BangEquals => {
                        if let Some(ce) = ComparisonExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::Comparison(ce),
                            )));
                        }
                    }

                    PuncKind::QuestionMark => {
                        if let Some(ue) = UnwrapExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::UnwrapExpr(ue),
                            )));
                        }
                    }

                    PuncKind::PlusEquals
                    | PuncKind::MinusEquals
                    | PuncKind::AsteriskEquals
                    | PuncKind::ForwardSlashEquals
                    | PuncKind::PercentEquals => {
                        if let Some(cae) = CompoundAssignmentExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::CompoundAssign(cae),
                            )));
                        }
                    }

                    PuncKind::DblAmpersand | PuncKind::DblPipe => {
                        if let Some(lb) = LazyBoolExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::LazyBool(lb),
                            )));
                        }
                    }

                    PuncKind::DblDot => {
                        if let Some(rft) = RangeFromToExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::RangeExpr(
                                RangeExprKind::RangeFromToExpr(rft),
                            )));
                        }

                        if let Some(rfe) = RangeFromExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::RangeExpr(
                                RangeExprKind::RangeFromExpr(rfe),
                            )));
                        }
                    }

                    PuncKind::DotDotEquals => {
                        if let Some(rie) = RangeInclusiveExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::RangeExpr(
                                RangeExprKind::RangeInclusiveExpr(rie),
                            )));
                        }
                    }

                    _ => (),
                }
            } else if let Some(k) = parser.peek_next::<Keyword>() {
                println!("Expression: second token: {:#?}", &k);

                match &k.keyword_kind {
                    KeywordKind::KwAs => {
                        if let Some(tc) = TypeCastExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::TypeCast(tc),
                            )));
                        }
                    }

                    _ => (),
                }
            } else {
                let path_expr = PathInExpr {
                    first_segment: PathIdenSegmentKind::Iden(id),
                    subsequent_segments: None,
                };

                println!("Expression: exit as `PathInExpr` ({:#?})", &path_expr);

                return Ok(Some(ExprWithoutBlock::PathExpr(path_expr)));
            }
        } else if let Some(d) = parser.peek_current::<Delimiter>() {
            match &d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(par) = ParenthesizedExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::ParenthesizedExpr(par)));
                    }

                    if let Some(fc) = FunctionCallExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::FunctionCallExpr(fc)));
                    }

                    if let Some(mc) = MethodCallExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::MethodCallExpr(mc)));
                    }

                    if let Some(ti) = TupleIndexExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::TupleIndexExpr(ti)));
                    }

                    if let Some(te) = TupleExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::TupleExpr(te)));
                    }

                    if let Some(ce) = ComparisonExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::OperatorExpr(
                            OperatorExprKind::Comparison(ce),
                        )));
                    }
                }

                (DelimKind::Bracket, DelimOrientation::Open) => {
                    if let Some(ie) = IndexExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::IndexExpr(ie)));
                    }

                    if let Some(ae) = ArrayExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::ArrayExpr(ae)));
                    }

                    if let Some(al) = ArithmeticOrLogicalExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::OperatorExpr(
                            OperatorExprKind::ArithmeticOrLogical(al),
                        )));
                    }

                    if let Some(ce) = ComparisonExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::OperatorExpr(
                            OperatorExprKind::Comparison(ce),
                        )));
                    }

                    if let Some(cae) = CompoundAssignmentExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::OperatorExpr(
                            OperatorExprKind::CompoundAssign(cae),
                        )));
                    }

                    if let Some(ne) = NegationExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::OperatorExpr(
                            OperatorExprKind::Negation(ne),
                        )));
                    }

                    if let Some(de) = DereferenceExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::OperatorExpr(
                            OperatorExprKind::Dereference(de),
                        )));
                    }

                    if let Some(re) = ReferenceExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::OperatorExpr(
                            OperatorExprKind::Reference(re),
                        )));
                    }

                    if let Some(ue) = UnwrapExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::OperatorExpr(
                            OperatorExprKind::UnwrapExpr(ue),
                        )));
                    }

                    if let Some(lb) = LazyBoolExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::OperatorExpr(
                            OperatorExprKind::LazyBool(lb),
                        )));
                    }

                    if let Some(rft) = RangeFromToExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::RangeExpr(
                            RangeExprKind::RangeFromToExpr(rft),
                        )));
                    }

                    if let Some(rfe) = RangeFromExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::RangeExpr(
                            RangeExprKind::RangeFromExpr(rfe),
                        )));
                    }

                    if let Some(rie) = RangeInclusiveExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(ExprWithoutBlock::RangeExpr(
                            RangeExprKind::RangeInclusiveExpr(rie),
                        )));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(l) = parser.peek_current::<LiteralKind>() {
            if let Some(k) = parser.peek_next::<Keyword>() {
                match &k.keyword_kind {
                    KeywordKind::KwAs => {
                        if let Some(tc) = TypeCastExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::TypeCast(tc),
                            )));
                        }
                    }

                    _ => (),
                }

                match parser.peek_next::<Punctuation>() {
                    Some(Punctuation {
                        punc_kind: PuncKind::Plus,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::Minus,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::Asterisk,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::ForwardSlash,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::Percent,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::Ampersand,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::Pipe,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::Caret,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::DblLessThan,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::DblGreaterThan,
                        ..
                    }) => {
                        if let Some(al) = ArithmeticOrLogicalExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::ArithmeticOrLogical(al),
                            )));
                        }
                    }

                    Some(Punctuation {
                        punc_kind: PuncKind::LessThan,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::GreaterThan,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::LessThanEquals,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::GreaterThanEquals,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::DblEquals,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::BangEquals,
                        ..
                    }) => {
                        if let Some(ce) = ComparisonExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::Comparison(ce),
                            )));
                        }
                    }

                    Some(Punctuation {
                        punc_kind: PuncKind::DblDot,
                        ..
                    }) => {
                        if let Some(rft) = RangeFromToExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::RangeExpr(
                                RangeExprKind::RangeFromToExpr(rft),
                            )));
                        }

                        if let Some(rfe) = RangeFromExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::RangeExpr(
                                RangeExprKind::RangeFromExpr(rfe),
                            )));
                        }
                    }

                    Some(Punctuation {
                        punc_kind: PuncKind::DotDotEquals,
                        ..
                    }) => {
                        if let Some(rie) = RangeInclusiveExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::RangeExpr(
                                RangeExprKind::RangeInclusiveExpr(rie),
                            )));
                        }
                    }

                    Some(Punctuation {
                        punc_kind: PuncKind::PlusEquals,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::MinusEquals,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::AsteriskEquals,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::ForwardSlashEquals,
                        ..
                    })
                    | Some(Punctuation {
                        punc_kind: PuncKind::PercentEquals,
                        ..
                    }) => {
                        if let Some(cae) = CompoundAssignmentExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::CompoundAssign(cae),
                            )));
                        }
                    }

                    _ => (),
                }

                if let Some(ne) = NegationExpr::parse(parser).unwrap_or(None) {
                    return Ok(Some(ExprWithoutBlock::OperatorExpr(
                        OperatorExprKind::Negation(ne),
                    )));
                }

                return Ok(Some(ExprWithoutBlock::Literal(l)));
            } else if let Some(k) = parser.peek_current::<Keyword>() {
                match &k.keyword_kind {
                    KeywordKind::KwBreak => {
                        if let Some(be) = BreakExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::BreakExpr(be)));
                        }
                    }
                    KeywordKind::KwContinue => {
                        if let Some(ce) = ContinueExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::ContinueExpr(ce)));
                        }
                    }

                    KeywordKind::KwCrate
                    | KeywordKind::KwSelf
                    | KeywordKind::KwSelfType
                    | KeywordKind::KwSuper => match parser.peek_next::<Punctuation>() {
                        Some(Punctuation {
                            punc_kind: PuncKind::DblColon,
                            ..
                        }) => {
                            if let Some(pth) = PathInExpr::parse(parser).unwrap_or(None) {
                                return Ok(Some(ExprWithoutBlock::PathExpr(pth)));
                            }
                        }

                        _ => return Ok(None),
                    },

                    KeywordKind::KwReturn => {
                        if let Some(rtn) = ReturnExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::ReturnExpr(rtn)));
                        }
                    }

                    _ => return Ok(None),
                }
            } else if let Some(p) = parser.peek_current::<Punctuation>() {
                match &p.punc_kind {
                    PuncKind::Bang | PuncKind::Minus => {
                        if let Some(ne) = NegationExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::Negation(ne),
                            )));
                        }

                        if let Some(al) = ArithmeticOrLogicalExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::ArithmeticOrLogical(al),
                            )));
                        }

                        if let Some(ce) = ComparisonExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::Comparison(ce),
                            )));
                        }

                        if let Some(cae) = CompoundAssignmentExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::CompoundAssign(cae),
                            )));
                        }

                        if let Some(rft) = RangeFromToExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::RangeExpr(
                                RangeExprKind::RangeFromToExpr(rft),
                            )));
                        }

                        if let Some(rfe) = RangeFromExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::RangeExpr(
                                RangeExprKind::RangeFromExpr(rfe),
                            )));
                        }

                        if let Some(rie) = RangeInclusiveExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::RangeExpr(
                                RangeExprKind::RangeInclusiveExpr(rie),
                            )));
                        }

                        if let Some(lb) = LazyBoolExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::LazyBool(lb),
                            )));
                        }
                    }

                    PuncKind::Asterisk => {
                        if let Some(de) = DereferenceExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::Dereference(de),
                            )));
                        }
                    }

                    PuncKind::Ampersand => {
                        if let Some(re) = ReferenceExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::OperatorExpr(
                                OperatorExprKind::Reference(re),
                            )));
                        }
                    }

                    PuncKind::DblDot => {
                        if let Some(rte) = RangeToExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::RangeExpr(
                                RangeExprKind::RangeToExpr(rte),
                            )));
                        }
                    }

                    PuncKind::DotDotEquals => {
                        if let Some(rti) = RangeToInclusiveExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::RangeExpr(
                                RangeExprKind::RangeToInclusiveExpr(rti),
                            )));
                        }
                    }

                    PuncKind::DblPipe => {
                        if let Some(c) = ClosureWithoutBlock::parse(parser).unwrap_or(None) {
                            return Ok(Some(ExprWithoutBlock::ClosureWithoutBlock(c)));
                        }
                    }

                    _ => return Ok(None),
                }
            } else {
                return Ok(None);
            }
        }

        Err(parser.errors())
    }
}

///////////////////////////////////////////////////////////////////////////////

impl ParseItem for Item {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(Punctuation {
            punc_kind: PuncKind::HashSign,
            ..
        }) = parser.peek_current()
        {
            if let Some(_) = OuterAttr::parse(parser)? {
                parser.next_token();

                if let Some(i) = get_item_by_keyword(parser)? {
                    return Ok(Some(i));
                }
            }
        } else if let Some(_) = parser.peek_current::<Keyword>() {
            return get_item_by_keyword(parser);
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

///////////////////////////////////////////////////////////////////////////////

impl ParsePatt for Pattern {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            if &id.name == "_" {
                return Ok(Some(Pattern::WildcardPatt(WildcardPatt(id))));
            }

            match parser.peek_next::<Delimiter>() {
                Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(tsp) = TupleStructPatt::parse(parser).unwrap_or(None) {
                        return Ok(Some(Pattern::TupleStructPatt(tsp)));
                    }
                }

                Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(sp) = StructPatt::parse(parser).unwrap_or(None) {
                        return Ok(Some(Pattern::StructPatt(sp)));
                    }
                }

                _ => (),
            }

            match parser.peek_next::<Punctuation>() {
                Some(Punctuation {
                    punc_kind: PuncKind::DblColon,
                    ..
                }) => {
                    if let Some(path_patt) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Pattern::PathPatt(path_patt)));
                    }
                }

                Some(Punctuation {
                    punc_kind: PuncKind::DblDot,
                    ..
                }) => {
                    if let Some(rfp) = RangeFromPatt::parse(parser).unwrap_or(None) {
                        return Ok(Some(Pattern::RangePatt(RangePattKind::RangeFromPatt(rfp))));
                    }
                }

                Some(Punctuation {
                    punc_kind: PuncKind::DotDotEquals,
                    ..
                }) => {
                    if let Some(rip) = RangeInclusivePatt::parse(parser).unwrap_or(None) {
                        return Ok(Some(Pattern::RangePatt(RangePattKind::RangeInclusivePatt(
                            rip,
                        ))));
                    }
                }

                _ => (),
            }

            let identifier_patt = IdentifierPatt {
                kw_ref_opt: None,
                kw_mut_opt: None,
                name: id,
            };

            return Ok(Some(Pattern::IdentifierPatt(identifier_patt)));
        }

        if let Some(d) = parser.peek_current::<Delimiter>() {
            match &d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(par) = ParenthesizedPatt::parse(parser).unwrap_or(None) {
                        return Ok(Some(Pattern::ParenthesizedPatt(par)));
                    }

                    if let Some(tp) = TuplePatt::parse(parser).unwrap_or(None) {
                        return Ok(Some(Pattern::TuplePatt(tp)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(l) = parser.peek_current::<LiteralKind>() {
            if let Some(rfp) = RangeFromPatt::parse(parser).unwrap_or(None) {
                return Ok(Some(Pattern::RangePatt(RangePattKind::RangeFromPatt(rfp))));
            }

            if let Some(rip) = RangeInclusivePatt::parse(parser).unwrap_or(None) {
                return Ok(Some(Pattern::RangePatt(RangePattKind::RangeInclusivePatt(
                    rip,
                ))));
            }

            return Ok(Some(Pattern::Literal(l)));
        }

        if let Some(k) = parser.peek_current::<Keyword>() {
            match &k.keyword_kind {
                KeywordKind::KwCrate
                | KeywordKind::KwSelf
                | KeywordKind::KwSelfType
                | KeywordKind::KwSuper => match parser.peek_next::<Punctuation>() {
                    Some(Punctuation {
                        punc_kind: PuncKind::DblColon,
                        ..
                    }) => {
                        if let Some(path_patt) = PathInExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Pattern::PathPatt(path_patt)));
                        }
                    }

                    _ => return Ok(None),
                },

                _ => return Ok(None),
            }
        } else if let Some(p) = parser.peek_current::<Punctuation>() {
            match &p.punc_kind {
                PuncKind::DotDotEquals => {
                    if let Some(rti) = RangeToInclusivePatt::parse(parser).unwrap_or(None) {
                        return Ok(Some(Pattern::RangePatt(
                            RangePattKind::RangeToInclusivePatt(rti),
                        )));
                    }
                }

                _ => return Ok(None),
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

///////////////////////////////////////////////////////////////////////////////

impl ParseStatement for Statement {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            println!("Statement: first token: {:#?}", &id);

            if let Some(es) = ExprStatement::parse(parser)? {
                println!("Statement: {:#?}", &es);
                return Ok(Some(Statement::ExprStatement(es)));
            }
        } else if let Some(d) = parser.peek_current::<Delimiter>() {
            println!("Statement: first token: {:#?}", &d);

            if let Some(es) = ExprStatement::parse(parser)? {
                return Ok(Some(Statement::ExprStatement(es)));
            }
        } else if let Some(l) = parser.peek_current::<LiteralKind>() {
            println!("Statement: first token: {:#?}", &l);

            if let Some(es) = ExprStatement::parse(parser)? {
                println!("Statement: {:#?}", &es);
                return Ok(Some(Statement::ExprStatement(es)));
            }
        } else if let Some(k) = parser.peek_current::<Keyword>() {
            println!("Statement: first token: {:#?}", &k);

            match &k.keyword_kind {
                KeywordKind::KwLet => {
                    if let Some(ls) = LetStatement::parse(parser)? {
                        println!("Statement: {:#?}", &ls);
                        return Ok(Some(Statement::LetStatement(ls)));
                    }
                }

                KeywordKind::KwConst
                | KeywordKind::KwEnum
                | KeywordKind::KwFunc
                | KeywordKind::KwImpl
                | KeywordKind::KwImport
                | KeywordKind::KwMod
                | KeywordKind::KwPub
                | KeywordKind::KwStatic
                | KeywordKind::KwStruct
                | KeywordKind::KwTrait
                | KeywordKind::KwType => {
                    if let Some(i) = get_item_by_keyword(parser)? {
                        println!("Statement: {:#?}", &i);
                        return Ok(Some(Statement::Item(i)));
                    }
                }

                _ => {
                    if let Some(es) = ExprStatement::parse(parser)? {
                        println!("Statement: {:#?}", &es);
                        return Ok(Some(Statement::ExprStatement(es)));
                    }
                }
            }
        } else if let Some(p) = parser.peek_current::<Punctuation>() {
            println!("Statement: first token: {:#?}", &p);

            match p.punc_kind {
                PuncKind::HashSign => {
                    if let Some(oa) = OuterAttr::parse(parser)? {
                        println!("Statement: encountered attribute ({:#?})", &oa);

                        parser.next_token();

                        if let Some(Keyword {
                            keyword_kind: KeywordKind::KwLet,
                            ..
                        }) = parser.peek_current()
                        {
                            if let Some(ls) = LetStatement::parse(parser)? {
                                println!("Statement: {:#?}", &ls);
                                return Ok(Some(Statement::LetStatement(ls)));
                            }
                        } else {
                            return Ok(None);
                        }

                        if let Some(i) = get_item_by_keyword(parser)? {
                            println!("Statement: {:#?}", &i);
                            return Ok(Some(Statement::Item(i)));
                        }
                    }
                }

                _ => {
                    if let Some(es) = ExprStatement::parse(parser)? {
                        println!("Statement: {:#?}", &es);
                        return Ok(Some(Statement::ExprStatement(es)));
                    }
                }
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

///////////////////////////////////////////////////////////////////////////////

impl ParseType for Type {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            if &id.name == "_" {
                if let Some(bit) = BuiltInType::parse(parser).unwrap_or(None) {
                    return Ok(Some(Type::InferredType(bit)));
                }
            }

            if let Some(bit) = BuiltInType::parse(parser).unwrap_or(None) {
                return Ok(Some(Type::PrimitiveType(bit)));
            }

            match parser.peek_next::<Punctuation>() {
                Some(Punctuation {
                    punc_kind: PuncKind::DblColon,
                    ..
                }) => {
                    if let Some(path_type) = PathType::parse(parser).unwrap_or(None) {
                        return Ok(Some(Type::UserDefinedType(path_type)));
                    }
                }

                _ => (),
            }

            let path_type = PathType {
                first_segment: PathTypeSegment::Iden(id),
                subsequent_segments: None,
            };

            return Ok(Some(Type::UserDefinedType(path_type)));
        }

        if let Some(d) = parser.peek_current::<Delimiter>() {
            match &d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(ut) = UnitType::parse(parser).unwrap_or(None) {
                        return Ok(Some(Type::UnitType(ut)));
                    }
                    if let Some(par) = ParenthesizedType::parse(parser).unwrap_or(None) {
                        return Ok(Some(Type::ParenthesizedType(par)));
                    }

                    if let Some(tup) = TupleType::parse(parser).unwrap_or(None) {
                        return Ok(Some(Type::TupleType(tup)));
                    }
                }
                (DelimKind::Bracket, DelimOrientation::Open) => {
                    if let Some(arr) = ArrayType::parse(parser).unwrap_or(None) {
                        return Ok(Some(Type::ArrayType(arr)));
                    }
                }

                _ => return Ok(None),
            }
        }

        if let Some(k) = parser.peek_current::<Keyword>() {
            match &k.keyword_kind {
                KeywordKind::KwCrate
                | KeywordKind::KwSelf
                | KeywordKind::KwSelfType
                | KeywordKind::KwSuper => match parser.peek_next::<Punctuation>() {
                    Some(Punctuation {
                        punc_kind: PuncKind::DblColon,
                        ..
                    }) => {
                        if let Some(path_type) = PathType::parse(parser).unwrap_or(None) {
                            return Ok(Some(Type::UserDefinedType(path_type)));
                        }
                    }

                    _ => {
                        if let Some(st) = SelfType::parse(parser).unwrap_or(None) {
                            return Ok(Some(Type::SelfType(st)));
                        }
                    }
                },

                KeywordKind::KwFunc => {
                    if let Some(ft) = FunctionType::parse(parser).unwrap_or(None) {
                        return Ok(Some(Type::FunctionType(ft)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(p) = parser.peek_current::<Punctuation>() {
            match &p.punc_kind {
                PuncKind::Ampersand => {
                    if let Some(rt) = ReferenceType::parse(parser).unwrap_or(None) {
                        return Ok(Some(Type::ReferenceType(rt)));
                    }
                }

                PuncKind::Pipe => {
                    if let Some(clo) = ClosureType::parse(parser).unwrap_or(None) {
                        return Ok(Some(Type::ClosureType(clo)));
                    }
                }

                PuncKind::DblPipe => {
                    if let Some(clo) = ClosureType::parse(parser).unwrap_or(None) {
                        return Ok(Some(Type::ClosureType(clo)));
                    }
                }

                _ => return Ok(None),
            }
        }

        Err(parser.errors())
    }
}

///////////////////////////////////////////////////////////////////////////////

impl ParseTerm for Value {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            if &id.name == "_" {
                return Ok(Some(Value::UnderscoreExpr(UnderscoreExpr(id))));
            }

            match parser.peek_next::<Delimiter>() {
                Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(tse) = TupleStructExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Value::TupleStructExpr(tse)));
                    }

                    if let Some(fc) = FunctionCallExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Value::FunctionCallExpr(fc)));
                    }
                }

                Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(se) = StructExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Value::StructExpr(se)));
                    }
                }

                _ => (),
            }

            let path_expr = PathExpr {
                first_segment: PathIdenSegmentKind::Iden(id),
                subsequent_segments: None,
            };

            return Ok(Some(Value::PathExpr(path_expr)));
        }

        if let Some(d) = parser.peek_current::<Delimiter>() {
            match &d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(par) = ParenthesizedExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Value::ParenthesizedExpr(par)));
                    }

                    if let Some(te) = TupleExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Value::TupleExpr(te)));
                    }
                }

                (DelimKind::Bracket, DelimOrientation::Open) => {
                    if let Some(ae) = ArrayExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Value::ArrayExpr(ae)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(l) = parser.peek_current::<LiteralKind>() {
            return Ok(Some(Value::Literal(l)));
        }

        if let Some(p) = parser.peek_current::<Punctuation>() {
            match &p.punc_kind {
                PuncKind::Bang | PuncKind::Minus => {
                    if let Some(ne) = NegationExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Value::NegationExpr(ne)));
                    }
                }

                PuncKind::Asterisk => {
                    if let Some(de) = DereferenceExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Value::DereferenceExpr(de)));
                    }
                }

                PuncKind::Ampersand => {
                    if let Some(re) = ReferenceExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Value::ReferenceExpr(re)));
                    }
                }

                _ => return Ok(None),
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

///////////////////////////////////////////////////////////////////////////////

fn get_item_by_keyword(parser: &mut Parser) -> Result<Option<Item>, Vec<CompilerError>> {
    if let Some(cvd) = ConstantVarDef::parse(parser)? {
        return Ok(Some(Item::ConstantVarDef(cvd)));
    } else if let Some(ed) = EnumDef::parse(parser)? {
        return Ok(Some(Item::EnumDef(ed)));
    } else if let Some(fs) = FunctionSig::parse(parser)? {
        return Ok(Some(Item::FunctionSig(fs)));
        // } else if let Some(fwb) = FunctionWithBlock::parse(parser)? {
        //     return Ok(Some(Item::FunctionWithBlock(fwb)));
    } else if let Some(iib) = InherentImplBlock::parse(parser)? {
        return Ok(Some(Item::InherentImplBlock(iib)));
    } else if let Some(tib) = TraitImplBlock::parse(parser)? {
        return Ok(Some(Item::TraitImplBlock(tib)));
    } else if let Some(imp) = ImportDecl::parse(parser)? {
        return Ok(Some(Item::ImportDecl(imp)));
    // } else if let Some(mwb) = ModWithBody::parse(parser)? {
    //     return Ok(Some(Item::ModWithBody(mwb)));
    } else if let Some(m) = ModWithoutBody::parse(parser)? {
        return Ok(Some(Item::ModWithoutBody(m)));
    } else if let Some(svd) = StaticVarDef::parse(parser)? {
        return Ok(Some(Item::StaticVarDef(svd)));
    } else if let Some(sd) = StructDef::parse(parser)? {
        return Ok(Some(Item::StructDef(sd)));
    } else if let Some(tsd) = TupleStructDef::parse(parser)? {
        return Ok(Some(Item::TupleStructDef(tsd)));
    // } else if let Some(td) = TraitDef::parse(parser)? {
    //     return Ok(Some(Item::TraitDef(td)));
    } else if let Some(tad) = TypeAliasDef::parse(parser)? {
        return Ok(Some(Item::TypeAliasDef(tad)));
    } else {
        return Ok(None);
    }
}
