use feo_ast::token::Token;
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
    pub fn token_precedence(parser: &mut Parser) -> Option<Precedence> {
        match parser.current_token() {
            Some(Token::Keyword(k)) => match k.keyword_kind {
                KeywordKind::KwLoop | KeywordKind::KwWhile | KeywordKind::KwFor => {
                    Some(Precedence::Loop)
                }

                KeywordKind::KwIf | KeywordKind::KwMatch => Some(Precedence::If),

                KeywordKind::KwSelfType => match parser.peek_num_tokens_ahead(1) {
                    Some(t) => match t {
                        Token::Delim(d) => match d.delim {
                            (DelimKind::Parenthesis, DelimOrientation::Open)
                            | (DelimKind::Brace, DelimOrientation::Open) => {
                                Some(Precedence::Struct)
                            }
                            _ => None,
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
                            Some(Token::Identifier(_)) => Some(Precedence::Path),
                            _ => None,
                        },

                        _ => None,
                    },

                    None => Some(Precedence::Path),
                },

                KeywordKind::KwSelf => match parser.peek_num_tokens_ahead(1) {
                    Some(Token::Punc(Punctuation {
                        punc_kind: PuncKind::FullStop,
                        ..
                    })) => match parser.peek_num_tokens_ahead(2) {
                        Some(Token::Identifier(_)) => Some(Precedence::FieldAccess),

                        Some(Token::Delim(Delimiter {
                            delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                            ..
                        })) => Some(Precedence::Call),

                        _ => None,
                    },

                    Some(_) => None,

                    None => Some(Precedence::Path),
                },

                KeywordKind::KwPackage | KeywordKind::KwSuper => {
                    match parser.peek_num_tokens_ahead(1) {
                        Some(Token::Punc(Punctuation {
                            punc_kind: PuncKind::DblColon,
                            ..
                        })) => match parser.peek_num_tokens_ahead(2) {
                            Some(Token::Identifier(_)) => Some(Precedence::Path),
                            _ => None,
                        },

                        _ => None,
                    }
                }

                KeywordKind::KwMut => Some(Precedence::Prefix),

                KeywordKind::KwBreak | KeywordKind::KwContinue | KeywordKind::KwReturn => {
                    Some(Precedence::Lowest)
                }

                _ => None,
            },

            Some(Token::Delim(d)) => match d.delim {
                (DelimKind::Brace, DelimOrientation::Open) => Some(Precedence::Block),

                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    match parser.peek_num_tokens_ahead(2) {
                        Some(Token::Punc(Punctuation {
                            punc_kind: PuncKind::Comma,
                            ..
                        })) => Some(Precedence::Tuple),

                        Some(_) => Some(Precedence::Parentheses),

                        _ => None,
                    }
                }

                (DelimKind::Bracket, DelimOrientation::Open) => Some(Precedence::Array),

                _ => None,
            },

            Some(Token::Identifier(id)) => {
                if &id.name == "_" {
                    return Some(Precedence::Lowest);
                }

                match parser.peek_num_tokens_ahead(1) {
                    Some(t) => match t {
                        Token::Delim(d) => match d.delim {
                            (DelimKind::Parenthesis, DelimOrientation::Open) => {
                                Some(Precedence::Call)
                                // what about tuple structs?
                            }

                            (DelimKind::Bracket, DelimOrientation::Open) => Some(Precedence::Index),

                            (DelimKind::Brace, DelimOrientation::Open) => Some(Precedence::Struct),
                            _ => None,
                        },

                        Token::Keyword(Keyword {
                            keyword_kind: KeywordKind::KwAs,
                            ..
                        }) => Some(Precedence::TypeCast),

                        Token::Punc(p) => match p.punc_kind {
                            PuncKind::FullStop => match parser.peek_num_tokens_ahead(2) {
                                Some(t) => match t {
                                    Token::Delim(Delimiter {
                                        delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                                        ..
                                    }) => Some(Precedence::Call),

                                    Token::Identifier(_) => Some(Precedence::FieldAccess),

                                    Token::UIntLit(_) => Some(Precedence::Index),

                                    _ => None,
                                },

                                None => None,
                            },

                            PuncKind::PlusEquals
                            | PuncKind::MinusEquals
                            | PuncKind::AsteriskEquals
                            | PuncKind::ForwardSlashEquals
                            | PuncKind::PercentEquals => Some(Precedence::CompoundAssignment),

                            PuncKind::DblDot | PuncKind::DotDotEquals => Some(Precedence::Range),

                            PuncKind::DblColon | PuncKind::ColonColonAsterisk => {
                                Some(Precedence::Path)
                            }

                            PuncKind::Asterisk | PuncKind::ForwardSlash | PuncKind::Percent => {
                                Some(Precedence::Product)
                            }

                            PuncKind::Plus | PuncKind::Minus => Some(Precedence::Sum),

                            PuncKind::DblLessThan | PuncKind::DblGreaterThan => {
                                Some(Precedence::Shift)
                            }

                            PuncKind::LessThan
                            | PuncKind::GreaterThan
                            | PuncKind::LessThanEquals
                            | PuncKind::GreaterThanEquals => Some(Precedence::Comparison),

                            PuncKind::DblEquals | PuncKind::BangEquals => {
                                Some(Precedence::Equality)
                            }

                            PuncKind::Ampersand => Some(Precedence::BitwiseAnd),

                            PuncKind::Caret => Some(Precedence::BitwiseXor),

                            PuncKind::Pipe => Some(Precedence::BitwiseOr),

                            PuncKind::DblAmpersand => Some(Precedence::And),

                            PuncKind::DblPipe => Some(Precedence::Or),

                            PuncKind::QuestionMark => Some(Precedence::Unwrap),

                            PuncKind::Equals => Some(Precedence::Assignment),

                            _ => None,
                        },

                        _ => None,
                    },

                    None => Some(Precedence::Path),
                }
            }

            Some(Token::Punc(p)) => match p.punc_kind {
                PuncKind::DblDot | PuncKind::DotDotEquals => Some(Precedence::Range),

                PuncKind::Pipe | PuncKind::DblPipe => Some(Precedence::Closure),

                PuncKind::Minus
                | PuncKind::Bang
                | PuncKind::Asterisk
                | PuncKind::Ampersand
                | PuncKind::HashSign
                | PuncKind::HashBang => Some(Precedence::Prefix),

                _ => None,
            },

            Some(Token::IntLit(_))
            | Some(Token::UIntLit(_))
            | Some(Token::U256Lit(_))
            | Some(Token::FloatLit(_)) => match parser.peek_num_tokens_ahead(1) {
                Some(t) => match t {
                    Token::Keyword(Keyword {
                        keyword_kind: KeywordKind::KwAs,
                        ..
                    }) => Some(Precedence::TypeCast),

                    Token::Punc(p) => match p.punc_kind {
                        PuncKind::DblDot | PuncKind::DotDotEquals => Some(Precedence::Range),

                        PuncKind::Asterisk | PuncKind::ForwardSlash | PuncKind::Percent => {
                            Some(Precedence::Product)
                        }

                        PuncKind::Plus | PuncKind::Minus => Some(Precedence::Sum),

                        PuncKind::DblLessThan | PuncKind::DblGreaterThan => Some(Precedence::Shift),

                        PuncKind::LessThan
                        | PuncKind::GreaterThan
                        | PuncKind::LessThanEquals
                        | PuncKind::GreaterThanEquals => Some(Precedence::Comparison),

                        PuncKind::BangEquals | PuncKind::DblEquals => Some(Precedence::Equality),

                        PuncKind::Ampersand => Some(Precedence::BitwiseAnd),

                        PuncKind::Caret => Some(Precedence::BitwiseXor),

                        PuncKind::Pipe => Some(Precedence::BitwiseOr),

                        PuncKind::DblAmpersand => Some(Precedence::And),

                        PuncKind::DblPipe => Some(Precedence::Or),

                        _ => None,
                    },

                    _ => None,
                },

                None => Some(Precedence::Literal),
            },

            Some(Token::CharLit(_)) | Some(Token::StringLit(_)) | Some(Token::BoolLit(_)) => {
                Some(Precedence::Literal)
            }

            _ => None,
        }
    }
}
