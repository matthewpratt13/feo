use feo_ast::{
    attribute::OuterAttr,
    expression::{BlockExpr, Expression, IfExpr, MatchArm, MatchArmGuard, MatchArms, MatchExpr},
    pattern::Pattern,
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, utils::KwElse, Keyword};

use crate::{
    parse::{ParseExpr, ParsePatt, ParseTerm},
    parser::Parser,
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

            if let Some(condition_operand) = Expression::parse(parser)? {
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
                                expected: "`IfExpr`".to_string(),
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
                                expected: "`BlockExpr`".to_string(),
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
                        expected: "`BlockExpr`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`BooleanOperand`".to_string(),
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
                expected: "`Expression`".to_string(),
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
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(oa) = OuterAttr::parse(parser)? {
            attributes.push(oa);
            parser.next_token();
        }

        if let Some(pattern) = Pattern::parse(parser)? {
            parser.next_token();

            let match_arm_guard_opt = if let Some(mag) = MatchArmGuard::parse(parser)? {
                Some(mag)
            } else {
                None
            };

            match &attributes.is_empty() {
                true => {
                    return Ok(Some(MatchArm {
                        attributes_opt: None,
                        pattern: Box::new(pattern),
                        match_arm_guard_opt,
                    }))
                }
                false => {
                    return Ok(Some(MatchArm {
                        attributes_opt: Some(attributes),
                        pattern: Box::new(pattern),
                        match_arm_guard_opt,
                    }))
                }
            }
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
        todo!()
    }
}

impl ParseExpr for MatchExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_if_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        if foo < 2 { 
            print!("bar")
        }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let if_expr = IfExpr::parse(&mut parser).expect("unable to parse if expression");

        Ok(println!("{:#?}", if_expr))
    }
}
