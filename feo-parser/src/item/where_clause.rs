use feo_ast::{
    item::{TypeBound, TypeParamBounds, WhereClause},
    path::PathType,
    token::Token,
    ty::TraitBound,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Keyword, Punctuation};

use crate::{
    parse::{ParseTerm, ParseType},
    parser::Parser,
};

impl ParseTerm for TypeParamBounds {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_bounds: Vec<TraitBound> = Vec::new();

        if let Some(first_bound) = PathType::parse(parser)? {
            parser.next_token();

            while let Some(Punctuation {
                punc_kind: PuncKind::Plus,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(next_bound) = PathType::parse(parser)? {
                    subsequent_bounds.push(next_bound);
                    parser.next_token();
                } else {
                    break;
                }
            }

            match &subsequent_bounds.is_empty() {
                true => Ok(Some(TypeParamBounds {
                    first_bound,
                    subsequent_bounds_opt: None,
                })),
                false => Ok(Some(TypeParamBounds {
                    first_bound,
                    subsequent_bounds_opt: Some(subsequent_bounds),
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for TypeBound {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(ty) = Type::parse(parser)? {
            let colon_opt = parser.peek_current::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = colon_opt
            {
                parser.next_token();

                let type_param_bounds_opt = if let Some(t) = TypeParamBounds::parse(parser)? {
                    Some(t)
                } else {
                    None
                };

                return Ok(Some(TypeBound {
                    ty,
                    type_param_bounds_opt,
                }));
            }
            Ok(None)
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for WhereClause {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_bounds: Vec<TypeBound> = Vec::new();

        let kw_where_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwWhere,
            ..
        }) = kw_where_opt
        {
            parser.next_token();

            if let Some(first_bound) = TypeBound::parse(parser)? {
                while let Some(Punctuation {
                    punc_kind: PuncKind::Comma,
                    ..
                }) = parser.peek_current::<Punctuation>()
                {
                    parser.next_token();

                    if let Some(next_bound) = TypeBound::parse(parser)? {
                        subsequent_bounds.push(next_bound);
                        parser.next_token();
                    } else {
                        break;
                    }
                }

                match &subsequent_bounds.is_empty() {
                    true => {
                        return Ok(Some(WhereClause {
                            kw_where: kw_where_opt.unwrap(),
                            first_bound,
                            subsequent_bounds_opt: None,
                        }))
                    }
                    false => {
                        return Ok(Some(WhereClause {
                            kw_where: kw_where_opt.unwrap(),
                            first_bound,
                            subsequent_bounds_opt: Some(subsequent_bounds),
                        }))
                    }
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`TypeBound`".to_string(),
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
    fn parse_type_param_bounds() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"Foo + Bar + Baz"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let type_param_bounds =
            TypeParamBounds::parse(&mut parser).expect("unable to parse type param bounds");

        Ok(println!("{:#?}", type_param_bounds))
    }

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
            Self: Foo,
            "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let where_clause = WhereClause::parse(&mut parser).expect("unable to parse where clause");

        // Ok(())

        Ok(println!("{:#?}", where_clause))
    }
}
