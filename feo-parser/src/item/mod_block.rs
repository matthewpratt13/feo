use feo_ast::{
    item::{ModWithBody, ModWithoutBody},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    Delimiter, Identifier, Keyword, Punctuation,
};

use crate::{
    parse::ParseItem,
    parser::Parser,
    utils::{self, LogMsgType},
};

impl ParseItem for ModWithoutBody {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = utils::get_visibility(parser)?;

        let kw_mod_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwMod,
            ..
        }) = kw_mod_opt
        {
            utils::log_msg(LogMsgType::Enter, "module definition", parser);

            parser.next_token();

            if let Some(mod_name) = parser.peek_current::<Identifier>() {
                parser.next_token();

                let semicolon_opt = parser.peek_current();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Semicolon,
                    ..
                }) = semicolon_opt
                {
                    utils::log_msg(LogMsgType::Exit, "module definition", parser);

                    return Ok(Some(ModWithoutBody {
                        attributes_opt,
                        visibility_opt,
                        kw_mod: kw_mod_opt.unwrap(),
                        mod_name,
                        semicolon: semicolon_opt.unwrap(),
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`;`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
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

impl ParseItem for ModWithBody {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = utils::get_visibility(parser)?;

        let kw_mod_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwMod,
            ..
        }) = kw_mod_opt
        {
            utils::log_msg(LogMsgType::Detect, "`mod` keyword", parser);

            parser.next_token();

            if let Some(mod_name) = parser.peek_current::<Identifier>() {
                utils::log_msg(LogMsgType::Detect, "module name", parser);

                parser.next_token();

                let open_brace_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) = open_brace_opt
                {
                    utils::log_msg(LogMsgType::Enter, "mod body", parser);

                    parser.next_token();

                    let items_opt = utils::get_items(parser)?;

                    let close_brace_opt = parser.peek_current();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        utils::log_msg(LogMsgType::Exit, "module with body", parser);

                        return Ok(Some(ModWithBody {
                            attributes_opt,
                            visibility_opt,
                            kw_mod: kw_mod_opt.unwrap(),
                            mod_name,
                            open_brace: open_brace_opt.unwrap(),
                            items_opt,
                            close_brace: close_brace_opt.unwrap(),
                        }));
                    }

                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`}`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`{`".to_string(),
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
    fn parse_mod_without_body() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        pub mod some_mod;"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let mod_without_body =
            ModWithoutBody::parse(&mut parser).expect("unable to parse module without body");

        Ok(println!("{:#?}", mod_without_body))
    }

    #[test]
    fn parse_mod_with_body() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        mod some_mod {
            pub import some_module::SomeObject;

            pub const foo: u64 = 10;
            static mut bar: bool = true;

            #[abstract]
            pub enum Foo {
                Bar,
                Baz(u64),
            }

            struct Foo {
                bar: u64
            }

            pub func baz(some_param: ParamType) -> ReturnType;

            mod some_without_body;
        }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let mod_with_body =
            ModWithBody::parse(&mut parser).expect("unable to parse module with body");

        Ok(println!("{:#?}", mod_with_body))
    }
}
