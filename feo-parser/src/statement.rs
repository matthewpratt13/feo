use feo_ast::{
    expression::Expression,
    pattern::Pattern,
    statement::{ExprStatement, LetStatement},
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Keyword, Punctuation};

use crate::{
    parse::{ParseExpr, ParsePatt, ParseStatement, ParseType},
    parser::Parser,
    test_utils::{self, LogMsgType},
    utils,
};

impl ParseStatement for ExprStatement {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        test_utils::log_msg(LogMsgType::Enter, "expression statement", parser);

        if let Some(expression) = Expression::parse(parser)? {
            parser.next_token();

            let semicolon_opt = parser.peek_current::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::Semicolon,
                ..
            }) = semicolon_opt
            {
                test_utils::log_msg(LogMsgType::Detect, "semicolon", parser);

                parser.next_token();
            }

            test_utils::log_msg(LogMsgType::Exit, "expression statement", parser);

            return Ok(Some(ExprStatement {
                expression,
                semicolon_opt,
            }));
        } else {
            return Ok(None);
        }
    }
}

impl ParseStatement for LetStatement {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let kw_let_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwLet,
            ..
        }) = kw_let_opt
        {
            parser.next_token();

            if let Some(pattern) = Pattern::parse(parser)? {
                parser.next_token();

                let type_ann_opt = if let Some(Punctuation {
                    punc_kind: PuncKind::Colon,
                    ..
                }) = parser.peek_current()
                {
                    parser.next_token();

                    if let Some(t) = Type::parse(parser)? {
                        parser.next_token();
                        Some(t)
                    } else {
                        None
                    }
                } else {
                    None
                };

                let assignment_opt = if let Some(Punctuation {
                    punc_kind: PuncKind::Equals,
                    ..
                }) = parser.peek_current()
                {
                    parser.next_token();

                    if let Some(e) = Expression::parse(parser)? {
                        parser.next_token();
                        Some(e)
                    } else {
                        None
                    }
                } else {
                    None
                };

                let semicolon_opt = parser.peek_current();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Semicolon,
                    ..
                }) = semicolon_opt
                {
                    parser.next_token();

                    return Ok(Some(LetStatement {
                        attributes_opt,
                        kw_let: kw_let_opt.unwrap(),
                        pattern: Box::new(pattern),
                        type_ann_opt,
                        assignment_opt,
                        semicolon: semicolon_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`;`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`pattern`".to_string(),
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

    use super::*;

    #[test]
    fn parse_expr_statement() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"x + 2"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let expr_statement =
            ExprStatement::parse(&mut parser).expect("unable to parse expression statement");

        Ok(println!("{:#?}", expr_statement))
    }

    #[test]
    fn parse_let_statement() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"let x = 12 * 4;"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let let_statement =
            LetStatement::parse(&mut parser).expect("unable to parse let statement");

        Ok(println!("{:#?}", let_statement))
    }
}
