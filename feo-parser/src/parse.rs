use feo_ast::{
    expression::{
        ArithmeticOrLogicalExpr, ArrayExpr, Assignable, BlockExpr, BooleanOperand, Callable,
        Castable, ClosureWithBlock, ClosureWithoutBlock, DereferenceExpr, FieldAccessExpr,
        FunctionCallExpr, IndexExpr, Iterable, MethodCallExpr, NegationExpr, Operable,
        ParenthesizedExpr, RangeExprKind, RangeFromExpr, RangeFromToExpr, RangeInclusiveExpr,
        RangeToExpr, RangeToInclusiveExpr, ReferenceExpr, Returnable, StructExpr, StructExprKind,
        TupleExpr, TupleIndexExpr, TupleStructExpr, TypeCastExpr, UnderscoreExpr, UnwrapExpr,
    },
    path::{PathIdenSegmentKind, PathInExpr},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    literal::LiteralKind,
    punctuation::PuncKind,
    Delimiter, Identifier, Keyword, Punctuation,
};

use crate::parser::Parser;

pub trait ParseExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized;
}

// literals, attributes, paths, parenthesized expressions, helper types (e.g., `StructExprField`)
pub trait ParseTerm {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized;
}

///////////////////////////////////////////////////////////////////////////////

