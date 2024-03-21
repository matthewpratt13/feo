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
    utils,
};

impl ParseStatement for ExprStatement {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(expression) = Expression::parse(parser)? {
            parser.next_token();

            let semicolon_opt = parser.peek_current::<Punctuation>();

            match &semicolon_opt {
                Some(Punctuation {
                    punc_kind: PuncKind::Semicolon,
                    ..
                })
                | None => {
                    parser.next_token();

                    return Ok(Some(ExprStatement {
                        expression,
                        semicolon_opt,
                    }));
                }

                _ => return Ok(None),
            }
        } else {
            return Ok(None);
        }
    }
}

impl ParseStatement for LetStatement {
    #[allow(unused_variables)]
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

                let assignment_opt = if let Some(e) = Expression::parse(parser)? {
                    parser.next_token();
                    Some(e)
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
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`;`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
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

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_expr_statement() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"x + 2"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let expr_statement =
            ExprStatement::parse(&mut parser).expect("unable to parse expression statement");

        Ok(println!("{:#?}", expr_statement))
    }
}
