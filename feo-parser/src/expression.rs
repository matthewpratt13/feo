#![allow(dead_code)]
#![allow(unused_variables)]

use feo_ast::{
    expression::{
        ArithmeticOrLogicalExpr, ArrayExpr, Assignable, ClosureWithBlock, ClosureWithoutBlock,
        DereferenceExpr, FunctionCallExpr, MethodCallExpr, NegationExpr, ReferenceExpr, Returnable,
        StructExpr, StructExprKind, TupleStructExpr, TypeCastExpr, UnderscoreExpr, UnitStructExpr,
    },
    path::PathInExpr,
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

impl ParseExpr for Assignable {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            if let Some(se) = StructExpr::parse(parser).unwrap_or(None) {
                return Ok(Some(Assignable::StructExpr(StructExprKind::Struct(se))));
            } else if let Some(ts) = TupleStructExpr::parse(parser).unwrap_or(None) {
                return Ok(Some(Assignable::StructExpr(StructExprKind::TupleStruct(
                    ts,
                ))));
            } else if let Some(us) = UnitStructExpr::parse(parser).unwrap_or(None) {
                return Ok(Some(Assignable::StructExpr(StructExprKind::UnitStruct(us))));
            } else if let Some(pat) = PathInExpr::parse(parser).unwrap_or(None) {
                return Ok(Some(Assignable::PathExpr(pat)));
            } else {
                return Ok(Some(Assignable::Identifier(id)));
            }
        } else if let Some(_) = parser.peek_current::<Delimiter>() {
            if let Some(ae) = ArrayExpr::parse(parser).unwrap_or(None) {
                return Ok(Some(Assignable::ArrayExpr(ae)));
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`Assignable`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else if let Some(_) = parser.peek_current::<Keyword>() {
            if let Some(pe) = PathInExpr::parse(parser).unwrap_or(None) {
                return Ok(Some(Assignable::PathExpr(pe)));
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`Assignable`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else if let Some(_) = parser.peek_current::<Punctuation>() {
            if let Some(ue) = UnderscoreExpr::parse(parser).unwrap_or(None) {
                return Ok(Some(Assignable::UnderscoreExpr(ue)));
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`Assignable`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            parser.log_error(ParserErrorKind::InvalidToken {
                token: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        }

        parser.next_token();

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
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) => {
                    // `ok()` to discard the `Err` and keep on checking
                    if let Some(se) = StructExpr::parse(parser).ok().unwrap_or(None) {
                        return Ok(Some(Returnable::StructExpr(StructExprKind::Struct(se))));
                    }
                }

                Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(ts) = TupleStructExpr::parse(parser).ok().unwrap_or(None) {
                        return Ok(Some(Returnable::StructExpr(StructExprKind::TupleStruct(
                            ts,
                        ))));
                    }

                    if let Some(us) = UnitStructExpr::parse(parser).ok().unwrap_or(None) {
                        return Ok(Some(Returnable::StructExpr(StructExprKind::UnitStruct(us))));
                    }

                    if let Some(fc) = FunctionCallExpr::parse(parser).ok().unwrap_or(None) {
                        return Ok(Some(Returnable::FunctionCallExpr(fc)));
                    }

                    if let Some(pat) = PathInExpr::parse(parser).ok().unwrap_or(None) {
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
                    if let Some(mc) = MethodCallExpr::parse(parser).ok().unwrap_or(None) {
                        return Ok(Some(Returnable::MethodCallExpr(mc)));
                    }

                    if let Some(ts) = TupleStructExpr::parse(parser).ok().unwrap_or(None) {
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
                    if let Some(al) = ArithmeticOrLogicalExpr::parse(parser).ok().unwrap_or(None) {
                        return Ok(Some(Returnable::ArithmeticOrLogicalExpr(al)));
                    }
                }

                _ => (),
            }

            return Ok(Some(Returnable::Identifier(id)));
        }

        if let Some(_) = parser.peek_current::<Delimiter>() {
            // if let Some(ae) = ArrayExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::ArrayExpr(ae)))
            // } else if let Some(ie) = IndexExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::IndexExpr(ie)))
            // } else if let Some(te) = TupleExpr::parse(parser).unwrap_or(None) {
            //    return Ok( Some(Returnable::TupleExpr(te)))
            // } else if let Some(ti) = TupleIndexExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::TupleIndexExpr(ti)))
            // } else if let Some(par) = ParenthesizedExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::ParenthesizedExpr(par)))
            // } else {
            // parser.log_error(ParserErrorKind::UnexpectedToken {
            //     expected: "`Returnable`".to_string(),
            //     found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            // });
            // }
        }

        if let Some(l) = parser.peek_current::<LiteralKind>() {
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
                    if let Some(al) = ArithmeticOrLogicalExpr::parse(parser).ok().unwrap_or(None) {
                        return Ok(Some(Returnable::ArithmeticOrLogicalExpr(al)));
                    }
                }

                _ => (),
            }

            if let Some(k) = parser.peek_next::<Keyword>() {
                match k.keyword_kind {
                    KeywordKind::KwAs => {
                        if let Some(tc) = TypeCastExpr::parse(parser).ok().unwrap_or(None) {
                            return Ok(Some(Returnable::TypeCastExpr(tc)));
                        }
                    }

                    _ => (),
                }
            }

            return Ok(Some(Returnable::Literal(l)));
        }

        if let Some(_) = parser.peek_current::<Keyword>() {
            if let Some(pe) = PathInExpr::parse(parser).ok().unwrap_or(None) {
                return Ok(Some(Returnable::PathExpr(pe)));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`Returnable`".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        }

        if let Some(p) = parser.peek_current::<Punctuation>() {
            match p.punc_kind {
                PuncKind::Underscore => {
                    if let Some(ue) = UnderscoreExpr::parse(parser).ok().unwrap_or(None) {
                        return Ok(Some(Returnable::UnderscoreExpr(ue)));
                    }
                }

                PuncKind::Bang | PuncKind::Minus => {
                    if let Some(ne) = NegationExpr::parse(parser).ok().unwrap_or(None) {
                        return Ok(Some(Returnable::NegationExpr(ne)));
                    }
                }

                PuncKind::Ampersand => {
                    if let Some(re) = ReferenceExpr::parse(parser).ok().unwrap_or(None) {
                        return Ok(Some(Returnable::ReferenceExpr(re)));
                    }
                }

                PuncKind::Asterisk => {
                    if let Some(de) = DereferenceExpr::parse(parser).ok().unwrap_or(None) {
                        return Ok(Some(Returnable::DereferenceExpr(de)));
                    }
                }

                PuncKind::Pipe => {
                    if let Some(cwb) = ClosureWithBlock::parse(parser).ok().unwrap_or(None) {
                        return Ok(Some(Returnable::ClosureWithBlock(cwb)));
                    }
                }

                PuncKind::DblPipe => {
                    if let Some(c) = ClosureWithoutBlock::parse(parser).ok().unwrap_or(None) {
                        return Ok(Some(Returnable::ClosureWithoutBlock(c)));
                    }
                }

                _ => (),
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`Returnable`".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        } else {
            parser.log_error(ParserErrorKind::InvalidToken {
                token: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        }

        parser.next_token();

        Err(parser.errors())
    }
}
