use feo_ast::{item::TypeDef, token::Token, Type};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Identifier, Keyword, Punctuation};

use crate::{
    parse::{ParseItem, ParseType},
    parser::Parser,
    test_utils::{self, LogMsgType},
    utils,
};

impl ParseItem for TypeDef {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = utils::get_visibility(parser)?;

        let kw_type_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwType,
            ..
        }) = kw_type_opt
        {
            test_utils::log_msg(LogMsgType::Detect, "`type` keyword", parser);

            if let Some(type_name) = parser.peek_next::<Identifier>() {
                parser.next_token();

                test_utils::log_msg(LogMsgType::Detect, "type name", parser);

                if let Some(Punctuation {
                    punc_kind: PuncKind::Equals,
                    ..
                }) = parser.peek_next()
                {
                    parser.next_token();

                    test_utils::log_msg(LogMsgType::Detect, "type definition assignment", parser);

                    parser.next_token();

                    let type_opt = if let Some(ty) = Type::parse(parser)? {
                        parser.next_token();
                        Some(ty)
                    } else {
                        None
                    };

                    let semicolon_opt = parser.peek_current();

                    if let Some(Punctuation {
                        punc_kind: PuncKind::Semicolon,
                        ..
                    }) = semicolon_opt
                    {
                        test_utils::log_msg(LogMsgType::Exit, "type definition", parser);

                        return Ok(Some(TypeDef {
                            attributes_opt,
                            visibility_opt,
                            kw_type: kw_type_opt.unwrap(),
                            type_name,
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

    use super::*;

    #[test]
    fn parse_type_alias_def() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        pub type Foo = Bar;
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let type_alias_def = TypeDef::parse(&mut parser).expect("unable to parse type definition");

        Ok(println!("{:#?}", type_alias_def))
    }
}
