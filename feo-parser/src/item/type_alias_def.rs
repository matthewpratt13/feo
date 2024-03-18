use feo_ast::{
    item::{TypeAliasDef, TypeParamBounds, VisibilityKind},
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Identifier, Keyword, Punctuation};

use crate::{
    parse::{ParseItem, ParseTerm, ParseType},
    parser::Parser,
    utils,
};

impl ParseItem for TypeAliasDef {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = if let Some(v) = VisibilityKind::parse(parser)? {
            parser.next_token();
            Some(v)
        } else {
            None
        };

        let kw_type_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwType,
            ..
        }) = kw_type_opt
        {
            parser.next_token();

            if let Some(type_name) = parser.peek_current::<Identifier>() {
                let type_param_bounds_opt = if let Some(t) = TypeParamBounds::parse(parser)? {
                    parser.next_token();
                    Some(t)
                } else {
                    None
                };

                parser.next_token();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Equals,
                    ..
                }) = parser.peek_current::<Punctuation>()
                {
                    parser.next_token();

                    let type_opt = if let Some(ty) = Type::parse(parser)? {
                        parser.next_token();
                        Some(ty)
                    } else {
                        None
                    };

                    let semicolon_opt = parser.peek_current::<Punctuation>();

                    if let Some(Punctuation {
                        punc_kind: PuncKind::Semicolon,
                        ..
                    }) = semicolon_opt
                    {
                        return Ok(Some(TypeAliasDef {
                            attributes_opt,
                            visibility_opt,
                            kw_type: kw_type_opt.unwrap(),
                            type_name,
                            type_param_bounds_opt,
                            type_opt,
                            semicolon: semicolon_opt.unwrap(),
                        }));
                    }

                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`;`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`=`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "identifier".to_string(),
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
    fn parse_type_alias_def() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        pub type foo: u64 = Bar;
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let type_alias_def =
            TypeAliasDef::parse(&mut parser).expect("unable to parse type alias definition");

        Ok(println!("{:#?}", type_alias_def))
    }
}