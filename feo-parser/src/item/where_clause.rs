use feo_ast::{item::TypeBound, path::PathType, token::Token, Type};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{punctuation::PuncKind, Identifier, Punctuation};

use crate::{
    parse::{ParseTerm, ParseType},
    parser::Parser,
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
}
