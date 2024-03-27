use feo_ast::token::Token;
use feo_error::parser_error::ParserErrorKind;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    Delimiter, Keyword, Punctuation,
};

use crate::parser::Parser;

/// Helps to control the order in which operations are parsed.
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Precedence {
    Lowest,
    Assignment,         // =, +=
    Unwrap,             // ?
    Or,                 // ||
    And,                // &&
    BitwiseOr,          // |
    BitwiseXor,         // ^
    BitwiseAnd,         // &
    Equality,           // ==, !=
    Comparison,         // <, >, <=, >=
    Shift,              // <<, >>
    Sum,                // +, -
    Product,            // *, /, %
    Prefix,             // -X, !X, * (dereference), & and &mut (reference)
    TypeCast,           // as (Type Casting)
    Call,               // func(args), object.method(args)
    Index,              // array[index], tuple.0
    FieldAccess,        // expr.field
    Path,               // foo::bar, foo.bar
    Closure,            // |args| expr
    Literal,            // 123, "string", true/false
    Parentheses,        // (expr)
    Array,              // [expr, expr]
    Tuple,              // (expr, expr)
    Struct,             // StructName { field: expr, .. }
    Block,              // { ... }
    If,                 // if condition { ... } else { ... }
    Loop,               // loop { ... }, while (condition) { ... }, for item in collection { ... }
    Range,              // .., ..=
    CompoundAssignment, // +=, -=, *=, /=
}

