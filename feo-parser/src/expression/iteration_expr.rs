use feo_ast::{
    expression::{
        BlockExpr, BreakExpr, ContinueExpr, InfiniteLoopExpr, IterLoopExpr, ParenthesizedExpr,
        PredicateLoopExpr,
    },
    pattern::Pattern,
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, Keyword};

use crate::{
    parse::{ParseExpr, ParsePatt, ParseTerm},
    parser::Parser,
};

impl ParseExpr for BreakExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_break_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwBreak,
            ..
        }) = kw_break_opt
        {
            parser.next_token();
            return Ok(Some(BreakExpr(kw_break_opt.unwrap())));
        } else {
            return Ok(None);
        }
    }
}

impl ParseExpr for ContinueExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_continue_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwContinue,
            ..
        }) = kw_continue_opt
        {
            parser.next_token();
            return Ok(Some(ContinueExpr(kw_continue_opt.unwrap())));
        } else {
            return Ok(None);
        }
    }
}

impl ParseExpr for InfiniteLoopExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_loop_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwLoop,
            ..
        }) = kw_loop_opt
        {
            parser.next_token();

            if let Some(block) = BlockExpr::parse(parser)? {
                return Ok(Some(InfiniteLoopExpr {
                    kw_loop: kw_loop_opt.unwrap(),
                    block,
                }));
            }

            parser.log_error(ParserErrorKind::UnexpectedToken {
                expected: "`BlockExpr`".to_string(),
                found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseExpr for PredicateLoopExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_while_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwWhile,
            ..
        }) = kw_while_opt
        {
            parser.next_token();

            if let Some(conditional_operand) = ParenthesizedExpr::parse(parser)? {
                parser.next_token();

                if let Some(block) = BlockExpr::parse(parser)? {
                    parser.next_token();

                    return Ok(Some(PredicateLoopExpr {
                        kw_while: kw_while_opt.unwrap(),
                        conditional_operand: Box::new(conditional_operand),
                        block: Box::new(block),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`BlockExpr`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
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

impl ParseExpr for IterLoopExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_for_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwFor,
            ..
        }) = kw_for_opt
        {
            parser.next_token();

            if let Some(pattern) = Pattern::parse(parser)? {
                parser.next_token();

                let kw_in_opt = parser.peek_current::<Keyword>();

                if let Some(Keyword {
                    keyword_kind: KeywordKind::KwIn,
                    ..
                }) = kw_in_opt
                {
                    parser.next_token();

                    if let Some(iterator) = ParenthesizedExpr::parse(parser)? {
                        parser.next_token();

                        if let Some(block) = BlockExpr::parse(parser)? {
                            return Ok(Some(IterLoopExpr {
                                kw_for: kw_for_opt.unwrap(),
                                pattern: Box::new(pattern),
                                kw_in: kw_in_opt.unwrap(),
                                iterator: Box::new(iterator),
                                block,
                            }));
                        }
                    }

                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`ParenthesizedExpr`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`in`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`Pattern`".to_string(),
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
    fn parse_break_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"break"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let break_expr = BreakExpr::parse(&mut parser).expect("unable to parse break expression");

        Ok(println!("{:#?}", break_expr))
    }

    #[test]
    fn parse_continue_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"continue"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let continue_expr =
            ContinueExpr::parse(&mut parser).expect("unable to parse continue expression");

        Ok(println!("{:#?}", continue_expr))
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_infinite_loop_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        loop {
            foo += 2
        }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let infinite_loop_expr =
            InfiniteLoopExpr::parse(&mut parser).expect("unable to parse infinite loop expression");

        Ok(println!("{:#?}", infinite_loop_expr))
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_predicate_loop_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        while foo < 100 {
            foo += 2
        }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let predicate_loop_expr = PredicateLoopExpr::parse(&mut parser)
            .expect("unable to parse predicate loop expression");

        Ok(println!("{:#?}", predicate_loop_expr))
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_iter_loop_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        for x in (1..10) {
            x += 2
        }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let iter_loop_expr = IterLoopExpr::parse(&mut parser)
            .expect("unable to parse iterator loop expression");

        Ok(println!("{:#?}", iter_loop_expr))
    }
}
