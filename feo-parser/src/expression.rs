#![allow(dead_code)]
#![allow(unused_variables)]

use feo_ast::{
    expression::{
        ArithmeticOrLogicalExpr, ArrayExpr, ClosureWithBlock, ClosureWithoutBlock, DereferenceExpr,
        FunctionCallExpr, IndexExpr, MethodCallExpr, NegationExpr, ParenthesizedExpr,
        ReferenceExpr, Returnable, StructExpr, StructExprKind, TupleExpr, TupleIndexExpr,
        TupleStructExpr, TypeCastExpr, UnderscoreExpr, UnitStructExpr,
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
mod struct_expr;
mod tuple_expr;
mod underscore_expr;

impl ParseExpr for Returnable {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            match parser.peek_next::<Delimiter>() {
                Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(se) = StructExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::StructExpr(StructExprKind::Struct(se))));
                    }
                }

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

                    if let Some(pat) = PathInExpr::parse(parser).unwrap_or(None) {
                        return Ok(Some(Returnable::PathExpr(pat)));
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

                _ => (),
            }

            let path_expr = PathInExpr {
                first_segment: PathIdenSegmentKind::Iden(id),
                subsequent_segments: None,
            };

            return Ok(Some(Returnable::PathExpr(path_expr)));
        }

        if let Some(_) = parser.peek_current::<Delimiter>() {
            // TODO: these may give us problems later
            if let Some(ae) = ArrayExpr::parse(parser).unwrap_or(None) {
                return Ok(Some(Returnable::ArrayExpr(ae)));
            }

            if let Some(ie) = IndexExpr::parse(parser).unwrap_or(None) {
                return Ok(Some(Returnable::IndexExpr(ie)));
            }

            if let Some(te) = TupleExpr::parse(parser).unwrap_or(None) {
                return Ok(Some(Returnable::TupleExpr(te)));
            }

            if let Some(ti) = TupleIndexExpr::parse(parser).unwrap_or(None) {
                return Ok(Some(Returnable::TupleIndexExpr(ti)));
            }

            if let Some(par) = ParenthesizedExpr::parse(parser).unwrap_or(None) {
                return Ok(Some(Returnable::ParenthesizedExpr(par)));
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

                _ => (),
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

                _ => (),
            }
        } else {
            parser.log_error(ParserErrorKind::InvalidToken {
                token: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        }

        Err(parser.errors())
    }
}
