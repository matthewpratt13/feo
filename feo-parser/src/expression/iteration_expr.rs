use feo_ast::{
    expression::{BlockExpr, BooleanOperand, InfiniteLoopExpr, IterLoopExpr, PredicateLoopExpr},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, Keyword};

use crate::{parse::ParseExpr, parser::Parser};

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

            if let Some(conditional_operand) = BooleanOperand::parse(parser)? {
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
        todo!()
    }
}
