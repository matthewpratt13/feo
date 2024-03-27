use feo_ast::{
    expression::{
        ArithmeticOrLogicalExpr, ArithmeticOrLogicalOperatorKind, ArrayExpr, AssignmentExpr,
        BlockExpr, BreakExpr, ComparisonExpr, ComparisonOperatorKind, CompoundAssignOperatorKind,
        CompoundAssignmentExpr, ContinueExpr, DereferenceExpr, Expression, FieldAccessExpr,
        FunctionCallExpr, IfExpr, IndexExpr, InfiniteLoopExpr, IterLoopExpr, IterationExprKind,
        LazyBoolExpr, LazyBoolOperatorKind, MatchExpr, MethodCallExpr, NegationExpr,
        OperatorExprKind, ParenthesizedExpr, PredicateLoopExpr, RangeExprKind, RangeFromExpr,
        RangeFromToExpr, RangeInclusiveExpr, RangeToExpr, RangeToInclusiveExpr, ReferenceExpr,
        ReturnExpr, StructExpr, TupleExpr, TupleIndexExpr, TupleStructExpr, TypeCastExpr,
        UnderscoreExpr, UnwrapExpr, Value,
    },
    path::PathInExpr,
    token::{Token, TokenStream},
};
use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    parser_error::{ParserError, ParserErrorKind},
};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    literal::LiteralKind,
    punctuation::PuncKind,
    span::{Position, Spanned},
    Keyword,
};

use crate::{
    parse::{ParseExpr, ParseTerm},
    peek::{Peek, Peeker},
    precedence::Precedence,
};

/// Struct that stores a token stream and the current character index, and handles errors.
pub struct Parser {
    stream: TokenStream,
    pos: usize,
    handler: Handler,
}

impl Parser {
    pub fn new(stream: TokenStream, handler: Handler) -> Self {
        Parser {
            stream,
            pos: 0,
            handler,
        }
    }

    pub fn stream(&self) -> &TokenStream {
        &self.stream
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left_expr = self.parse_prefix()?;

        if let Some(input) = Precedence::token_precedence(self) {
            while precedence < input {
                let infix = self.next_token().expect("token not found");
                left_expr = self.parse_infix(infix, left_expr)?;
            }
        }

        Some(left_expr)
    }