impl Precedence {
    pub fn token_precedence(parser: &mut Parser) -> Result<Option<Precedence>, ParserErrorKind> {
        match parser.current_token() {
            Some(Token::Keyword(k)) => match k.keyword_kind {
                KeywordKind::KwLoop | KeywordKind::KwWhile | KeywordKind::KwFor => {
                    Ok(Some(Precedence::Loop))
                }

                KeywordKind::KwIf | KeywordKind::KwMatch => Ok(Some(Precedence::If)),

                KeywordKind::KwSelfType => match parser.peek_num_tokens_ahead(1) {
                    Some(t) => match t {
                        Token::Delim(d) => match d.delim {
                            (DelimKind::Parenthesis, DelimOrientation::Open)
                            | (DelimKind::Brace, DelimOrientation::Open) => {
                                Ok(Some(Precedence::Struct))
                            }

                            _ => Ok(None),
                        },

                        Token::Punc(
                            Punctuation {
                                punc_kind: PuncKind::DblColon,
                                ..
                            }
                            | Punctuation {
                                punc_kind: PuncKind::ColonColonAsterisk,
                                ..
                            },
                        ) => match parser.peek_num_tokens_ahead(2) {
                            Some(Token::Identifier(_)) => Ok(Some(Precedence::Path)),
                            _ => Ok(None),
                        },

                        _ => Ok(None),
                    },

                    None => Ok(Some(Precedence::Path)),
                },

                KeywordKind::KwSelf => match parser.peek_num_tokens_ahead(1) {
                    Some(Token::Punc(Punctuation {
                        punc_kind: PuncKind::FullStop,
                        ..
                    })) => match parser.peek_num_tokens_ahead(2) {
                        Some(Token::Delim(Delimiter {
                            delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                            ..
                        })) => Ok(Some(Precedence::Call)),

                        Some(Token::Identifier(_)) => Ok(Some(Precedence::FieldAccess)),

                        _ => Ok(None),
                    },

                    Some(_) => Ok(None),

                    None => Ok(Some(Precedence::Path)),
                },

                KeywordKind::KwPackage | KeywordKind::KwSuper => {
                    match parser.peek_num_tokens_ahead(1) {
                        Some(Token::Punc(Punctuation {
                            punc_kind: PuncKind::DblColon,
                            ..
                        })) => match parser.peek_num_tokens_ahead(2) {
                            Some(Token::Identifier(_)) => Ok(Some(Precedence::Path)),
                            _ => Ok(None),
                        },

                        _ => Ok(None),
                    }
                }

                KeywordKind::KwMut => Ok(Some(Precedence::Prefix)),

                KeywordKind::KwBreak | KeywordKind::KwContinue | KeywordKind::KwReturn => {
                    Ok(Some(Precedence::Lowest))
                }

                _ => Ok(None),
            },

            Some(Token::Delim(d)) => match d.delim {
                (DelimKind::Brace, DelimOrientation::Open) => Ok(Some(Precedence::Block)),

                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    match parser.peek_num_tokens_ahead(2) {
                        Some(Token::Punc(Punctuation {
                            punc_kind: PuncKind::Comma,
                            ..
                        })) => Ok(Some(Precedence::Tuple)),

                        Some(_) => Ok(Some(Precedence::Parentheses)),

                        _ => Ok(None),
                    }
                }

                (DelimKind::Bracket, DelimOrientation::Open) => Ok(Some(Precedence::Array)),

                _ => Ok(None),
            },

            Some(Token::Identifier(id)) => {
                if &id.name == "_" {
                    return Ok(Some(Precedence::Lowest));
                }

                match parser.peek_num_tokens_ahead(1) {
                    Some(t) => match t {
                        Token::Delim(d) => match d.delim {
                            (DelimKind::Parenthesis, DelimOrientation::Open) => {
                                Ok(Some(Precedence::Call))
                                // what about tuple structs?
                            }

                            (DelimKind::Bracket, DelimOrientation::Open) => {
                                Ok(Some(Precedence::Index))
                            }

                            (DelimKind::Brace, DelimOrientation::Open) => {
                                Ok(Some(Precedence::Struct))
                            }

                            _ => Ok(None),
                        },

                        Token::Keyword(Keyword {
                            keyword_kind: KeywordKind::KwAs,
                            ..
                        }) => Ok(Some(Precedence::TypeCast)),

                        Token::Punc(p) => match p.punc_kind {
                            PuncKind::FullStop => match parser.peek_num_tokens_ahead(2) {
                                Some(t) => match t {
                                    Token::Delim(Delimiter {
                                        delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                                        ..
                                    }) => Ok(Some(Precedence::Call)),

                                    Token::Identifier(_) => Ok(Some(Precedence::FieldAccess)),

                                    Token::UIntLit(_) => Ok(Some(Precedence::Index)),

                                    _ => Ok(None),
                                },

                                None => Ok(None),
                            },

                            PuncKind::PlusEquals
                            | PuncKind::MinusEquals
                            | PuncKind::AsteriskEquals
                            | PuncKind::ForwardSlashEquals
                            | PuncKind::PercentEquals => Ok(Some(Precedence::CompoundAssignment)),

                            PuncKind::DblDot | PuncKind::DotDotEquals => {
                                Ok(Some(Precedence::Range))
                            }

                            PuncKind::DblColon | PuncKind::ColonColonAsterisk => {
                                Ok(Some(Precedence::Path))
                            }

                            PuncKind::Asterisk | PuncKind::ForwardSlash | PuncKind::Percent => {
                                Ok(Some(Precedence::Product))
                            }

                            PuncKind::Plus | PuncKind::Minus => Ok(Some(Precedence::Sum)),

                            PuncKind::DblLessThan | PuncKind::DblGreaterThan => {
                                Ok(Some(Precedence::Shift))
                            }

                            PuncKind::LessThan
                            | PuncKind::GreaterThan
                            | PuncKind::LessThanEquals
                            | PuncKind::GreaterThanEquals => Ok(Some(Precedence::Comparison)),

                            PuncKind::DblEquals | PuncKind::BangEquals => {
                                Ok(Some(Precedence::Equality))
                            }

                            PuncKind::Ampersand => Ok(Some(Precedence::BitwiseAnd)),

                            PuncKind::Caret => Ok(Some(Precedence::BitwiseXor)),

                            PuncKind::Pipe => Ok(Some(Precedence::BitwiseOr)),

                            PuncKind::DblAmpersand => Ok(Some(Precedence::And)),

                            PuncKind::DblPipe => Ok(Some(Precedence::Or)),

                            PuncKind::QuestionMark => Ok(Some(Precedence::Unwrap)),

                            PuncKind::Equals => Ok(Some(Precedence::Assignment)),

                            _ => Ok(None),
                        },

                        _ => Ok(None),
                    },

                    None => Ok(Some(Precedence::Path)),
                }
            }

            Some(Token::Punc(p)) => match p.punc_kind {
                PuncKind::DblDot | PuncKind::DotDotEquals => Ok(Some(Precedence::Range)),

                PuncKind::Pipe | PuncKind::DblPipe => Ok(Some(Precedence::Closure)),

                PuncKind::Minus
                | PuncKind::Bang
                | PuncKind::Asterisk
                | PuncKind::Ampersand
                | PuncKind::HashSign
                | PuncKind::HashBang => Ok(Some(Precedence::Prefix)),

                _ => Ok(None),
            },

            Some(Token::IntLit(_))
            | Some(Token::UIntLit(_))
            | Some(Token::U256Lit(_))
            | Some(Token::FloatLit(_)) => match parser.peek_num_tokens_ahead(1) {
                Some(t) => match t {
                    Token::Keyword(Keyword {
                        keyword_kind: KeywordKind::KwAs,
                        ..
                    }) => Ok(Some(Precedence::TypeCast)),

                    Token::Punc(p) => match p.punc_kind {
                        PuncKind::DblDot | PuncKind::DotDotEquals => Ok(Some(Precedence::Range)),

                        PuncKind::Asterisk | PuncKind::ForwardSlash | PuncKind::Percent => {
                            Ok(Some(Precedence::Product))
                        }

                        PuncKind::Plus | PuncKind::Minus => Ok(Some(Precedence::Sum)),

                        PuncKind::DblLessThan | PuncKind::DblGreaterThan => {
                            Ok(Some(Precedence::Shift))
                        }

                        PuncKind::LessThan
                        | PuncKind::GreaterThan
                        | PuncKind::LessThanEquals
                        | PuncKind::GreaterThanEquals => Ok(Some(Precedence::Comparison)),

                        PuncKind::BangEquals | PuncKind::DblEquals => {
                            Ok(Some(Precedence::Equality))
                        }

                        PuncKind::Ampersand => Ok(Some(Precedence::BitwiseAnd)),

                        PuncKind::Caret => Ok(Some(Precedence::BitwiseXor)),

                        PuncKind::Pipe => Ok(Some(Precedence::BitwiseOr)),

                        PuncKind::DblAmpersand => Ok(Some(Precedence::And)),

                        PuncKind::DblPipe => Ok(Some(Precedence::Or)),

                        _ => Ok(None),
                    },

                    _ => Ok(None),
                },

                None => Ok(Some(Precedence::Literal)),
            },

            Some(Token::CharLit(_)) | Some(Token::StringLit(_)) | Some(Token::BoolLit(_)) => {
                Ok(Some(Precedence::Literal))
            }

            _ => Ok(Some(Precedence::Lowest)),
        }
    }
}
