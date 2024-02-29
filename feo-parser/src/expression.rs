#![allow(dead_code)]
#![allow(unused_variables)]

use feo_ast::{
    expression::{
        ArithmeticOrLogicalExpr, ArrayExpr, Assignable, Castable, ClosureWithBlock,
        ClosureWithoutBlock, DereferenceExpr, FunctionCallExpr, IndexExpr, MethodCallExpr,
        NegationExpr, ParenthesizedExpr, ReferenceExpr, Returnable, StructExpr, StructExprKind,
        TupleExpr, TupleIndexExpr, TupleStructExpr, TypeCastExpr, UnderscoreExpr, UnitStructExpr,
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

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
};

mod array_expr;
mod call_expr;
mod closure_expr;
mod field_access_expr;
mod literal_expr;
mod operator_expr;
mod parenthesized_expr;
mod return_expr;
mod struct_expr;
mod tuple_expr;
mod underscore_expr;

impl ParseExpr for Assignable {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            match parser.peek_next::<Delimiter>() {
                Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(ts) = TupleStructExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Assignable::StructExpr(StructExprKind::TupleStruct(
                            ts,
                        ))));
                    }

                    if let Some(us) = UnitStructExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Assignable::StructExpr(StructExprKind::UnitStruct(us))));
                    }
                }

                Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(se) = StructExpr::parse(parser)? {
                        return Ok(Some(Assignable::StructExpr(StructExprKind::Struct(se))));
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
            match d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(te) = TupleExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Assignable::TupleExpr(te)));
                    }
                }

                (DelimKind::Bracket, DelimOrientation::Open) => {
                    if let Some(ae) = ArrayExpr::parse(parser)? {
                        return Ok(Some(Assignable::ArrayExpr(ae)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(k) = parser.peek_current::<Keyword>() {
            match k.keyword_kind {
                KeywordKind::KwCrate | KeywordKind::KwSelf | KeywordKind::KwSuper => {
                    if let Some(pe) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Assignable::PathExpr(pe)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(p) = parser.peek_current::<Punctuation>() {
            match p.punc_kind {
                PuncKind::Underscore => {
                    if let Some(ue) = UnderscoreExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Assignable::UnderscoreExpr(ue)));
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

impl ParseExpr for Returnable {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            match parser.peek_next::<Delimiter>() {
                Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(ts) = TupleStructExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::StructExpr(StructExprKind::TupleStruct(
                            ts,
                        ))));
                    }

                    if let Some(us) = UnitStructExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::StructExpr(StructExprKind::UnitStruct(us))));
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

            match parser.peek_next::<Punctuation>() {
                Some(Punctuation {
                    punc_kind: PuncKind::FullStop,
                    ..
                }) => {
                    if let Some(mc) = MethodCallExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::MethodCallExpr(mc)));
                    }

                    if let Some(ts) = TupleStructExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::StructExpr(StructExprKind::TupleStruct(
                            ts,
                        ))));
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
                    if let Some(pat) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::PathExpr(pat)));
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
            match d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(te) = TupleExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::TupleExpr(te)));
                    }

                    if let Some(ti) = TupleIndexExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::TupleIndexExpr(ti)));
                    }

                    if let Some(par) = ParenthesizedExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::ParenthesizedExpr(par)));
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
                        return Ok(Some(Returnable::ArithmeticOrLogicalExpr(al)));
                    }
                }

                _ => (),
            }

            if let Some(k) = parser.peek_next::<Keyword>() {
                match k.keyword_kind {
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
            match k.keyword_kind {
                KeywordKind::KwCrate | KeywordKind::KwSelf | KeywordKind::KwSuper => {
                    if let Some(pe) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::PathExpr(pe)));
                    }
                }

                _ => return Ok(None),
            }
        } else if let Some(p) = parser.peek_current::<Punctuation>() {
            match p.punc_kind {
                PuncKind::Underscore => {
                    if let Some(ue) = UnderscoreExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::UnderscoreExpr(ue)));
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
