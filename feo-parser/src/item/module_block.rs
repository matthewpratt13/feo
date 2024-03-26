use feo_ast::{
    item::{ModuleWithBlock, ModuleWithoutBlock},
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
    test_utils::{self, LogMsgType},
    utils,
};

impl ParseItem for ModuleWithoutBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = utils::get_visibility(parser)?;

        let kw_module_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwModule,
            ..
        }) = kw_module_opt
        {
            test_utils::log_msg(LogMsgType::Detect, "`module` keyword", parser);

            if let Some(module_name) = parser.peek_next::<Identifier>() {
                parser.next_token();

                test_utils::log_msg(LogMsgType::Detect, "module name", parser);

                parser.next_token();

                let semicolon_opt = parser.peek_current();

                if let Some(Punctuation {
                    punc_kind: PuncKind::Semicolon,
                    ..
                }) = semicolon_opt
                {
                    test_utils::log_msg(LogMsgType::Exit, "module definition", parser);

                    return Ok(Some(ModuleWithoutBlock {
                        attributes_opt,
                        visibility_opt,
                        kw_module: kw_module_opt.unwrap(),
                        module_name,
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

impl ParseItem for ModuleWithBlock {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = utils::get_visibility(parser)?;

        let kw_module_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwModule,
            ..
        }) = kw_module_opt
        {
            test_utils::log_msg(LogMsgType::Detect, "`module` keyword", parser);

            if let Some(module_name) = parser.peek_next::<Identifier>() {
                parser.next_token();

                test_utils::log_msg(LogMsgType::Detect, "module name", parser);

                parser.next_token();

                let open_brace_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) = open_brace_opt
                {
                    test_utils::log_msg(LogMsgType::Enter, "module body", parser);

                    parser.next_token();

                    let items_opt = utils::get_items(parser)?;

                    let close_brace_opt = parser.peek_current();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        test_utils::log_msg(LogMsgType::Exit, "module body", parser);

                        return Ok(Some(ModuleWithBlock {
                            attributes_opt,
                            visibility_opt,
                            kw_module: kw_module_opt.unwrap(),
                            module_name,
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

    use super::*;

    #[test]
    fn parse_module_without_body() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        pub module some_mod;"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let mod_without_body =
            ModuleWithoutBlock::parse(&mut parser).expect("unable to parse module without body");

        Ok(println!("{:#?}", mod_without_body))
    }

    #[test]
    fn parse_module_with_body() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        module some_mod {
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

            module some_without_body;
        }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let mod_with_body =
            ModuleWithBlock::parse(&mut parser).expect("unable to parse module with body");

        Ok(println!("{:#?}", mod_with_body))
    }
}
