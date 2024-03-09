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

#[cfg(test)]
mod tests {
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_infinite_loop_expr() {
        let source_code = r#"
        loop {
            foo += 2
        }"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let infinite_loop_expr =
            InfiniteLoopExpr::parse(&mut parser).expect("unable to parse infinite loop expression");

        println!("{:#?}", infinite_loop_expr);
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_predicate_loop_expr() {
        let source_code = r#"
        while foo < 100 {
            foo += 2
        }"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let predicate_loop_expr = PredicateLoopExpr::parse(&mut parser)
            .expect("unable to parse predicate loop expression");

        println!("{:#?}", predicate_loop_expr);
    }
}