impl ParseExpr for Assignable {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            match &parser.peek_next::<Delimiter>() {
                Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(ts) = TupleStructExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Assignable::StructExpr(StructExprKind::TupleStruct(
                            ts,
                        ))));
                    }
                }

                Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(se) = StructExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Assignable::StructExpr(StructExprKind::Struct(se))));
                    }
                }

                _ => (),
            }

            match &parser.peek_next::<Punctuation>() {
                Some(Punctuation {
                    punc_kind: PuncKind::DblColon,
                    ..
                }) => {
                    if let Some(pth) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Assignable::PathExpr(pth)));
                    }
                }

                _ => (),
            }

            let path_expr = PathInExpr {
                first_segment: PathIdenSegmentKind::Iden(id),
                subsequent_segments: None,
            };

            return Ok(Some(Assignable::PathExpr(path_expr)));
        }

        if let Some(d) = parser.peek_current::<Delimiter>() {
            match &d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(te) = TupleExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Assignable::TupleExpr(te)));
                    }
                }

                (DelimKind::Bracket, DelimOrientation::Open) => {
                    if let Some(ae) = ArrayExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Assignable::ArrayExpr(ae)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(k) = parser.peek_current::<Keyword>() {
            match &k.keyword_kind {
                KeywordKind::KwCrate | KeywordKind::KwSelf | KeywordKind::KwSuper => {
                    if let Some(pe) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Assignable::PathExpr(pe)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(p) = parser.peek_current::<Punctuation>() {
            match &p.punc_kind {
                PuncKind::Underscore => {
                    if let Some(und) = UnderscoreExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Assignable::UnderscoreExpr(und)));
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

impl ParseExpr for BooleanOperand {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            match &parser.peek_next::<Delimiter>() {
                Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(fc) = FunctionCallExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(BooleanOperand::FunctionCallExpr(fc)));
                    }
                }

                _ => (),
            }

            match &parser.peek_next::<Punctuation>() {
                Some(Punctuation {
                    punc_kind: PuncKind::FullStop,
                    ..
                }) => {
                    if let Some(mc) = MethodCallExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(BooleanOperand::MethodCallExpr(mc)));
                    }

                    if let Some(fa) = FieldAccessExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(BooleanOperand::FieldAccessExpr(fa)));
                    }
                }

                Some(Punctuation {
                    punc_kind: PuncKind::DblColon,
                    ..
                }) => {
                    if let Some(pth) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(BooleanOperand::PathExpr(pth)));
                    }
                }

                Some(Punctuation {
                    punc_kind: PuncKind::QuestionMark,
                    ..
                }) => {
                    if let Some(ue) = UnwrapExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(BooleanOperand::UnwrapExpr(ue)));
                    }
                }

                _ => (),
            }

            let path_expr = PathInExpr {
                first_segment: PathIdenSegmentKind::Iden(id),
                subsequent_segments: None,
            };

            return Ok(Some(BooleanOperand::PathExpr(path_expr)));
        }

        if let Some(d) = parser.peek_current::<Delimiter>() {
            match &d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(par) = ParenthesizedExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(BooleanOperand::ParenthesizedExpr(par)));
                    }

                    if let Some(tie) = TupleIndexExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(BooleanOperand::TupleIndexExpr(tie)));
                    }
                }

                (DelimKind::Bracket, DelimOrientation::Open) => {
                    if let Some(ie) = IndexExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(BooleanOperand::IndexExpr(ie)));
                    }
                }

                (DelimKind::Brace, DelimOrientation::Open) => {
                    if let Some(be) = BlockExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(BooleanOperand::BlockExpr(be)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(l) = parser.peek_current::<LiteralKind>() {
            return Ok(Some(BooleanOperand::Literal(l)));
        }

        if let Some(k) = parser.peek_current::<Keyword>() {
            match &k.keyword_kind {
                KeywordKind::KwCrate | KeywordKind::KwSelf | KeywordKind::KwSuper => {
                    if let Some(pe) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(BooleanOperand::PathExpr(pe)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(p) = parser.peek_current::<Punctuation>() {
            match &p.punc_kind {
                PuncKind::Underscore => {
                    if let Some(und) = UnderscoreExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(BooleanOperand::UnderscoreExpr(und)));
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

impl ParseExpr for Callable {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            match &parser.peek_next::<Punctuation>() {
                Some(Punctuation {
                    punc_kind: PuncKind::DblColon,
                    ..
                }) => {
                    if let Some(pth) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Callable::PathExpr(pth)));
                    }
                }

                _ => (),
            }

            let path_expr = PathInExpr {
                first_segment: PathIdenSegmentKind::Iden(id),
                subsequent_segments: None,
            };

            return Ok(Some(Callable::PathExpr(path_expr)));
        }

        if let Some(d) = parser.peek_current::<Delimiter>() {
            match &d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(pe) = ParenthesizedExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Callable::ParenthesizedExpr(pe)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(k) = parser.peek_current::<Keyword>() {
            match &k.keyword_kind {
                KeywordKind::KwCrate | KeywordKind::KwSelf | KeywordKind::KwSuper => {
                    if let Some(pe) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Callable::PathExpr(pe)));
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

impl ParseExpr for Castable {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            let path_expr = PathInExpr {
                first_segment: PathIdenSegmentKind::Iden(id),
                subsequent_segments: None,
            };

            return Ok(Some(Castable::PathExpr(path_expr)));
        }

        if let Some(l) = parser.peek_current::<LiteralKind>() {
            match l {
                LiteralKind::Char(c) => return Ok(Some(Castable::Char(c))),
                LiteralKind::Bool(b) => return Ok(Some(Castable::Bool(b))),
                LiteralKind::I32(i) => return Ok(Some(Castable::I32(i))),
                LiteralKind::I64(i) => return Ok(Some(Castable::I64(i))),
                LiteralKind::U8(ui) => return Ok(Some(Castable::U8(ui))),
                LiteralKind::U16(ui) => return Ok(Some(Castable::U16(ui))),
                LiteralKind::U32(ui) => return Ok(Some(Castable::U32(ui))),
                LiteralKind::U64(ui) => return Ok(Some(Castable::U64(ui))),
                LiteralKind::U256(u) => return Ok(Some(Castable::U256(u))),
                LiteralKind::F32(f) => return Ok(Some(Castable::F32(f))),
                LiteralKind::F64(f) => return Ok(Some(Castable::F64(f))),

                _ => {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "numeric type".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            }
        }

        Err(parser.errors())
    }
}

impl ParseExpr for Iterable {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            match &parser.peek_next::<Delimiter>() {
                Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(fc) = FunctionCallExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::FunctionCallExpr(fc)));
                    }
                }

                _ => (),
            }

            match &parser.peek_next::<Punctuation>() {
                Some(Punctuation {
                    punc_kind: PuncKind::FullStop,
                    ..
                }) => {
                    if let Some(mc) = MethodCallExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::MethodCallExpr(mc)));
                    }

                    if let Some(fa) = FieldAccessExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::FieldAccessExpr(fa)));
                    }
                }

                Some(Punctuation {
                    punc_kind: PuncKind::QuestionMark,
                    ..
                }) => {
                    if let Some(ue) = UnwrapExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::UnwrapExpr(ue)));
                    }
                }

                Some(Punctuation {
                    punc_kind: PuncKind::DblDot,
                    ..
                }) => {
                    if let Some(rte) = RangeToExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::RangeExpr(RangeExprKind::RangeToExpr(rte))));
                    }
                }

                Some(Punctuation {
                    punc_kind: PuncKind::DotDotEquals,
                    ..
                }) => {
                    if let Some(rti) = RangeToInclusiveExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::RangeExpr(
                            RangeExprKind::RangeToInclusiveExpr(rti),
                        )));
                    }
                }

                _ => (),
            }

            let path_expr = PathInExpr {
                first_segment: PathIdenSegmentKind::Iden(id),
                subsequent_segments: None,
            };

            return Ok(Some(Iterable::PathExpr(path_expr)));
        }

        if let Some(d) = parser.peek_current::<Delimiter>() {
            match &d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(par) = ParenthesizedExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::ParenthesizedExpr(par)));
                    }

                    if let Some(ti) = TupleIndexExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::TupleIndexExpr(ti)));
                    }

                    if let Some(te) = TupleExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::TupleExpr(te)));
                    }
                }

                (DelimKind::Bracket, DelimOrientation::Open) => {
                    if let Some(ie) = IndexExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::IndexExpr(ie)));
                    }

                    if let Some(ae) = ArrayExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::ArrayExpr(ae)));
                    }
                }

                (DelimKind::Brace, DelimOrientation::Open) => {
                    if let Some(be) = BlockExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::BlockExpr(be)));
                    }
                }
                _ => return Ok(None),
            }
        } else if let Some(l) = parser.peek_current::<LiteralKind>() {
            if let Some(p) = parser.peek_next::<Punctuation>() {
                match &p.punc_kind {
                    PuncKind::DblDot => {
                        if let Some(rft) = RangeFromToExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Iterable::RangeExpr(RangeExprKind::RangeFromToExpr(
                                rft,
                            ))));
                        }

                        if let Some(rfe) = RangeFromExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Iterable::RangeExpr(RangeExprKind::RangeFromExpr(
                                rfe,
                            ))));
                        }
                    }

                    PuncKind::DotDotEquals => {
                        if let Some(rie) = RangeInclusiveExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Iterable::RangeExpr(
                                RangeExprKind::RangeInclusiveExpr(rie),
                            )));
                        }
                    }

                    _ => (),
                }
            }

            if let Some(k) = parser.peek_next::<Keyword>() {
                match &k.keyword_kind {
                    KeywordKind::KwAs => {
                        if let Some(tc) = TypeCastExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Iterable::TypeCastExpr(tc)));
                        }
                    }

                    _ => (),
                }
            }

            return Ok(Some(Iterable::Literal(l)));
        }

        if let Some(k) = parser.peek_current::<Keyword>() {
            match &k.keyword_kind {
                KeywordKind::KwCrate | KeywordKind::KwSelf | KeywordKind::KwSuper => {
                    if let Some(pe) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::PathExpr(pe)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(p) = parser.peek_current::<Punctuation>() {
            match &p.punc_kind {
                PuncKind::Asterisk => {
                    if let Some(de) = DereferenceExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::DereferenceExpr(de)));
                    }
                }

                PuncKind::Ampersand => {
                    if let Some(re) = ReferenceExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::ReferenceExpr(re)));
                    }
                }

                PuncKind::DblDot => {
                    if let Some(rte) = RangeToExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::RangeExpr(RangeExprKind::RangeToExpr(rte))));
                    }
                }

                PuncKind::DotDotEquals => {
                    if let Some(rti) = RangeToInclusiveExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Iterable::RangeExpr(
                            RangeExprKind::RangeToInclusiveExpr(rti),
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

impl ParseExpr for Operable {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            match &parser.peek_next::<Delimiter>() {
                Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(fc) = FunctionCallExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Operable::FunctionCallExpr(fc)));
                    }
                }

                _ => (),
            }

            match &parser.peek_next::<Punctuation>() {
                Some(Punctuation {
                    punc_kind: PuncKind::FullStop,
                    ..
                }) => {
                    if let Some(mc) = MethodCallExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Operable::MethodCallExpr(mc)));
                    }

                    if let Some(fa) = FieldAccessExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Operable::FieldAccessExpr(fa)));
                    }
                }

                Some(Punctuation {
                    punc_kind: PuncKind::QuestionMark,
                    ..
                }) => {
                    if let Some(ue) = UnwrapExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Operable::UnwrapExpr(ue)));
                    }
                }

                _ => (),
            }

            let path_expr = PathInExpr {
                first_segment: PathIdenSegmentKind::Iden(id),
                subsequent_segments: None,
            };

            return Ok(Some(Operable::PathExpr(path_expr)));
        }

        if let Some(d) = parser.peek_current::<Delimiter>() {
            match &d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(par) = ParenthesizedExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Operable::ParenthesizedExpr(par)));
                    }

                    if let Some(ti) = TupleIndexExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Operable::TupleIndexExpr(ti)));
                    }
                }

                (DelimKind::Bracket, DelimOrientation::Open) => {
                    if let Some(ie) = IndexExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Operable::IndexExpr(ie)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(l) = parser.peek_current::<LiteralKind>() {
            if let Some(k) = parser.peek_next::<Keyword>() {
                match &k.keyword_kind {
                    KeywordKind::KwAs => {
                        if let Some(tc) = TypeCastExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Operable::TypeCastExpr(tc)));
                        }
                    }

                    _ => (),
                }
            }

            return Ok(Some(Operable::Literal(l)));
        }

        if let Some(k) = parser.peek_current::<Keyword>() {
            match &k.keyword_kind {
                KeywordKind::KwCrate | KeywordKind::KwSelf | KeywordKind::KwSuper => {
                    if let Some(pe) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Operable::PathExpr(pe)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(p) = parser.peek_current::<Punctuation>() {
            match &p.punc_kind {
                PuncKind::Ampersand => {
                    if let Some(re) = ReferenceExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Operable::ReferenceExpr(re)));
                    }
                }

                PuncKind::Asterisk => {
                    if let Some(de) = DereferenceExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Operable::DereferenceExpr(de)));
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

impl ParseExpr for Returnable {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            match &parser.peek_next::<Delimiter>() {
                Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(ts) = TupleStructExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::StructExpr(StructExprKind::TupleStruct(
                            ts,
                        ))));
                    }

                    if let Some(fc) = FunctionCallExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::FunctionCallExpr(fc)));
                    }
                }

                Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(se) = StructExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::StructExpr(StructExprKind::Struct(se))));
                    }
                }

                _ => (),
            }

            match &parser.peek_next::<Punctuation>() {
                Some(Punctuation {
                    punc_kind: PuncKind::FullStop,
                    ..
                }) => {
                    if let Some(mc) = MethodCallExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::MethodCallExpr(mc)));
                    }

                    if let Some(fa) = FieldAccessExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::FieldAccessExpr(fa)));
                    }
                }

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
                        return Ok(Some(Returnable::ArithmeticOrLogicalExpr(al)));
                    }
                }

                Some(Punctuation {
                    punc_kind: PuncKind::DblColon,
                    ..
                }) => {
                    if let Some(path_expr) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::PathExpr(path_expr)));
                    }
                }

                Some(Punctuation {
                    punc_kind: PuncKind::QuestionMark,
                    ..
                }) => {
                    if let Some(ue) = UnwrapExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::UnwrapExpr(ue)));
                    }
                }

                _ => (),
            }

            let path_expr = PathInExpr {
                first_segment: PathIdenSegmentKind::Iden(id),
                subsequent_segments: None,
            };

            return Ok(Some(Returnable::PathExpr(path_expr)));
        }

        if let Some(d) = parser.peek_current::<Delimiter>() {
            match &d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(par) = ParenthesizedExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::ParenthesizedExpr(par)));
                    }

                    if let Some(ti) = TupleIndexExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::TupleIndexExpr(ti)));
                    }

                    if let Some(te) = TupleExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::TupleExpr(te)));
                    }
                }

                (DelimKind::Bracket, DelimOrientation::Open) => {
                    if let Some(ie) = IndexExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::IndexExpr(ie)));
                    }

                    if let Some(ae) = ArrayExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::ArrayExpr(ae)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(l) = parser.peek_current::<LiteralKind>() {
            match &parser.peek_next::<Punctuation>() {
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
                        return Ok(Some(Returnable::ArithmeticOrLogicalExpr(al)));
                    }
                }

                _ => (),
            }

            if let Some(k) = parser.peek_next::<Keyword>() {
                match &k.keyword_kind {
                    KeywordKind::KwAs => {
                        if let Some(tc) = TypeCastExpr::parse(parser).unwrap_or(None) {
                            return Ok(Some(Returnable::TypeCastExpr(tc)));
                        }
                    }

                    _ => (),
                }
            }

            return Ok(Some(Returnable::Literal(l)));
        }

        if let Some(k) = parser.peek_current::<Keyword>() {
            match &k.keyword_kind {
                KeywordKind::KwCrate | KeywordKind::KwSelf | KeywordKind::KwSuper => {
                    if let Some(pe) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::PathExpr(pe)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(p) = parser.peek_current::<Punctuation>() {
            match &p.punc_kind {
                PuncKind::Underscore => {
                    if let Some(und) = UnderscoreExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::UnderscoreExpr(und)));
                    }
                }

                PuncKind::Bang | PuncKind::Minus => {
                    if let Some(ne) = NegationExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::NegationExpr(ne)));
                    }
                }

                PuncKind::Ampersand => {
                    if let Some(re) = ReferenceExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::ReferenceExpr(re)));
                    }
                }

                PuncKind::Asterisk => {
                    if let Some(de) = DereferenceExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::DereferenceExpr(de)));
                    }
                }

                PuncKind::Pipe => {
                    if let Some(cwb) = ClosureWithBlock::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::ClosureWithBlock(cwb)));
                    }
                }

                PuncKind::DblPipe => {
                    if let Some(c) = ClosureWithoutBlock::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::ClosureWithoutBlock(c)));
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
