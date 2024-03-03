use feo_ast::{
    expression::{
        ArithmeticOrLogicalExpr, ArithmeticOrLogicalOperatorKind, Assignable, AssignmentExpr,
        ComparisonExpr, ComparisonOperatorKind, CompoundAssignOperatorKind, CompoundAssignmentExpr,
        DereferenceExpr, LazyBoolExpr, LazyBoolOperatorKind, NegationExpr, NegationOperatorKind,
        Operable, ReferenceExpr, TypeCastExpr, UnwrapExpr, UnwrapOperandKind,
    },
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Keyword, Punctuation};

use crate::{
    parse::ParseExpr,
    parser::Parser,
    peek::{Peek, Peeker},
};

impl Peek for ArithmeticOrLogicalOperatorKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let operator_kind = if let Some(p) = Punctuation::peek(peeker) {
            match p.punc_kind {
                PuncKind::Percent => ArithmeticOrLogicalOperatorKind::Modulus(p),
                PuncKind::Ampersand => ArithmeticOrLogicalOperatorKind::LogicalAnd(p),
                PuncKind::Asterisk => ArithmeticOrLogicalOperatorKind::Multiply(p),
                PuncKind::Plus => ArithmeticOrLogicalOperatorKind::Add(p),
                PuncKind::Minus => ArithmeticOrLogicalOperatorKind::Subtract(p),
                PuncKind::ForwardSlash => ArithmeticOrLogicalOperatorKind::Divide(p),
                PuncKind::Caret => ArithmeticOrLogicalOperatorKind::LogicalXOr(p),
                PuncKind::Pipe => ArithmeticOrLogicalOperatorKind::LogicalOr(p),
                PuncKind::DblLessThan => ArithmeticOrLogicalOperatorKind::ShiftLeft(p),
                PuncKind::DblGreaterThan => ArithmeticOrLogicalOperatorKind::ShiftRight(p),
                _ => return None,
            }
        } else {
            return None;
        };

        Some(operator_kind)
    }
}

impl Peek for ComparisonOperatorKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let operator_kind = if let Some(p) = Punctuation::peek(peeker) {
            match p.punc_kind {
                PuncKind::LessThan => ComparisonOperatorKind::LessThan(p),
                PuncKind::GreaterThan => ComparisonOperatorKind::GreaterThan(p),
                PuncKind::BangEquals => ComparisonOperatorKind::NotEqual(p),
                PuncKind::LessThanEquals => ComparisonOperatorKind::LessThanOrEqual(p),
                PuncKind::DblEquals => ComparisonOperatorKind::Equality(p),
                PuncKind::GreaterThanEquals => ComparisonOperatorKind::GreaterThanOrEqual(p),
                _ => return None,
            }
        } else {
            return None;
        };

        Some(operator_kind)
    }
}

impl Peek for CompoundAssignOperatorKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let operator_kind = if let Some(p) = Punctuation::peek(peeker) {
            match p.punc_kind {
                PuncKind::PercentEquals => CompoundAssignOperatorKind::ModulusAssign(p),
                PuncKind::AsteriskEquals => CompoundAssignOperatorKind::MultiplyAssign(p),
                PuncKind::PlusEquals => CompoundAssignOperatorKind::AddAssign(p),
                PuncKind::MinusEquals => CompoundAssignOperatorKind::SubtractAssign(p),
                PuncKind::ForwardSlashEquals => CompoundAssignOperatorKind::DivideAssign(p),
                _ => return None,
            }
        } else {
            return None;
        };

        Some(operator_kind)
    }
}

impl Peek for LazyBoolOperatorKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let operator_kind = if let Some(p) = Punctuation::peek(peeker) {
            match p.punc_kind {
                PuncKind::DblAmpersand => LazyBoolOperatorKind::LazyAnd(p),
                PuncKind::DblPipe => LazyBoolOperatorKind::LazyOr(p),
                _ => return None,
            }
        } else {
            return None;
        };

        Some(operator_kind)
    }
}

