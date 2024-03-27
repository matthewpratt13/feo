use feo_ast::{
    expression::{
        ArithmeticOrLogicalExpr, ArithmeticOrLogicalOperatorKind, AssignmentExpr, ComparisonExpr,
        ComparisonOperatorKind, CompoundAssignOperatorKind, CompoundAssignmentExpr, Expression,
        FieldAccessExpr, LazyBoolExpr, LazyBoolOperatorKind, MethodCallExpr, OperatorExprKind,
        RangeExprKind, RangeFromExpr, RangeFromToExpr, RangeInclusiveExpr, TupleIndexExpr,
        TypeCastExpr, UnwrapExpr, Value,
    },
    path::{PathIdenSegmentKind, PathInExpr},
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
    parse::ParseExpr,
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
            Some(Token::BoolLit(b)) => Some(Expression::Literal(LiteralKind::Bool(b))),
            Some(Token::IntLit(i)) => Some(Expression::Literal(LiteralKind::Int(i))),
            Some(Token::UIntLit(ui)) => Some(Expression::Literal(LiteralKind::UInt(ui))),
            Some(Token::U256Lit(u)) => Some(Expression::Literal(LiteralKind::U256(u))),
            Some(Token::FloatLit(f)) => Some(Expression::Literal(LiteralKind::Float(f))),
            Some(Token::Identifier(id)) => Some(Expression::PathExpr(PathInExpr {
                first_segment: PathIdenSegmentKind::Identifier(id),
                subsequent_segments: None,
            })),

            _ => None,
        }
    }

    fn parse_infix(&mut self, infix: Token, left: Expression) -> Option<Expression> {
        match infix {
            // Token::Keyword(k) => match k.keyword_kind {
            //     KeywordKind::KwAs => {
            //         if let Some(precedence) = Precedence::token_precedence(self) {
            //             let right = self.parse_expression(precedence)?;

            //             return Some(Expression::OperatorExpr(OperatorExprKind::TypeCast(
            //                 TypeCastExpr {
            //                     operator: k,
            //                     lhs: Box::new(Value::try_from(left).ok()?),
            //                     rhs: todo!(),
            //                 },
            //             )));
            //         } else {
            //             return None;
            //         }
            //     }

            //     _ => None,
            // },
            Token::Delim(d) => match d.delim {
                (DelimKind::Parenthesis, DelimOrientation::Open) => todo!(),

                (DelimKind::Brace, DelimOrientation::Open) => todo!(),

                (DelimKind::Bracket, DelimOrientation::Open) => todo!(),

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
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;

                        if let Some(_) = RangeFromToExpr::parse(self).ok()? {
                            return Some(Expression::RangeExpr(RangeExprKind::RangeFromToExpr(
                                RangeFromToExpr {
                                    from_operand: Box::new(Value::try_from(left).ok()?),
                                    dbl_dot: p,
                                    to_operand_excl: Box::new(Value::try_from(right).ok()?),
                                },
                            )));
                        } else if let Some(_) = RangeFromExpr::parse(self).ok()? {
                            let right = self.parse_expression(precedence)?;

                            return Some(Expression::RangeExpr(RangeExprKind::RangeFromToExpr(
                                RangeFromToExpr {
                                    from_operand: Box::new(Value::try_from(left).ok()?),
                                    dbl_dot: p,
                                    to_operand_excl: Box::new(Value::try_from(right).ok()?),
                                },
                            )));
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }

                PuncKind::DotDotEquals => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::RangeExpr(RangeExprKind::RangeInclusiveExpr(
                            RangeInclusiveExpr {
                                from_operand: Box::new(Value::try_from(left).ok()?),
                                dot_dot_equals: p,
                                to_operand_incl: Box::new(Value::try_from(right).ok()?),
                            },
                        )));
                    } else {
                        return None;
                    }
                }

                PuncKind::FullStop => {
                    if let Some(_) = Precedence::token_precedence(self) {
                        if let Some(mc) = MethodCallExpr::parse(self).ok()? {
                            return Some(Expression::MethodCallExpr(MethodCallExpr {
                                receiver: Box::new(Value::try_from(left).ok()?),
                                full_stop: p,
                                method_name: mc.clone().method_name,
                                open_parenthesis: mc.clone().open_parenthesis,
                                call_params_opt: mc.call_params_opt,
                                close_parenthesis: mc.open_parenthesis,
                            }));
                        } else if let Some(fa) = FieldAccessExpr::parse(self).ok()? {
                            return Some(Expression::FieldAccessExpr(FieldAccessExpr {
                                container_operand: Box::new(Value::try_from(left).ok()?),
                                field_name: fa.field_name,
                            }));
                        } else if let Some(tie) = TupleIndexExpr::parse(self).ok()? {
                            return Some(Expression::TupleIndexExpr(TupleIndexExpr {
                                operand: Box::new(Value::try_from(left).ok()?),
                                index: tie.index,
                            }));
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }

                PuncKind::DblColon => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::PathExpr(PathInExpr {
                            first_segment: todo!(),
                            subsequent_segments: todo!(),
                        }));
                    } else {
                        return None;
                    }
                }

                PuncKind::ColonColonAsterisk => {
                    if let Some(precedence) = Precedence::token_precedence(self) {
                        let right = self.parse_expression(precedence)?;
                        return Some(Expression::PathExpr(PathInExpr {
                            first_segment: todo!(),
                            subsequent_segments: todo!(),
                        }));
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
                    if let Some(_) = Precedence::token_precedence(self) {
                        return Some(Expression::OperatorExpr(OperatorExprKind::UnwrapExpr(
                            UnwrapExpr {
                                operand: Box::new(Value::try_from(left).ok()?),
                                operator: p,
                            },
                        )));
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
