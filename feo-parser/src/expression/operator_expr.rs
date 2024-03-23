use feo_ast::{
    expression::{
        ArithmeticOrLogicalExpr, ArithmeticOrLogicalOperatorKind, AssignmentExpr, ComparisonExpr,
        ComparisonOperatorKind, CompoundAssignOperatorKind, CompoundAssignmentExpr,
        DereferenceExpr, LazyBoolExpr, LazyBoolOperatorKind, NegationExpr, NegationOperatorKind,
        ReferenceExpr, TypeCastExpr, UnwrapExpr, Value,
    },
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Keyword, Punctuation};

use crate::{
    parse::{ParseExpr, ParseTerm, ParseType},
    parser::Parser,
    peek::{Peek, Peeker},
    utils::{self, LogMsgType},
};

impl Peek for ArithmeticOrLogicalOperatorKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(p) = Punctuation::peek(peeker) {
            match &p.punc_kind {
                PuncKind::Percent => Some(ArithmeticOrLogicalOperatorKind::Modulus(p)),
                PuncKind::Ampersand => Some(ArithmeticOrLogicalOperatorKind::LogicalAnd(p)),
                PuncKind::Asterisk => Some(ArithmeticOrLogicalOperatorKind::Multiply(p)),
                PuncKind::Plus => Some(ArithmeticOrLogicalOperatorKind::Add(p)),
                PuncKind::Minus => Some(ArithmeticOrLogicalOperatorKind::Subtract(p)),
                PuncKind::ForwardSlash => Some(ArithmeticOrLogicalOperatorKind::Divide(p)),
                PuncKind::Caret => Some(ArithmeticOrLogicalOperatorKind::LogicalXOr(p)),
                PuncKind::Pipe => Some(ArithmeticOrLogicalOperatorKind::LogicalOr(p)),
                PuncKind::DblLessThan => Some(ArithmeticOrLogicalOperatorKind::ShiftLeft(p)),
                PuncKind::DblGreaterThan => Some(ArithmeticOrLogicalOperatorKind::ShiftRight(p)),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Peek for ComparisonOperatorKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(p) = Punctuation::peek(peeker) {
            match &p.punc_kind {
                PuncKind::LessThan => Some(ComparisonOperatorKind::LessThan(p)),
                PuncKind::GreaterThan => Some(ComparisonOperatorKind::GreaterThan(p)),
                PuncKind::BangEquals => Some(ComparisonOperatorKind::NotEqual(p)),
                PuncKind::LessThanEquals => Some(ComparisonOperatorKind::LessThanOrEqual(p)),
                PuncKind::DblEquals => Some(ComparisonOperatorKind::Equality(p)),
                PuncKind::GreaterThanEquals => Some(ComparisonOperatorKind::GreaterThanOrEqual(p)),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Peek for CompoundAssignOperatorKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(p) = Punctuation::peek(peeker) {
            match &p.punc_kind {
                PuncKind::PercentEquals => Some(CompoundAssignOperatorKind::ModulusAssign(p)),
                PuncKind::AsteriskEquals => Some(CompoundAssignOperatorKind::MultiplyAssign(p)),
                PuncKind::PlusEquals => Some(CompoundAssignOperatorKind::AddAssign(p)),
                PuncKind::MinusEquals => Some(CompoundAssignOperatorKind::SubtractAssign(p)),
                PuncKind::ForwardSlashEquals => Some(CompoundAssignOperatorKind::DivideAssign(p)),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Peek for LazyBoolOperatorKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(p) = Punctuation::peek(peeker) {
            match &p.punc_kind {
                PuncKind::DblAmpersand => Some(LazyBoolOperatorKind::LazyAnd(p)),
                PuncKind::DblPipe => Some(LazyBoolOperatorKind::LazyOr(p)),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Peek for NegationOperatorKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(p) = Punctuation::peek(peeker) {
            match &p.punc_kind {
                PuncKind::Minus => Some(NegationOperatorKind::InvertNumeric(p)),
                PuncKind::Bang => Some(NegationOperatorKind::InvertBool(p)),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl ParseExpr for ArithmeticOrLogicalExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        utils::log_msg(
            LogMsgType::Enter,
            "arithmetic or logical expression",
            parser,
        );

        if let Some(lhs) = Value::parse(parser)? {
            parser.next_token();

            if let Some(p) = parser.peek_current::<Punctuation>() {
                parser.next_token();

                let operator = match p {
                    Punctuation {
                        punc_kind: PuncKind::Plus,
                        ..
                    } => ArithmeticOrLogicalOperatorKind::Add(p),

                    Punctuation {
                        punc_kind: PuncKind::Minus,
                        ..
                    } => ArithmeticOrLogicalOperatorKind::Subtract(p),

                    Punctuation {
                        punc_kind: PuncKind::Asterisk,
                        ..
                    } => ArithmeticOrLogicalOperatorKind::Multiply(p),

                    Punctuation {
                        punc_kind: PuncKind::ForwardSlash,
                        ..
                    } => ArithmeticOrLogicalOperatorKind::Divide(p),

                    Punctuation {
                        punc_kind: PuncKind::Percent,
                        ..
                    } => ArithmeticOrLogicalOperatorKind::Modulus(p),

                    Punctuation {
                        punc_kind: PuncKind::Ampersand,
                        ..
                    } => ArithmeticOrLogicalOperatorKind::LogicalAnd(p),

                    Punctuation {
                        punc_kind: PuncKind::Pipe,
                        ..
                    } => ArithmeticOrLogicalOperatorKind::LogicalOr(p),

                    Punctuation {
                        punc_kind: PuncKind::Caret,
                        ..
                    } => ArithmeticOrLogicalOperatorKind::LogicalXOr(p),

                    Punctuation {
                        punc_kind: PuncKind::DblLessThan,
                        ..
                    } => ArithmeticOrLogicalOperatorKind::ShiftLeft(p),

                    Punctuation {
                        punc_kind: PuncKind::DblGreaterThan,
                        ..
                    } => ArithmeticOrLogicalOperatorKind::ShiftRight(p),

                    _ => return Ok(None),
                };

                if let Some(rhs) = Value::parse(parser)? {
                    utils::log_msg(LogMsgType::Exit, "arithmetic or logical expression", parser);

                    return Ok(Some(ArithmeticOrLogicalExpr {
                        lhs: Box::new(lhs),
                        operator,
                        rhs: Box::new(rhs),
                    }));
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`ArithmeticOrLogicalOperatorKind`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for AssignmentExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(assignee) = Value::parse(parser)? {
            parser.next_token();

            let equals_opt = parser.peek_current();

            if let Some(Punctuation {
                punc_kind: PuncKind::Equals,
                ..
            }) = equals_opt
            {
                parser.next_token();

                if let Some(new_value) = Value::parse(parser)? {
                    return Ok(Some(AssignmentExpr {
                        assignee,
                        operator: equals_opt.unwrap(),
                        new_value,
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "value".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`=`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for CompoundAssignmentExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(assignee) = Value::parse(parser)? {
            parser.next_token();

            if let Some(p) = parser.peek_current::<Punctuation>() {
                parser.next_token();

                let operator = match p {
                    Punctuation {
                        punc_kind: PuncKind::PlusEquals,
                        ..
                    } => CompoundAssignOperatorKind::AddAssign(p),

                    Punctuation {
                        punc_kind: PuncKind::Minus,
                        ..
                    } => CompoundAssignOperatorKind::SubtractAssign(p),

                    Punctuation {
                        punc_kind: PuncKind::Asterisk,
                        ..
                    } => CompoundAssignOperatorKind::MultiplyAssign(p),

                    Punctuation {
                        punc_kind: PuncKind::ForwardSlash,
                        ..
                    } => CompoundAssignOperatorKind::DivideAssign(p),

                    Punctuation {
                        punc_kind: PuncKind::Percent,
                        ..
                    } => CompoundAssignOperatorKind::ModulusAssign(p),

                    _ => return Ok(None),
                };

                if let Some(new_value) = Value::parse(parser)? {
                    return Ok(Some(CompoundAssignmentExpr {
                        assignee,
                        operator,
                        new_value,
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "value".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`CompoundAssignOperatorKind`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for ComparisonExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        utils::log_msg(LogMsgType::Enter, "comparison expression", parser);

        if let Some(lhs) = Value::parse(parser)? {
            parser.next_token();

            if let Some(p) = parser.peek_current::<Punctuation>() {
                parser.next_token();

                let operator = match p {
                    Punctuation {
                        punc_kind: PuncKind::DblEquals,
                        ..
                    } => ComparisonOperatorKind::Equality(p),

                    Punctuation {
                        punc_kind: PuncKind::BangEquals,
                        ..
                    } => ComparisonOperatorKind::NotEqual(p),

                    Punctuation {
                        punc_kind: PuncKind::LessThan,
                        ..
                    } => ComparisonOperatorKind::LessThan(p),

                    Punctuation {
                        punc_kind: PuncKind::GreaterThan,
                        ..
                    } => ComparisonOperatorKind::GreaterThan(p),

                    Punctuation {
                        punc_kind: PuncKind::LessThanEquals,
                        ..
                    } => ComparisonOperatorKind::LessThanOrEqual(p),

                    Punctuation {
                        punc_kind: PuncKind::GreaterThanEquals,
                        ..
                    } => ComparisonOperatorKind::GreaterThanOrEqual(p),

                    _ => return Ok(None),
                };

                if let Some(rhs) = Value::parse(parser)? {
                    utils::log_msg(
                        LogMsgType::Exit,
                        "comparison expression",
                        parser,
                    );
                    return Ok(Some(ComparisonExpr { lhs, operator, rhs }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "value".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`ComparisonOperatorKind`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for DereferenceExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let operator_opt = parser.peek_current();

        if let Some(Punctuation {
            punc_kind: PuncKind::Asterisk,
            ..
        }) = operator_opt
        {
            parser.next_token();

            if let Some(operand) = Value::parse(parser)? {
                return Ok(Some(DereferenceExpr {
                    operator: operator_opt.unwrap(),
                    operand: Box::new(operand),
                }));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "value".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for LazyBoolExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(lhs) = Value::parse(parser)? {
            parser.next_token();

            if let Some(p) = parser.peek_current::<Punctuation>() {
                parser.next_token();

                let operator = match p {
                    Punctuation {
                        punc_kind: PuncKind::DblAmpersand,
                        ..
                    } => LazyBoolOperatorKind::LazyAnd(p),

                    Punctuation {
                        punc_kind: PuncKind::DblPipe,
                        ..
                    } => LazyBoolOperatorKind::LazyOr(p),

                    _ => {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`LazyBoolOperatorKind`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });

                        return Err(parser.errors());
                    }
                };

                if let Some(rhs) = Value::parse(parser)? {
                    return Ok(Some(LazyBoolExpr { lhs, operator, rhs }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "value".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`LazyBoolOperatorKind`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for NegationExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(p) = parser.peek_current::<Punctuation>() {
            parser.next_token();

            let operator = match p {
                Punctuation {
                    punc_kind: PuncKind::Minus,
                    ..
                } => NegationOperatorKind::InvertNumeric(p),

                Punctuation {
                    punc_kind: PuncKind::Bang,
                    ..
                } => NegationOperatorKind::InvertBool(p),

                _ => {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "value".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });

                    return Err(parser.errors());
                }
            };

            if let Some(operand) = Value::parse(parser)? {
                return Ok(Some(NegationExpr {
                    operator,
                    operand: Box::new(operand),
                }));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "value".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for ReferenceExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let ampersand_opt = parser.peek_current();

        if let Some(Punctuation {
            punc_kind: PuncKind::Ampersand,
            ..
        }) = ampersand_opt
        {
            parser.next_token();

            let kw_mut_opt = parser.peek_current();

            if let Some(Keyword {
                keyword_kind: KeywordKind::KwMut,
                ..
            }) = kw_mut_opt
            {
                parser.next_token();
            }

            let operator = (ampersand_opt.unwrap(), kw_mut_opt);

            if let Some(operand) = Value::parse(parser)? {
                return Ok(Some(ReferenceExpr {
                    operator,
                    operand: Box::new(operand),
                }));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "value".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for TypeCastExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(lhs) = Value::parse(parser)? {
            parser.next_token();

            let kw_as_opt = parser.peek_current();

            if let Some(Keyword {
                keyword_kind: KeywordKind::KwAs,
                ..
            }) = kw_as_opt
            {
                parser.next_token();

                if let Some(rhs) = Type::parse(parser)? {
                    return Ok(Some(TypeCastExpr {
                        lhs,
                        operator: kw_as_opt.unwrap(),
                        rhs,
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "value".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`as`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for UnwrapExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(operand) = Value::parse(parser)? {
            parser.next_token();

            let question_mark_opt = parser.peek_current();

            if let Some(Punctuation {
                punc_kind: PuncKind::QuestionMark,
                ..
            }) = question_mark_opt
            {
                parser.next_token();

                return Ok(Some(UnwrapExpr {
                    operand: Box::new(operand),
                    operator: question_mark_opt.unwrap(),
                }));
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`?`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_arithmetic_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"x + 2"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let arithmetic_expr = ArithmeticOrLogicalExpr::parse(&mut parser)
            .expect("unable to parse arithmetic expression");

        Ok(println!("{:#?}", arithmetic_expr))
    }

    #[test]
    fn parse_logical_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"1 | 2"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let logical_expr = ArithmeticOrLogicalExpr::parse(&mut parser)
            .expect("unable to parse logical expression");

        Ok(println!("{:#?}", logical_expr))
    }

    #[test]
    fn parse_assignment_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"x = 2"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let assignment_expr =
            AssignmentExpr::parse(&mut parser).expect("unable to parse assignment expression");

        Ok(println!("{:#?}", assignment_expr))
    }

    #[test]
    fn parse_compound_assignment_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"x += 2"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let compound_assignment_expr = CompoundAssignmentExpr::parse(&mut parser)
            .expect("unable to parse compound assignment expression");

        Ok(println!("{:#?}", compound_assignment_expr))
    }

    #[test]
    fn parse_comparison_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"x > 2"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let comparison_expr =
            ComparisonExpr::parse(&mut parser).expect("unable to parse comparison expression");

        Ok(println!("{:#?}", comparison_expr))
    }

    #[test]
    fn parse_dereference_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"*x"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let dereference_expr =
            DereferenceExpr::parse(&mut parser).expect("unable to parse dereference expression");

        Ok(println!("{:#?}", dereference_expr))
    }

    #[test]
    fn parse_lazy_bool_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"x && y"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let lazy_bool_expr =
            LazyBoolExpr::parse(&mut parser).expect("unable to parse lazy bool expression");

        Ok(println!("{:#?}", lazy_bool_expr))
    }

    #[test]
    fn parse_negation_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"!x"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let negation_expr =
            NegationExpr::parse(&mut parser).expect("unable to parse negation expression");

        Ok(println!("{:#?}", negation_expr))
    }

    #[test]
    fn parse_reference_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"&mut x"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let reference_expr =
            ReferenceExpr::parse(&mut parser).expect("unable to parse reference expression");

        Ok(println!("{:#?}", reference_expr))
    }

    #[test]
    fn parse_type_cast_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"1 as f64"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let type_cast_expr =
            TypeCastExpr::parse(&mut parser).expect("unable to parse type cast expression");

        Ok(println!("{:#?}", type_cast_expr))
    }

    #[test]
    fn parse_unwrap_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"foo?"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let unwrap_expr =
            UnwrapExpr::parse(&mut parser).expect("unable to parse unwrap expression");

        Ok(println!("{:#?}", unwrap_expr))
    }
}