impl Peek for NegationOperatorKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let operator_kind = if let Some(p) = Punctuation::peek(peeker) {
            match p.punc_kind {
                PuncKind::Minus => NegationOperatorKind::InvertNumeric(p),
                PuncKind::Bang => NegationOperatorKind::InvertBool(p),
                _ => return None,
            }
        } else {
            return None;
        };

        Some(operator_kind)
    }
}

// TODO: how ??
impl Peek for UnwrapOperandKind {
    fn peek(peeker: &Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for ArithmeticOrLogicalExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(lhs) = Operable::parse(parser)? {
            parser.next_token();

            if let Some(operator) = parser.peek_current::<Punctuation>() {
                match operator {
                    Punctuation {
                        punc_kind: PuncKind::Plus,
                        ..
                    } => {
                        parser.next_token();

                        if let Some(rhs) = Operable::parse(parser)? {
                            parser.next_token();

                            return Ok(Some(ArithmeticOrLogicalExpr {
                                lhs: Box::new(lhs),
                                operator: ArithmeticOrLogicalOperatorKind::Add(operator),
                                rhs: Box::new(rhs),
                            }));
                        }

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`Operable`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                    Punctuation {
                        punc_kind: PuncKind::Minus,
                        ..
                    } => {
                        parser.next_token();

                        if let Some(rhs) = Operable::parse(parser)? {
                            parser.next_token();

                            return Ok(Some(ArithmeticOrLogicalExpr {
                                lhs: Box::new(lhs),
                                operator: ArithmeticOrLogicalOperatorKind::Subtract(operator),
                                rhs: Box::new(rhs),
                            }));
                        }

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`Operable`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                    Punctuation {
                        punc_kind: PuncKind::Asterisk,
                        ..
                    } => {
                        parser.next_token();

                        if let Some(rhs) = Operable::parse(parser)? {
                            parser.next_token();

                            return Ok(Some(ArithmeticOrLogicalExpr {
                                lhs: Box::new(lhs),
                                operator: ArithmeticOrLogicalOperatorKind::Multiply(operator),
                                rhs: Box::new(rhs),
                            }));
                        }

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`Operable`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                    Punctuation {
                        punc_kind: PuncKind::ForwardSlash,
                        ..
                    } => {
                        parser.next_token();

                        if let Some(rhs) = Operable::parse(parser)? {
                            parser.next_token();

                            return Ok(Some(ArithmeticOrLogicalExpr {
                                lhs: Box::new(lhs),
                                operator: ArithmeticOrLogicalOperatorKind::Divide(operator),
                                rhs: Box::new(rhs),
                            }));
                        }

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`Operable`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                    Punctuation {
                        punc_kind: PuncKind::Percent,
                        ..
                    } => {
                        parser.next_token();

                        if let Some(rhs) = Operable::parse(parser)? {
                            parser.next_token();

                            return Ok(Some(ArithmeticOrLogicalExpr {
                                lhs: Box::new(lhs),
                                operator: ArithmeticOrLogicalOperatorKind::Modulus(operator),
                                rhs: Box::new(rhs),
                            }));
                        }

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "Operable`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                    Punctuation {
                        punc_kind: PuncKind::Ampersand,
                        ..
                    } => {
                        parser.next_token();

                        if let Some(rhs) = Operable::parse(parser)? {
                            parser.next_token();

                            return Ok(Some(ArithmeticOrLogicalExpr {
                                lhs: Box::new(lhs),
                                operator: ArithmeticOrLogicalOperatorKind::LogicalAnd(operator),
                                rhs: Box::new(rhs),
                            }));
                        }

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`Operable`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                    Punctuation {
                        punc_kind: PuncKind::Pipe,
                        ..
                    } => {
                        parser.next_token();

                        if let Some(rhs) = Operable::parse(parser)? {
                            parser.next_token();

                            return Ok(Some(ArithmeticOrLogicalExpr {
                                lhs: Box::new(lhs),
                                operator: ArithmeticOrLogicalOperatorKind::LogicalOr(operator),
                                rhs: Box::new(rhs),
                            }));
                        }

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`Operable`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                    Punctuation {
                        punc_kind: PuncKind::Caret,
                        ..
                    } => {
                        parser.next_token();

                        if let Some(rhs) = Operable::parse(parser)? {
                            parser.next_token();

                            return Ok(Some(ArithmeticOrLogicalExpr {
                                lhs: Box::new(lhs),
                                operator: ArithmeticOrLogicalOperatorKind::LogicalXOr(operator),
                                rhs: Box::new(rhs),
                            }));
                        }

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`Operable`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                    Punctuation {
                        punc_kind: PuncKind::DblLessThan,
                        ..
                    } => {
                        parser.next_token();

                        if let Some(rhs) = Operable::parse(parser)? {
                            parser.next_token();

                            return Ok(Some(ArithmeticOrLogicalExpr {
                                lhs: Box::new(lhs),
                                operator: ArithmeticOrLogicalOperatorKind::ShiftLeft(operator),
                                rhs: Box::new(rhs),
                            }));
                        }

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`Operable`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                    Punctuation {
                        punc_kind: PuncKind::DblGreaterThan,
                        ..
                    } => {
                        parser.next_token();

                        if let Some(rhs) = Operable::parse(parser)? {
                            parser.next_token();

                            return Ok(Some(ArithmeticOrLogicalExpr {
                                lhs: Box::new(lhs),
                                operator: ArithmeticOrLogicalOperatorKind::ShiftRight(operator),
                                rhs: Box::new(rhs),
                            }));
                        }

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`Operable`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }

                    _ => {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`ArithmeticOrLogicalOperatorKind`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
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
        todo!()
    }
}

impl ParseExpr for ComparisonExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for CompoundAssignmentExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for DereferenceExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let operator_opt = parser.peek_current::<Punctuation>();

        if let Some(Punctuation {
            punc_kind: PuncKind::Asterisk,
            ..
        }) = operator_opt
        {
            parser.next_token();

            if let Some(operand) = Assignable::parse(parser)? {
                parser.next_token();

                return Ok(Some(DereferenceExpr {
                    operator: operator_opt.unwrap(),
                    operand: Box::new(operand),
                }));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`Assignable`".to_string(),
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
        todo!()
    }
}

impl ParseExpr for NegationExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseExpr for ReferenceExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let ampersand_opt = parser.peek_current::<Punctuation>();

        if let Some(Punctuation {
            punc_kind: PuncKind::Ampersand,
            ..
        }) = ampersand_opt
        {
            parser.next_token();

            let kw_mut_opt = parser.peek_current::<Keyword>();

            if let Some(Keyword {
                keyword_kind: KeywordKind::KwMut,
                ..
            }) = kw_mut_opt
            {
                parser.next_token();
            }

            let operator = (ampersand_opt.unwrap(), kw_mut_opt);

            if let Some(operand) = Assignable::parse(parser)? {
                parser.next_token();

                return Ok(Some(ReferenceExpr {
                    operator,
                    operand: Box::new(operand),
                }));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`Assignable".to_string(),
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
        todo!();
    }
}

impl ParseExpr for UnwrapExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_arithmetic_expr() {
        let source_code = r#"1+2"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let arithmetic_expr =
            DereferenceExpr::parse(&mut parser).expect("unable to parse arithmetic expression");

        println!("{:#?}", arithmetic_expr);
    }

    #[test]
    fn parse_deref_expr() {
        let source_code = r#"*x"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let deref_expr =
            DereferenceExpr::parse(&mut parser).expect("unable to parse dereference expression");

        println!("{:#?}", deref_expr);
    }

    #[test]
    fn parse_ref_expr() {
        let source_code = r#"&mut x"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let ref_expr =
            ReferenceExpr::parse(&mut parser).expect("unable to parse reference expression");

        println!("{:#?}", ref_expr);
    }
}