    fn parse_prefix(&mut self) -> Option<Expression> {
        match self.current_token() {
            Some(Token::Keyword(k)) => match k.keyword_kind {
                KeywordKind::KwBreak => Some(Expression::BreakExpr(BreakExpr(k))),

                KeywordKind::KwContinue => Some(Expression::ContinueExpr(ContinueExpr(k))),

                KeywordKind::KwSelfType => {
                    if let Some(pth) = PathInExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::PathExpr(pth));
                    } else {
                        return None;
                    }
                }

                KeywordKind::KwSuper | KeywordKind::KwPackage => {
                    if let Some(pth) = PathInExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::PathExpr(pth));
                    } else {
                        return None;
                    }
                }

                KeywordKind::KwFor => {
                    if let Some(ile) = IterLoopExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::IterationExpr(IterationExprKind::IterLoop(ile)));
                    } else {
                        return None;
                    }
                }

                KeywordKind::KwIf => {
                    if let Some(ife) = IfExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::IfExpr(ife));
                    } else {
                        return None;
                    }
                }

                KeywordKind::KwLoop => {
                    if let Some(inf) = InfiniteLoopExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::IterationExpr(IterationExprKind::InfiniteLoop(
                            inf,
                        )));
                    } else {
                        return None;
                    }
                }

                KeywordKind::KwMatch => {
                    if let Some(me) = MatchExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::MatchExpr(me));
                    } else {
                        return None;
                    }
                }

                KeywordKind::KwReturn => {
                    if let Some(rtn) = ReturnExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::ReturnExpr(rtn));
                    } else {
                        return None;
                    }
                }

                KeywordKind::KwWhile => {
                    if let Some(pre) = PredicateLoopExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::IterationExpr(IterationExprKind::PredicateLoop(
                            pre,
                        )));
                    } else {
                        return None;
                    }
                }

                _ => None,
            },

            Some(Token::Identifier(id)) => {
                if &id.name == "_" {
                    return Some(Expression::UnderscoreExpr(UnderscoreExpr(id)));
                }

                if let Some(pth) = PathInExpr::parse(self).unwrap_or(None) {
                    return Some(Expression::PathExpr(pth));
                } else {
                    return None;
                }
            }

            Some(Token::Delim(d)) => match d.delim {
                (DelimKind::Brace, DelimOrientation::Open) => {
                    if let Some(be) = BlockExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::BlockExpr(be));
                    } else {
                        return None;
                    }
                }

                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(te) = TupleExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::TupleExpr(te));
                    } else if let Some(par) = ParenthesizedExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::ParenthesizedExpr(par));
                    } else {
                        return None;
                    }
                }

                (DelimKind::Bracket, DelimOrientation::Open) => {
                    if let Some(ae) = ArrayExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::ArrayExpr(ae));
                    } else {
                        return None;
                    }
                }

                _ => None,
            },

            Some(Token::Punc(p)) => match p.punc_kind {
                PuncKind::Bang | PuncKind::Minus => {
                    if let Some(ne) = NegationExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::OperatorExpr(OperatorExprKind::Negation(ne)));
                    } else {
                        return None;
                    }
                }

                PuncKind::Asterisk => {
                    if let Some(de) = DereferenceExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::OperatorExpr(OperatorExprKind::Dereference(de)));
                    } else {
                        return None;
                    }
                }

                PuncKind::Ampersand => {
                    if let Some(re) = ReferenceExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::OperatorExpr(OperatorExprKind::Reference(re)));
                    } else {
                        return None;
                    }
                }

                PuncKind::DblDot => {
                    if let Some(rte) = RangeToExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::RangeExpr(RangeExprKind::RangeToExpr(rte)));
                    } else {
                        return None;
                    }
                }

                PuncKind::DotDotEquals => {
                    if let Some(rti) = RangeToInclusiveExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::RangeExpr(RangeExprKind::RangeToInclusiveExpr(
                            rti,
                        )));
                    } else {
                        return None;
                    }
                }

                _ => None,
            },

            Some(Token::CharLit(c)) => Some(Expression::Literal(LiteralKind::Char(c))),
            Some(Token::StringLit(s)) => Some(Expression::Literal(LiteralKind::String(s))),
            Some(Token::BoolLit(b)) => Some(Expression::Literal(LiteralKind::Bool(b))),
            Some(Token::IntLit(i)) => Some(Expression::Literal(LiteralKind::Int(i))),
            Some(Token::UIntLit(ui)) => Some(Expression::Literal(LiteralKind::UInt(ui))),
            Some(Token::U256Lit(u)) => Some(Expression::Literal(LiteralKind::U256(u))),
            Some(Token::FloatLit(f)) => Some(Expression::Literal(LiteralKind::Float(f))),

            _ => None,
        }
    }

    fn parse_infix(&mut self, infix: Token, left: Expression) -> Option<Expression> {
        match infix {
            Token::Keyword(Keyword {
                keyword_kind: KeywordKind::KwAs,
                ..
            }) => {
                if let Some(tc) = TypeCastExpr::parse(self).unwrap_or(None) {
                    return Some(Expression::OperatorExpr(OperatorExprKind::TypeCast(tc)));
                } else {
                    return None;
                }
            }

            Token::Delim(d) => match d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => {
                    if let Some(fc) = FunctionCallExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::FunctionCallExpr(fc));
                    } else if let Some(ts) = TupleStructExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::TupleStructExpr(ts));
                    } else {
                        return None;
                    }
                }

                (DelimKind::Brace, DelimOrientation::Open) => {
                    if let Some(se) = StructExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::StructExpr(se));
                    } else {
                        return None;
                    }
                }

                (DelimKind::Bracket, DelimOrientation::Open) => {
                    if let Some(ie) = IndexExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::IndexExpr(ie));
                    } else {
                        return None;
                    }
                }

                _ => None,
            },

            Token::Punc(p) => match p.punc_kind {
                PuncKind::AsteriskEquals => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(OperatorExprKind::CompoundAssign(
                            CompoundAssignmentExpr {
                                assignee: Value::try_from(left).ok()?,
                                operator: CompoundAssignOperatorKind::MultiplyAssign(p),
                                new_value: Value::try_from(right).ok()?,
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::ForwardSlashEquals => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(OperatorExprKind::CompoundAssign(
                            CompoundAssignmentExpr {
                                assignee: Value::try_from(left).ok()?,
                                operator: CompoundAssignOperatorKind::DivideAssign(p),
                                new_value: Value::try_from(right).ok()?,
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::PercentEquals => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(OperatorExprKind::CompoundAssign(
                            CompoundAssignmentExpr {
                                assignee: Value::try_from(left).ok()?,
                                operator: CompoundAssignOperatorKind::ModulusAssign(p),
                                new_value: Value::try_from(right).ok()?,
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::PlusEquals => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(OperatorExprKind::CompoundAssign(
                            CompoundAssignmentExpr {
                                assignee: Value::try_from(left).ok()?,
                                operator: CompoundAssignOperatorKind::AddAssign(p),
                                new_value: Value::try_from(right).ok()?,
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::MinusEquals => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(OperatorExprKind::CompoundAssign(
                            CompoundAssignmentExpr {
                                assignee: Value::try_from(left).ok()?,
                                operator: CompoundAssignOperatorKind::SubtractAssign(p),
                                new_value: Value::try_from(right).ok()?,
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::DblDot => {
                    if let Some(rft) = RangeFromToExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::RangeExpr(RangeExprKind::RangeFromToExpr(rft)));
                    } else if let Some(rfe) = RangeFromExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::RangeExpr(RangeExprKind::RangeFromExpr(rfe)));
                    } else {
                        return None;
                    }
                }

                PuncKind::DotDotEquals => {
                    if let Some(rie) = RangeInclusiveExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::RangeExpr(RangeExprKind::RangeInclusiveExpr(
                            rie,
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::FullStop => {
                    if let Some(mc) = MethodCallExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::MethodCallExpr(mc));
                    } else if let Some(fa) = FieldAccessExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::FieldAccessExpr(fa));
                    } else if let Some(tie) = TupleIndexExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::TupleIndexExpr(tie));
                    } else {
                        return None;
                    }
                }

                PuncKind::DblColon | PuncKind::ColonColonAsterisk => {
                    if let Some(path_expr) = PathInExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::PathExpr(path_expr));
                    } else {
                        return None;
                    }
                }

                PuncKind::Plus => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(
                            OperatorExprKind::ArithmeticOrLogical(ArithmeticOrLogicalExpr {
                                lhs: Box::new(Value::try_from(left).ok()?),
                                operator: ArithmeticOrLogicalOperatorKind::Add(p),
                                rhs: Box::new(Value::try_from(right).ok()?),
                            }),
                        ));
                    } else {
                        return None;
                    }
                }

                PuncKind::Minus => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(
                            OperatorExprKind::ArithmeticOrLogical(ArithmeticOrLogicalExpr {
                                lhs: Box::new(Value::try_from(left).ok()?),
                                operator: ArithmeticOrLogicalOperatorKind::Subtract(p),
                                rhs: Box::new(Value::try_from(right).ok()?),
                            }),
                        ));
                    } else {
                        return None;
                    }
                }

                PuncKind::Asterisk => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(
                            OperatorExprKind::ArithmeticOrLogical(ArithmeticOrLogicalExpr {
                                lhs: Box::new(Value::try_from(left).ok()?),
                                operator: ArithmeticOrLogicalOperatorKind::Multiply(p),
                                rhs: Box::new(Value::try_from(right).ok()?),
                            }),
                        ));
                    } else {
                        return None;
                    }
                }

                PuncKind::ForwardSlash => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(
                            OperatorExprKind::ArithmeticOrLogical(ArithmeticOrLogicalExpr {
                                lhs: Box::new(Value::try_from(left).ok()?),
                                operator: ArithmeticOrLogicalOperatorKind::Divide(p),
                                rhs: Box::new(Value::try_from(right).ok()?),
                            }),
                        ));
                    } else {
                        return None;
                    }
                }

                PuncKind::Percent => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(
                            OperatorExprKind::ArithmeticOrLogical(ArithmeticOrLogicalExpr {
                                lhs: Box::new(Value::try_from(left).ok()?),
                                operator: ArithmeticOrLogicalOperatorKind::Modulus(p),
                                rhs: Box::new(Value::try_from(right).ok()?),
                            }),
                        ));
                    } else {
                        return None;
                    }
                }

                PuncKind::Ampersand => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(
                            OperatorExprKind::ArithmeticOrLogical(ArithmeticOrLogicalExpr {
                                lhs: Box::new(Value::try_from(left).ok()?),
                                operator: ArithmeticOrLogicalOperatorKind::BitwiseAnd(p),
                                rhs: Box::new(Value::try_from(right).ok()?),
                            }),
                        ));
                    } else {
                        return None;
                    }
                }

                PuncKind::Pipe => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(
                            OperatorExprKind::ArithmeticOrLogical(ArithmeticOrLogicalExpr {
                                lhs: Box::new(Value::try_from(left).ok()?),
                                operator: ArithmeticOrLogicalOperatorKind::BitwiseOr(p),
                                rhs: Box::new(Value::try_from(right).ok()?),
                            }),
                        ));
                    } else {
                        return None;
                    }
                }

                PuncKind::Caret => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(
                            OperatorExprKind::ArithmeticOrLogical(ArithmeticOrLogicalExpr {
                                lhs: Box::new(Value::try_from(left).ok()?),
                                operator: ArithmeticOrLogicalOperatorKind::BitwiseXor(p),
                                rhs: Box::new(Value::try_from(right).ok()?),
                            }),
                        ));
                    } else {
                        return None;
                    }
                }

                PuncKind::DblLessThan => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(
                            OperatorExprKind::ArithmeticOrLogical(ArithmeticOrLogicalExpr {
                                lhs: Box::new(Value::try_from(left).ok()?),
                                operator: ArithmeticOrLogicalOperatorKind::ShiftLeft(p),
                                rhs: Box::new(Value::try_from(right).ok()?),
                            }),
                        ));
                    } else {
                        return None;
                    }
                }

                PuncKind::DblGreaterThan => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(
                            OperatorExprKind::ArithmeticOrLogical(ArithmeticOrLogicalExpr {
                                lhs: Box::new(Value::try_from(left).ok()?),
                                operator: ArithmeticOrLogicalOperatorKind::ShiftRight(p),
                                rhs: Box::new(Value::try_from(right).ok()?),
                            }),
                        ));
                    } else {
                        return None;
                    }
                }

                PuncKind::LessThan => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(OperatorExprKind::Comparison(
                            ComparisonExpr {
                                lhs: Value::try_from(left).ok()?,
                                operator: ComparisonOperatorKind::LessThan(p),
                                rhs: Value::try_from(right).ok()?,
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::GreaterThan => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(OperatorExprKind::Comparison(
                            ComparisonExpr {
                                lhs: Value::try_from(left).ok()?,
                                operator: ComparisonOperatorKind::GreaterThan(p),
                                rhs: Value::try_from(right).ok()?,
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::LessThanEquals => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(OperatorExprKind::Comparison(
                            ComparisonExpr {
                                lhs: Value::try_from(left).ok()?,
                                operator: ComparisonOperatorKind::LessThanOrEqual(p),
                                rhs: Value::try_from(right).ok()?,
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::GreaterThanEquals => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(OperatorExprKind::Comparison(
                            ComparisonExpr {
                                lhs: Value::try_from(left).ok()?,
                                operator: ComparisonOperatorKind::GreaterThanOrEqual(p),
                                rhs: Value::try_from(right).ok()?,
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::DblEquals => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(OperatorExprKind::Comparison(
                            ComparisonExpr {
                                lhs: Value::try_from(left).ok()?,
                                operator: ComparisonOperatorKind::Equality(p),
                                rhs: Value::try_from(right).ok()?,
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::BangEquals => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(OperatorExprKind::Comparison(
                            ComparisonExpr {
                                lhs: Value::try_from(left).ok()?,
                                operator: ComparisonOperatorKind::NotEqual(p),
                                rhs: Value::try_from(right).ok()?,
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::DblAmpersand => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(OperatorExprKind::LazyBool(
                            LazyBoolExpr {
                                lhs: Value::try_from(left).ok()?,
                                operator: LazyBoolOperatorKind::LazyAnd(p),
                                rhs: Value::try_from(right).ok()?,
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::DblPipe => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(OperatorExprKind::LazyBool(
                            LazyBoolExpr {
                                lhs: Value::try_from(left).ok()?,
                                operator: LazyBoolOperatorKind::LazyOr(p),
                                rhs: Value::try_from(right).ok()?,
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::QuestionMark => {
                    if let Some(ue) = UnwrapExpr::parse(self).unwrap_or(None) {
                        return Some(Expression::OperatorExpr(OperatorExprKind::UnwrapExpr(ue)));
                    } else {
                        return None;
                    }
                }

                PuncKind::Equals => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::OperatorExpr(OperatorExprKind::Assignment(
                            AssignmentExpr {
                                assignee: Value::try_from(left).ok()?,
                                operator: p,
                                new_value: Value::try_from(right).ok()?,
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                _ => None,
            },

            _ => None,
        }
    }

    /// Return the current token.
    pub fn current_token(&self) -> Option<Token> {
        self.stream.tokens().get(self.pos).cloned()
    }

    /// Advance the parser and return the current token.
    pub fn next_token(&mut self) -> Option<Token> {
        let token = self.current_token();
        if token.is_some() {
            self.pos += 1;
        }

        token
    }

    pub fn peek_num_tokens_ahead(&self, num_tokens: usize) -> Option<Token> {
        self.stream.tokens().get(self.pos + num_tokens).cloned()
    }

    /// Return the previous token.
    pub fn previous_token(&mut self) -> Option<Token> {
        if self.pos > 0 {
            self.stream.tokens().get(self.pos - 1).cloned()
        } else {
            None
        }
    }

    /// Peek at the current `T` and return it if it exists (without advancing) or return `None`.
    pub fn peek_current<T: Peek>(&self) -> Option<T> {
        Peeker::with(&self.stream().tokens(), self.pos)
    }

    /// Peek at the next `T` and return it if it exists (without advancing) or return `None`.
    pub fn peek_next<T: Peek>(&self) -> Option<T> {
        Peeker::with(&self.stream().tokens(), self.pos + 1)
    }

    /// Peek at the `T` at `num_tokens` index and return it if it exists (without advancing)
    /// or return `None`.
    pub fn peek_ahead<T: Peek>(&self, offset: usize) -> Option<T> {
        Peeker::with(&self.stream().tokens(), self.pos + offset)
    }

    /// Push `ParserError` to the `Handler`.
    /// Return `ErrorEmitted` just to confirm that the action happened.
    pub fn log_error(&self, error_kind: ParserErrorKind) -> ErrorEmitted {
        let err = ParserError {
            error_kind,
            position: Position::new(
                &self.stream.span().source(),
                self.stream()
                    .tokens()
                    .get(self.pos)
                    .expect("PositionError: token not found")
                    .span()
                    .start(),
            ),
        };

        self.handler.emit_err(CompilerError::Parser(err))
    }
    pub fn errors(&self) -> Vec<CompilerError> {
        self.handler.clone().get_inner().0
    }
}
