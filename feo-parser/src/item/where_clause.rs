use feo_ast::{
    item::{TypeBound, TypeParamBounds, WhereClause},
    path::PathType,
    token::Token,
    ty::TraitBound,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Keyword, Punctuation};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for TypeParamBounds {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_bounds: Vec<TraitBound> = Vec::new();

        if let Some(first_bound) = PathType::parse(parser)? {
            let mut next_plus_opt = parser.peek_current::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::Plus,
                ..
            }) = next_plus_opt
            {
                parser.next_token();

                if let Some(next_bound) = PathType::parse(parser)? {
                    subsequent_bounds.push(next_bound);

                    if let Some(p) = parser.peek_current::<Punctuation>() {
                        next_plus_opt = Some(p);
                    } else {
                        break;
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`TraitBound`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                    break;
                }
            }

            let trailing_comma_opt = parser.peek_current::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = trailing_comma_opt
            {
                parser.next_token();
            }

            match &subsequent_bounds.is_empty() {
                true => Ok(Some(TypeParamBounds {
                    first_bound,
                    subsequent_bounds: None,
                    trailing_comma_opt,
                })),
                false => Ok(Some(TypeParamBounds {
                    first_bound,
                    subsequent_bounds: Some(subsequent_bounds),
                    trailing_comma_opt,
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
            let colon_opt = parser.peek_next::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = colon_opt
            {
                parser.next_token();

                if let Some(type_param_bounds) = TypeParamBounds::parse(parser)? {
                    parser.next_token();

                    return Ok(Some(TypeBound {
                        ty,
                        type_param_bounds_opt: Some(type_param_bounds),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`TypeParamBounds`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
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
                let mut next_comma_opt = parser.peek_current::<Punctuation>();

                while let Some(Punctuation {
                    punc_kind: PuncKind::Comma,
                    ..
                }) = next_comma_opt
                {
                    parser.next_token();

                    if let Some(next_bound) = TypeBound::parse(parser)? {
                        subsequent_bounds.push(next_bound);

                        if let Some(p) = parser.peek_current::<Punctuation>() {
                            next_comma_opt = Some(p);
                        } else {
                            break;
                        }
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`TypeBound`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                        break;
                    }
                }

                // `TypeBound::parse` moves the parser one token forward is `TypeBound` exists
                let trailing_type_bound_opt = TypeBound::parse(parser)?;

                match &subsequent_bounds.is_empty() {
                    true => {
                        return Ok(Some(WhereClause {
                            kw_where: kw_where_opt.unwrap(),
                            first_bound,
                            subsequent_bounds: None,
                            trailing_type_bound_opt,
                        }))
                    }
                    false => {
                        return Ok(Some(WhereClause {
                            kw_where: kw_where_opt.unwrap(),
                            first_bound,
                            subsequent_bounds: Some(subsequent_bounds),
                            trailing_type_bound_opt,
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
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_type_param_bounds() {
        let source_code = r#"Foo + Bar + Baz"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let type_param_bounds =
            TypeParamBounds::parse(&mut parser).expect("unable to parse type param bounds");

        println!("{:#?}", type_param_bounds);
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_type_bound() {
        let source_code = r#"Self: Foo + Bar + Baz"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let type_bound = TypeBound::parse(&mut parser).expect("unable to parse type bound");

        println!("{:#?}", type_bound);
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_where_clause() {
        let source_code = r#"
        where 
            Self: Foo + Bar + Baz, 
            T: Foo"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let where_clause = TypeBound::parse(&mut parser).expect("unable to parse where clause");

        println!("{:#?}", where_clause);
    }
}
