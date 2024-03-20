use feo_ast::{
    item::{TypeBound, WhereClause},
    path::{PathIdenSegmentKind, PathType},
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Identifier, Keyword, Punctuation};

use crate::{
    parse::{ParseTerm, ParseType},
    parser::Parser,
    utils,
};

impl ParseTerm for TypeBound {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut type_param_bounds: Vec<PathType> = Vec::new();

        if let Some(ty) = Type::parse(parser)? {
            // parser.next_token();

            println!("type: {:?}", ty);

            let colon_opt = parser.peek_current();

            if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = colon_opt
            {
                parser.next_token();

                if let Some(first_bound) = PathType::parse(parser)? {
                    type_param_bounds.push(first_bound);
                    // parser.next_token();

                    while let Some(Punctuation {
                        punc_kind: PuncKind::Plus,
                        ..
                    }) = parser.peek_next()
                    {
                        parser.next_token();

                        if let Some(_) = parser.peek_next::<Identifier>() {
                            parser.next_token();

                            if let Some(next_bound) = PathType::parse(parser)? {
                                type_param_bounds.push(next_bound);
                                // parser.next_token();
                            } else {
                                parser.log_error(ParserErrorKind::UnexpectedToken {
                                    expected: "path type".to_string(),
                                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                                });
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    println!("current token: {:?}", parser.current_token());

                    return Ok(Some(TypeBound {
                        ty,
                        type_param_bounds,
                    }));
                } else {
                    return Ok(None);
                }
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        }
    }
}

impl ParseTerm for WhereClause {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_where_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwWhere,
            ..
        }) = kw_where_opt
        {
            parser.next_token();

            if let Some(type_bounds) = utils::get_term_collection::<TypeBound>(parser)? {
                return Ok(Some(WhereClause {
                    kw_where: kw_where_opt.unwrap(),
                    type_bounds,
                }));
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_type_bound() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"Self: Foo + Bar + Baz"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let type_bound = TypeBound::parse(&mut parser).expect("unable to parse type bound");

        Ok(println!("{:#?}", type_bound))
    }

    #[test]
    fn parse_where_clause() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        where 
            Self: Foo + Bar + Baz,
            Self: Foo
            "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let where_clause = WhereClause::parse(&mut parser).expect("unable to parse where clause");

        Ok(println!("{:#?}", where_clause))
    }
}
