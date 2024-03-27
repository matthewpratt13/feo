use feo_ast::token::Token;
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Keyword};

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
            Some(Token::CharLit(_)) => todo!(),
            Some(Token::IntLit(_))
            | Some(Token::UIntLit(_))
            | Some(Token::U256Lit(_))
            | Some(Token::FloatLit(_)) => match parser.peek_next_token() {
                Some(t) => match t {
                    Token::Punc(p) => match p.punc_kind {
                        PuncKind::DblDot | PuncKind::DotDotEquals => Some(Precedence::Range),
                        PuncKind::Asterisk | PuncKind::ForwardSlash | PuncKind::Percent => {
                            Some(Precedence::Product)
                        }
                        PuncKind::Plus | PuncKind::Minus => Some(Precedence::Sum),
                        PuncKind::Pipe => Some(Precedence::BitwiseOr),
                        PuncKind::Ampersand => Some(Precedence::BitwiseAnd),
                        PuncKind::LessThan
                        | PuncKind::GreaterThan
                        | PuncKind::LessThanEquals
                        | PuncKind::GreaterThanEquals => Some(Precedence::Comparison),
                        PuncKind::Caret => Some(Precedence::BitwiseXor),
                        PuncKind::BangEquals | PuncKind::DblEquals => Some(Precedence::Equality),
                        PuncKind::DblLessThan | PuncKind::DblGreaterThan => Some(Precedence::Shift),
                        PuncKind::DblAmpersand => Some(Precedence::And),
                        PuncKind::DblPipe => Some(Precedence::Or),
                        _ => Some(Precedence::Lowest),
                    },

                    Token::Keyword(Keyword {
                        keyword_kind: KeywordKind::KwAs,
                        ..
                    }) => Some(Precedence::TypeCast),
                    _ => Some(Precedence::Literal),
                },

                _ => Some(Precedence::Literal),
            },
            
            Some(Token::StringLit(_)) => todo!(),
            Some(Token::BoolLit(_)) => todo!(),
            Some(Token::Identifier(_)) => todo!(),
            Some(Token::Keyword(_)) => todo!(),
            Some(Token::Comment(_)) => todo!(),
            Some(Token::DocComment(_)) => todo!(),
            Some(Token::Delim(_)) => todo!(),
            Some(Token::Punc(_)) => todo!(),
            Some(Token::EOF) => todo!(),
            _ => None,
        }
    }
}
