use feo_ast::{
    expression::{
        BlockExpr, Expression, IfExpr, MatchArm, MatchArmGuard, MatchArms, MatchExpr,
        ParenthesizedExpr,
    },
    pattern::Pattern,
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    type_utils::KwElse,
    Delimiter, Keyword, Punctuation,
};

use crate::{
    parse::{ParseExpr, ParsePatt, ParseTerm},
    parser::Parser,
    utils,
};

impl ParseExpr for IfExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut else_if_blocks: Vec<(KwElse, Box<IfExpr>)> = Vec::new();

        let kw_if_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwIf,
            ..
        }) = kw_if_opt
        {
            parser.next_token();

            if let Some(condition_operand) = ParenthesizedExpr::parse(parser)? {
                parser.next_token();

                if let Some(if_block) = BlockExpr::parse(parser)? {
                    parser.next_token();

                    let mut next_kw_else_opt = parser.peek_current::<Keyword>();

                    while let Some(Keyword {
                        keyword_kind: KeywordKind::KwElse,
                        ..
                    }) = next_kw_else_opt
                    {
                        parser.next_token();

                        if let Some(next_if_expr) = IfExpr::parse(parser)? {
                            parser.next_token();

                            else_if_blocks
                                .push((next_kw_else_opt.unwrap(), Box::new(next_if_expr)));

                            if let Some(k) = parser.peek_current::<Keyword>() {
                                next_kw_else_opt = Some(k)
                            } else {
                                break;
                            }
                        } else {
                            parser.log_error(ParserErrorKind::UnexpectedToken {
                                expected: "if expression".to_string(),
                                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                            });
                            break;
                        }
                    }

                    let trailing_kw_else_opt = parser.peek_current::<Keyword>();

                    let trailing_else_block_opt = if let Some(Keyword {
                        keyword_kind: KeywordKind::KwElse,
                        ..
                    }) = trailing_kw_else_opt
                    {
                        parser.next_token();

                        if let Some(trailing_block_expr) = BlockExpr::parse(parser)? {
                            Some((trailing_kw_else_opt.unwrap(), trailing_block_expr))
                        } else {
                            parser.log_error(ParserErrorKind::UnexpectedToken {
                                expected: "block expression".to_string(),
                                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                            });
                            return Err(parser.errors());
                        }
                    } else {
                        None
                    };

                    match else_if_blocks.is_empty() {
                        true => {
                            return Ok(Some(IfExpr {
                                kw_if: kw_if_opt.unwrap(),
                                condition_operand: Box::new(condition_operand),
                                if_block: Box::new(if_block),
                                else_if_blocks_opt: None,
                                trailing_else_block_opt,
                            }))
                        }

                        false => {
                            return Ok(Some(IfExpr {
                                kw_if: kw_if_opt.unwrap(),
                                condition_operand: Box::new(condition_operand),
                                if_block: Box::new(if_block),
                                else_if_blocks_opt: Some(else_if_blocks),
                                trailing_else_block_opt,
                            }))
                        }
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "block expression".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "parenthesized expression".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseTerm for MatchArmGuard {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_if_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwIf,
            ..
        }) = kw_if_opt
        {
            parser.next_token();

            if let Some(operand) = Expression::parse(parser)? {
                parser.next_token();

                return Ok(Some(MatchArmGuard {
                    kw_if: kw_if_opt.unwrap(),
                    operand: Box::new(operand),
                }));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "expression".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseTerm for MatchArm {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        if let Some(pattern) = Pattern::parse(parser)? {
            parser.next_token();

            let match_arm_guard_opt = if let Some(mag) = MatchArmGuard::parse(parser)? {
                Some(mag)
            } else {
                None
            };

            return Ok(Some(MatchArm {
                attributes_opt,
                pattern: Box::new(pattern),
                match_arm_guard_opt,
            }));
        } else {
            return Ok(None);
        }
    }
}

impl ParseTerm for MatchArms {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut arms: Vec<(MatchArm, Expression)> = Vec::new();

        let first_arm = if let Some(arm) = self::get_arm(parser)? {
            arm
        } else {
            return Ok(None);
        };

        if let Some(Punctuation {
            punc_kind: PuncKind::Comma,
            ..
        }) = parser.peek_current::<Punctuation>()
        {
            parser.next_token();
            arms.push(first_arm.clone());
        }

        while let Some(Punctuation {
            punc_kind: PuncKind::Comma,
            ..
        }) = parser.peek_current::<Punctuation>()
        {
            parser.next_token();

            if let Some(arm) = self::get_arm(parser)? {
                arms.push(arm);
                parser.next_token();
            }
        }

        let final_arm = if let Some(arm) = self::get_arm(parser)? {
            (arm.0, Box::new(arm.1))
        } else {
            arms.clear();
            (first_arm.0, Box::new(first_arm.1))
        };

        match &arms.is_empty() {
            true => {
                return Ok(Some(MatchArms {
                    arms_opt: None,
                    final_arm,
                }))
            }
            false => {
                return Ok(Some(MatchArms {
                    arms_opt: Some(arms),
                    final_arm,
                }))
            }
        }
    }
}

impl ParseExpr for MatchExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_match_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwMatch,
            ..
        }) = kw_match_opt
        {
            parser.next_token();

            if let Some(scrutinee) = ParenthesizedExpr::parse(parser)? {
                let open_brace_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) = open_brace_opt
                {
                    parser.next_token();

                    let attributes_opt = utils::get_attributes(parser)?;

                    let match_arms_opt = if let Some(ma) = MatchArms::parse(parser)? {
                        Some(ma)
                    } else {
                        None
                    };

                    if let Some(Punctuation {
                        punc_kind: PuncKind::Comma,
                        ..
                    }) = parser.peek_current::<Punctuation>()
                    {
                        parser.next_token();
                    }

                    let close_brace_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        parser.next_token();

                        return Ok(Some(MatchExpr {
                            kw_match: kw_match_opt.unwrap(),
                            scrutinee: Box::new(scrutinee),
                            open_brace: open_brace_opt.unwrap(),
                            attributes_opt,
                            match_arms_opt,
                            close_brace: close_brace_opt.unwrap(),
                        }));
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`}`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`{`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "value expression".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

fn get_arm(parser: &mut Parser) -> Result<Option<(MatchArm, Expression)>, Vec<CompilerError>> {
    if let Some(arm) = MatchArm::parse(parser)? {
        if let Some(Punctuation {
            punc_kind: PuncKind::FatArrow,
            ..
        }) = parser.peek_current::<Punctuation>()
        {
            parser.next_token();

            if let Some(expr) = Expression::parse(parser)? {
                parser.next_token();
                return Ok(Some((arm, expr)));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "expression".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        } else {
            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`=>`".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        }
    } else {
        return Ok(None);
    }

    Err(parser.errors())
}

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_match_arm_guard() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"if x > 2"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let match_arm_guard =
            MatchArmGuard::parse(&mut parser).expect("unable to parse match arm guard");

        Ok(println!("{:#?}", match_arm_guard))
    }

    #[test]
    fn parse_match_arm() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        true
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let match_arm = MatchArm::parse(&mut parser).expect("unable to parse match arm");

        Ok(println!("{:#?}", match_arm))
    }

    #[test]
    fn parse_match_arms() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        true => x + 2,
        false => x - 2
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let match_arms = MatchArms::parse(&mut parser).expect("unable to parse `MatchArms`");

        Ok(println!("{:#?}", match_arms))
    }

    #[test]
    fn parse_match_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        match (foo) {
            #![unsafe]
            #[abstract]
            true => x + 2,
            false => x - 2
        }
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let match_expr = MatchExpr::parse(&mut parser).expect("unable to parse match expression");

        Ok(println!("{:#?}", match_expr))
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_if_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        if (foo < 2) { 
            print!("bar")
        }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let if_expr = IfExpr::parse(&mut parser).expect("unable to parse if expression");

        Ok(println!("{:#?}", if_expr))
    }
}
